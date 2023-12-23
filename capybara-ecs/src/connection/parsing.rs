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
        stream: &TcpStream,
        encryption: EncryptionState,
        compression: CompressionState,
    ) -> anyhow::Result<Self> {
        let thread_pool = AsyncComputeTaskPool::get();
        let shared_state = SharedConnectionState::new(encryption, compression);
        let (to_send_sender, to_send_receiver) = flume::unbounded();

        let (new_packet_sender, new_packet_receiver) = flume::unbounded();

        let reader = Reader::new(new_packet_sender, stream.try_clone()?, shared_state.clone());
        let writer = Writer::new(to_send_receiver, stream.try_clone()?, shared_state.clone());

        let recv_task = thread_pool.spawn(async move { reader.run() });
        let send_task = thread_pool.spawn(async move { writer.run().await });

        Ok(Self(
            to_send_sender,
            new_packet_receiver,
            recv_task,
            send_task,
            shared_state,
        ))
    }

    pub fn send_packet_serialize(
        &self,
        packet: &(impl Serialize + Id + std::fmt::Debug),
    ) -> anyhow::Result<()> {
        let rawpacket = RawPacket::build_from_serialize(packet)?;
        info!("{:?}", rawpacket);
        self.0.send(rawpacket.data)?;
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

    pub fn send_bytes(&mut self, bytes: &Bytes) -> anyhow::Result<()> {
        let mut bytes = bytes.to_vec();

        self.shared_state
            .get_encryptionlayer()
            .encrypt(bytes.as_mut_slice());

        self.tcp_stream.write_all(bytes.as_slice())?;
        Ok(())
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        while let Ok(to_send) = self.to_send.recv_async().await {
            self.send_bytes(&to_send)?;
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

    pub fn decrypt(&self, decrypt: &mut [u8]) {
        self.shared_state.get_encryptionlayer().decrypt(decrypt)
    }

    pub fn read_u8(&mut self) -> anyhow::Result<u8> {
        let mut buf = [0; 1];
        self.tcp_stream.read_exact(&mut buf)?;

        self.decrypt(&mut buf);

        println!("Added {:?}", self.packet);
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
        let Some(lenght) = self.packet.0 else {
            return Err(anyhow!("No lenght"));
        };
        let packet =
            RawPacket::read_lenght_given(&self.packet.1.clone().freeze(), i32::try_from(lenght)?)?;

        self.packet.0 = None;
        self.packet.1.clear();

        println!("NEW PACKET: {:?}", packet);

        Ok(packet)
    }

    pub fn run(mut self) -> anyhow::Result<()> {
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

            if let (Some(length), Ok(bytes)) = (self.packet.0, self.read_u8()) {
                let mut packetbytes = BytesMut::new();
                packetbytes.put_slice(&VarInt::encode(length as i32).unwrap());
                packetbytes.put_slice(&self.packet.1);
                println!("{:?}", &packetbytes);

                if self.packet.1.len() != length {
                    self.packet.1.put_u8(bytes);
                }

                if self.packet.1.len() == length {
                    let packet = self.try_parse_packet()?;

                    self.new_packet.send(packet)?;
                }

                continue;
            }

            return Err(anyhow!(""));
        }
    }
}
