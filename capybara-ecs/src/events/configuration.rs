use bevy_ecs::{
    entity::Entity,
    system::{Commands, Query},
};
use capybara_packet::{
    ClientInformation, ClientboundPluginMessage, FinishConfigAcknowledged, FinishConfiguration,
};

use crate::{connection::parsing::NetworkTask, player::player_status_marker};

pub fn client_information(mut command: Commands, loginacks: Query<(Entity, &ClientInformation)>) {
    for (entity, client_information) in loginacks.iter() {
        let mut entity_command = command.entity(entity);

        info!("{:?}", client_information);
        entity_command.remove::<ClientInformation>();
    }
}

pub fn config_plugin(
    mut command: Commands,
    loginacks: Query<(Entity, &ClientboundPluginMessage, &NetworkTask)>,
) {
    for (entity, client_information, nettask) in loginacks.iter() {
        let mut entity_command = command.entity(entity);
        info!("{:?}", client_information);
        if let Err(e) = nettask.send_packet_serialize(&FinishConfiguration) {
            error!("{e}");
            continue;
        }

        entity_command.remove::<ClientboundPluginMessage>();
    }
}

pub fn finish_config(mut command: Commands, finish: Query<(Entity, &FinishConfigAcknowledged)>) {
    for (entity, _) in finish.iter() {
        let mut entity_command = command.entity(entity);
        info!("{:?} is in play mode", entity);

        entity_command.insert(player_status_marker::Status);

        entity_command.remove::<player_status_marker::Configuration>();
        entity_command.remove::<FinishConfigAcknowledged>();
    }
}
