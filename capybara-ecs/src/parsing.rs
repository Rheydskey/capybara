use bevy::{
    prelude::Component,
    tasks::{AsyncComputeTaskPool, Task},
};
use bytes::{BufMut, Bytes, BytesMut};
use capybara_packet::types::{RawPacket, VarInt};
use std::{io::Read, net::TcpStream};
use std::{io::Write, sync::Arc};

#[derive(Component)]
pub struct ParseTask(
    flume::Sender<Bytes>,
    flume::Receiver<RawPacket>,
    Task<()>,
    Task<()>,
);

impl ParseTask {
    pub fn new(stream: Arc<TcpStream>) -> Self {
        let thread_pool = AsyncComputeTaskPool::get();

        let (to_send_sender, to_send_receiver) = flume::unbounded();
        let (new_packet_sender, new_packet_receiver) = flume::unbounded();

        let reader = Reader::new(new_packet_sender, stream.try_clone().unwrap());
        let writer = Writer::new(to_send_receiver, stream.try_clone().unwrap());

        let recv_task = thread_pool.spawn(async move { reader.run().await });
        let send_task = thread_pool.spawn(async move { writer.run().await });

        Self(to_send_sender, new_packet_receiver, recv_task, send_task)
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

impl Iterator for ParseTask {
    type Item = RawPacket;

    fn next(&mut self) -> Option<Self::Item> {
        self.1.iter().next()
    }
}

impl Iterator for &ParseTask {
    type Item = RawPacket;

    fn next(&mut self) -> Option<Self::Item> {
        self.1.iter().next()
    }
}

pub struct Writer {
    to_send: flume::Receiver<Bytes>,
    tcp_stream: TcpStream,
}

impl Writer {
    pub fn new(to_send: flume::Receiver<Bytes>, tcp_stream: TcpStream) -> Self {
        Self {
            to_send,
            tcp_stream,
        }
    }

    pub fn send_bytes(&mut self, bytes: Bytes) -> anyhow::Result<()> {
        self.tcp_stream.write_all(bytes.to_vec().as_slice())?;

        Ok(())
    }

    pub async fn run(mut self) {
        while let Ok(to_send) = self.to_send.recv() {
            info!("Send {to_send:?}");
            self.send_bytes(to_send).unwrap();
        }
    }
}

pub struct Reader {
    new_packet: flume::Sender<RawPacket>,
    tcp_stream: TcpStream,
}

impl Reader {
    pub fn new(new_packet: flume::Sender<RawPacket>, tcp_stream: TcpStream) -> Self {
        Self {
            new_packet,
            tcp_stream,
        }
    }

    pub fn read_u8(&mut self) -> anyhow::Result<u8> {
        let mut buf = [0; 1];
        self.tcp_stream.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    pub fn read_varint(&mut self) -> anyhow::Result<i32> {
        let mut varint = VarInt::new();

        while let Ok(byte) = self.read_u8() {
            let value = varint.try_with(byte)?;

            if let Some(value) = value {
                return Ok(value);
            }
        }

        Err(anyhow::anyhow!("Weird error"))
    }

    pub async fn run(mut self) {
        println!("Spawn");
        let mut packet_lenght = None;
        let mut packet_bytes = BytesMut::new();

        loop {
            if packet_lenght.is_none() {
                if let Ok(lenght) = self.read_varint() {
                    if lenght <= 1 {
                        continue;
                    }
                    packet_lenght = Some(lenght.unsigned_abs() as usize);
                    packet_bytes.reserve(lenght.unsigned_abs() as usize);
                }
            }

            if let (Some(lenght), Ok(bytes)) = (packet_lenght, self.read_u8()) {
                if packet_bytes.len() != lenght {
                    packet_bytes.put_u8(bytes)
                }

                if packet_bytes.len() == lenght {
                    match RawPacket::read_lenght_given(
                        &packet_bytes.clone().freeze(),
                        lenght as i32,
                    ) {
                        Ok(value) => {
                            self.new_packet.send(value).unwrap();
                            packet_lenght = None;
                            packet_bytes.clear();
                        }
                        Err(error) => {
                            error!("{}", error);
                            return;
                        }
                    };
                }

                continue;
            }

            return;
        }
    }
}
