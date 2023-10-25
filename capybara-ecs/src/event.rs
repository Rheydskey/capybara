use bevy::{
    ecs::system::SystemParam,
    prelude::{
        Commands, Entity, Event, EventReader, EventWriter, IntoSystemConfigs, Plugin, Query, Res,
        Update, With,
    },
};
use capybara_packet::{
    types::RawPacket, EncryptionResponse as EncryptionResponsePacket, Handshake as HandshakePacket,
    IntoResponse, Login as LoginPacket, LoginSuccessPacket, Packet,
    PingRequest as PingRequestPacket, StatusPacket,
};

use crate::{
    config::GlobalServerConfig,
    parsing::ParseTask,
    player::{EncryptionLayer, EncryptionState, Name, PlayerStatusMarker, VerifyToken},
};

pub struct PacketEventPlugin;

impl Plugin for PacketEventPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<PingRequest>()
            .add_event::<Handshake>()
            .add_event::<Login>()
            .add_event::<EncryptionResponse>()
            .add_systems(
                Update,
                (
                    ping_handler,
                    handshake_handler,
                    login_handler,
                    response_encryption,
                )
                    .chain(),
            );
    }
}

#[derive(Event)]
pub struct PingRequest(pub Entity, pub PingRequestPacket);

#[derive(Event)]
pub struct Handshake(pub Entity, pub HandshakePacket);

#[derive(Event)]
pub struct Login(pub Entity, pub LoginPacket);

#[derive(Event)]
pub struct EncryptionResponse(pub Entity, pub EncryptionResponsePacket);

#[derive(SystemParam)]
pub struct GlobalEventWriter<'w> {
    ping: EventWriter<'w, PingRequest>,
    handshake: EventWriter<'w, Handshake>,
    login: EventWriter<'w, Login>,
    encryption_response: EventWriter<'w, EncryptionResponse>,
}

impl<'w> GlobalEventWriter<'w> {
    pub fn ping_writer(&mut self) -> &mut EventWriter<'w, PingRequest> {
        &mut self.ping
    }

    pub fn handshake_writer(&mut self) -> &mut EventWriter<'w, Handshake> {
        &mut self.handshake
    }

    pub fn login_writer(&mut self) -> &mut EventWriter<'w, Login> {
        &mut self.login
    }

    pub fn encryption_response_writer(&mut self) -> &mut EventWriter<'w, EncryptionResponse> {
        &mut self.encryption_response
    }
}

pub fn ping_handler(
    parse_task: Query<&ParseTask, With<PlayerStatusMarker::Status>>,
    mut ping: EventReader<PingRequest>,
) {
    for PingRequest(entity, packet) in ping.iter() {
        let Ok(a) = parse_task.get(*entity) else {
            continue;
        };

        let respacket = Packet::new();

        let rpacket = RawPacket::from_bytes(
            &PingRequestPacket {
                value: packet.value,
            }
            .to_response(&respacket)
            .unwrap(),
            0x1,
        );

        a.send_packet(rpacket.data).unwrap()
    }
}

pub fn handshake_handler(
    mut command: Commands,
    parse_task: Query<&ParseTask, With<PlayerStatusMarker::Handshaking>>,
    mut handshakes: EventReader<Handshake>,
) {
    for Handshake(entity, handshake) in handshakes.iter() {
        let packet = Packet::new();
        info!("{entity:?}");
        let Ok(p) = parse_task.get(*entity) else {
            continue;
        };

        let mut entitycommand = command.entity(*entity);

        let next_state = handshake.next_state;
        if next_state == 1 {
            entitycommand.remove::<PlayerStatusMarker::Handshaking>();
            entitycommand.insert(PlayerStatusMarker::Status);

            let rpacket =
                RawPacket::from_bytes(&StatusPacket::default().to_response(&packet).unwrap(), 0x0);

            p.send_packet(rpacket.data).unwrap();
        } else if next_state == 2 {
            entitycommand.remove::<PlayerStatusMarker::Handshaking>();
            entitycommand.insert(PlayerStatusMarker::Login);
            info!("{entity:?} is now in logging state");
        } else {
            info!("Weird next_state > 2");

            let rawpacket = RawPacket::from_bytes(
                &capybara_packet::DisconnectPacket::from_reason("Unsupported next_state")
                    .to_response(&packet)
                    .unwrap(),
                0x0,
            );

            p.send_packet(rawpacket.data).unwrap();
        }
    }
}

pub fn login_handler(
    mut command: Commands,
    parse_task: Query<&ParseTask>,
    rsa: Res<GlobalServerConfig>,
    mut logins: EventReader<Login>,
) {
    for Login(entity, login) in logins.iter() {
        info!("Login for {entity:?}");
        let Ok(task) = parse_task.get(*entity) else {
            info!("No task for {entity:?}");
            continue;
        };

        let mut entity_command = command.entity(*entity);
        entity_command.insert(crate::player::Uuid(login.uuid));
        entity_command.insert(crate::player::Name(login.name.clone()));

        let to_send = capybara_packet::EncryptionRequest::new(
            &rsa.network_config.get_privkey().to_public_key(),
        )
        .unwrap();

        let token = to_send.verify_token.0.clone();
        info!("{token:?}");
        entity_command.insert(VerifyToken(token));

        let packet = Packet::new();
        let rawpacket = RawPacket::from_bytes(&to_send.to_response(&packet).unwrap(), 0x1);

        task.send_packet(rawpacket.data);
    }
}

pub fn response_encryption(
    mut command: Commands,
    mut parse_task: Query<(
        &ParseTask,
        &VerifyToken,
        &mut EncryptionState,
        &crate::player::Uuid,
        &crate::player::Name,
    )>,
    mut responses: EventReader<EncryptionResponse>,
    rsa: Res<GlobalServerConfig>,
) {
    for EncryptionResponse(entity, response) in responses.iter() {
        info!("{:?}", entity);
        let Ok((e, a, es, uuid, name)) = parse_task.get_mut(*entity) else {
            info!("No entity for this");
            continue;
        };
        let rsa_key = rsa.network_config.get_privkey();
        let Ok(res) = response.decrypt_verify_token(&rsa_key) else {
            continue;
        };

        if res != a.0 {
            continue;
        }

        let Ok(shared_secret) = response.decrypt_shared_secret(&rsa_key) else {
            info!("Error");
            continue;
        };

        es.set_encryption(EncryptionLayer::new(&shared_secret));

        command.entity(*entity).remove::<VerifyToken>();

        let packet = Packet::new();

        e.send_packet(
            RawPacket::from_bytes(
                &LoginSuccessPacket::new(name.0.clone(), uuid.0)
                    .to_response(&packet)
                    .unwrap(),
                0x02,
            )
            .data,
        )
        .unwrap();
    }
}
