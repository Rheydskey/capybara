use bytes::BufMut;
use bytes::Bytes;
use bytes::BytesMut;
use capybara_packet_parser::Parsable;
use capybara_packet_parser::VarInt;
use serde::Deserialize;
use serde::Serialize;

use crate::Id;

#[derive(Debug)]
pub struct RawPacket {
    pub lenght: i32,
    pub packetid: i32,
    pub data: Bytes,
}

impl RawPacket {
    pub fn read_lenght_given(bytes: &[u8], lenght: i32) -> anyhow::Result<Self> {
        let mut a = bytes;
        let Ok(packetid) = VarInt::parse(&mut a) else {
            return Err(anyhow::anyhow!("Cannot parse varint"));
        };

        Ok(Self {
            lenght,
            packetid,
            data: Bytes::copy_from_slice(a),
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

    pub fn build_from_serialize(to_send_packet: &(impl Serialize + Id)) -> anyhow::Result<Self> {
        let a = capybara_packet_serde::to_bytes(&to_send_packet)?;
        let id = to_send_packet.id();
        Self::from_bytes(&Bytes::copy_from_slice(a.as_slice()), i32::try_from(id)?)
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
    #[must_use]
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
