use super::server::Stream;
use bevy::prelude::Event;
use bytes::Bytes;

#[derive(Debug, Event)]
pub enum Events {
    Connected(Stream),
    Message(Stream, Bytes),
}
