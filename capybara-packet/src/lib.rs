pub mod helper;
pub mod types;

use anyhow::anyhow;
use bytes::{Buf, BufMut, Bytes};
use capybara_macros::packet;
use rand::{thread_rng, Rng};
use rsa::{pkcs8::EncodePublicKey, Error, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use std::{fmt::Debug, str::FromStr};
use thiserror::Error;
use types::{Chat, RawPacket, Text};
use uuid::Uuid;

use crate::helper::parse_packet;
use helper::{PacketBool, PacketBytes, PacketEnum, PacketState, PacketString, PacketUUID};

use crate::types::VarInt;

#[macro_use]
extern crate log;

#[derive(Debug, Clone)]
pub struct Packet {
    pub lenght: i32,
    pub packetid: i32,
    pub packetdata: PacketEnum,
}

impl Packet {
    pub const fn new() -> Self {
        Self {
            lenght: 0,
            packetid: 0,
            packetdata: PacketEnum::None,
        }
    }

    pub fn parse_from_rawpacket(
        &mut self,
        state: &PacketState,
        rawpacket: &RawPacket,
    ) -> anyhow::Result<()> {
        let packet = parse_packet(rawpacket.packetid, state, &rawpacket.data)?;

        info!("{packet:?}");

        self.packetdata = packet;

        Ok(())
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
    fn id(&self) -> usize;

    fn to_response(self, packet: &Packet) -> anyhow::Result<Bytes>;
}

pub trait PacketTrait {
    /// # Errors
    /// Return if error if cannot parse packet
    fn from_bytes(bytes: &Bytes) -> anyhow::Result<Self>
    where
        Self: Sized;
}

#[derive(Debug, Clone, Default, packet)]
#[id(0x00)]
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
#[id(0x00)]
pub struct Login {
    #[string]
    pub name: String,
    #[bool]
    pub has_uuid: bool,
    #[uuid]
    pub uuid: uuid::Uuid,
}

#[derive(Debug, Clone, packet)]
#[id(0x01)]
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
        info!("token: {token:?}");
        Ok(Self {
            server_id: String::new(),
            publickey: PacketBytes(key),
            verify_token: PacketBytes(token.to_vec()),
        })
    }
}

#[derive(packet, Debug, Clone)]
#[id(0x01)]
pub struct EncryptionResponse {
    #[arraybytes]
    sharedsecret: PacketBytes,
    #[arraybytes]
    verify_token: PacketBytes,
}

impl EncryptionResponse {
    pub fn decrypt_verify_token(&self, rsa: &RsaPrivateKey) -> Result<Vec<u8>, Error> {
        rsa.decrypt(Pkcs1v15Encrypt, &self.verify_token.0)
    }

    /// # Errors
    /// Return errors if cannot decrypt from Rsa key
    pub fn decrypt_shared_secret(&self, rsa: &RsaPrivateKey) -> Result<Vec<u8>, Error> {
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

#[derive(packet)]
#[id(0x02)]
pub struct LoginSuccessPacket {
    #[uuid]
    uuid: uuid::Uuid,
    #[string]
    username: String,
    #[varint]
    length_properties: i32,
}

impl LoginSuccessPacket {
    pub fn new(username: String, uuid: Uuid) -> Self {
        Self {
            uuid,
            username,
            length_properties: 0,
        }
    }

    pub fn new_uuid_str(username: String, uuid: &str) -> Self {
        Self::new(username, uuid::Uuid::from_str(uuid).unwrap())
    }
}

#[derive(Debug, packet)]
#[id(0x00)]
pub struct DisconnectPacket {
    #[string]
    reason: String,
}

impl DisconnectPacket {
    pub fn from_reason(reason: &str) -> Self {
        let reason = Chat::SimpleText(Text::new(reason)).to_string().unwrap();

        Self { reason }
    }
}

#[derive(Debug, packet)]
#[id(0x00)]
pub struct StatusPacket {
    #[string]
    json_response: String,
}

impl Default for StatusPacket {
    fn default() -> Self {
        Self {
            json_response: String::from(
                r#"
{
    "version": {
        "name": "1.19.4",
        "protocol": 762
    },
    "players": {
        "max": 100,
        "online": 1,
        "sample": [
            {
                "name": "NoName",
                "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
            }
        ]
    },
    "description": {
        "text": "Hello world"
    },
    "enforcesSecureChat": true,
    "previewsChat": true
}
"#,
            ),
        }
    }
}

#[derive(packet, Clone, Debug)]
#[id(0x01)]
pub struct PingRequest {
    #[i64]
    pub value: i64,
}
