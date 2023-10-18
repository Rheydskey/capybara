mod component;
mod event;
mod logger;
mod parsing;
mod server;

use crate::{logger::Log, server::Message};
use capybara_packet::{helper::PacketState, types::RawPacket, Packet};
use event::Events;

use bevy::{
    app::{App, ScheduleRunnerPlugin},
    prelude::{
        Bundle, Commands, Component, Entity, EventReader, IntoSystemConfigs, IntoSystemSetConfigs,
        PreUpdate, Query, ResMut, SystemSet, TaskPoolPlugin, Update,
    },
};
use parsing::ParseTask;
use server::{SendQueue, ServerPlugin, Stream};
use std::time::Duration;

use log::{error, info};

pub fn init() {
    App::new()
        .add_plugins(TaskPoolPlugin::default())
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
                (ping_status).chain().in_set(PacketSet::Handling),
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

#[derive(Debug, Component)]
struct Uuid(uuid::Uuid);

#[derive(Bundle)]
struct Player {
    pub stream: Stream,
    pub state: PacketStateComponent,
    pub event: ParseTask,
}

#[derive(Debug, Component)]
struct PacketStateComponent {
    state: PacketState,
}

#[derive(Debug, Component)]
struct PingStatus(pub Packet);

fn packet_parse(
    mut commands: Commands,
    mut events: EventReader<Events>,
    mut query_player: Query<(Entity, &Stream, Option<&PacketStateComponent>)>,
) {
    for event in &mut events {
        if let Events::Message(stream, rawpacket) = event {
            for (player, strm, state) in query_player.iter_mut() {
                if !strm.is_eq(stream) {
                    continue;
                }
                let mut packet = capybara_packet::Packet::new();

                let packet_state = if let Some(packet_state) = &state {
                    &packet_state.state
                } else {
                    &PacketState::None
                };

                if let Err(error) = packet.parse_from_rawpacket(&packet_state, &rawpacket) {
                    error!("{error}");
                };

                commands.entity(player).insert(PingStatus(packet));
            }
        }
    }
}

fn ping_status(
    mut commands: Commands,
    mut transport: ResMut<SendQueue>,
    query_player: Query<(Entity, &Stream, &PingStatus)>,
) {
    for (player, stream, packet) in query_player.iter() {
        let disconnect =
            RawPacket::from_intoresponse(capybara_packet::StatusPacket::default(), &packet.0, 0x0);

        transport
            .0
            .push_front(Message(stream.clone(), disconnect.data));

        commands.entity(player).remove::<PingStatus>();
    }
}

fn packet_handler(
    mut commands: Commands,
    mut transport: ResMut<SendQueue>,
    mut query_player: Query<(Entity, &Stream)>,
) {
    for (player, socket) in query_player.iter_mut() {
        /*        for packet in packet_queue.0.drain(..) {
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
        }*/
    }
}

fn connection_handler(mut commands: Commands, mut events: EventReader<Events>) {
    for event in &mut events {
        if let Events::Connected(socket) = event {
            let entity = commands.spawn(Player {
                event: ParseTask::new(socket.stream.clone()),
                stream: socket.clone(),
                state: PacketStateComponent {
                    state: PacketState::None,
                },
            });

            info!("Entity {:?}", entity.id());
        }
    }
}
