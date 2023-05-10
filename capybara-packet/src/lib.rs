pub mod helper;
pub mod types;

use anyhow::anyhow;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use capybara_macros::packet;
use rand::{thread_rng, Rng};
use rsa::{pkcs8::EncodePublicKey, Error, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use std::{fmt::Debug, sync::Arc};
use thiserror::Error;
use types::RawPacket;

use crate::{helper::parse_packet, types::State};
use helper::{PacketBool, PacketBytes, PacketEnum, PacketState, PacketString, PacketUUID};

use crate::types::VarInt;

#[macro_use]
extern crate log;

#[derive(Debug)]
pub struct Packet {
    pub lenght: i32,
    pub packetid: i32,
    pub packetstate: PacketState,
    pub packetdata: PacketEnum,
}

impl Packet {
    pub const fn new() -> Self {
        Self {
            lenght: 0,
            packetid: 0,
            packetstate: PacketState::None,
            packetdata: PacketEnum::None,
        }
    }

    pub const fn new_from_state(state: PacketState) -> Self {
        Self {
            lenght: 0,
            packetid: 0,
            packetstate: state,
            packetdata: PacketEnum::None,
        }
    }

    pub fn parse_from_rawpacket(&mut self, rawpacket: &RawPacket) {
        let packet =
            parse_packet(rawpacket.packetid, &rawpacket.data, &mut self.packetstate).unwrap();

        info!("{packet:?}");

        self.packetdata = packet;
    }

    /// # Errors
    /// Return error if cannot encode varint
    pub fn write(&mut self, packetenum: PacketEnum) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();

        buf.append(&mut VarInt::encode(self.packetid));
        match packetenum {
            PacketEnum::None => Ok(()),
            PacketEnum::HandShake(Handshake {
                protocol,
                address,
                port,
                next_state,
            }) => {
                buf.append(&mut VarInt::encode(protocol));
                buf.append(&mut address.as_bytes().to_vec());
                buf.append(&mut port.to_be_bytes().to_vec());
                buf.push(next_state);

                Ok(())
            }
            PacketEnum::Login(Login {
                name,
                has_uuid,
                uuid,
            }) => {
                buf.append(&mut VarInt::encode(i32::try_from(name.len())?));
                buf.append(&mut name.as_bytes().to_vec());
                buf.push(u8::from(has_uuid));
                buf.append(&mut uuid.to_u128_le().swap_bytes().to_be_bytes().to_vec());

                Ok(())
            }
            _ => Err(anyhow!("Cannot write")),
        }?;

        Ok(buf)
    }
}

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Bad Packet : CannotParse({0})")]
    CannotParse(i32),
    #[error("Unknow error")]
    Unknow,
}

pub trait IntoResponse {
    fn to_response(self, state: &Arc<State>, packet: &Packet) -> Bytes;
}

pub trait PacketTrait {
    /// # Errors
    /// Return if error if cannot parse packet
    fn from_bytes(bytes: &Bytes) -> Result<Self, PacketError>
    where
        Self: Sized;
}

#[derive(Debug, Clone, Default, packet)]
pub struct Handshake {
    #[varint]
    pub protocol: i32,
    #[string]
    pub address: String,
    #[u16]
    pub port: u16,
    #[u8]
    pub next_state: u8,
}

#[derive(Debug, Clone, packet)]
pub struct Login {
    #[string]
    pub name: String,
    #[bool]
    pub has_uuid: bool,
    #[uuid]
    pub uuid: uuid::Uuid,
}

#[derive(Debug, Clone, packet)]
pub struct EncryptionRequest {
    #[string]
    pub server_id: String,
    #[arraybytes]
    pub publickey: PacketBytes,
    #[arraybytes]
    pub verify_token: PacketBytes,
}

impl EncryptionRequest {
    /// # Errors
    /// Return error if cannot convert public key to DER format
    pub fn new(rsa: &RsaPublicKey) -> Result<Self, rsa::pkcs8::Error> {
        let key = rsa.to_public_key_der()?.to_vec();
        let mut rng = thread_rng();
        let mut token = [0; 4];
        rng.fill(&mut token[..]);
        println!("{token:?}");
        Ok(Self {
            server_id: String::new(),
            publickey: PacketBytes(key),
            verify_token: PacketBytes(token.to_vec()),
        })
    }
}

#[derive(packet, Debug, Clone)]
pub struct EncryptionResponse {
    #[arraybytes]
    sharedsecret: PacketBytes,
    #[arraybytes]
    verify_token: PacketBytes,
}

impl EncryptionResponse {
    /// # Errors
    /// Return errors if cannot decrypt from Rsa key
    pub fn decrypt(&self, rsa: &RsaPrivateKey) -> Result<Vec<u8>, Error> {
        rsa.decrypt(Pkcs1v15Encrypt, self.get_shared_secret())
    }

    #[must_use]
    pub const fn get_shared_secret(&self) -> &Vec<u8> {
        &self.sharedsecret.0
    }

    #[must_use]
    pub fn get_shared_secret_lenght(&self) -> usize {
        self.sharedsecret.0.len()
    }

    #[must_use]
    pub fn get_verify_token_lenght(&self) -> usize {
        self.verify_token.0.len()
    }
}
