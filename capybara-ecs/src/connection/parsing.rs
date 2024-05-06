use anyhow::anyhow;
use bevy_ecs::prelude::Component;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use bytes::{BufMut, Bytes, BytesMut};
use capybara_packet::{
    capybara_packet_parser::{Parsable, VarInt},
    types::RawPacket,
    Id,
};

use serde::Serialize;
use std::io::Write;
use std::{io::Read, net::TcpStream};

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
pub struct ParseTask {
    sender: flume::Sender<Bytes>,
    receiver: flume::Receiver<RawPacket>,
    reader: Task<anyhow::Result<()>>,
    writer: Task<anyhow::Result<()>>,
}

impl ParseTask {
    pub fn new(
        stream: &TcpStream,
        encryption: EncryptionState,
        compression: CompressionState,
    ) -> anyhow::Result<Self> {
        let thread_pool = AsyncComputeTaskPool::get();
        let shared_state = SharedConnectionState::new(encryption, compression);
        let (sender, to_send_receiver) = flume::unbounded();

        let (new_packet_sender, receiver) = flume::unbounded();

        let reader = Reader::new(new_packet_sender, stream.try_clone()?, shared_state.clone());
        let writer = Writer::new(to_send_receiver, stream.try_clone()?, shared_state);

        let reader = thread_pool.spawn(async move { reader.run() });
        let writer = thread_pool.spawn(async move { writer.run().await });

        Ok(Self {
            sender,
            receiver,
            reader,
            writer,
        })
    }

    pub fn send_packet_serialize<P>(&self, packet: &P) -> anyhow::Result<()>
    where
        P: Serialize + Id + std::fmt::Debug,
    {
        let rawpacket = RawPacket::build_from_serialize(packet)?;
        info!("{:?}", rawpacket);
        self.sender.send(rawpacket.data)?;
        Ok(())
    }

    pub fn get_packet(&self) -> Vec<RawPacket> {
        self.receiver.drain().collect::<Vec<RawPacket>>()
    }

    #[inline]
    pub fn is_finished(&self) -> bool {
        self.reader.is_finished() || self.writer.is_finished()
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

    pub fn send_bytes(&mut self, bytes: &Bytes) -> anyhow::Result<()> {
        let mut bytes = bytes.to_vec();

        self.shared_state.get_encryptionlayer().encrypt(&mut bytes);

        self.tcp_stream.write_all(&bytes)?;
        Ok(())
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        while let Ok(to_send) = self.to_send.recv_async().await {
            self.send_bytes(&to_send)?;
        }

        Err(anyhow!(""))
    }
}

#[derive(Clone)]
pub struct BufPacket {
    length: Option<usize>,
    data: BytesMut,
}

impl BufPacket {
    pub fn new() -> Self {
        Self {
            length: None,
            data: BytesMut::new(),
        }
    }

    pub const fn length(&self) -> Option<usize> {
        self.length
    }

    pub fn length_i32(&self) -> Option<i32> {
        self.length.and_then(|f| i32::try_from(f).ok())
    }

    pub const fn has_length(&self) -> bool {
        self.length.is_some()
    }

    pub fn set_length(&mut self, length: i32) {
        self.length = Some(length.unsigned_abs() as usize);
        self.data.reserve(self.length.unwrap());
    }

    pub fn data_length(&self) -> usize {
        self.data.len()
    }

    pub fn freeze(self) -> Bytes {
        self.data.freeze()
    }
}

pub struct Reader {
    new_packet: flume::Sender<RawPacket>,
    tcp_stream: TcpStream,
    packet: BufPacket,
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
            packet: BufPacket::new(),
        }
    }

    pub fn decrypt(&self, decrypt: &mut [u8]) {
        self.shared_state.get_encryptionlayer().decrypt(decrypt)
    }

    pub fn read_u8(&mut self) -> anyhow::Result<u8> {
        let mut buf = [0; 1];
        self.tcp_stream.read_exact(&mut buf)?;

        self.decrypt(&mut buf);

        Ok(buf[0])
    }

    pub fn read_varint(&mut self) -> anyhow::Result<i32> {
        let mut buf = Vec::new();
        while let Ok(byte) = self.read_u8() {
            buf.push(byte);
            if let Ok(value) = VarInt::parse(&mut buf.as_slice()) {
                return Ok(value);
            }
        }

        Err(anyhow::anyhow!("Weird error"))
    }

    pub fn try_parse_packet(&mut self) -> anyhow::Result<RawPacket> {
        let Some(length) = self.packet.length_i32() else {
            return Err(anyhow!("No length"));
        };

        let data = self.packet.clone().freeze();

        let packet = RawPacket::read_length_given(&data, length)?;

        self.packet = BufPacket::new();

        println!("NEW PACKET: {:?}", packet);

        Ok(packet)
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        loop {
            if !self.packet.has_length() {
                if let Ok(lenght) = self.read_varint() {
                    if lenght <= 1 {
                        continue;
                    }

                    self.packet.set_length(lenght);
                }
            }

            if let (Some(length), Ok(bytes)) = (self.packet.length(), self.read_u8()) {
                let mut packetbytes = BytesMut::new();
                packetbytes.put_slice(&VarInt::encode(length as i32).unwrap());
                packetbytes.put_slice(&self.packet.data);

                if self.packet.data_length() != length {
                    self.packet.data.put_u8(bytes);
                }

                if self.packet.data_length() == length {
                    let packet = self.try_parse_packet()?;

                    self.new_packet.send(packet)?;
                }

                continue;
            }

            return Err(anyhow!(""));
        }
    }
}
