#[cfg(test)]
mod test;

use nom::{bytes::complete::take, combinator::map, IResult};
use uuid::Uuid;

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

pub struct VarInt(i32);

impl VarInt {
    const SEGMENT_BITS: i32 = 0x7F;
    const CONTINUE_BIT: i32 = 0x80;

    pub fn parse(bytes: &[u8]) -> IResult<&[u8], i32> {
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

            result |= (byte as i32 & Self::SEGMENT_BITS) << position;

            position += 7;

            if position >= 32 {
                return Err(nom::Err::Error(nom::error::Error {
                    input: remainder,
                    code: nom::error::ErrorKind::Fail,
                }));
            }

            if (byte as i32 & Self::CONTINUE_BIT) == 0 {
                return Ok((remainder, result));
            }
        }
    }

    pub fn encode(mut value: i32) -> Vec<u8> {
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

#[derive(Debug, Clone)]
pub struct PacketString(pub String);

impl PacketString {
    pub fn parse(bytes: &[u8]) -> IResult<&[u8], String> {
        let (input, value) = VarInt::parse(bytes)?;
        let take_bytes =
            nom::bytes::complete::take::<usize, &[u8], ()>(value.unsigned_abs() as usize);

        let transform_to_string = std::str::from_utf8;

        let Ok((input, value)) = take_bytes(input) else {
            return Err(nom::Err::Incomplete(nom::Needed::Unknown));
        };

        let Ok(string) = transform_to_string(value) else {
            return Err(nom::Err::Failure(nom::error::Error {
                input: value,
                code: nom::error::ErrorKind::Fail,
            }));
        };

        Ok((input, string.to_string()))
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

pub fn read_u8(bytes: &[u8]) -> IResult<&[u8], u8> {
    let Ok((remain, bytes)) = take::<usize, &[u8], ()>(1)(bytes) else {
        return Err(nom::Err::Incomplete(nom::Needed::Unknown));
    };

    Ok((remain, bytes[0]))
}

pub fn read_u16(bytes: &[u8]) -> IResult<&[u8], u16> {
    let Ok((remain, bytes)) = take::<usize, &[u8], ()>(2)(bytes) else {
        return Err(nom::Err::Incomplete(nom::Needed::Unknown));
    };

    Ok((remain, (u16::from(bytes[0]) << 8) | u16::from(bytes[1])))
}

pub fn read_i64(bytes: &[u8]) -> IResult<&[u8], i64> {
    let Ok((remain, bytes)) = take::<usize, &[u8], ()>(8)(bytes) else {
        return Err(nom::Err::Incomplete(nom::Needed::Unknown));
    };

    let Ok(bytes_into) = bytes.try_into() else {
        return Err(nom::Err::Failure(nom::error::Error {
            input: bytes,
            code: nom::error::ErrorKind::Fail,
        }));
    };

    Ok((remain, i64::from_be_bytes(bytes_into)))
}
