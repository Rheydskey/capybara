use std::{collections::VecDeque, sync::Arc, time::Duration};

pub use bevy::prelude::*;
use bevy::utils::HashMap;
use capybara_packet::helper::PacketEnum;
use tokio::{
    net::{unix::SocketAddr, TcpSocket, TcpStream},
    sync::Mutex,
};

struct Payload(PlayerSocket, Vec<u8>);

struct SendQueue(VecDeque<Payload>);

#[derive(Resource)]
pub struct PlayerSocket(TcpSocket);

pub struct PacketChannel(flume::Receiver<PacketEnum>);

pub struct Player(Arc<TcpStream>);

pub struct Players(Arc<Mutex<Player>>);

#[derive(Default, Resource)]
pub struct NetworkRessource(HashMap<SocketAddr, Duration>);

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkRessource::default())
            .add_event::<NetworkEvent>()
            .add_system(handler);
    }
}

pub fn handler(
    playersocket: Res<PlayerSocket>,
    mut events: EventWriter<NetworkEvent>,
    mut net: ResMut<NetworkRessource>,
) {
}

pub enum NetworkEvent {
    NewConnectionm(SocketAddr),
    Disconnected(SocketAddr),
}

pub fn init() {
    App::new()
        .add_plugin(NetworkPlugin)
        .add_startup_system(hell)
        .run()
}

fn hell() {
    println!("We're in hell");
}
