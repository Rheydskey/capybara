use std::collections::VecDeque;

use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

use bevy::prelude::{
    App, Commands, Component, Entity, EventWriter, IntoSystemConfigs, Plugin, PreUpdate, Query,
    Res, Resource, SystemSet,
};
use bytes::Bytes;
use capybara_packet::helper::{PacketEnum, PacketState};
use capybara_packet::types::RawPacket;
use capybara_packet::{Handshake, IntoResponse, Packet, PingRequest, StatusPacket};
use log::info;

use crate::event::Events;
use crate::parsing::ParseTask;
use crate::PacketStateComponent;

#[derive(Component, Clone, Debug)]
pub struct Stream {
    pub stream: Arc<TcpStream>,
}

impl Stream {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: Arc::new(stream),
        }
    }

    pub fn read(&self) -> TcpStream {
        self.stream.try_clone().unwrap()
    }

    pub fn is_eq(&self, cmp: &Self) -> bool {
        if let (Ok(peer_addr), Ok(cmp_peer)) = (self.read().peer_addr(), cmp.read().peer_addr()) {
            return peer_addr == cmp_peer;
        }

        false
    }
}

#[derive(Resource, Default)]
pub struct SendQueue(pub VecDeque<Message>);

#[derive(Component, Debug)]
pub struct Message(pub Stream, pub Bytes);

#[derive(Resource)]
pub struct Listener(pub TcpListener);

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let socket = TcpListener::bind("127.0.0.1:25565").unwrap();
        socket.set_nonblocking(true).unwrap();

        app.insert_resource(SendQueue::default())
            .insert_resource(Listener(socket))
            .add_event::<Events>()
            .configure_sets(
                PreUpdate,
                (
                    ConnexionSet::CleanNetManager,
                    ConnexionSet::InsertNewConnection,
                    ConnexionSet::ReadStream,
                ),
            )
            .add_systems(
                PreUpdate,
                (
                    clear_dead_socket.in_set(ConnexionSet::CleanNetManager),
                    recv_connection.in_set(ConnexionSet::InsertNewConnection),
                    recv_packet.in_set(ConnexionSet::ReadStream),
                ),
            );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ConnexionSet {
    CleanNetManager,
    InsertNewConnection,
    ReadStream,
}

pub fn clear_dead_socket(mut commands: Commands, tasks: Query<(Entity, &ParseTask)>) {
    for (entity, task) in tasks.iter() {
        if task.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn recv_connection(socket: Res<Listener>, mut events: EventWriter<Events>) {
    if let Ok((tcpstream, _)) = socket.0.accept() {
        let stream = Stream::new(tcpstream);
        events.send(Events::Connected(stream));
    }
}

pub fn recv_packet(mut tasks: Query<(&mut PacketStateComponent, &ParseTask)>) {
    for (mut packetstate, i) in tasks.iter_mut() {
        for rawpacket in i.get_packet() {
            let mut packet = Packet::new();

            packet.parse_from_rawpacket(&packetstate.state, &rawpacket);

            if let PacketEnum::HandShake(Handshake {
                protocol,
                address,
                port,
                next_state,
            }) = &packet.packetdata
            {
                if *next_state == 1 {
                    let rpacket = RawPacket::from_bytes(
                        &StatusPacket::default().to_response(&packet).unwrap(),
                        0x0,
                    );

                    i.send_packet(rpacket.data).unwrap();

                    packetstate.state = PacketState::Status
                }
            }

            if let PacketEnum::PingRequest(pingrequest) = &packet.packetdata {
                let rpacket = RawPacket::from_bytes(
                    &PingRequest {
                        value: pingrequest.value,
                    }
                    .to_response(&packet)
                    .unwrap(),
                    0x1,
                );

                i.send_packet(rpacket.data).unwrap();
            }

            info!("{packet:?}");
            let rawpacket = RawPacket::from_bytes(
                &capybara_packet::DisconnectPacket::from_reason("Implementing")
                    .to_response(&packet)
                    .unwrap(),
                0x0,
            );
            i.send_packet(rawpacket.data).unwrap();
        }
    }
}
