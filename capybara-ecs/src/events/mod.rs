mod encryption;
mod handshake;
mod login;
mod ping;

use bevy_app::{Plugin, Update};
use bevy_ecs::{component::Component, schedule::IntoSystemConfigs};

use capybara_packet::{
    EncryptionResponse as EncryptionResponsePacket, Handshake as HandshakePacket,
    Login as LoginPacket, PingRequest as PingRequestPacket,
};

pub struct PacketEventPlugin;

impl Plugin for PacketEventPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(
            Update,
            (
                ping::ping_handler,
                handshake::handshake_handler,
                login::login_handler,
                encryption::response_encryption,
            )
                .chain(),
        );
    }
}

#[derive(Debug, Component)]
pub struct PingRequest(pub PingRequestPacket);

#[derive(Debug, Component)]
pub struct Handshake(pub HandshakePacket);

#[derive(Debug, Component)]
pub struct Login(pub LoginPacket);

#[derive(Debug, Component)]
pub struct EncryptionResponse(pub EncryptionResponsePacket);
