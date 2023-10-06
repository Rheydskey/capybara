use std::collections::VecDeque;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

use bevy::prelude::{
    App, Component, EventWriter, Plugin, PostUpdate, PreUpdate, Res, ResMut, Resource, Update,
};
use bytes::Bytes;
use log::{error, info};

use crate::event::Events;

#[derive(Clone, Debug)]
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

    pub fn write(&self) -> TcpStream {
        self.stream.try_clone().unwrap()
    }
}

#[derive(Resource, Default)]
pub struct SendQueue(pub VecDeque<Message>);

#[derive(Component, Debug)]
pub struct Message(pub Stream, pub Bytes);

#[derive(Resource)]
pub struct Listener(pub TcpListener);

#[derive(Resource, Default, Debug)]
pub struct NetworkManager(Vec<Stream>);

#[derive(Resource, Default, Debug)]
pub struct DeleteQueue(pub VecDeque<usize>);

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let socket = TcpListener::bind("127.0.0.1:25565").unwrap();
        socket.set_nonblocking(true).unwrap();

        app.insert_resource(NetworkManager::default())
            .insert_resource(SendQueue::default())
            .insert_resource(Listener(socket))
            .insert_resource(DeleteQueue::default())
            .add_event::<Events>()
            .add_systems(PreUpdate, clear_dead_socket)
            .add_systems(PreUpdate, recv_packet)
            .add_systems(PostUpdate, send_packet);
    }
}

pub fn read(stream: &mut TcpStream) -> Option<Bytes> {
    let mut buf = [0; 4096];

    let read = stream.read(&mut buf);
    if let Ok(n) = read {
        if n > 0 {
            return Some(Bytes::copy_from_slice(&buf[..n]));
        }
    } else {
        error!("{:?}", read);
    }

    None
}

pub fn clear_dead_socket(mut net: ResMut<NetworkManager>, mut deletequeue: ResMut<DeleteQueue>) {
    while let Some(index) = deletequeue.0.pop_back() {
        info!("Remove: {}", index);
        net.0.remove(index);
    }
}

pub fn recv_packet(
    socket: Res<Listener>,
    mut events: EventWriter<Events>,
    mut net: ResMut<NetworkManager>,
    mut deletequeue: ResMut<DeleteQueue>,
) {
    while let Ok((tcpstream, _)) = socket.0.accept() {
        info!("Received an new stream");
        let stream = Stream::new(tcpstream);
        net.0.push(stream.clone());
        events.send(Events::Connected(stream));
    }

    for (i, tcpstream) in net.0.iter().enumerate() {
        let mut lock = tcpstream.write();

        read(&mut lock).map_or_else(
            || deletequeue.0.push_front(i),
            |buf| events.send(Events::Message(tcpstream.clone(), buf)),
        );
    }
}

pub fn send_packet(mut transport: ResMut<SendQueue>) {
    let to_send = transport.0.drain(..);
    for i in to_send {
        info!("Send packet");

        println!("{:?}", i.1);

        let stream = i.0;

        stream.write().write(&i.1).unwrap();

        println!("Sended");
    }
}
