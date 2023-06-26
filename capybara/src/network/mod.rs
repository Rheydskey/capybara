//! Network packet handling for Minecraft server

use anyhow::Result;
use bytes::{Bytes, BytesMut};
use capybara_packet::Packet;
use capybara_packet::{helper::PacketState, types::RawPacket};
use tokio::{io::AsyncReadExt, net::TcpStream};

#[derive(Debug)]
pub struct ClientConnection {
    pub stream: TcpStream,
    buffer: BytesMut,
}

impl ClientConnection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(4096),
        }
    }

    pub async fn read(&mut self, state: &PacketState) -> Result<Option<Packet>> {
        self.buffer = BytesMut::with_capacity(4096);
        loop {
            if let Some(frame) = self.parse_frame(state)? {
                return Ok(Some(frame.clone()));
            }

            if self.stream.read_buf(&mut self.buffer).await? == 0 {
                if self.buffer.is_empty() {
                    return Ok(None);
                }

                return Err(anyhow::anyhow!("Connection reset"));
            }
        }
    }

    pub fn parse_frame(&mut self, state: &PacketState) -> Result<Option<Packet>> {
        if self.buffer.is_empty() {
            return Ok(None);
        }

        let rawpacket = RawPacket::read(&Bytes::copy_from_slice(&self.buffer[..]));

        if rawpacket.lenght <= 1 {
            return Ok(None);
        }

        let mut packet = Packet::new();

        packet.parse_from_rawpacket(state, &rawpacket);

        Ok(Some(packet))
    }
}
