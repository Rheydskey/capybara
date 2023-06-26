use bytes::Bytes;
use capybara_packet::{
    helper::{PacketEnum, PacketState},
    types::RawPacket,
    EncryptionRequest, IntoResponse, LoginSuccessPacket,
};
use rsa::{RsaPrivateKey, RsaPublicKey};
use tokio::{io::AsyncWriteExt, net::TcpStream};
use uuid::Uuid;

use crate::network::ClientConnection;

#[derive(Debug)]
pub struct Player {
    pub name: Option<String>,
    pub uuid: Option<Uuid>,
    pub state: PacketState,
    connection: ClientConnection,
}

impl Player {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            name: None,
            uuid: None,
            state: PacketState::None,
            connection: ClientConnection::new(stream),
        }
    }

    pub async fn handle(mut self, rsa: &RsaPrivateKey) -> anyhow::Result<()> {
        while let Ok(Some(packet)) = self.connection.read().await {
            match &packet.packetdata {
                PacketEnum::HandShake(_) => {}
                PacketEnum::Login(_) => {
                    let a = EncryptionRequest::new(&rsa.to_public_key())?.to_response(&packet);
                    let mut packet = RawPacket::from_bytes(&a, 0x01);
                    self.connection
                        .stream
                        .write_all_buf(&mut packet.data)
                        .await
                        .unwrap();
                }
                PacketEnum::EncryptionResponse(encryptionpacket) => {
                    let shared = encryptionpacket.decrypt_shared_secret(&rsa)?.clone();
                    let verify = encryptionpacket.decrypt_verify_token(&rsa)?.clone();
                    info!("{:?}", Bytes::copy_from_slice(&shared));

                    info!("{:?}", &verify);

                    let response =
                        LoginSuccessPacket::new("Rheydskey".to_string()).to_response(&packet);
                    let mut packet = RawPacket::from_bytes(&response, 0x02);
                    println!("{packet:?}");
                    self.connection
                        .stream
                        .write_all_buf(&mut packet.data)
                        .await?;
                }
                _ => todo!(),
            }
        }

        Ok(())
    }
}
