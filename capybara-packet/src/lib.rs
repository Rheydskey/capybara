pub mod helper;
pub mod types;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use capybara_macros::packet;
use rand::{thread_rng, Rng};
use rsa::{pkcs8::EncodePublicKey, RsaPublicKey};
use std::{fmt::Debug, io::Cursor, sync::Arc};
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

    pub fn write(&mut self, packetenum: PacketEnum) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.append(&mut VarInt::encode(self.packetid));
        match packetenum {
            PacketEnum::None => {}
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
            }
            PacketEnum::Login(Login {
                name,
                has_uuid,
                uuid,
            }) => {
                buf.append(&mut VarInt::encode(i32::try_from(name.len()).unwrap()));
                buf.append(&mut name.as_bytes().to_vec());
                buf.push(u8::from(has_uuid));
                buf.append(&mut uuid.to_u128_le().swap_bytes().to_be_bytes().to_vec());
            }
            _ => todo!(),
        }

        buf
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
    fn from_bytes(bytes: &Bytes) -> Result<Self, PacketError>
    where
        Self: Sized;
}

#[derive(Debug, Clone, Default)]
pub struct Handshake {
    pub protocol: i32,
    pub address: String,
    pub port: u16,
    pub next_state: u8,
}

impl PacketTrait for Handshake {
    fn from_bytes(bytes: &Bytes) -> Result<Self, PacketError> {
        let mut bytes = Cursor::new(&bytes[..]);
        let protocol = VarInt::new().read_from_cursor(&mut bytes).unwrap();
        let address = PacketString::from_cursor(&mut bytes).unwrap().to_string();
        let port = ((u16::from(bytes.get_u8())) << 8) | u16::from(bytes.get_u8());
        let next_state = bytes.get_u8();

        Ok(Self {
            protocol,
            address,
            port,
            next_state,
        })
    }
}
#[derive(Debug, Clone)]
pub struct Login {
    pub name: String,
    pub has_uuid: bool,
    pub uuid: uuid::Uuid,
}

impl PacketTrait for Login {
    fn from_bytes(bytes: &Bytes) -> Result<Self, PacketError> {
        let mut bytes = Cursor::new(&bytes[..]);
        let name = PacketString::from_cursor(&mut bytes).unwrap().to_string();

        let has_uuid = *PacketBool::from_cursor(&mut bytes).unwrap();
        let uuid = PacketUUID::from_cursor(&mut bytes).to_uuid();

        Ok(Self {
            name,
            has_uuid,
            uuid,
        })
    }
}

pub struct EncryptionRequest {
    pub server_id: String,
    pub publickey_len: usize,
    pub publickey: Vec<u8>,
    pub verify_token_lenght: usize,
    pub verify_token: Vec<u8>,
}

impl EncryptionRequest {
    #[must_use]
    pub fn new(rsa: &RsaPublicKey) -> Self {
        let key = rsa.to_public_key_der().unwrap().to_vec();
        let mut rng = thread_rng();
        let mut token = [0; 4];
        rng.fill(&mut token[..]);
        Self {
            server_id: String::new(),
            publickey_len: key.len(),
            publickey: key,
            verify_token: token.to_vec(),
            verify_token_lenght: token.len(),
        }
    }
}

impl IntoResponse for EncryptionRequest {
    fn to_response(self, state: &Arc<State>, packet: &Packet) -> Bytes {
        let mut bytes = BytesMut::new();

        bytes.put(PacketString::to_bytes(self.server_id));

        bytes.put_slice(&VarInt::encode(i32::try_from(self.publickey_len).unwrap()));

        bytes.put_slice(&self.publickey);

        bytes.put_slice(&VarInt::encode(
            i32::try_from(self.verify_token_lenght).unwrap(),
        ));

        bytes.put_slice(&self.verify_token);

        bytes.freeze()
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
    pub fn get_shared_key_lenght(&self) -> usize {
        self.sharedsecret.0.len()
    }

    pub fn get_verify_token_lenght(&self) -> usize {
        self.verify_token.0.len()
    }
}
