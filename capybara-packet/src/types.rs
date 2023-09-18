use bytes::Buf;
use bytes::BufMut;
use bytes::Bytes;
use bytes::BytesMut;
use capybara_macros::packet;
use serde::Deserialize;
use serde::Serialize;
use std::io::Cursor;
use std::iter::Iterator;

use rsa::RsaPrivateKey;

use crate::IntoResponse;
use crate::Packet;

#[derive(Debug)]
pub struct RawPacket {
    pub lenght: i32,
    pub packetid: i32,
    pub data: Bytes,
}

impl RawPacket {
    /// # Errors
    /// Return error if cannot read length or id of the packet
    pub fn read(bytes: &Bytes) -> anyhow::Result<Self> {
        let mut cursor = Cursor::new(bytes);
        let mut lenght = VarInt::new();
        let lenght = lenght.read_from_cursor_bytes(&mut cursor)?;

        let mut packetid = VarInt::new();
        let packetid = packetid.read_from_cursor_bytes(&mut cursor)?;
        Ok(Self {
            lenght,
            packetid,
            data: Bytes::copy_from_slice(&bytes[cursor.position() as usize..]),
        })
    }

    pub fn from_bytes(bytes: &Bytes, packetid: i32) -> Self {
        let mut bytespacketid = BytesMut::new();

        bytespacketid.put_slice(&VarInt::encode(packetid));

        let lenght = i32::try_from(bytespacketid.len() + bytes.len()).unwrap();

        let byteslenght = VarInt::encode(lenght);

        Self {
            lenght,
            packetid,
            data: Bytes::copy_from_slice(
                &[&byteslenght[..], &bytespacketid[..], &bytes[..]].concat(),
            ),
        }
    }

    pub fn from_intoresponse(
        toresponse: impl IntoResponse,
        packet: &Packet,
        packetid: i32,
    ) -> Self {
        let bytes = toresponse.to_response(packet).unwrap();

        Self::from_bytes(&bytes, packetid)
    }
}

#[derive(Debug)]
pub struct State {
    pub rsa: rsa::RsaPrivateKey,
}

impl State {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            rsa: RsaPrivateKey::new(&mut rng, 1024).unwrap(),
        }
    }
}

macro_rules! create_var_num {
    ($t:ty, $n:tt, $max_pos:expr) => {
        #[derive(Debug)]
        pub struct $n {
            pub result: $t,
            pub position: u8,
        }
        impl $n {
            const SEGMENT_BITS: $t = 0x7F;
            const CONTINUE_BIT: $t = 0x80;
            pub const fn new() -> Self {
                Self {
                    result: 0,
                    position: 0,
                }
            }

            pub fn try_with(&mut self, byte: u8) -> ::anyhow::Result<Option<$t>> {
                self.result |= ((byte.clone() as $t & Self::SEGMENT_BITS) << self.position);

                self.position += 7;

                if self.position >= $max_pos {
                    return Err(::anyhow::anyhow!("Too long").into());
                }

                if (byte.clone() as $t & Self::CONTINUE_BIT) == 0 {
                    return Ok(Some(self.result));
                }

                Ok(None)
            }

            pub fn read_from_bytes(&mut self, bytes: &Bytes) -> Option<$t> {
                while let Some(byte) = bytes.iter().next() {
                    match self.try_with(*byte) {
                        Ok(Some(result)) => return Some(result),
                        Ok(None) => {}
                        Err(_) => return None,
                    }
                }

                None
            }
            pub fn read_from_cursor(
                &mut self,
                cursor: &mut std::io::Cursor<&[u8]>,
            ) -> ::anyhow::Result<$t> {
                loop {
                    let byte = cursor.get_u8();

                    match self.try_with(byte) {
                        Ok(Some(result)) => return Ok(result),
                        Ok(None) => {}
                        Err(error) => return Err(error),
                    }
                }
            }

            pub fn read_from_cursor_bytes(
                &mut self,
                cursor: &mut std::io::Cursor<&Bytes>,
            ) -> ::anyhow::Result<$t> {
                loop {
                    let byte = cursor.get_u8();

                    self.result |= ((byte.clone() as $t & Self::SEGMENT_BITS) << self.position);

                    self.position += 7;

                    if self.position >= $max_pos {
                        return Err(::anyhow::anyhow!("Too long"));
                    }

                    if (byte.clone() as $t & Self::CONTINUE_BIT) == 0 {
                        return Ok(self.result);
                    }
                }
            }

            pub fn encode(mut value: $t) -> Vec<u8> {
                let mut buf: Vec<u8> = Vec::new();
                loop {
                    if (value & !Self::SEGMENT_BITS) == 0 {
                        buf.push(value as u8);
                        return buf;
                    }

                    buf.push(((value & Self::SEGMENT_BITS) | Self::CONTINUE_BIT) as u8);

                    value >>= 7;
                }
            }
        }
    };
}

create_var_num!(i32, VarInt, 32);
create_var_num!(i64, VarLong, 64);

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Chat {
    SimpleText(Text),
    MultiText(Text),
}

impl Chat {
    pub fn to_string(self) -> anyhow::Result<String> {
        return Ok(serde_json::to_string(&self)?);
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
        Text {
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
