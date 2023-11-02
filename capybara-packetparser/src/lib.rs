#[cfg(test)]
mod test;

use std::marker::PhantomData;

use nom::{bytes::complete::take, combinator::map, IResult};
use uuid::Uuid;

pub trait Parsable {
    type Target;

    fn parse(bytes: &[u8]) -> IResult<&[u8], Self::Target>;
}

pub struct PacketUuid(uuid::Uuid);

impl PacketUuid {
    pub fn parse(bytes: &[u8]) -> IResult<&[u8], Uuid> {
        let (bytes, uuid) = map(
            nom::number::streaming::u128(nom::number::Endianness::Big),
            uuid::Uuid::from_u128,
        )(bytes)?;

        Ok((bytes, uuid))
    }
}

impl Parsable for PacketUuid {
    type Target = Uuid;

    fn parse(bytes: &[u8]) -> IResult<&[u8], Self::Target> {
        let (bytes, uuid) = map(
            nom::number::streaming::u128(nom::number::Endianness::Big),
            uuid::Uuid::from_u128,
        )(bytes)?;

        Ok((bytes, uuid))
    }
}

#[macro_export]
macro_rules! create_var_number {
    ($n:tt, $t:ty, $max_pos:expr) => {
        pub struct $n;

        impl $n {
            const SEGMENT_BITS: $t = 0x7F;
            const CONTINUE_BIT: $t = 0x80;

            pub fn parse(bytes: &[u8]) -> IResult<&[u8], $t> {
                let mut remainder = bytes;
                let mut result = 0;
                let mut position = 0;
                loop {
                    let byte = match nom::bytes::complete::take::<usize, &[u8], ()>(1)(remainder) {
                        Ok((remain, bytes)) => {
                            remainder = remain;
                            bytes[0]
                        }
                        Err(_) => return Err(nom::Err::Incomplete(nom::Needed::Unknown)),
                    };

                    result |= (<$t>::from(byte) & Self::SEGMENT_BITS) << position;

                    position += 7;

                    if position >= $max_pos {
                        return Err(nom::Err::Error(nom::error::Error {
                            input: remainder,
                            code: nom::error::ErrorKind::Fail,
                        }));
                    }

                    if (<$t>::from(byte) & Self::CONTINUE_BIT) == 0 {
                        return Ok((remainder, result));
                    }
                }
            }

            pub fn encode(mut value: $t) -> anyhow::Result<Vec<u8>> {
                let mut buf: Vec<u8> = Vec::new();
                loop {
                    if (value & !Self::SEGMENT_BITS) == 0 {
                        buf.push(u8::try_from(value)?);
                        return Ok(buf);
                    }

                    buf.push(u8::try_from(
                        (value & Self::SEGMENT_BITS) | Self::CONTINUE_BIT,
                    )?);

                    value >>= 7;
                }
            }
        }

        impl Parsable for $n {
            type Target = $t;

            fn parse(bytes: &[u8]) -> IResult<&[u8], Self::Target> {
                Self::parse(bytes)
            }
        }
    };
}

crate::create_var_number!(VarLong, i64, 64);
crate::create_var_number!(VarInt, i32, 32);

pub fn transform_to_string(bytes: &[u8]) -> IResult<&[u8], String> {
    let Ok(string) = std::str::from_utf8(bytes) else {
        return Err(nom::Err::Failure(nom::error::Error {
            input: bytes,
            code: nom::error::ErrorKind::Fail,
        }));
    };

    Ok((bytes, string.to_string()))
}

#[derive(Debug, Clone)]
pub struct PacketString(pub String);

impl PacketString {
    pub fn parse(bytes: &[u8]) -> IResult<&[u8], String> {
        let (input, value) = VarInt::parse(bytes)?;
        let take_bytes =
            nom::bytes::complete::take::<usize, &[u8], ()>(value.unsigned_abs() as usize);

        let Ok((input, value)) = take_bytes(input) else {
            return Err(nom::Err::Incomplete(nom::Needed::Unknown));
        };

        let (bytes, string) = transform_to_string(value)?;

        Ok((input, string))
    }

    pub fn encode(string: &str) -> anyhow::Result<Vec<u8>> {
        let mut bytes = Vec::new();

        bytes.append(&mut VarInt::encode(i32::try_from(string.len())?)?);
        bytes.extend_from_slice(string.as_bytes());

        Ok(bytes)
    }
}

impl Parsable for PacketString {
    type Target = String;

    fn parse(bytes: &[u8]) -> IResult<&[u8], Self::Target> {
        PacketString::parse(bytes)
    }
}

pub struct PacketBool(bool);

impl PacketBool {
    pub fn parse(bytes: &[u8]) -> IResult<&[u8], bool> {
        let Ok((remain, bytes)) = nom::bytes::complete::take::<usize, &[u8], ()>(1)(bytes) else {
            return Err(nom::Err::Incomplete(nom::Needed::Unknown));
        };

        Ok((remain, bytes[0] == 0x01))
    }
}

