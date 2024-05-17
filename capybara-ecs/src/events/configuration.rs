use bevy_ecs::{
    entity::Entity,
    system::{Commands, Query},
};
use capybara_packet::{
    capybara_packet_parser::VarInt, ClientInformation, ClientboundPluginMessage,
    FinishConfigAcknowledged, FinishConfiguration, PlayLogin,
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

pub fn finish_config(
    mut command: Commands,
    finish: Query<(Entity, &FinishConfigAcknowledged, &NetworkTask)>,
) {
    for (entity, _, nettask) in finish.iter() {
        let mut entity_command = command.entity(entity);
        info!("{:?} is in play mode", entity);

        entity_command.insert(player_status_marker::Play);

        nettask
            .send_packet_serialize(&PlayLogin {
                entity_id: entity.index(),
                is_hardcore: false,
                dimension_names: Vec::new(),
                max_player: VarInt(20),
                view_distance: VarInt(10),
                simulation_distance: VarInt(10),
                reduced_debug_info: false,
                enable_respawn_screen: false,
                limited_crafting: false,
                dimension_type: String::new(),
                dimension_name: String::new(),
                hashed_seed: 0,
                gamemode: 1,
                previous_gamemode: -1,
                is_debug: false,
                is_flat: true,
                death_location: None,
                portal_cooldown: VarInt(10),
            })
            .unwrap();

        entity_command.remove::<player_status_marker::Configuration>();
        entity_command.remove::<FinishConfigAcknowledged>();
    }
}
