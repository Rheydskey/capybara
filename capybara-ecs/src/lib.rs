mod component;
mod event;
mod server;

use event::Events;

use bevy::{app::App, log::LogPlugin, prelude::*, MinimalPlugins};
use bytes::Bytes;
use server::{Listener, SendQueue, ServerPlugin};
use std::net::TcpListener;

use crate::server::Message;

#[derive(Resource, Debug, Default)]
struct Players(Vec<Entity>);

pub async fn init() {
    let socket = TcpListener::bind("127.0.0.1:25565").unwrap();
    socket.set_nonblocking(true).unwrap();

    App::new()
        .insert_resource(Listener(socket))
        .insert_resource(Players::default())
        .add_plugin(ServerPlugin)
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default())
        .add_system(connection_handler)
        .add_system(player_play)
        .run()
}

#[derive(Debug, Component)]
struct PacketName(String);

#[derive(Debug, Component)]
struct Name(String);

#[derive(Bundle)]
struct Player {
    pub name: Name,
}

fn player_play(
    mut commands: Commands,
    query: Query<&Name, With<PacketName>>,
    mut players: ResMut<Players>,
) {
    if let Some(player) = players.0.first() {
        commands.get_entity(*player).unwrap().remove::<PacketName>();
        commands.get_entity(*player).unwrap().despawn_recursive();
        players.0.remove(0);
    }

    query.par_iter().for_each(|packetname| {
        println!("{:?}", packetname);
    });
}

fn connection_handler(
    mut commands: Commands,
    mut events: EventReader<Events>,
    mut transport: ResMut<SendQueue>,
    mut player: ResMut<Players>,
) {
    for event in events.iter() {
        match event {
            Events::Connected(socket) => {
                println!("Socket address : {:?}", socket)
            }
            Events::Message(socket, msg) => {
                let id = commands
                    .spawn((
                        Player {
                            name: Name("test".to_string()),
                        },
                        PacketName("test".to_string()),
                    ))
                    .id();
                player.0.push(id);
                println!("{:?}", msg);
                transport.0.push_front(Message(
                    socket.clone(),
                    Bytes::copy_from_slice(&[1, 2, 3, 4]),
                ))
            }
        }
    }
}
