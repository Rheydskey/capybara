use super::server::Stream;
use bevy::prelude::Event;
use capybara_packet::types::RawPacket;

#[derive(Debug, Event)]
pub enum Events {
    Connected(Stream),
    Message(Stream, RawPacket),
}
