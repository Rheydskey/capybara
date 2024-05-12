use crate::{connection::parsing::NetworkTask, player::player_status_marker};
use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Commands, Query},
};

use capybara_packet::PingRequest;

pub fn ping_handler(
    parse_task: Query<(Entity, &NetworkTask, &PingRequest), With<player_status_marker::Status>>,
    mut command: Commands,
) {
    for (entity, parse, packet) in parse_task.iter() {
        if let Err(error) = parse.send_packet_serialize(&PingRequest {
            value: packet.value,
        }) {
            error!("{error}");
        }

        command.entity(entity).remove::<PingRequest>();
    }
}
