use bevy::{
    ecs::system::SystemParam,
    prelude::{Commands, Entity, Event, EventReader, EventWriter, Plugin, Query, Update, With},
};
use capybara_packet::{
    types::RawPacket, Handshake as HandshakePacket, IntoResponse, Login as LoginPacket, Packet,
    PingRequest as PingRequestPacket, StatusPacket,
};

use crate::{parsing::ParseTask, player::PlayerStatusMarker};

pub struct PacketEventPlugin;

impl Plugin for PacketEventPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<PingRequest>()
            .add_event::<Handshake>()
            .add_event::<Login>()
            .add_systems(Update, (ping_handler, handshake_handler));
    }
}

#[derive(Event)]
pub struct PingRequest(pub Entity, pub PingRequestPacket);

#[derive(Event)]
pub struct Handshake(pub Entity, pub HandshakePacket);

#[derive(Event)]
pub struct Login(pub Entity, pub LoginPacket);

#[derive(SystemParam)]
pub struct GlobalEventWriter<'w> {
    ping: EventWriter<'w, PingRequest>,
    handshake: EventWriter<'w, Handshake>,
    login: EventWriter<'w, Login>,
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
    parse_task: Query<&ParseTask, With<PlayerStatusMarker::Login>>,
    mut logins: EventReader<Login>,
) {
    for Login(entity, ping) in logins.iter() {
        let Ok(task) = parse_task.get(*entity) else {
            continue;
        };

        let packet = Packet::new();
        let rawpacket = RawPacket::from_bytes(
            &capybara_packet::EncryptionRequest {
                server_id: todo!(),
                publickey: todo!(),
                verify_token: todo!(),
            }
            .to_response(&packet)
            .unwrap(),
            0x1,
        );

        task.send_packet(rawpacket.data);
    }
}
