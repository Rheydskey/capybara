mod component;
mod event;
mod logger;
mod server;

use crate::{logger::Log, server::Message};
use capybara_packet::{types::RawPacket, IntoResponse};
use event::Events;

use bevy::{
    app::App,
    prelude::{
        Bundle, Commands, Component, DespawnRecursiveExt, Entity, EventReader, Query, ResMut, With,
    },
    MinimalPlugins,
};
use server::{Listener, SendQueue, ServerPlugin};
use std::{collections::VecDeque, net::TcpListener};

use log::info;

pub fn init() {
    let socket = TcpListener::bind("127.0.0.1:25565").unwrap();
    socket.set_nonblocking(true).unwrap();

    App::new()
        .insert_resource(Listener(socket))
        .add_plugin(ServerPlugin)
        .add_plugins(MinimalPlugins)
        .add_plugin(Log)
        .add_system(connection_handler)
        .add_system(player_play)
        .run();
}

#[derive(Debug, Component)]
struct PacketName(String);

#[derive(Debug, Component)]
struct Name(String);

#[derive(Debug, Component)]
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
struct Uuid(uuid::Uuid);

#[derive(Debug, Component)]
struct Packet(capybara_packet::Packet);

#[derive(Debug, Component)]
struct Packets(VecDeque<Packet>);

#[derive(Bundle)]
struct Player {
    pub stream: Stream,
}

fn player_play(
    mut commands: Commands,
    query: Query<&Name, With<PacketName>>,
    players: Query<Entity>,
) {
    query.par_iter().for_each(|packetname| {
        log::info!("{:?}", packetname);
    });

    players.iter().for_each(|player| {
        commands.get_entity(player).unwrap().remove::<PacketName>();
        commands.get_entity(player).unwrap().despawn_recursive();
    });
}

fn connection_handler(
    mut commands: Commands,
    mut events: EventReader<Events>,
    mut transport: ResMut<SendQueue>,
    player: Query<(Entity, &Stream)>,
) {
    for event in &mut events {
        match event {
            Events::Connected(socket) => {
                let id = commands
                    .spawn(Player {
                        stream: Stream(socket.clone()),
                    })
                    .id();

                info!("Entity {:?}", id);
                let mut new_entity = commands.spawn_empty();
                new_entity.insert(Player {
                    stream: Stream(socket.clone()),
                });
            }

            Events::Message(socket, msg) => {
                info!("{msg:?}");

                let entity = player
                    .iter()
                    .filter_map(|(entity, stream)| {
                        if stream.is_eq(socket) {
                            return Some(entity);
                        }

                        None
                    })
                    .collect::<Vec<_>>();

                info!("{:?}", entity);

                let Some(entity) = entity.first() else {
                    return;
                };

                commands
                    .get_entity(*entity)
                    .unwrap()
                    .insert(PacketName("ee".to_string()));

                let rawpacket = RawPacket::read(msg).unwrap();

                let mut packet = capybara_packet::Packet::new();

                let disconnect = RawPacket::from_bytes(
                    &capybara_packet::DisconnectPacket::from_reason("Implementing")
                        .to_response(&packet)
                        .unwrap(),
                    0x0,
                );

                transport
                    .0
                    .push_front(Message(socket.clone(), disconnect.data));
            }
        }
    }
}
