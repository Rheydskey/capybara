use bytes::Bytes;
use capybara_packet_serde::from_bytes;

use crate::{
    ClientInformation, ClientboundPluginMessage, EncryptionResponse, FinishConfigAcknowledged,
    Handshake, Login, LoginAcknowledged, PacketError, PingRequest, StatusRequest,
};

macro_rules! parse {
    ($state:ident, $packet:ident,$bytes:ident, $($id:expr =>  {
      $($($t:path),* => $value:path),* $(,)?
    }),* $(,)?) => {
        match $packet {
            $(
                $id => {
                    $(
                        if $(matches!($state, $t))||* {
                            return Ok($value(from_bytes($bytes)?));
                        }
                    )*
                }
            )*


            _ => return Err(PacketError::CannotParse($packet).into()),
        }
    };
}

/// Return parsed packet
///
/// # Errors
/// Return error if cannot parse the packet
pub fn parse_packet(
    packetid: i32,
    state: &PacketState,
    bytes: &Bytes,
) -> anyhow::Result<PacketEnum> {
    parse!(state, packetid, bytes,
        0x0 => {
            PacketState::Handshake, PacketState::None => PacketEnum::HandShake,
            PacketState::Status => PacketEnum::StatusRequest,
            PacketState::Login => PacketEnum::Login,
            PacketState::Configuration => PacketEnum::ClientInformation
        },
        0x1 => {
            PacketState::Status => PacketEnum::PingRequest,
            PacketState::Login => PacketEnum::EncryptionResponse
        },
        0x2 => {
            PacketState::Configuration => PacketEnum::ClientboundPluginMessage,
        },
        0x3 => {
            PacketState::Configuration => PacketEnum::FinishConfigAcknowledged,
            PacketState::Login => PacketEnum::LoginAcknowledged
        }
    );

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
