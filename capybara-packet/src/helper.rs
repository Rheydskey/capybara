use crate::{EncryptionResponse, Handshake, Login, PacketError, PacketTrait};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::{io::Cursor, ops::Deref};
use uuid::Uuid;

use crate::types::VarInt;

pub fn parse_packet(packetid: i32, bytes: &Bytes) -> Result<PacketEnum, PacketError> {
    match packetid {
        0x0 => {
            if let Ok(handshake) = Handshake::from_bytes(bytes) {
                return Ok(PacketEnum::HandShake(handshake));
            }
            if let Ok(login) = Login::from_bytes(bytes) {
                return Ok(PacketEnum::Login(login));
            }

            Err(PacketError::CannotParse(-1))
        }

        0x1 => Ok(PacketEnum::EncryptionResponse(
            EncryptionResponse::from_bytes(bytes).unwrap(),
        )),

        _ => Err(PacketError::CannotParse(packetid)),
    }
}

#[derive(Debug, Default, Clone)]
pub enum PacketEnum {
    #[default]
    None,
    HandShake(Handshake),
    Login(Login),
    EncryptionResponse(EncryptionResponse),
    UnknowPacket(String),
}

#[derive(Debug, Clone)]
pub enum PacketState {
    None,
    Handshake,
    State,
    Login,
    Play,
}

pub struct PacketUUID(u128);

impl PacketUUID {
    pub fn from_cursor(bytes: &mut Cursor<&[u8]>) -> Self {
        let uuid_bytes = Bytes::copy_from_slice(&bytes.chunk()[..16]);

        bytes.advance(16);

        let mut uuid: u128 = 0;

        for (n, byte) in uuid_bytes.into_iter().enumerate() {
            uuid |= u128::from(byte) << (120 - (8 * n));
        }

        Self(uuid)
    }

    pub const fn to_uuid(&self) -> Uuid {
        uuid::Uuid::from_u128(self.0)
    }
}

pub struct PacketString {
    size: i32,
    inner: String,
}

impl PacketString {
    pub const fn new(size: i32, inner: String) -> Self {
        Self { size, inner }
    }

    pub fn from_cursor(bytes: &mut Cursor<&[u8]>) -> Option<Self> {
        let string_size = VarInt::new().read_from_cursor(bytes)?;

        let bytes_string = bytes.chunk()[..string_size.unsigned_abs() as usize].to_vec();

        bytes.advance(string_size.unsigned_abs() as usize);

        let string = String::from_utf8(bytes_string).unwrap();

        Some(Self::new(string_size, string))
    }

    pub fn to_bytes(string: String) -> Bytes {
        let mut bytes = BytesMut::new();

        bytes.put_slice(&VarInt::encode(i32::try_from(string.len()).unwrap()));
        bytes.put_slice(string.as_bytes());

        bytes.freeze()
    }
}

impl ToString for PacketString {
    fn to_string(&self) -> String {
        self.inner.clone()
    }
}

pub struct PacketBool(bool);

impl PacketBool {
    pub fn from_cursor(bytes: &mut Cursor<&[u8]>) -> Option<Self> {
        let boolean = bytes.get_u8();

        Some(Self(boolean == 0x01))
    }
}

impl Deref for PacketBool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct PacketBytes(pub Vec<u8>);

impl PacketBytes {
    pub fn from_cursor(bytes: &mut Cursor<&[u8]>) -> Option<Self> {
        let lenght = VarInt::new().read_from_cursor(bytes)?;
        let usize_lenght = usize::try_from(lenght).unwrap();
        let arraybytes = bytes.chunk()[..usize_lenght].to_vec();

        bytes.advance(usize_lenght);

        Some(Self(arraybytes))
    }
}
