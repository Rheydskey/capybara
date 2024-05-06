use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Commands, Query},
};

use crate::{connection::parsing::ParseTask, player::player_status_marker};

use super::PingRequest;
use capybara_packet::PingRequest as PingRequestPacket;

pub fn ping_handler(
    parse_task: Query<(Entity, &ParseTask, &PingRequest), With<player_status_marker::Status>>,
    mut command: Commands,
) {
    for (entity, parse, packet) in parse_task.iter() {
        if let Err(error) = parse.send_packet_serialize(&PingRequestPacket {
            value: packet.0.value,
        }) {
            error!("{error}");
        }

        command.entity(entity).remove::<PingRequest>();
    }
}
