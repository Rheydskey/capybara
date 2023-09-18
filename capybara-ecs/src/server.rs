use std::collections::VecDeque;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

use bevy::prelude::{App, EventWriter, Plugin, Res, ResMut, Resource, Update};
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

pub struct Message(pub Stream, pub Bytes);

#[derive(Resource)]
pub struct Listener(pub TcpListener);

#[derive(Resource, Default, Debug)]
pub struct NetworkManager(Vec<Stream>);

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkManager::default())
            .insert_resource(SendQueue::default())
            .add_event::<Events>()
            .add_systems(Update, recv_packet)
            .add_systems(Update, send_packet);
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

pub fn recv_packet(
    socket: Res<Listener>,
    mut events: EventWriter<Events>,
    mut net: ResMut<NetworkManager>,
) {
    while let Ok((tcpstream, _)) = socket.0.accept() {
        info!("Received an new stream");
        let stream = Stream::new(tcpstream);
        net.0.push(stream.clone());
        events.send(Events::Connected(stream));
    }

    let mut to_remove = Vec::new();
    for (i, tcpstream) in net.0.iter().enumerate() {
        let mut lock = tcpstream.write();

        read(&mut lock).map_or_else(
            || to_remove.push(i),
            |buf| events.send(Events::Message(tcpstream.clone(), buf)),
        );
    }

    for index in to_remove {
        info!("Remove: {}", index);
        net.0.remove(index);
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
