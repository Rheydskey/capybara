use std::{thread::sleep, time::Duration};

use bevy_ecs::{
    entity::Entity,
    system::{Commands, Query},
};
use capybara_packet::{
    capybara_packet_parser::VarInt, ClientInformation, ClientboundPluginMessage,
    FinishConfigAcknowledged, FinishConfiguration, Identifier, PlayLogin,
};

use crate::{
    connection::parsing::NetworkTask,
    player::{player_status_marker, PluginChannel},
};

pub fn client_information(mut command: Commands, loginacks: Query<(Entity, &ClientInformation)>) {
    for (entity, client_information) in loginacks.iter() {
        let mut entity_command = command.entity(entity);

        info!("{:?}", client_information);
        entity_command.remove::<ClientInformation>();
    }
}

pub fn registry_data(
    mut command: Commands,
    mut loginacks: Query<(
        Entity,
        &ClientboundPluginMessage,
        &NetworkTask,
        &mut PluginChannel,
    )>,
) {
    for (entity, client_information, nettask, mut plugin) in loginacks.iter_mut() {
        let mut entity_command = command.entity(entity);
        let string = String::from_utf8(client_information.data.clone()).unwrap();

        plugin.0.push(string);

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
                dimension_names: vec![Identifier::new(
                    "minecraft".to_string(),
                    "overworld".to_string(),
                )],
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

        entity_command.remove::<FinishConfigAcknowledged>();
    }
}
