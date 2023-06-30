use super::server::Stream;
use bytes::Bytes;

pub enum Events {
    Connected(Stream),
    Message(Stream, Bytes),
}
