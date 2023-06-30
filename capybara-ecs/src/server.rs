use std::{
    collections::VecDeque,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, RwLock},
};

use bevy::prelude::{App, EventWriter, Plugin, Res, ResMut, Resource};
use bytes::Bytes;

use crate::event::Events;

pub type Stream = Arc<RwLock<TcpStream>>;

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
            .add_system(recv_packet)
            .add_system(send_packet);
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
        println!("{:?}", read);
    }

    None
}

pub fn recv_packet(
    socket: Res<Listener>,
    mut events: EventWriter<Events>,
    mut net: ResMut<NetworkManager>,
) {
    while let Ok((tcpstream, _)) = socket.0.accept() {
        let tcpstream = Arc::new(RwLock::new(tcpstream));
        net.0.push(tcpstream.clone());
        events.send(Events::Connected(tcpstream.clone()));
    }

    let mut to_remove = Vec::new();
    for (i, tcpstream) in net.0.iter().enumerate() {
        let mut lock = tcpstream.write().unwrap();

        if let Some(buf) = read(&mut *lock) {
            events.send(Events::Message(tcpstream.clone(), buf))
        } else {
            to_remove.push(i);
        }
    }

    for index in to_remove {
        println!("Remove: {}", index);
        net.0.remove(index);
    }
}

pub fn send_packet(mut transport: ResMut<SendQueue>) {
    let to_send = transport.0.drain(..);
    for i in to_send {
        println!("Send packet");
        let stream = i.0;

        let mut lock = stream.write().unwrap();

        lock.write(&i.1).unwrap();
    }
}
