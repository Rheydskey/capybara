use bevy_ecs::{
    entity::Entity,
    system::{Commands, Query, Res},
};
use capybara_packet::{Login, LoginAcknowledged};

use crate::{
    config::GlobalServerConfig,
    connection::parsing::NetworkTask,
    player::{player_status_marker, VerifyToken},
};

pub fn login_handler(
    mut command: Commands,
    logins: Query<(Entity, &Login, &NetworkTask)>,
    rsa: Res<GlobalServerConfig>,
) {
    for (entity, packet, parse_task) in logins.iter() {
        let mut entity_command = command.entity(entity);
        entity_command.insert(crate::player::Uuid(packet.uuid.0));
        entity_command.insert(crate::player::Name(packet.name.clone()));

        let Ok(to_send) = capybara_packet::EncryptionRequest::new(
            &rsa.network_config.get_privkey().to_public_key(),
        ) else {
            error!("Cannot create encryption request");
            continue;
        };

        let token = to_send.verify_token.clone();

        entity_command.insert(VerifyToken(token.0));

        if let Err(error) = parse_task.send_packet_serialize(&to_send) {
            error!("{error}");
        }

        entity_command.remove::<Login>();
    }
}

pub fn login_ack(mut command: Commands, loginacks: Query<(Entity, &LoginAcknowledged)>) {
    for (entity, _) in loginacks.iter() {
        let mut entity_command = command.entity(entity);
        entity_command.remove::<player_status_marker::Login>();
        entity_command.insert(player_status_marker::Configuration);
        info!("{:?} is now in configuration mode", entity);
        entity_command.remove::<LoginAcknowledged>();
    }
}
