use bevy_ecs::component::Component;
pub use capybara_packet_parser;
pub use capybara_packet_serde;

pub mod tests;

pub mod helper;
pub mod types;

use capybara_packet_parser::{PacketUuid, Parsable, VarInt};
use rand::{thread_rng, Rng};
use rsa::{pkcs8::EncodePublicKey, Error, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use serde::{de::Visitor, ser::SerializeStruct, Deserialize, Serialize, Serializer};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
use thiserror::Error;
use types::{Chat, RawPacket, Text};

use crate::helper::parse_packet;
use helper::{PacketEnum, PacketState};

#[derive(Serialize, Debug, Clone)]
pub struct Uuid(pub uuid::Uuid);

impl<'de> Deserialize<'de> for Uuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct UuidVisitor;

        impl<'de> Visitor<'de> for UuidVisitor {
            type Value = uuid::Uuid;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("uuid")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let mut bytes = v;

                match PacketUuid::parse(&mut bytes) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        let error = capybara_packet_serde::Error::WinnowError(err);
                        Err(serde::de::Error::custom(error))
                    }
                }
            }
        }

        deserializer
            .deserialize_enum("uuid", &[], UuidVisitor)
            .map(Self)
    }
}

#[derive(Debug, Clone)]
pub struct ArrayBytes(pub Vec<u8>);

impl<'de> Deserialize<'de> for ArrayBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ArrayBytesVisitor;

        impl<'de> Visitor<'de> for ArrayBytesVisitor {
            type Value = Vec<u8>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ArrayBytes")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v.to_owned())
            }
        }

        deserializer
            .deserialize_enum("arraybytes", &[], ArrayBytesVisitor)
            .map(Self)
    }
}

impl Serialize for ArrayBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("", 3)?;

        state.serialize_field("lenght", &VarInt(self.0.len().try_into().unwrap()))?;
        state.serialize_field("bytes", &self.0)?;

        state.end()
    }
}

#[derive(Debug)]
pub struct Identifier(capybara_packet_parser::Identifier);

impl Identifier {
    #[must_use]
    pub const fn new(namespace: String, value: String) -> Self {
        Self(capybara_packet_parser::Identifier { namespace, value })
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: String = Deserialize::deserialize(deserializer)?;
        match capybara_packet_parser::Identifier::parse(&mut value.as_bytes()) {
            Ok(val) => Ok(Self(val)),
            Err(err) => Err(serde::de::Error::custom(
                capybara_packet_serde::Error::WinnowError(err),
            )),
        }
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Serializer::serialize_str(serializer, &self.0.to_string())
    }
}

#[macro_use]
extern crate log;

#[derive(Debug, Clone)]
pub struct Packet {
    pub lenght: i32,
    pub packetid: i32,
    pub packetdata: PacketEnum,
}

impl Packet {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            lenght: 0,
            packetid: 0,
            packetdata: PacketEnum::None,
        }
    }

    pub fn parse_from_rawpacket(
        &mut self,
        state: &PacketState,
        rawpacket: &RawPacket,
    ) -> anyhow::Result<()> {
        let packet = parse_packet(rawpacket.packetid, state, &rawpacket.data)?;

        info!("{rawpacket:?} => {packet:?}");

        self.packetdata = packet;

        Ok(())
    }
}

impl Default for Packet {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Bad Packet : CannotParse({0})")]
    CannotParse(i32),
    #[error("Unknow error")]
    Unknow,
}

pub trait Id {
    const ID: usize;

    #[must_use]
    fn id(&self) -> usize {
        Self::ID
    }
}

pub trait PacketTrait {
    /// # Errors
    /// Return if error if cannot parse packet
    fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self>
    where
        Self: Sized;
}

#[derive(Debug, Clone, Default, Deserialize, Component)]
pub struct Handshake {
    pub protocol: VarInt,
    pub address: String,
    pub port: u16,
    pub next_state: u8,
}

impl_id!(Handshake, 0x00);

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Login {
    pub name: String,
    pub uuid: Uuid,
}

impl_id!(Login, 0x00);

#[derive(Debug, Clone, Serialize)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub publickey: ArrayBytes,
    pub verify_token: ArrayBytes,
    should_auth: bool,
}

impl EncryptionRequest {
    /// # Errors
    /// Return error if cannot convert public key to DER format
    pub fn new(rsa: &RsaPublicKey) -> Result<Self, rsa::pkcs8::Error> {
        let key = rsa.to_public_key_der()?.to_vec();
        let mut rng = thread_rng();
        let mut token = [0; 4];
        rng.fill(&mut token[..]);
        info!("token: {token:?}");
        Ok(Self {
            server_id: String::new(),
            publickey: ArrayBytes(key),
            verify_token: ArrayBytes(token.to_vec()),
            should_auth: false,
        })
    }
}

impl_id!(EncryptionRequest, 0x01);

#[derive(Deserialize, Serialize, Debug, Clone, Component)]
pub struct EncryptionResponse {
    sharedsecret: Vec<u8>,
    verify_token: Vec<u8>,
}

impl EncryptionResponse {
    pub fn decrypt_verify_token(&self, rsa: &RsaPrivateKey) -> Result<Vec<u8>, Error> {
        rsa.decrypt(Pkcs1v15Encrypt, &self.verify_token)
    }

    /// # Errors
    /// Return errors if cannot decrypt from Rsa key
    pub fn decrypt_shared_secret(&self, rsa: &RsaPrivateKey) -> Result<Vec<u8>, Error> {
        rsa.decrypt(Pkcs1v15Encrypt, self.get_shared_secret())
    }

