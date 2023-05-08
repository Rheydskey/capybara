use bytes::Buf;
use bytes::BufMut;
use bytes::Bytes;
use bytes::BytesMut;
use std::io::Cursor;
use std::iter::Iterator;

use rsa::RsaPrivateKey;

#[derive(Debug)]
pub struct RawPacket {
    pub lenght: i32,
    pub packetid: i32,
    pub data: Bytes,
}

impl RawPacket {
    pub fn read(bytes: &Bytes) -> Self {
        let mut cursor = Cursor::new(bytes);
        let mut lenght = VarInt::new();
        let lenght = lenght.read_from_cursor_bytes(&mut cursor).unwrap();

        let mut packetid = VarInt::new();
        let packetid = packetid.read_from_cursor_bytes(&mut cursor).unwrap();

        Self {
            lenght,
            packetid,
            data: Bytes::copy_from_slice(&bytes[cursor.position() as usize..]),
        }
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

            pub fn read_from_iter<'a, T>(&mut self, iter: &mut T) -> Option<$t>
            where
                T: Iterator<Item = &'a u8>,
            {
                while let Some(byte) = iter.next() {
                    self.result |= ((byte.clone() as $t & Self::SEGMENT_BITS) << self.position);

                    self.position += 7;

                    if self.position >= $max_pos {
                        panic!("Too long")
                    }

                    if (byte.clone() as $t & Self::CONTINUE_BIT) == 0 {
                        return Some(self.result);
                    }
                }

                None
            }
            pub fn read_from_bytes(&mut self, bytes: &Bytes) -> Option<$t> {
                while let Some(byte) = bytes.iter().next() {
                    self.result |= ((byte.clone() as $t & Self::SEGMENT_BITS) << self.position);

                    self.position += 7;

                    if self.position >= $max_pos {
                        panic!("Too long")
                    }

                    if (byte.clone() as $t & Self::CONTINUE_BIT) == 0 {
                        return Some(self.result);
                    }
                }

                None
            }
            pub fn read_from_cursor(&mut self, cursor: &mut std::io::Cursor<&[u8]>) -> Option<$t> {
                while let byte = cursor.get_u8() {
                    self.result |= ((byte.clone() as $t & Self::SEGMENT_BITS) << self.position);

                    self.position += 7;

                    if self.position >= $max_pos {
                        panic!("Too long")
                    }

                    if (byte.clone() as $t & Self::CONTINUE_BIT) == 0 {
                        return Some(self.result);
                    }
                }
                None
            }

            pub fn read_from_cursor_bytes(
                &mut self,
                cursor: &mut std::io::Cursor<&Bytes>,
            ) -> Option<$t> {
                while let byte = cursor.get_u8() {
                    self.result |= ((byte.clone() as $t & Self::SEGMENT_BITS) << self.position);

                    self.position += 7;

                    if self.position >= $max_pos {
                        panic!("Too long")
                    }

                    if (byte.clone() as $t & Self::CONTINUE_BIT) == 0 {
                        return Some(self.result);
                    }
                }
                None
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
