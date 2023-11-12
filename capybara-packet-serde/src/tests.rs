use capybara_packet_parser::VarInt;

use crate::to_bytes;

#[test]
pub fn test() {
    #[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
    struct Test {
        a: VarInt,
        b: String,
        c: u16,
        d: u8,
    }

    let sample = Test {
        a: VarInt(762),
        b: "127.0.0.1".to_string(),
        c: 25565,
        d: 1,
    };
    let a = to_bytes(&sample).unwrap();

    assert_eq!(a, b"\xfa\x05\t127.0.0.1c\xdd\x01");
    assert_eq!(crate::from_bytes::<Test>(&a).unwrap(), sample);
}
