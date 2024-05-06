use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Commands, EntityCommands, Query},
};
use capybara_packet::{Description, Player, ServerStatus, StatusPacket};

use crate::{connection::parsing::ParseTask, player::player_status_marker};

use super::Handshake;

pub fn handshake_status_handler(mut command: EntityCommands, p: &ParseTask) {
    command.remove::<player_status_marker::Handshaking>();
    command.insert(player_status_marker::Status);

    if let Err(error) =
        p.send_packet_serialize(&StatusPacket::from_serializable(&ServerStatus::new(
            Description {
                text: "Capybara Server".to_string(),
            },
            false,
            capybara_packet::Players {
                max: 10,
                online: 1,
                sample: vec![Player {
                    id: String::new(),
                    name: "Rheydskey".to_string(),
                }],
            },
            false,
            capybara_packet::ServerVersion {
                name: "1.20.4".to_string(),
                protocol: 765,
            },
        )))
    {
        error!("{error}");
    }
}

pub fn handshake_handler(
    mut command: Commands,
    handshakes: Query<(Entity, &ParseTask, &Handshake), With<player_status_marker::Handshaking>>,
) {
    for (entity, parse_task, handshake) in handshakes.iter() {
        let mut entitycommand = command.entity(entity);

        let next_state = handshake.0.next_state;
        if next_state == 1 {
            handshake_status_handler(entitycommand, parse_task);
            continue;
        }

        if next_state == 2 {
            entitycommand.remove::<player_status_marker::Handshaking>();
            entitycommand.insert(player_status_marker::Login);
            info!("{entity:?} is now in logging state");
            continue;
        }

        info!("Weird next_state > 2");
        let Ok(packet) = capybara_packet::DisconnectPacket::from_reason("Unsupported next_state")
        else {
            error!("Cannot serialize DisconnectPacket : {:?}", entity);
            continue;
        };

        if let Err(error) = parse_task.send_packet_serialize(&packet) {
            info!("{error:?}");
        }
    }
}
