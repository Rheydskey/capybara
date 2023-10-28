use nom::sequence::tuple;

use crate::{read_u16, PacketString, VarInt};

#[test]
pub fn test() {
    let bytes = &[0];
    let (_, _) = tuple((read_u16, VarInt::parse))(bytes).unwrap();
}

#[test]
pub fn parse_packet() {
    let parse = b"\0\xfa\x05\t127.0.0.1c\xdd\x01";

    let (remain, (id, version, ip, port, next_state)) = tuple((
        VarInt::parse,
        VarInt::parse,
        PacketString::parse,
        read_u16,
        VarInt::parse,
    ))(parse)
    .unwrap();

    let a = format!(
        "{id:?} | {version} | {} | {port:?} | {next_state:?} | {remain:?}",
        ip
    );

    assert_eq!("0 | 762 | 127.0.0.1 | 25565 | 1 | []", a)
}
