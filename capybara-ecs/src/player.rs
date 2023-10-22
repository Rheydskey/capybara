use bevy::prelude::{Bundle, Component};
use capybara_packet::helper::PacketState;

use crate::parsing::ParseTask;

#[derive(Debug, Component)]
pub struct Name(String);

#[derive(Debug, Component)]
pub struct Uuid(uuid::Uuid);

#[derive(Bundle)]
pub struct Player {
    pub event: ParseTask,
    pub player_status: PlayerStatus,
}

#[derive(Debug, Component)]
pub struct PlayerStatus(pub PacketState);

pub mod PlayerStatusMarker {
    use bevy::prelude::Component;

    #[derive(Debug, Component)]
    pub struct Handshaking;

    #[derive(Debug, Component)]
    pub struct Status;

    #[derive(Debug, Component)]
    pub struct Login;

    #[derive(Debug, Component)]
    pub struct Configuration;

    #[derive(Debug, Component)]
    pub struct Play;
}
