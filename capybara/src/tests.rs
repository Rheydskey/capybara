use capybara_packet::types::{VarInt, VarLong};

#[test]
pub fn test_varint() {
    let test_var = |data: &[u8], result: i32| {
        let val = VarInt::new().read_from_iter(&mut data.iter());
        assert_eq!(val, Some(result));
    };

    test_var(&[0x01], 1);
    test_var(&[0xAC, 0x02], 300);
    test_var(&[0xFF, 0xFF, 0xFF, 0x0F], 33554431);
}
#[test]
pub fn test_varfloat() {
    let test_var = |data: &[u8], result: i64| {
        let val = VarLong::new().read_from_iter(&mut data.iter());
        assert_eq!(val, Some(result));
    };

    test_var(&[0x01], 1);
    test_var(&[0xAC, 0x02], 300);
    test_var(&[0xFF, 0xFF, 0xFF, 0x0F], 33554431);
}
