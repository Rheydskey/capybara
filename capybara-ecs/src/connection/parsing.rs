use std::sync::{atomic::AtomicBool, Arc};

use anyhow::anyhow;
use bevy_ecs::prelude::Component;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use capybara_packet::{
    capybara_packet_parser::{Parsable, VarInt},
    types::RawPacket,
    Id,
};
use parking_lot::RwLock;
use serde::Serialize;
use smol::{
    io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    net::TcpStream,
};

use crate::connection::{CompressionState, EncryptionState};

#[derive(Debug, Clone)]
pub struct SharedConnectionState(EncryptionState, CompressionState);

impl SharedConnectionState {
    pub const fn new(
        encryption_state: EncryptionState,
        compression_state: CompressionState,
    ) -> Self {
        Self(encryption_state, compression_state)
    }

    pub fn get_encryptionlayer(&self) -> EncryptionState {
        self.0.clone()
    }
}

#[derive(Component)]
pub struct NetworkTask {
    sender: flume::Sender<Bytes>,
    receiver: flume::Receiver<RawPacket>,
    reader: Task<anyhow::Result<()>>,
    writer: Task<anyhow::Result<()>>,
    parse_task: Task<anyhow::Result<()>>,
}

impl NetworkTask {
    pub fn new(
        stream: std::net::TcpStream,
        encryption: EncryptionState,
        compression: CompressionState,
    ) -> anyhow::Result<Self> {
        let thread_pool = AsyncComputeTaskPool::get();
        let shared_state = SharedConnectionState::new(encryption, compression);
        let (sender, to_send_receiver) = flume::unbounded();

        let (new_packet_sender, receiver) = flume::unbounded();
        let stream = TcpStream::try_from(stream)?;

        let (read_half, write_half) = smol::io::split(stream);

        let shared_buffer = SharedBuffer::new(RwLock::new(BytesMut::new()));

        let is_close = Arc::new(AtomicBool::new(false));
        let reader = Reader::new(shared_buffer.clone(), read_half, shared_state.clone());
        let parse_task = ParseTask::new(shared_buffer, new_packet_sender, is_close.clone());
        let writer = Writer::new(to_send_receiver, write_half, shared_state);

        let parse_task = thread_pool.spawn(async move { parse_task.run().await });
        let reader = thread_pool.spawn(async move {
            let close = reader.run().await;
            is_close.store(true, std::sync::atomic::Ordering::SeqCst);
            close
        });
        let writer = thread_pool.spawn(async move { writer.run().await });

        Ok(Self {
            sender,
            receiver,
            reader,
            writer,
            parse_task,
        })
    }

    pub fn send_packet_serialize<P>(&self, packet: &P) -> anyhow::Result<()>
    where
        P: Serialize + Id + std::fmt::Debug,
    {
        let rawpacket = RawPacket::build_from_serialize(packet)?;
        info!("{:?} => {:?}", packet, rawpacket);
        self.sender.send(rawpacket.data)?;
        Ok(())
    }

    pub fn get_packet(&self) -> Vec<RawPacket> {
        self.receiver.drain().collect::<Vec<RawPacket>>()
    }

    #[inline]
    pub fn is_finished(&self) -> bool {
        self.parse_task.is_finished() || self.reader.is_finished() || self.writer.is_finished()
    }
}

pub struct Writer {
    to_send: flume::Receiver<Bytes>,
    tcp_stream: WriteHalf<TcpStream>,
    shared_state: SharedConnectionState,
}

impl Writer {
    pub fn new(
        to_send: flume::Receiver<Bytes>,
        tcp_stream: WriteHalf<TcpStream>,
        shared_state: SharedConnectionState,
    ) -> Self {
        Self {
            to_send,
            tcp_stream,
            shared_state,
        }
    }

    pub async fn send_bytes(&mut self, bytes: &Bytes) -> anyhow::Result<()> {
        let mut bytes = bytes.to_vec();
        self.shared_state.get_encryptionlayer().encrypt(&mut bytes);
        self.tcp_stream.write_all(&bytes).await.unwrap();
        Ok(())
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        while let Ok(to_send) = self.to_send.recv_async().await {
            self.send_bytes(&to_send).await?;
        }

        Err(anyhow!(""))
    }
}

type SharedBuffer = Arc<RwLock<BytesMut>>;

pub struct ParseTask {
    buffer: SharedBuffer,
    new_packet: flume::Sender<RawPacket>,
    is_closed: Arc<AtomicBool>,
}

impl ParseTask {
    pub fn new(
        buffer: SharedBuffer,
        new_packet: flume::Sender<RawPacket>,
        is_closed: Arc<AtomicBool>,
    ) -> Self {
        Self {
            buffer,
            new_packet,
            is_closed,
        }
    }

    pub fn try_next_packet(&mut self) -> anyhow::Result<Option<RawPacket>> {
        let mut buf = self.buffer.write();
        let buffer = &mut &buf[..];

        let Ok(length) = VarInt::parse(buffer) else {
            return Ok(None);
        };

        // info!("{length:?}");

        if buffer.len() < length as usize {
            return Ok(None);
        }

        // Calculate number of bytes read for length of packet
        let cnt = buf.len() - buffer.len();

        // Advance of this size
        buf.advance(cnt);

        let data = buf.split_to(length as usize).freeze();

        let packet = RawPacket::read_length_given(data.as_ref(), length)?;
        warn!("NEW PACKET: {:X?}", packet);
        Ok(Some(packet))
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        loop {
            if self.is_closed.load(std::sync::atomic::Ordering::Relaxed) {
                return Ok(());
            }

            if let Some(packet) = self.try_next_packet().unwrap() {
                self.new_packet.send(packet)?;
            }
        }
    }
}

pub struct Reader {
    tcp_stream: ReadHalf<TcpStream>,
    buffer: SharedBuffer,
    shared_state: SharedConnectionState,
    is_encrypted: bool,
}

impl Reader {
    pub fn new(
        buffer: SharedBuffer,
        tcp_stream: ReadHalf<TcpStream>,
        shared_state: SharedConnectionState,
    ) -> Self {
        Self {
            tcp_stream,
            shared_state,
            buffer,
            is_encrypted: false,
        }
    }

    pub fn decrypt(&self, decrypt: &mut [u8]) {
        let encryption_state = self.shared_state.get_encryptionlayer();
        if encryption_state.0.read().is_none() {
            return;
        }

        info!("FROM {:?}", decrypt);
        _ = encryption_state.decrypt(decrypt);
        info!("TO: {:?}", decrypt);
    }

    pub async fn read_buffer(&mut self) -> anyhow::Result<()> {
        let mut buffer = BytesMut::zeroed(4096);

        let read = self.tcp_stream.read(&mut buffer).await?;

        if read == 0 {
            return Err(anyhow!("End of Stream"));
        }

        self.decrypt(&mut buffer[..read]);

        self.buffer.write().put_slice(&buffer[..read]);

        Ok(())
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        loop {
            if !self.is_encrypted {
                let mut buf = self.buffer.write();
                self.decrypt(&mut buf);
                self.is_encrypted = true;
            }

            self.read_buffer().await?;
        }
    }
}
