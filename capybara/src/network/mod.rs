//! Network packet handling for Minecraft server
//!

//pub mod packet;

use capybara_packet::helper::PacketState;
use capybara_packet::types::State;
use capybara_packet::EncryptionRequest;
use capybara_packet::IntoResponse;
use std::sync::Arc;

use anyhow::Result;
use bytes::{Bytes, BytesMut};
use capybara_packet::helper::PacketEnum;
use capybara_packet::types::RawPacket;
use capybara_packet::Packet;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[derive(Debug)]
pub struct ClientConnection {
    gstate: Arc<State>,
    stream: TcpStream,
    buffer: BytesMut,
    state: PacketState,
    last_packet: Packet,
}

impl ClientConnection {
    pub fn new(stream: TcpStream, state: Arc<State>) -> Self {
        Self {
            gstate: state.clone(),
            stream,
            state: PacketState::None,
            buffer: BytesMut::with_capacity(4096),
            last_packet: Packet::new(),
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Packet>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            if self.stream.read_buf(&mut self.buffer).await? == 0 {
                if self.buffer.is_empty() {
                    return Ok(None);
                }

                return Err(anyhow::anyhow!("Connection reset"));
            }
        }
    }

    pub fn parse_frame(&mut self) -> Result<Option<Packet>> {
        if self.buffer.is_empty() {
            return Ok(None);
        }

        let rawpacket = RawPacket::read(&Bytes::copy_from_slice(&self.buffer[..]));

        if rawpacket.lenght <= 1 {
            return Ok(None);
        }

        let mut packet = Packet::new_from_state(self.state.clone());

        packet.parse_from_rawpacket(&rawpacket);

        self.state = packet.packetstate.clone();

        Ok(Some(packet))
    }

    pub async fn handle(&mut self) {
        while let Ok(Some(frame)) = self.read_frame().await {
            handler(&self.gstate.clone(), &frame, &mut self.stream).await;
            self.send(frame);

            self.buffer = BytesMut::with_capacity(4096);
        }
    }

    pub fn send(&mut self, packetenum: Packet) {
        let mut packet = Packet::new();

        //packet.write(packet.packetdata);
    }
}

pub async fn handler(state: &Arc<State>, packet: &Packet, stream: &mut TcpStream) {
    match packet.packetdata {
        PacketEnum::None => todo!(),
        PacketEnum::HandShake(_) => {}
        PacketEnum::Login(_) => {
            let a = EncryptionRequest::new(&state.rsa.to_public_key()).to_response(state, packet);
            let mut packet = RawPacket::from_bytes(&a, 0x01);
            stream.write_all_buf(&mut packet.data).await;
        }
        PacketEnum::UnknowPacket(_) => todo!(),
        PacketEnum::EncryptionResponse(_) => {}
        _ => todo!(),
    }
}
