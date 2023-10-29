use bytes::Bytes;

use crate::{EncryptionResponse, Handshake, Login, PacketError, PacketTrait, PingRequest};

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
            if matches!(state, PacketState::Handshake) || matches!(state, PacketState::None) {
                return Ok(PacketEnum::HandShake(Handshake::from_bytes(bytes)?));
            }

            if matches!(state, PacketState::Login) {
                return Ok(PacketEnum::Login(Login::from_bytes(bytes)?));
            }

            Err(PacketError::CannotParse(-1).into())
        }

        0x1 => {
            if matches!(state, PacketState::Status) {
                return Ok(PacketEnum::PingRequest(PingRequest::from_bytes(bytes)?));
            }

            Ok(PacketEnum::EncryptionResponse(
                EncryptionResponse::from_bytes(bytes)?,
            ))
        }

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
    PingRequest(PingRequest),
    UnknowPacket(String),
}

#[derive(Debug, Clone)]
pub enum PacketState {
    None,
    Status,
    Handshake,
    State,
    Login,
    Play,
}
