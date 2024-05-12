use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Commands, Query},
};
use capybara_packet::{
    Description, Handshake, PingRequest, Player, ServerStatus, StatusPacket, StatusRequest,
};

use crate::{connection::parsing::NetworkTask, player::player_status_marker};

pub fn status_handler(
    mut command: Commands,
    status: Query<(Entity, &NetworkTask, &StatusRequest), With<player_status_marker::Status>>,
) {
    for (entity, parse_task, _) in status.iter() {
        info!("status send");
        if let Err(error) =
            parse_task.send_packet_serialize(&StatusPacket::from_serializable(&ServerStatus::new(
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

        command.entity(entity).remove::<PingRequest>();
    }
}

pub fn handshake_handler(
    mut command: Commands,
    handshakes: Query<(Entity, &NetworkTask, &Handshake), With<player_status_marker::Handshaking>>,
) {
    for (entity, parse_task, packet) in handshakes.iter() {
        let mut entitycommand = command.entity(entity);

        let next_state = packet.next_state;
        if next_state == 1 {
            entitycommand.remove::<player_status_marker::Handshaking>();
            entitycommand.insert(player_status_marker::Status);
            entitycommand.remove::<Handshake>();
            entitycommand.log_components();
            info!("{entity:?} is now in status state");
            continue;
        }

        if next_state == 2 {
            entitycommand.remove::<player_status_marker::Handshaking>();
            entitycommand.insert(player_status_marker::Login);
            entitycommand.remove::<Handshake>();

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