    #[must_use]
    pub const fn get_shared_secret(&self) -> &Vec<u8> {
        &self.sharedsecret
    }

    #[must_use]
    pub fn get_shared_secret_lenght(&self) -> usize {
        self.sharedsecret.len()
    }

    #[must_use]
    pub fn get_verify_token_lenght(&self) -> usize {
        self.verify_token.len()
    }
}

impl_id!(EncryptionResponse, 0x01);

#[derive(Debug)]
pub struct LoginSuccessPacket {
    uuid: Uuid,
    username: String,
    length_properties: VarInt,
    strict_error_handling: bool,
}

impl LoginSuccessPacket {
    #[must_use]
    pub fn new(username: String, uuid: uuid::Uuid) -> Self {
        assert!(username.len() <= 16);
        Self {
            uuid: Uuid(uuid),
            username,
            length_properties: VarInt(0),
            strict_error_handling: true,
        }
    }

    pub fn new_uuid_str(username: String, uuid: &str) -> anyhow::Result<Self> {
        Ok(Self::new(username, uuid::Uuid::from_str(uuid)?))
    }
}

impl_id!(LoginSuccessPacket, 0x02);

impl Serialize for LoginSuccessPacket {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("", 4)?;

        state.serialize_field("uuid", &self.uuid.0.to_bytes_le())?;
        state.serialize_field("username", &self.username)?;
        state.serialize_field("lenght_properties", &self.length_properties)?;
        state.serialize_field("strict_error_handling", &self.strict_error_handling)?;

        state.end()
    }
}

#[derive(Debug, Serialize)]
pub struct DisconnectPacket {
    reason: String,
}

impl DisconnectPacket {
    pub fn from_reason(reason: &str) -> anyhow::Result<Self> {
        let reason = Chat::SimpleText(Text::new(reason)).to_string()?;

        Ok(Self { reason })
    }
}

impl_id!(DisconnectPacket, 0x00);

#[derive(Serialize, Deserialize)]
pub struct Description {
    pub text: String,
}
#[derive(Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct Players {
    pub max: i64,
    pub online: i64,
    pub sample: Vec<Player>,
}
#[derive(Serialize, Deserialize)]
pub struct ServerVersion {
    pub name: String,
    pub protocol: i64,
}
#[derive(Serialize, Deserialize)]
pub struct ServerStatus {
    description: Description,
    enforces_secure_chat: bool,
    players: Players,
    previews_chat: bool,
    version: ServerVersion,
}

impl ServerStatus {
    #[must_use]
    pub const fn new(
        description: Description,
        enforces_secure_chat: bool,
        players: Players,
        previews_chat: bool,
        version: ServerVersion,
    ) -> Self {
        Self {
            description,
            enforces_secure_chat,
            players,
            previews_chat,
            version,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct StatusPacket {
    json_response: String,
}

impl StatusPacket {
    pub fn from_serializable(a: &impl serde::Serialize) -> Self {
        Self {
            json_response: serde_json::to_string(a).unwrap(),
        }
    }
}

impl Default for StatusPacket {
    fn default() -> Self {
        Self {
            json_response: String::from(
                r#"
{
    "version": {
        "name": "1.20.4",
        "protocol": 765
    },
    "players": {
        "max": 100,
        "online": 1,
        "sample": [
            {
                "name": "NoName",
                "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
            }
        ]
    },
    "description": {
        "text": "Capybara server"
    },
    "enforcesSecureChat": false,
    "previewsChat": true
}
"#,
            ),
        }
    }
}

impl_id!(StatusPacket, 0x00);

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Component)]
pub struct PingRequest {
    pub value: i64,
}

impl_id!(PingRequest, 0x01);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PlayLogin {
    pub entity_id: u32,
    pub is_hardcore: bool,
    pub dimension_names: Vec<Identifier>,
    pub max_player: VarInt,
    pub view_distance: VarInt,
    pub simulation_distance: VarInt,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub limited_crafting: bool,
    pub dimension_type: String,
    pub dimension_name: String,
    pub hashed_seed: u64,
    pub gamemode: u8,
    pub previous_gamemode: i8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death_location: Option<DeathLocation>,
    pub portal_cooldown: VarInt,
}

impl_id!(PlayLogin, 0x2B);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DeathLocation {
    dimension_name: Identifier,
    //    location: Location,
}

#[macro_export]
macro_rules! impl_id {
    ($name:ty, $value:expr) => {
        impl Id for $name {
            const ID: usize = $value;
        }
    };
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Component)]
pub struct StatusRequest;

impl_id!(StatusRequest, 0x0);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Component)]
pub struct LoginAcknowledged;

impl_id!(LoginAcknowledged, 0x3);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Component)]
pub struct ClientboundPluginMessage {
    pub channel: String,
    pub data: Vec<u8>,
}

impl_id!(ClientboundPluginMessage, 0x1);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Component)]
pub struct ClientInformation {
    locale: String,
    view_distance: u8,
    chat_mode: VarInt,
    chat_color: bool,
    displayed_skin: u8,
    main_hand: VarInt,
    enable_text_filtering: bool,
    allow_server_listing: bool,
}

impl_id!(ClientInformation, 0x0);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Component)]
pub struct FinishConfiguration;

impl_id!(FinishConfiguration, 0x3);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Component)]
pub struct FinishConfigAcknowledged;

impl_id!(FinishConfigAcknowledged, 0x3);
