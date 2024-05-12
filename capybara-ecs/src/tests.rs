use capybara_packet::{
    capybara_packet_serde::{from_bytes, to_bytes},
    types::RawPacket,
    EncryptionResponse, LoginAcknowledged, LoginSuccessPacket,
};
use uuid::Uuid;

use crate::{
    config::NetworkConfig,
    connection::{EncryptionLayer, EncryptionState},
};

#[test]
pub fn encryption_response() {
    let encryption_state = EncryptionState::default();
    let raw = RawPacket { lenght: 105, packetid: 1, data: bytes::Bytes::from_static(b"\x80\x01\x8e\xb0q\xa1\xd0y\x15\xb5&\x85\xbb\xa77~\xf0\xdd~\x8f7\xbd\x8b\x9b\x9b\x8f\xc2%8\x8a\x17\x8d\xc4<\x1a\x93\t*Nm].je\xdcmXC\xe6M\x9f\xe8\xb9z*\xab\x88\x0f$\xcc\xe7\x05&U\xe8\xbal\xb4\xd3\x97\xd9z\xe3\xfd\x07\x1a\xbem\xd7\xf9\x18%lM[\xb7#~-\xc2\xf4\xfe\xadG_\xca \xcb\xe4v\x1b(wP\xcb\xd9\xf5J\x7f\xa4\xf5\x86\x9a\xb1\xd2a\xf2\xc9\xd4@\xe0'M\x9b\t\xe8\x17\xb0p$\x80\x01\x86H\x8eVD\x8f\xfc\xc7\x0b\x83\xdc\xcdE.\xae\xe6\xc2\xf8\xb8\x13k\xfc\xd80g\xc2Q\x8c_\xc7\x9cu|\x85K\x0b\xcc#j?\xd8\x18\xdf7(\x8a\xbb\xd5\x19\xf2\xc7N\xb3\x10+\"]\xc7\xac.W=\x80\xf9\xfa\x10x\x90\xbe}\x9a\xc0\x9aN\x13c\0|2\xed\xee\xd8\0\xf5\xdc\xd6\x15\x18O\x98\x0eo\xe3\"\x9e\x05\x12s\x91\xce~\xe3\xf6z2\x05Q\xb0\xd3\xb7\xeb\x1c]\xbaZ\xc1+\x88\x8a\xe4\x06\x89\xbb \xda W'") };

    let packet = from_bytes::<EncryptionResponse>(&raw.data).unwrap();

    let netconfig = NetworkConfig {
rsa_public: "MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCu20W0tylklCTeiE2932QLkadchKrbCNciUQNXKcnsYZ8o8X6w/EjSwklWObz1YRXbFgPb8om9xObCjbM3b0jdnY9qachU21apibuzU0Hr/F6c9aO7o6WAI4yexL6yUOtMBcLudMXUrdxEBnjyDtzDTn28x6W5pnTKDZ+769jrbwIDAQAB".to_string(),
rsa_private: "MIICdwIBADANBgkqhkiG9w0BAQEFAASCAmEwggJdAgEAAoGBAK7bRbS3KWSUJN6ITb3fZAuRp1yEqtsI1yJRA1cpyexhnyjxfrD8SNLCSVY5vPVhFdsWA9vyib3E5sKNszdvSN2dj2ppyFTbVqmJu7NTQev8Xpz1o7ujpYAjjJ7EvrJQ60wFwu50xdSt3EQGePIO3MNOfbzHpbmmdMoNn7vr2OtvAgMBAAECgYEAhYc/WPzSaFNVQHb4Xzn7zXXiR52sV+4NU9Bn66Um5RO7lmms5P0To5boqCcZbI1CQncyZUF2/GskCYhqRrf9/LLZj0/z1aYBfCZ5PPbP6AWQ3NcbJQqJwC/pd9q7t3dAPdUGKYuiRGEeehunt2mcyk/dQLiAVpW3HLHeBh0sPwkCQQDDs7wQRTWnMTzugW1unK2U9Spb/BW+LzTWXsV5BMMgDOOt1/crehrP1D/Bq1dZn7PZtKUZusVZ7l3//72QcXJrAkEA5LtQ//mh+CJS2gEftT5gy8xSfghtPKq8ixcL5jzsU5jphBkHzJfv7YQHOYPKsR/3LmcXuLc1ZQNX28oChmFUDQJAcAQIuSdkNnawcbQTdySnFoPd4xi/OUS78Zf9X++h4E6AY5kiGPijMCJE/A2eyeXng80qUzfcjkUp+MXhMzQ8UQJAOHof9/R1j8U52+ZnI+NxFyEuRTxjbJVj1JwuLf1hBQs+rYVYraXecjElb4ghJjcUW3rNGqjyIC0BJp3sna2uiQJBAMJaReUkXplAWUy0/xDbI3ofo13Lhg6ZR60kj23MM64YDrARbV8yEqf8lDpOdDruivqKnwLGxtNu3PgkrzXgu94=".to_string(),
port: 25565    };

    let rsa_key = netconfig.get_privkey();
    let verify_token: Vec<u8> = [224, 162, 148, 28].to_vec();

    let res = packet.decrypt_verify_token(&rsa_key).unwrap();

    info!("{verify_token:?} == {res:?}");

    assert_eq!(verify_token, res);

    let shared_secret = packet.decrypt_shared_secret(&rsa_key).unwrap();

    encryption_state.set_encryption(EncryptionLayer::new(&shared_secret));

    let loginack: Vec<u8> = [171, 68].to_vec();

    let first_byte = &mut [loginack[0].clone()];

    let second_byte = &mut [loginack[1].clone()];

    encryption_state.decrypt(first_byte).unwrap();

    encryption_state.decrypt(second_byte).unwrap();

    let loginack: Vec<u8> = vec![first_byte[0], second_byte[0]];

    let bytes = RawPacket::build_from_serialize(&LoginAcknowledged).unwrap();

    assert_eq!(loginack.to_vec(), bytes.data);
}

#[test]
pub fn encryption_layer() {
    let encryption_test = EncryptionState::default();
    let key: &[u8; 16] = b"1234567891234567";

    encryption_test.set_encryption(EncryptionLayer::new(key));

    let mut to_encrypt =
        to_bytes(&LoginSuccessPacket::new("Yes sir".to_string(), Uuid::max())).unwrap();
    encryption_test.encrypt(&mut to_encrypt);
    encryption_test.decrypt(&mut to_encrypt).unwrap();

    assert_eq!(
        to_encrypt,
        to_bytes(&LoginSuccessPacket::new("Yes sir".to_string(), Uuid::max())).unwrap()
    );
}

#[test]
pub fn test_decryption() {
    let encryption_test = EncryptionState::default();
    let key: &[u8; 16] = b"1234567891234567";

    encryption_test.set_encryption(EncryptionLayer::new(key));

    let mut to_encrypt = *b"Salut les amis";
    encryption_test.encrypt(&mut to_encrypt);
    encryption_test.decrypt(&mut to_encrypt).unwrap();

    assert_eq!(&to_encrypt, b"Salut les amis");
}
