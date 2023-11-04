use winnow::{binary::be_u16, Parser};

use crate::{PacketString, Parsable, VarInt};

#[test]
pub fn parse_packet() {
    let parse = b"\0\xfa\x05\t127.0.0.1c\xdd\x01";

    let (id, version, ip, port, next_state) = (
        VarInt::parse,
        VarInt::parse,
        PacketString::parse,
        be_u16,
        VarInt::parse,
    )
        .parse_next(&mut parse.as_slice())
        .unwrap();

    let a = format!("{id:?} | {version} | {} | {port:?} | {next_state:?}", ip);
    assert_eq!("0 | 762 | 127.0.0.1 | 25565 | 1", a)
}