impl Parsable for PacketBool {
    type Target = bool;

    fn parse(bytes: &[u8]) -> IResult<&[u8], Self::Target> {
        PacketBool::parse(bytes)
    }
}

pub struct PacketBytes(Vec<u8>);

impl PacketBytes {
    pub fn parse(bytes: &[u8]) -> IResult<&[u8], Vec<u8>> {
        let (input, value) = VarInt::parse(bytes)?;
        let take_bytes =
            nom::bytes::complete::take::<usize, &[u8], ()>(value.unsigned_abs() as usize);

        let Ok((input, value)) = take_bytes(input) else {
            return Err(nom::Err::Incomplete(nom::Needed::Unknown));
        };

        Ok((input, value.to_vec()))
    }
}

impl Parsable for PacketBytes {
    type Target = Vec<u8>;

    fn parse(bytes: &[u8]) -> IResult<&[u8], Self::Target> {
        PacketBytes::parse(bytes)
    }
}

crate::handler_number!(read_u8, u8, 1);
crate::handler_number!(read_i8, i8, 1);
crate::handler_number!(read_i16, i16, 2);
crate::handler_number!(read_u16, u16, 2);
crate::handler_number!(read_i32, i32, 4);
crate::handler_number!(read_u64, u64, 8);
crate::handler_number!(read_i64, i64, 8);
crate::handler_number!(read_f32, f32, 4);
crate::handler_number!(read_f64, f64, 8);

#[macro_export]
macro_rules! handler_number {
    ($name:tt, $type:ty, $nbbytes:expr) => {
        pub fn $name(bytes: &[u8]) -> IResult<&[u8], $type> {
            let Ok((remain, bytes)) = take::<usize, &[u8], ()>($nbbytes)(bytes) else {
                return Err(nom::Err::Incomplete(nom::Needed::Unknown));
            };

            let Ok(bytes_into) = bytes.try_into() else {
                return Err(nom::Err::Failure(nom::error::Error {
                    input: bytes,
                    code: nom::error::ErrorKind::Fail,
                }));
            };

            Ok((remain, <$type>::from_be_bytes(bytes_into)))
        }

        impl Parsable for $type {
            type Target = $type;

            fn parse(bytes: &[u8]) -> IResult<&[u8], $type> {
                $name(bytes)
            }
        }
    };
}

pub struct PacketBoolOption<T> {
    _phantom: PhantomData<T>,
}

impl<T: Parsable> PacketBoolOption<T> {
    pub fn parse(bytes: &[u8]) -> IResult<&[u8], Option<T::Target>> {
        let (remain, is_some) = PacketBool::parse(bytes)?;

        if is_some {
            let (remain, result) = T::parse(remain)?;

            return Ok((remain, Some(result)));
        }

        Ok((remain, None))
    }
}

#[derive(Debug)]
struct Angle(u8);

impl Angle {
    pub fn get_degree(&self) -> f64 {
        f64::from(self.0) / 256.
    }
}

impl Parsable for Angle {
    type Target = u8;

    fn parse(bytes: &[u8]) -> IResult<&[u8], Self::Target> {
        read_u8(bytes)
    }
}

struct Position(u64);

impl Position {
    fn x(&self) -> i32 {
        return 0;
    }
    fn y(&self) -> i16 {
        return 0;
    }
    fn z(&self) -> i32 {
        return 0;
    }
}

impl Parsable for Position {
    type Target = Self;

    fn parse(bytes: &[u8]) -> IResult<&[u8], Self::Target> {
        let (bytes, num) = read_u64(bytes)?;

        Ok((bytes, Self(num)))
    }
}

pub struct Identifier {
    namespace: String,
    value: String,
}

impl Parsable for Identifier {
    type Target = Self;

    fn parse(bytes: &[u8]) -> IResult<&[u8], Self::Target> {
        let (remain, namespace) = nom::bytes::complete::take_while(is_namespace_valid)(bytes)?;
        let (remain, _) = nom::bytes::complete::tag(":")(remain)?;
        let (remain, value) = nom::bytes::complete::take_while1(is_value_valid)(remain)?;

        let (_, namespace) = transform_to_string(namespace)?;

        let (_, value) = transform_to_string(value)?;

        if namespace.is_empty() {
            return Ok((
                remain,
                Self {
                    namespace: "minecraft".to_string(),
                    value,
                },
            ));
        }

        Ok((remain, Self { namespace, value }))
    }
}

#[inline]
pub fn is_namespace_valid(chr: u8) -> bool {
    (chr >= b'a' && chr <= b'z')
        || (chr >= b'0' && chr <= b'9')
        || chr == b'.'
        || chr == b'-'
        || chr == b'_'
}

#[inline]
pub fn is_value_valid(chr: u8) -> bool {
    is_namespace_valid(chr) || chr == b'/'
}
