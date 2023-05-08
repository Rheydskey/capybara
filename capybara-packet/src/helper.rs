use crate::{EncryptionResponse, Handshake, Login, PacketError, PacketTrait};
use bytes::{BufMut, Bytes, BytesMut};
use std::ops::Deref;
use uuid::Uuid;

use crate::types::VarInt;

pub fn parse_packet(
    packetid: i32,
    bytes: &Bytes,
    state: &mut PacketState,
) -> Result<PacketEnum, PacketError> {
    match packetid {
        0x0 => {
            if matches!(state, PacketState::None) {
                let handshake = Handshake::from_bytes(bytes)?;
                *state = PacketState::Handshake;
                return Ok(PacketEnum::HandShake(handshake));
            }

            if matches!(state, PacketState::Handshake) {
                let handshake = Login::from_bytes(bytes)?;
                return Ok(PacketEnum::Login(handshake));
            }

            Err(PacketError::CannotParse(-1))
        }

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
    pub fn from_iterator<'a, T>(bytes: &mut T) -> Self
    where
        T: Iterator<Item = &'a u8>,
    {
        let bytes = bytes.take(16).copied().collect::<Vec<u8>>();
        let mut uuid: u128 = 0;

        for (n, byte) in bytes.iter().enumerate() {
            uuid |= u128::from(*byte) << (120 - (8 * n));
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

    pub fn from_iterator<'a, T>(bytes: &mut T) -> Option<Self>
    where
        T: Iterator<Item = &'a u8>,
    {
        let string_size = VarInt::new().read_from_iter(bytes)?;
        let bytes_string = bytes
            .take(string_size.unsigned_abs() as usize)
            .copied()
            .collect::<Vec<u8>>();

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
    pub fn from_iterator<'a, T>(bytes: &mut T) -> Option<Self>
    where
        T: Iterator<Item = &'a u8>,
    {
        let boolean = bytes.next()?;

        Some(Self(*boolean == 0x01))
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
    pub fn from_iterator<'a, T>(bytes: &mut T) -> Option<Self>
    where
        T: Iterator<Item = &'a u8>,
    {
        let lenght = VarInt::new().read_from_iter(bytes)?;

        Some(Self(
            bytes
                .take(usize::try_from(lenght).unwrap())
                .copied()
                .collect(),
        ))
    }
}
