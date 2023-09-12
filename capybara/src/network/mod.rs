//! Network packet handling for Minecraft server

use aes::cipher::{AsyncStreamCipher, KeyIvInit};
use anyhow::Result;
use bytes::{Bytes, BytesMut};
use capybara_packet::Packet;
use capybara_packet::{helper::PacketState, types::RawPacket};
use tokio::io::AsyncWriteExt;
use tokio::{io::AsyncReadExt, net::TcpStream};

#[derive(Debug)]
struct EncryptionLayer {
    encode: cfb8::Encryptor<aes::Aes128>,
    decode: cfb8::Decryptor<aes::Aes128>,
}

impl EncryptionLayer {
    pub fn new(key: &[u8], iv: &[u8]) -> Self {
        Self {
            encode: cfb8::Encryptor::new(key.into(), iv.into()),
            decode: cfb8::Decryptor::new(key.into(), iv.into()),
        }
    }

    pub fn decode(&mut self, to_decode: &mut [u8]) {
        self.decode.clone().decrypt(to_decode)
    }

    pub fn encode(&mut self, to_encode: &mut [u8]) {
        self.encode.clone().encrypt(to_encode)
    }
}

#[derive(Debug)]
pub struct ClientConnection {
    pub stream: TcpStream,
    buffer: BytesMut,
    encryption: Option<EncryptionLayer>,
}

impl ClientConnection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(4096),
            encryption: None,
        }
    }

    pub fn set_encryption(&mut self, key_iv: &[u8]) {
        self.encryption = Some(EncryptionLayer::new(key_iv, key_iv))
    }

    pub async fn read(&mut self, state: &PacketState) -> Result<Option<Packet>> {
        self.buffer = BytesMut::with_capacity(4096);
        loop {
            if let Some(frame) = self.parse_frame(state)? {
                return Ok(Some(frame.clone()));
            }

            if self.stream.read_buf(&mut self.buffer).await? == 0 {
                if self.buffer.is_empty() {
                    return Ok(None);
                }

                return Err(anyhow::anyhow!("Connection reset"));
            }
        }
    }

    pub fn parse_frame(&mut self, state: &PacketState) -> anyhow::Result<Option<Packet>> {
        if self.buffer.is_empty() {
            return Ok(None);
        }

        let bytes = &mut self.buffer[..];

        if let Some(encryption) = &mut self.encryption {
            encryption.decode(bytes)
        }

        let rawpacket = RawPacket::read(&Bytes::copy_from_slice(bytes))?;

        if rawpacket.lenght <= 1 {
            return Ok(None);
        }

        let mut packet = Packet::new();

        packet.parse_from_rawpacket(state, &rawpacket);

        Ok(Some(packet))
    }

    pub async fn send_packet(&mut self, bytes: &Bytes, packetid: i32) -> anyhow::Result<()> {
        info!("Send packet with id = {}", packetid);
        let packet = RawPacket::from_bytes(bytes, packetid);

        let data = &mut packet.data.to_vec();

        if let Some(encryption) = &mut self.encryption {
            encryption.encode(data);
        }

        self.stream.write_all_buf(&mut data.as_slice()).await?;

        Ok(())
    }
}
