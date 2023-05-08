pub use rand;
pub use rsa;

use bytes::Buf;
use bytes::Bytes;
use std::iter::Iterator;

use rsa::RsaPrivateKey;

use crate::network::ClientConnection;

#[derive(Debug)]
pub struct State {
    connection: Vec<ClientConnection>,
    pub rsa: rsa::RsaPrivateKey,
}

impl State {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            connection: Vec::new(),
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
