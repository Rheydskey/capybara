use super::server::Stream;
use bevy::prelude::Event;
use bytes::Bytes;

#[derive(Event)]
pub enum Events {
    Connected(Stream),
    Message(Stream, Bytes),
}
