use std::collections::VecDeque;

use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

use bevy::prelude::{
    App, Commands, Component, Entity, EventWriter, IntoSystemConfigs, Plugin, PreUpdate, Query,
    Res, Resource, SystemSet,
};
use bytes::Bytes;
use capybara_packet::helper::{PacketEnum, PacketState};
use capybara_packet::Packet;

use crate::event::{Handshake, PacketEventPlugin, PingRequest};
use crate::parsing::ParseTask;
use crate::player::{Player, PlayerStatus, PlayerStatusMarker};

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
            )
            .add_plugins(PacketEventPlugin);
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

pub fn recv_connection(socket: Res<Listener>, mut commands: Commands) {
    if let Ok((tcpstream, _)) = socket.0.accept() {
        let stream = Stream::new(tcpstream);

        let mut entity = commands.spawn(Player {
            event: ParseTask::new(stream.stream.clone()),
            stream: stream.clone(),
            player_status: PlayerStatus(PacketState::Handshake),
        });

        entity.insert(PlayerStatusMarker::Handshaking);

        info!("Entity {:?}", entity.id());
    }
}

pub fn recv_packet(
    mut tasks: Query<(Entity, &ParseTask, &mut PlayerStatus)>,
    mut handshakeevent: EventWriter<Handshake>,
    mut pingevent: EventWriter<PingRequest>,
) {
    for (entity, i, mut state) in tasks.iter_mut() {
        for rawpacket in i.get_packet() {
            let mut packet = Packet::new();
            packet.parse_from_rawpacket(&state.0, &rawpacket).unwrap();

            info!("{packet:?}");
            match &packet.packetdata {
                PacketEnum::HandShake(packet) => {
                    if packet.next_state == 1 {
                        state.0 = PacketState::Status
                    }

                    if packet.next_state == 2 {
                        state.0 = PacketState::Login
                    }

                    handshakeevent.send(Handshake(entity, packet.clone()));
                    continue;
                }
                PacketEnum::PingRequest(pingrequest) => {
                    pingevent.send(PingRequest(entity, pingrequest.clone()));
                    continue;
                }
                PacketEnum::UnknowPacket(packet) => info!("{packet}"),
                _ => {
                    info!("{packet:?}");
                }
            }
        }
    }
}
