use bytes::Bytes;
use capybara_packet_serde::from_bytes;

use crate::{EncryptionResponse, Handshake, Login, PacketError, PingRequest};

/// Return parsed packet
///
/// # Errors
/// Return error if cannot parse the packet
pub fn parse_packet(
    packetid: i32,
    state: &PacketState,
    bytes: &Bytes,
) -> anyhow::Result<PacketEnum> {
    println!("{:?}", packetid);
    println!("{:?}", bytes);
    match packetid {
        0x0 => {
            if matches!(state, PacketState::Handshake) || matches!(state, PacketState::None) {
                return Ok(PacketEnum::HandShake(from_bytes::<Handshake>(bytes)?));
            }

            if matches!(state, PacketState::Login) {
                return Ok(PacketEnum::Login(from_bytes::<Login>(bytes)?));
            }

            Err(PacketError::CannotParse(-1).into())
        }

        0x1 => {
            if matches!(state, PacketState::Status) {
                return Ok(PacketEnum::PingRequest(
                    capybara_packet_serde::from_bytes::<PingRequest>(bytes)?,
                ));
            }

            Ok(PacketEnum::EncryptionResponse(from_bytes::<
                EncryptionResponse,
            >(bytes)?))
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
