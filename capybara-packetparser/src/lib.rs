#[cfg(test)]
mod test;

use std::marker::PhantomData;
use winnow::{
    binary::{be_u64, be_u8},
    error::AddContext,
    token::{take, take_while},
};

use uuid::Uuid;
use winnow::{
    binary::be_u128,
    stream::{AsBytes, Stream, StreamIsPartial},
    PResult, Parser,
};

pub use winnow;

pub trait Parsable {
    type Target;

    fn parse<I>(bytes: &mut I) -> PResult<Self::Target>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes;
}

pub struct PacketUuid(uuid::Uuid);

impl Parsable for PacketUuid {
    type Target = Uuid;

    fn parse<I>(bytes: &mut I) -> PResult<Self::Target>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        let a = be_u128.parse_next(bytes)?;
        let uuid = uuid::Uuid::from_u128(a);

        Ok(uuid)
    }
}

#[macro_export]
macro_rules! create_var_number {
    ($n:tt, $t:ty, $max_pos:expr, $parser:expr) => {
        pub struct $n;

        impl $n {
            const SEGMENT_BITS: $t = 0x7F;
            const CONTINUE_BIT: $t = 0x80;

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

            fn parse<I>(bytes: &mut I) -> PResult<Self::Target>
            where
                I: StreamIsPartial + Stream<Token = u8>,
                <I as Stream>::Slice: AsBytes,
            {
                let mut result = 0;
                let mut position = 0;
                loop {
                    let byte = be_u8.parse_next(bytes)?;

                    result |= (<$t>::from(byte) & Self::SEGMENT_BITS) << position;

                    position += 7;

                    if position >= $max_pos {
                        return Err(winnow::error::ErrMode::Cut(
                            winnow::error::ContextError::new()
                                .add_context(bytes, winnow::error::StrContext::Label("Too long")),
                        ));
                    }

                    if (<$t>::from(byte) & Self::CONTINUE_BIT) == 0 {
                        return Ok(result);
                    }
                }
            }
        }
    };
}

crate::create_var_number!(VarLong, i64, 64, be_i64);
crate::create_var_number!(VarInt, i32, 32, be_i32);

pub fn transform_to_string(bytes: &&[u8]) -> PResult<String> {
    let Ok(string) = std::str::from_utf8(bytes) else {
        return Err(winnow::error::ErrMode::Cut(
            winnow::error::ContextError::new().add_context(
                bytes,
                winnow::error::StrContext::Label("Cannot convert to String"),
            ),
        ));
    };

    Ok(string.to_string())
}

#[derive(Debug, Clone)]
pub struct PacketString(pub String);

impl PacketString {
    pub fn encode(string: &str) -> anyhow::Result<Vec<u8>> {
        let mut bytes = Vec::new();

        bytes.append(&mut VarInt::encode(i32::try_from(string.len())?)?);
        bytes.extend_from_slice(string.as_bytes());

        Ok(bytes)
    }
}

impl Parsable for PacketString {
    type Target = String;

    fn parse<I>(bytes: &mut I) -> PResult<Self::Target>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        let value = VarInt::parse(bytes)?;
        let mut take_bytes = take(value.unsigned_abs() as usize);

        let value = take_bytes.parse_next(bytes)?;

        let string = transform_to_string(&value.as_bytes())?;

        Ok(string)
    }
}

pub struct PacketBool(bool);

impl Parsable for PacketBool {
    type Target = bool;

    fn parse<I>(bytes: &mut I) -> PResult<Self::Target>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        Ok(be_u8(bytes)? == 0x01)
    }
}

pub struct PacketBytes(Vec<u8>);

impl Parsable for PacketBytes {
    type Target = Vec<u8>;

    fn parse<I>(bytes: &mut I) -> PResult<Self::Target>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        let length = VarInt::parse(bytes)?;
        let values = take(length.unsigned_abs() as usize).parse_next(bytes)?;

        Ok(values.as_bytes().to_vec())
    }
}

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
    pub fn parse(bytes: &mut &[u8]) -> PResult<Option<T::Target>> {
        if PacketBool::parse(bytes)? {
            let result = T::parse(bytes)?;

            return Ok(Some(result));
        }

        Ok(None)
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

    fn parse<I>(bytes: &mut I) -> PResult<Self::Target>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        be_u8(bytes)
    }
}

struct Position(u64);

impl Position {
    const fn x() -> i32 {
        0
    }
    const fn y() -> i16 {
        0
    }
    const fn z() -> i32 {
        0
    }
}

impl Parsable for Position {
    type Target = Self;

    fn parse<I>(bytes: &mut I) -> PResult<Self::Target>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        let num = be_u64(bytes)?;

        Ok(Self(num))
    }
}

pub struct Identifier {
    namespace: String,
    value: String,
}

impl Parsable for Identifier {
    type Target = Self;

    fn parse<I>(bytes: &mut I) -> PResult<Self::Target>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        let namespace = take_while(1.., is_namespace_valid).parse_next(bytes)?;
        let _ = b':'.parse_next(bytes)?;
        let value = take_while(1.., is_value_valid).parse_next(bytes)?;

        let namespace = transform_to_string(&namespace.as_bytes())?;

        let value = transform_to_string(&value.as_bytes())?;

        if namespace.is_empty() {
            return Ok(Self {
                namespace: "minecraft".to_string(),
                value,
            });
        }

        Ok(Self { namespace, value })
    }
}

#[inline]
#[must_use]
pub const fn is_namespace_valid(chr: u8) -> bool {
    chr.is_ascii_lowercase() || chr.is_ascii_digit() || chr == b'.' || chr == b'-' || chr == b'_'
}

#[inline]
#[must_use]
pub const fn is_value_valid(chr: u8) -> bool {
    is_namespace_valid(chr) || chr == b'/'
}
