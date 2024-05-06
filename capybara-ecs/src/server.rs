use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::prelude::{Commands, Entity, IntoSystemConfigs, Query, Res, Resource};
use bevy_hierarchy::DespawnRecursiveExt;

use std::net::TcpListener;

use capybara_packet::helper::{PacketEnum, PacketState};
use capybara_packet::Packet;

use crate::connection::{parsing::ParseTask, CompressionState, EncryptionState};
use crate::events::{Handshake, PacketEventPlugin, PingRequest};
use crate::player::{player_status_marker, Player, PlayerStatus};

#[derive(Resource)]
pub struct Listener(pub TcpListener);

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let socket = TcpListener::bind("127.0.0.1:25565").unwrap();
        socket.set_nonblocking(true).unwrap();

        app.insert_resource(Listener(socket))
            .add_systems(
                PreUpdate,
                (clear_dead_socket, recv_connection, recv_packet).chain(),
            )
            .add_plugins(PacketEventPlugin);
    }
}

pub fn clear_dead_socket(mut commands: Commands, tasks: Query<(Entity, &ParseTask)>) {
    for entity in tasks
        .iter()
        .filter(|(_, f)| f.is_finished())
        .map(|(e, _)| e)
    {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn recv_connection(socket: Res<Listener>, mut commands: Commands) {
    if let Ok((tcpstream, _)) = socket.0.accept() {
        let encryption_state = EncryptionState::default();
        let compression_state = CompressionState {};
        let Ok(task) = ParseTask::new(
            &tcpstream,
            encryption_state.clone(),
            compression_state.clone(),
        ) else {
            error!("Cannot make parsetask");
            return;
        };

        let mut entity = commands.spawn(Player {
            event: task,
            player_status: PlayerStatus(PacketState::Handshake),
            encryption_state,
            compression_state,
        });

        entity.insert(player_status_marker::Handshaking);

        info!("Entity {:?}", entity.id());
    }
}

pub fn recv_packet(
    mut tasks: Query<(Entity, &ParseTask, &mut PlayerStatus)>,
    mut commands: Commands,
) {
    for (entity, i, mut state) in &mut tasks {
        for rawpacket in i.get_packet() {
            let mut packet = Packet::new();

            if let Err(error) = packet.parse_from_rawpacket(&state.get_status(), &rawpacket) {
                error!("Cannot parse packet: {}", error);
                continue;
            }

            match &packet.packetdata {
                PacketEnum::HandShake(packet) => {
                    if packet.next_state == 1 {
                        state.set_status(PacketState::Status);
                    }

                    if packet.next_state == 2 {
                        state.set_status(PacketState::Login);
                    }

                    commands.entity(entity).insert(Handshake(packet.clone()));
                }
                PacketEnum::PingRequest(pingrequest) => {
                    commands
                        .entity(entity)
                        .insert(PingRequest(pingrequest.clone()));
                }
                PacketEnum::Login(login) => {
                    commands
                        .entity(entity)
                        .insert(crate::events::Login(login.clone()));
                }
                PacketEnum::EncryptionResponse(encryption) => {
                    commands
                        .entity(entity)
                        .insert(crate::events::EncryptionResponse(encryption.clone()));
                }
                PacketEnum::UnknowPacket(packet) => info!("{packet}"),
                PacketEnum::None => {
                    info!("{packet:?}");
                }
            }
        }
    }
}
