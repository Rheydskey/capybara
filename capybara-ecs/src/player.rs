use bevy_ecs::prelude::{Bundle, Component};
use capybara_packet::helper::PacketState;

use crate::connection::{parsing::NetworkTask, CompressionState, EncryptionState};

#[derive(Debug, Component)]
pub struct Name(pub String);

#[derive(Debug, Component)]
pub struct Uuid(pub uuid::Uuid);

#[derive(Bundle)]
pub struct Player {
    pub event: NetworkTask,
    pub player_status: PlayerStatus,
    pub encryption_state: EncryptionState,
    pub compression_state: CompressionState,
    pub plugin_channel: PluginChannel,
}

#[derive(Debug, Component)]
pub struct PlayerStatus(pub PacketState);

impl PlayerStatus {
    pub fn get_status(&self) -> PacketState {
        self.0.clone()
    }

    pub fn set_status(&mut self, state: PacketState) {
        self.0 = state;
    }
}

#[derive(Debug, Component, Clone)]
pub struct VerifyToken(pub Vec<u8>);

impl VerifyToken {
    pub fn is_eq(&self, bytes: &[u8]) -> bool {
        self.0.eq(bytes)
    }
}

pub mod player_status_marker {
    use bevy_ecs::prelude::Component;

    #[derive(Debug, Component)]
    pub struct Handshaking;

    #[derive(Debug, Component)]
    pub struct Status;

    #[derive(Debug, Component)]
    pub struct Login;

    #[derive(Debug, Component)]
    pub struct Play;
}

#[derive(Debug, Component, Clone, Default)]
pub struct PluginChannel(pub Vec<String>);
