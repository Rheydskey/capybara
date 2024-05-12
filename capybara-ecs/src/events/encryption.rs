use bevy_ecs::{
    entity::Entity,
    system::{Commands, Query, Res},
};
use capybara_packet::{EncryptionResponse, LoginSuccessPacket};

use crate::{
    config::GlobalServerConfig,
    connection::{parsing::NetworkTask, EncryptionLayer, EncryptionState},
    player::VerifyToken,
};

pub fn response_encryption(
    mut command: Commands,
    responses: Query<(
        Entity,
        &EncryptionResponse,
        &NetworkTask,
        &VerifyToken,
        &mut EncryptionState,
        &crate::player::Uuid,
        &crate::player::Name,
    )>,
    rsa: Res<GlobalServerConfig>,
) {
    for (entity, packet, parse_task, verify_token, encryption_state, uuid, name) in responses.iter()
    {
        let rsa_key = rsa.network_config.get_privkey();

        let Ok(res) = packet.decrypt_verify_token(&rsa_key) else {
            continue;
        };

        if !verify_token.is_eq(&res) {
            error!("Local token != Remote token");
            continue;
        }

        let Ok(shared_secret) = packet.decrypt_shared_secret(&rsa_key) else {
            info!("Error");
            continue;
        };

        encryption_state.set_encryption(EncryptionLayer::new(&shared_secret));

        command.entity(entity).remove::<VerifyToken>();

        if let Err(error) =
            parse_task.send_packet_serialize(&LoginSuccessPacket::new(name.0.clone(), uuid.0))
        {
            error!("Cannot send packet for {:?} : {}", entity, error);
        }

        command.entity(entity).remove::<EncryptionResponse>();
    }
}
