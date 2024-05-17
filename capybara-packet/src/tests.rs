#[cfg(test)]
fn headers_skip(bytes: &mut &[u8]) {
    use capybara_packet_parser::{Parsable, VarInt};

    _ = VarInt::parse(bytes).unwrap();
    _ = VarInt::parse(bytes).unwrap();
}

#[test]
fn ping() {
    use crate::Handshake;

    let mut test = b"\x10\0\xfd\x05\tlocalhostc\xdd\x01".as_slice();
    headers_skip(&mut test);
    _ = capybara_packet_serde::from_bytes::<Handshake>(&test).unwrap();
}

#[test]
fn login_start() {
    use crate::Login;
    let mut test = b"\x1b\x00\x09\x52\x68\x65\x79\x64\x73\x6b\x65\x79\xfb\x48\x8a\x18\x6b\x02\x4b\x62\x9c\x8f\x4e\xb2\x7e\x26\x58\x51".as_slice();
    headers_skip(&mut test);
    let Login { name, uuid } = capybara_packet_serde::from_bytes::<Login>(&test).unwrap();

    assert_eq!(name, "Rheydskey");
    assert_eq!(uuid.0.to_string(), "fb488a18-6b02-4b62-9c8f-4eb27e265851")
}

#[test]
fn encryption_request() {
    use crate::EncryptionResponse;

    let test = b"\x85\x02\x01\x80\x01\x2f\x2f\x9f\xa0\x76\xd6\xff
\xe2\x81\xb5\x65\x2a\x20\x0c\x56\x1f\x42\x78\x44\xd7\x5e\x8c\xbc
\x43\xaf\x5e\x89\x9f\xe6\xd4\x5e\x13\xe6\x55\xe2\xc1\x5c\xb1\x05
\x06\xd9\x84\x8e\xea\x50\xf9\x75\x63\x57\x74\xd6\x34\x69\xf1\x49
\x20\xe2\xd8\x3e\x0a\x3f\x90\xa8\xf7\xa2\x96\x6e\x3a\x56\x72\xb6
\x73\x6d\x9a\x10\x5f\xdd\xd3\x08\x68\x24\x07\x5c\xd4\xe2\xb9\xed
\xd3\xd3\x6a\xa6\xc5\x6b\x41\x1e\x42\x28\x7a\x3d\x00\xbd\xe1\x64
\x37\x68\xab\xa2\xda\x3e\x04\x55\x62\x92\x24\xb0\x97\x8c\xce\xdb
\x21\x12\x36\x84\xbf\xcd\xdf\x35\x0b\x80\x01\xa4\xec\x98\xc6\xed
\xa3\x26\x0c\x59\x82\x09\x83\x57\x16\x9f\x49\xca\xaa\x5f\x96\x2b
\x25\xa3\x88\xc4\x46\x16\xfd\x3c\xb3\x6a\x92\x27\xbe\x8a\xa6\xdb
\xf8\x88\x00\x2c\x4b\xf7\x83\x49\x8b\xb9\x46\x4d\x48\x20\xfb\x1f
\x52\x58\xee\x99\x0e\xc7\xe4\xc3\x3e\x8f\x43\x85\x8e\xfc\x55\x93
\x85\xff\xaf\xb2\xba\x05\x1c\x75\x53\xee\xda\x8e\xe0\xa1\xbd\x2b
\xd8\x1f\xe2\xe7\x48\xf3\x5c\x4b\x18\x63\x72\xb8\x8d\xc6\xa6\x36
\xbd\x35\x32\x1f\x54\xe3\x69\xeb\x23\x72\x0a\xf8\x99\xcd\x4e\x8f
\x93\x60\x5b\xbf\x3f\xbd\xae\x5a\x2d\xf3\x34"
        .as_slice();

    _ = capybara_packet_serde::from_bytes::<EncryptionResponse>(&test).unwrap();
}
