use bytes::BufMut;
use bytes::Bytes;
use bytes::BytesMut;
use capybara_packet_parser::VarInt;
use serde::Deserialize;
use serde::Serialize;

use rsa::RsaPrivateKey;

use crate::helper::PacketState;
use crate::IntoResponse;
use crate::Packet;

#[derive(Debug)]
pub struct RawPacket {
    pub lenght: i32,
    pub packetid: i32,
    pub data: Bytes,
}

impl RawPacket {
    pub fn read_lenght_given(bytes: &[u8], lenght: i32) -> anyhow::Result<Self> {
        let Ok((remain, packetid)) = VarInt::parse(bytes) else {
            return Err(anyhow::anyhow!("Cannot parse varint"));
        };

        Ok(Self {
            lenght,
            packetid,
            data: Bytes::copy_from_slice(&remain),
        })
    }

    pub fn from_bytes(bytes: &Bytes, packetid: i32) -> anyhow::Result<Self> {
        let mut bytespacketid = BytesMut::new();

        bytespacketid.put_slice(&VarInt::encode(packetid)?);

        let lenght = i32::try_from(bytespacketid.len() + bytes.len())?;

        let byteslenght = VarInt::encode(lenght)?;

        Ok(Self {
            lenght,
            packetid,
            data: Bytes::copy_from_slice(
                &[&byteslenght[..], &bytespacketid[..], &bytes[..]].concat(),
            ),
        })
    }

    pub fn from_intoresponse(
        toresponse: impl IntoResponse,
        packet: &Packet,
        packetid: i32,
    ) -> anyhow::Result<Self> {
        let bytes = toresponse.to_response(packet)?;

        Self::from_bytes(&bytes, packetid)
    }

    pub fn to_packet(self, player_status: &PacketState) -> anyhow::Result<Packet> {
        let mut packet = Packet::new();

        packet.parse_from_rawpacket(player_status, &self)?;

        Ok(packet)
    }

    pub fn build_from_packet(to_send_packet: impl IntoResponse) -> anyhow::Result<Self> {
        let packet = Packet::new();
        let id = to_send_packet.id();
        Self::from_bytes(&to_send_packet.to_response(&packet)?, i32::try_from(id)?)
    }
}

#[derive(Debug)]
pub struct State {
    pub rsa: rsa::RsaPrivateKey,
}

impl State {
    #[must_use]
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            rsa: RsaPrivateKey::new(&mut rng, 1024).unwrap(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Chat {
    SimpleText(Text),
    MultiText(Text),
}

impl Chat {
    pub fn to_string(self) -> anyhow::Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    text: String,

    #[serde(flatten)]
    component: Component,

    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<Vec<Text>>,
}

impl Text {
    pub fn new(str: &str) -> Self {
        Self {
            text: str.to_string(),
            component: Component::default(),
            extra: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Component {
    #[serde(skip_serializing_if = "Option::is_none")]
    bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    underlined: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    obfuscated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insertion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    click_event: Option<ClickEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hover_event: Option<HoverEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClickEvent {}

#[derive(Debug, Serialize, Deserialize)]
struct HoverEvent {}
