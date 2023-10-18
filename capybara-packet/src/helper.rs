use crate::{EncryptionResponse, Handshake, Login, PacketError, PacketTrait};
use anyhow::anyhow;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::{io::Cursor, ops::Deref};
use uuid::Uuid;

use crate::types::VarInt;

/// Return parsed packet
///
/// # Errors
/// Return error if cannot parse the packet
pub fn parse_packet(
    packetid: i32,
    state: &PacketState,
    bytes: &Bytes,
) -> anyhow::Result<PacketEnum> {
    match packetid {
        0x0 => {
            if matches!(state, PacketState::None) {
                return Ok(PacketEnum::HandShake(Handshake::from_bytes(bytes)?));
            }

            if matches!(state, PacketState::Login) {
                return Ok(PacketEnum::Login(Login::from_bytes(bytes)?));
            }

            Err(PacketError::CannotParse(-1).into())
        }

        0x1 => Ok(PacketEnum::EncryptionResponse(
            EncryptionResponse::from_bytes(bytes)?,
        )),

        _ => Err(PacketError::CannotParse(packetid).into()),
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

    #[must_use]
    pub const fn to_uuid(&self) -> Uuid {
        uuid::Uuid::from_u128(self.0)
    }
}

pub struct PacketString {
    inner: String,
}

impl PacketString {
    #[must_use]
    pub const fn new(inner: String) -> Self {
        Self { inner }
    }

    /// Return a `PacketString` from bytes
    ///
    /// # Errors
    /// Return error if cannot:
    /// - Read lenght of string
    /// - Get range of bytes
    /// - Transform bytes to string
    pub fn from_cursor(bytes: &mut Cursor<&[u8]>) -> anyhow::Result<Self> {
        let string_size = VarInt::new().read_from_cursor(bytes)?;

        let chunk = bytes.chunk();

        let bytes_string = chunk
            .get(..string_size.unsigned_abs() as usize)
            .ok_or_else(|| {
                anyhow!(
                    "Bytes array too small : Want {} but array lenght is {}",
                    string_size.unsigned_abs() as usize,
                    chunk.len()
                )
            })?
            .to_vec();

        bytes.advance(string_size.unsigned_abs() as usize);

        let string = String::from_utf8(bytes_string)?;

        Ok(Self::new(string))
    }

    /// Return encoded string for minecraft packet
    ///
    /// # Errors
    /// Return error if cannot transform string lenght in `i32`
    pub fn to_bytes(string: &str) -> anyhow::Result<Bytes> {
        let mut bytes = BytesMut::new();

        bytes.put_slice(&VarInt::encode(i32::try_from(string.len())?));
        bytes.put_slice(string.as_bytes());

        Ok(bytes.freeze())
    }
}

impl ToString for PacketString {
    fn to_string(&self) -> String {
        self.inner.clone()
    }
}

pub struct PacketBool(bool);

impl PacketBool {
    pub fn from_cursor(bytes: &mut Cursor<&[u8]>) -> Self {
        let boolean = bytes.get_u8();

        Self(boolean == 0x01)
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
    /// Return `Vec` of `u8` decoded from minecraft packet
    ///
    /// # Errors
    /// Return error if cannot:
    /// - Get length of arraybyte
    /// - Cannot get range of bytes
    pub fn from_cursor(bytes: &mut Cursor<&[u8]>) -> anyhow::Result<Self> {
        let lenght = VarInt::new().read_from_cursor(bytes)?;

        let usize_lenght = usize::try_from(lenght)?;
        let arraybytes = bytes
            .chunk()
            .get(..usize_lenght)
            .ok_or_else(|| anyhow!("Cannot get the range of bytes"))?
            .to_vec();

        bytes.advance(usize_lenght);

        Ok(Self(arraybytes))
    }
}
