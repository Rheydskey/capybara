use bytes::Bytes;
use capybara_packet::{
    helper::{PacketEnum, PacketState},
    DisconnectPacket, EncryptionRequest, Handshake, IntoResponse, Login, LoginSuccessPacket,
};
use rsa::RsaPrivateKey;
use tokio::net::TcpStream;
use uuid::Uuid;

use crate::network::ClientConnection;

#[derive(Debug)]
pub struct Player {
    pub name: Option<String>,
    pub uuid: Option<Uuid>,
    pub shared_key: Vec<u8>,
    pub state: PacketState,
    connection: ClientConnection,
}

impl Player {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            name: None,
            uuid: None,
            shared_key: vec![],
            state: PacketState::None,
            connection: ClientConnection::new(stream),
        }
    }

    pub async fn handle(mut self, rsa: &RsaPrivateKey) -> anyhow::Result<()> {
        while let Ok(Some(packet)) = self.connection.read(&self.state).await {
            match &packet.packetdata {
                PacketEnum::HandShake(Handshake {
                    protocol,
                    next_state,
                    ..
                }) => {
                    info!(
                        "New connection : Protocol : {} and next state is {}",
                        protocol, next_state
                    );
                    self.state = PacketState::Login;
                }
                PacketEnum::Login(Login {
                    name,
                    has_uuid,
                    uuid,
                }) => {
                    self.name = Some(name.clone());

                    if !*has_uuid {
                        return self
                            .connection
                            .send_packet(
                                &DisconnectPacket::from_reason("Not accepting crack account")
                                    .to_response(&packet)?,
                                0x0,
                            )
                            .await;
                    }

                    self.uuid = Some(uuid.clone());
                    let a = EncryptionRequest::new(&rsa.to_public_key())?.to_response(&packet)?;
                    self.connection.send_packet(&a, 0x01).await?;
                }

                PacketEnum::EncryptionResponse(encryptionpacket) => {
                    let shared = encryptionpacket.decrypt_shared_secret(&rsa)?.clone();
                    let verify = encryptionpacket.decrypt_verify_token(&rsa)?.clone();
                    info!("{:?}", Bytes::copy_from_slice(&shared));

                    info!("{:?}", &verify);

                    self.connection.set_encryption(&shared);

                    let response =
                        LoginSuccessPacket::new("Rheydskey".to_string()).to_response(&packet)?;

                    self.connection.send_packet(&response, 0x02).await?;
                }
                _ => todo!(),
            }
        }

        Ok(())
    }
}
