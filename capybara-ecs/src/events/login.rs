use bevy_ecs::{
    entity::Entity,
    system::{Commands, Query, Res},
};

use crate::{config::GlobalServerConfig, connection::parsing::ParseTask, player::VerifyToken};

use super::Login;

pub fn login_handler(
    mut command: Commands,
    logins: Query<(Entity, &Login, &ParseTask)>,
    rsa: Res<GlobalServerConfig>,
) {
    for (entity, login, parse_task) in logins.iter() {
        info!("Login for {entity:?}");

        let mut entity_command = command.entity(entity);

        entity_command.insert(crate::player::Uuid(login.0.uuid.0));
        entity_command.insert(crate::player::Name(login.0.name.clone()));

        let Ok(to_send) = capybara_packet::EncryptionRequest::new(
            &rsa.network_config.get_privkey().to_public_key(),
        ) else {
            error!("Cannot create encryption request");
            continue;
        };

        let token = to_send.verify_token.clone();
        info!("{token:?}");
        entity_command.insert(VerifyToken(token.0));

        if let Err(error) = parse_task.send_packet_serialize(&to_send) {
            error!("{error}");
        }

        entity_command.remove::<Login>();
    }
}
