use capybara_packet_parser::{Parsable, VarInt};

use crate::{types::RawPacket, Handshake, PingRequest};

fn headers_skip(bytes: &mut &[u8]) {
    _ = VarInt::parse(bytes).unwrap();
    _ = VarInt::parse(bytes).unwrap();
}

#[test]
fn ping() {
    let mut test = b"\x10\0\xfd\x05\tlocalhostc\xdd\x01".as_slice();
    headers_skip(&mut test);
    _ = capybara_packet_serde::from_bytes::<Handshake>(&test).unwrap();
}
