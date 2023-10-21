use bevy::prelude::{Commands, Entity, Event, EventReader, Plugin, Query, Update, With};
use capybara_packet::{
    types::RawPacket, Handshake as HandshakePacket, IntoResponse, Packet,
    PingRequest as PingRequestPacket, StatusPacket,
};

use crate::{parsing::ParseTask, player::PlayerStatusMarker};

pub struct PacketEventPlugin;

impl Plugin for PacketEventPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<PingRequest>()
            .add_event::<Handshake>()
            .add_systems(Update, (ping_handler, handshake_handler));
    }
}

#[derive(Event)]
pub struct PingRequest(pub Entity, pub PingRequestPacket);

#[derive(Event)]
pub struct Handshake(pub Entity, pub HandshakePacket);

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
    for Handshake(
        entity,
        HandshakePacket {
            protocol,
            address,
            port,
            next_state,
        },
    ) in handshakes.iter()
    {
        let packet = Packet::new();
        info!("{entity:?}");
        let Ok(p) = parse_task.get(*entity) else {
            continue;
        };
        let mut entitycommand = command.entity(*entity);

        if *next_state == 1 {
            entitycommand.remove::<PlayerStatusMarker::Handshaking>();
            entitycommand.insert(PlayerStatusMarker::Status);

            let rpacket =
                RawPacket::from_bytes(&StatusPacket::default().to_response(&packet).unwrap(), 0x0);

            p.send_packet(rpacket.data).unwrap();
        } else if *next_state == 2 {
            entitycommand.remove::<PlayerStatusMarker::Handshaking>();
            entitycommand.insert(PlayerStatusMarker::Login);

            let rawpacket = RawPacket::from_bytes(
                &capybara_packet::DisconnectPacket::from_reason("Implementing")
                    .to_response(&packet)
                    .unwrap(),
                0x0,
            );

            p.send_packet(rawpacket.data).unwrap();
        }
    }
}
