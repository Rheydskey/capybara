use bytes::Bytes;
use capybara_packet_serde::from_bytes;

use crate::{
    ClientInformation, ClientboundPluginMessage, EncryptionResponse, FinishConfigAcknowledged,
    Handshake, Login, LoginAcknowledged, PacketError, PingRequest, StatusRequest,
};

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
                return Ok(PacketEnum::HandShake(from_bytes(bytes)?));
            }

            if matches!(state, PacketState::Status) {
                return Ok(PacketEnum::StatusRequest(from_bytes(bytes)?));
            }

            if matches!(state, PacketState::Login) {
                return Ok(PacketEnum::Login(from_bytes(bytes)?));
            }

            if matches!(state, PacketState::Configuration) {
                return Ok(PacketEnum::ClientInformation(from_bytes(bytes)?));
            }
        }

        0x1 => {
            if matches!(state, PacketState::Status) {
                return Ok(PacketEnum::PingRequest(capybara_packet_serde::from_bytes(
                    bytes,
                )?));
            }

            if matches!(state, PacketState::Login) {
                return Ok(PacketEnum::EncryptionResponse(from_bytes(bytes)?));
            }
        }
        0x2 => {
            if matches!(state, PacketState::Configuration) {
                return Ok(PacketEnum::ClientboundPluginMessage(from_bytes(bytes)?));
            }
        }
        0x3 => {
            if matches!(state, PacketState::Configuration) {
                return Ok(PacketEnum::FinishConfigAcknowledged(from_bytes(bytes)?));
            }

            if matches!(state, PacketState::Login) {
                return Ok(PacketEnum::LoginAcknowledged(
                    capybara_packet_serde::from_bytes(bytes)?,
                ));
            }
        }
        _ => return Err(PacketError::CannotParse(packetid).into()),
    }

    Err(PacketError::CannotParse(-1).into())
}

#[derive(Debug, Default, Clone)]
pub enum PacketEnum {
    #[default]
    None,
    HandShake(Handshake),
    StatusRequest(StatusRequest),
    Login(Login),
    EncryptionResponse(EncryptionResponse),
    PingRequest(PingRequest),
    LoginAcknowledged(LoginAcknowledged),
    ClientboundPluginMessage(ClientboundPluginMessage),
    ClientInformation(ClientInformation),
    FinishConfigAcknowledged(FinishConfigAcknowledged),
    UnknowPacket(String),
}

#[derive(Debug, Clone)]
pub enum PacketState {
    None,
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
}
