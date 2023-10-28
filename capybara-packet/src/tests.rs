use nom::sequence::tuple;

use nom_mcpacket::{read_u16, PacketString, PacketUuid, VarInt};

#[test]
pub fn test_uuid() {
    let to_parse : u128 = 0b11101001100101000111110101010000001000010100111001000110000011001011100010100011101111111010110101001110100001001100100000110000;
    let to_parse = to_parse.to_be_bytes();

    let (_, a) = PacketUuid::parse(&to_parse).unwrap();

    assert_eq!("e9947d50-214e-460c-b8a3-bfad4e84c830", a.to_string());
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
