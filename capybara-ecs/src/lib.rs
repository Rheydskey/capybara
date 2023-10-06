mod component;
mod event;
mod logger;
mod server;

use crate::{logger::Log, server::Message};
use capybara_packet::{helper::PacketState, types::RawPacket, IntoResponse, Packet};
use event::Events;

use bevy::{
    app::{App, ScheduleRunnerPlugin},
    prelude::{
        Bundle, Commands, Component, Entity, EventReader, IntoSystemConfigs, IntoSystemSetConfigs,
        PreUpdate, Query, ResMut, SystemSet, Update,
    },
};
use server::{Listener, SendQueue, ServerPlugin};
use std::{net::TcpListener, time::Duration};

use log::{error, info};

pub fn init() {
    App::new()
        .add_plugins(ServerPlugin)
        .add_plugins(Log)
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1. / 20.,
        )))
        .configure_sets(Update, (PacketSet::Parsing, PacketSet::Handling).chain())
        .add_systems(PreUpdate, connection_handler)
        .add_systems(
            Update,
            (
                packet_parse.in_set(PacketSet::Parsing),
                packet_handler.in_set(PacketSet::Handling),
            ),
        )
        .run();
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PacketSet {
    Parsing,
    Handling,
}

#[derive(Debug, Component)]
struct PacketName(String);

#[derive(Debug, Component)]
struct Name(String);

#[derive(Debug, Clone, Component)]
struct Stream(server::Stream);

impl Stream {
    pub fn is_eq(&self, cmp: &server::Stream) -> bool {
        if let (Ok(peer_addr), Ok(cmp_peer)) = (self.0.read().peer_addr(), cmp.read().peer_addr()) {
            return peer_addr == cmp_peer;
        }

        false
    }
}

#[derive(Debug, Component)]
struct PacketQueue(Vec<Packet>);

impl PacketQueue {
    pub fn push(&mut self, packet: Packet) {
        self.0.push(packet);
    }
}

#[derive(Debug, Component)]
struct Uuid(uuid::Uuid);

#[derive(Bundle)]
struct Player {
    pub stream: Stream,
    pub packetqueue: PacketQueue,
    pub state: PacketStateComponent,
}

#[derive(Debug, Component)]
struct PacketStateComponent {
    state: PacketState,
}

fn packet_parse(
    mut events: EventReader<Events>,
    mut query_player: Query<(
        Entity,
        &Stream,
        &mut PacketQueue,
        Option<&PacketStateComponent>,
    )>,
) {
    for event in &mut events {
        if let Events::Message(stream, msg) = event {
            for (_, strm, mut packetqueue, state) in query_player.iter_mut() {
                if !strm.is_eq(stream) {
                    continue;
                }

                let rawpacket = match RawPacket::read(&msg) {
                    Ok(value) => value,
                    Err(error) => {
                        error!("{}", error);
                        return;
                    }
                };

                let mut packet = capybara_packet::Packet::new();

                let packet_state = if let Some(packet_state) = &state {
                    &packet_state.state
                } else {
                    &PacketState::None
                };

                if let Err(error) = packet.parse_from_rawpacket(&packet_state, &rawpacket) {
                    error!("{error}");
                };

                packetqueue.push(packet);
            }
        }
    }
}

fn packet_handler(
    mut commands: Commands,
    mut transport: ResMut<SendQueue>,
    mut query_player: Query<(Entity, &Stream, &mut PacketQueue)>,
) {
    for (player, socket, mut packet_queue) in query_player.iter_mut() {
        for packet in packet_queue.0.drain(..) {
            info!("{packet:?}");

            if let capybara_packet::helper::PacketEnum::HandShake(handshake) = &packet.packetdata {
                if handshake.next_state == 1 {
                    let disconnect = RawPacket::from_intoresponse(
                        capybara_packet::StatusPacket::default(),
                        &packet,
                        0x0,
                    );

                    transport
                        .0
                        .push_front(Message(socket.0.clone(), disconnect.data));

                    continue;
                }

                if handshake.next_state == 2 {
                    commands.entity(player).insert(PacketStateComponent {
                        state: PacketState::Handshake,
                    });
                    continue;
                }
            }

            let disconnect = RawPacket::from_bytes(
                &capybara_packet::DisconnectPacket::from_reason("Implementing")
                    .to_response(&packet)
                    .unwrap(),
                0x0,
            );

            transport
                .0
                .push_front(Message(socket.0.clone(), disconnect.data));
        }
    }
}

fn connection_handler(mut commands: Commands, mut events: EventReader<Events>) {
    for event in &mut events {
        if let Events::Connected(socket) = event {
            let entity = commands.spawn(Player {
                stream: Stream(socket.clone()),
                packetqueue: PacketQueue(Vec::new()),
                state: PacketStateComponent {
                    state: PacketState::None,
                },
            });

            info!("Entity {:?}", entity.id());
        }
    }
}
