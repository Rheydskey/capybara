use anyhow::anyhow;
use bevy::{
    prelude::Component,
    tasks::{AsyncComputeTaskPool, Task},
};
use bytes::{BufMut, Bytes, BytesMut};
use capybara_packet::types::{RawPacket, VarInt};
use std::{io::Read, net::TcpStream};
use std::{io::Write, sync::Arc};

use crate::player::{CompressionState, EncryptionState};

#[derive(Debug, Clone)]
pub struct SharedConnectionState(EncryptionState, CompressionState);

impl SharedConnectionState {
    pub fn new(encryption_state: EncryptionState, compression_state: CompressionState) -> Self {
        Self(encryption_state, compression_state)
    }

    pub fn get_encryptionlayer(&self) -> EncryptionState {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct CompressionLayer;

#[derive(Component)]
pub struct ParseTask(
    flume::Sender<Bytes>,
    flume::Receiver<RawPacket>,
    Task<anyhow::Result<()>>,
    Task<anyhow::Result<()>>,
    SharedConnectionState,
);

impl ParseTask {
    pub fn new(
        stream: Arc<TcpStream>,
        encryption: EncryptionState,
        compression: CompressionState,
    ) -> Self {
        let thread_pool = AsyncComputeTaskPool::get();
        let shared_state = SharedConnectionState::new(encryption, compression);
        let (to_send_sender, to_send_receiver) = flume::unbounded();

        let (new_packet_sender, new_packet_receiver) = flume::unbounded();

        let reader = Reader::new(
            new_packet_sender,
            stream.try_clone().unwrap(),
            shared_state.clone(),
        );
        let writer = Writer::new(
            to_send_receiver,
            stream.try_clone().unwrap(),
            shared_state.clone(),
        );

        let recv_task = thread_pool.spawn(async move { reader.run().await });
        let send_task = thread_pool.spawn(async move { writer.run().await });

        Self(
            to_send_sender,
            new_packet_receiver,
            recv_task,
            send_task,
            shared_state,
        )
    }

    pub fn send_packet(&self, packet: Bytes) -> anyhow::Result<()> {
        self.0.send(packet)?;

        Ok(())
    }

    pub fn get_packet(&self) -> Vec<RawPacket> {
        self.1.drain().collect::<Vec<RawPacket>>()
    }

    #[inline]
    pub fn is_finished(&self) -> bool {
        self.2.is_finished() || self.3.is_finished()
    }
}

pub struct Writer {
    to_send: flume::Receiver<Bytes>,
    tcp_stream: TcpStream,
    shared_state: SharedConnectionState,
}

impl Writer {
    pub fn new(
        to_send: flume::Receiver<Bytes>,
        tcp_stream: TcpStream,
        shared_state: SharedConnectionState,
    ) -> Self {
        Self {
            to_send,
            tcp_stream,
            shared_state,
        }
    }

    pub fn send_bytes(&mut self, bytes: Bytes) -> anyhow::Result<()> {
        let mut bytes = bytes.to_vec();

        self.shared_state
            .get_encryptionlayer()
            .encrypt(bytes.as_mut_slice());

        self.tcp_stream.write_all(bytes.as_slice())?;
        info!("Sended: {bytes:?}");
        Ok(())
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        while let Ok(to_send) = self.to_send.recv_async().await {
            info!("Send {to_send:?}");
            self.send_bytes(to_send)?;
        }

        Err(anyhow!(""))
    }
}

pub struct Reader {
    new_packet: flume::Sender<RawPacket>,
    tcp_stream: TcpStream,
    packet: (Option<usize>, BytesMut),
    shared_state: SharedConnectionState,
}

impl Reader {
    pub fn new(
        new_packet: flume::Sender<RawPacket>,
        tcp_stream: TcpStream,
        shared_state: SharedConnectionState,
    ) -> Self {
        Self {
            new_packet,
            tcp_stream,
            shared_state,
            packet: (None, BytesMut::new()),
        }
    }

    pub fn read_u8(&mut self) -> anyhow::Result<u8> {
        let mut buf = [0; 1];
        self.tcp_stream.read_exact(&mut buf)?;

        self.shared_state.get_encryptionlayer().decrypt(&mut buf);

        Ok(buf[0])
    }

    pub fn read_varint(&mut self) -> anyhow::Result<i32> {
        let mut varint = VarInt::new();

        while let Ok(byte) = self.read_u8() {
            info!("{:?}", self.packet);
            let value = varint.try_with(byte)?;

            if let Some(value) = value {
                return Ok(value);
            }
        }

        Err(anyhow::anyhow!("Weird error"))
    }

    pub fn try_parse_packet(&mut self) -> anyhow::Result<RawPacket> {
        let Some(lenght) = self.packet.0 else {
            return Err(anyhow!("No lenght"));
        };

        let packet = RawPacket::read_lenght_given(&self.packet.1.clone().freeze(), lenght as i32)?;

        self.packet.0 = None;
        self.packet.1.clear();

        Ok(packet)
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        loop {
            if self.packet.0.is_none() {
                if let Ok(lenght) = self.read_varint() {
                    if lenght <= 1 {
                        continue;
                    }
                    self.packet.0 = Some(lenght.unsigned_abs() as usize);
                    self.packet.1.reserve(lenght.unsigned_abs() as usize);
                }
            }

            if let (Some(lenght), Ok(bytes)) = (self.packet.0, self.read_u8()) {
                if self.packet.1.len() != lenght {
                    self.packet.1.put_u8(bytes)
                }

                if self.packet.1.len() == lenght {
                    if let Ok(rawpacket) = self.try_parse_packet() {
                        self.new_packet.send(rawpacket).unwrap();
                        info!("New packet sended");
                    } else {
                        return Err(anyhow!(""));
                    }
                }

                continue;
            }

            return Err(anyhow!(""));
        }
    }
}
