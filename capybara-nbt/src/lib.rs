pub mod mcregion;

use nom::{bytes::complete::take, IResult, Needed, Parser};
use std::num::{NonZeroU16, NonZeroUsize};

#[derive(Debug)]
pub struct RootCompound {
    pub tags: Vec<Tag>,
}

impl RootCompound {
    fn empty() -> Self {
        Self { tags: Vec::new() }
    }
    fn parse_compound(bytes: &[u8]) -> IResult<&[u8], Self> {
        let mut remainder = bytes;
        let mut tags = Vec::new();
        while !remainder.is_empty() {
            let (remain, tag) = Tag::parse(bytes)?;
            tags.push(tag);
            remainder = remain;
        }

        Ok((remainder, Self { tags }))
    }

    #[must_use]
    pub fn parse(bytes: &[u8]) -> Self {
        let (_, value) = Self::parse_compound(bytes).unwrap();

        value
    }
}

#[derive(Debug)]
pub struct Tag {
    pub name: String,
    pub tag: TagInner,
}

impl Tag {
    fn parse_header(bytes: &[u8]) -> IResult<&[u8], (u8, String)> {
        let (remain, value) = nom::bytes::complete::take::<usize, &[u8], ()>(1)(bytes).unwrap();
        let id = value[0];

        if id == 0 {
            return Ok((remain, (id, String::new())));
        }

        let (remain, value) = nom::bytes::complete::take::<usize, &[u8], ()>(2)(remain).unwrap();

        let lenght = u16::from_be_bytes(value.try_into().unwrap());

        let Ok((remain, name)) = nom::bytes::complete::take::<u16, &[u8], ()>(lenght)(remain)
        else {
            return Err(nom::Err::Incomplete(
                NonZeroU16::new(lenght)
                    .map_or(Needed::Unknown, |f| Needed::Size(NonZeroUsize::from(f))),
            ));
        };

        let name = String::from_utf8(name.to_vec()).unwrap();

        Ok((remain, (id, name)))
    }

    fn parse_byte(bytes: &[u8]) -> IResult<&[u8], TagInner> {
        let (remain, value) = read_i8(bytes)?;
        Ok((remain, TagInner::Byte(value)))
    }

    fn parse_short(bytes: &[u8]) -> IResult<&[u8], TagInner> {
        let (remain, value) = read_i16(bytes)?;
        Ok((remain, TagInner::Short(value)))
    }

    fn parse_int(bytes: &[u8]) -> IResult<&[u8], TagInner> {
        let (remain, value) = read_i32(bytes)?;
        Ok((remain, TagInner::Int(value)))
    }

    fn parse_long(bytes: &[u8]) -> IResult<&[u8], TagInner> {
        let (remain, value) = read_i64(bytes)?;
        Ok((remain, TagInner::Long(value)))
    }

    fn parse_float(bytes: &[u8]) -> IResult<&[u8], TagInner> {
        let (remain, value) = read_f32(bytes)?;
        Ok((remain, TagInner::Float(value)))
    }

    fn parse_double(bytes: &[u8]) -> IResult<&[u8], TagInner> {
        let (remain, value) = read_f64(bytes)?;
        Ok((remain, TagInner::Double(value)))
    }

    fn parse_byte_array(bytes: &[u8]) -> IResult<&[u8], TagInner> {
        let (remain, value) = read_i32(bytes)?;
        let (remain, bytes) = take::<u32, &[u8], ()>(value.unsigned_abs())(remain).unwrap();

        Ok((remain, TagInner::Array(bytes.to_vec())))
    }

    fn parse_string(bytes: &[u8]) -> IResult<&[u8], TagInner> {
        let (remain, value) = read_u16(bytes)?;
        let (remain, bytes) = take::<u16, &[u8], ()>(value)(remain).unwrap();

        Ok((
            remain,
            TagInner::String(String::from_utf8(bytes.to_vec()).unwrap()),
        ))
    }

    fn parse_array_of<I, O, E1, P>(
        size_parser: fn(I) -> IResult<I, i32, E1>,
        mut parse: P,
    ) -> impl FnMut(I) -> IResult<I, Vec<O>, E1>
    where
        P: Parser<I, O, E1>,
        I: Clone,
    {
        move |b: I| -> IResult<I, Vec<O>, E1> {
            let (remain, size) = size_parser(b)?;
            let mut result = Vec::new();
            let mut remainder = remain;
            for _ in 0..size.unsigned_abs() {
                let (i, o) = parse.parse(remainder)?;
                remainder = i;
                result.push(o);
            }

            Ok((remainder, result))
        }
    }

    fn parse_list(bytes: &[u8]) -> IResult<&[u8], TagInner> {
        let (remain, value) = take::<usize, &[u8], ()>(1)(bytes).unwrap();
        let parser = Self::tag_parse(value[0]);
        let (remain, elements) = Self::parse_array_of(read_i32, parser)(remain)?;
        Ok((remain, TagInner::List(elements)))
    }

    const fn parse_end(bytes: &[u8]) -> IResult<&[u8], TagInner> {
        Ok((bytes, TagInner::End))
    }

    fn parse_compound(bytes: &[u8]) -> IResult<&[u8], TagInner> {
        let mut children = Vec::new();
        let mut remainder = bytes;

        while !remainder.is_empty() {
            let (remain, child) = Self::parse(remainder)?;
            remainder = remain;
            if matches!(child.tag, TagInner::End) {
                break;
            }

            children.push(Box::new(child));
        }

        Ok((remainder, TagInner::Compound(children)))
    }

    fn parse_int_array(bytes: &[u8]) -> IResult<&[u8], TagInner> {
        let (remain, elements) = Self::parse_array_of(read_i32, read_i32)(bytes)?;
        Ok((remain, TagInner::IntArray(elements)))
    }

    fn parse_long_array(bytes: &[u8]) -> IResult<&[u8], TagInner> {
        let (remain, elements) = Self::parse_array_of(read_i32, read_i64)(bytes)?;
        Ok((remain, TagInner::LongArray(elements)))
    }

    #[must_use]
    pub fn tag_parse(id: u8) -> fn(&[u8]) -> IResult<&[u8], TagInner> {
        match id {
            0 => Self::parse_end,
            1 => Self::parse_byte,
            2 => Self::parse_short,
            3 => Self::parse_int,
            4 => Self::parse_long,
            5 => Self::parse_float,
            6 => Self::parse_double,
            7 => Self::parse_byte_array,
            8 => Self::parse_string,
            9 => Self::parse_list,
            10 => Self::parse_compound,
            11 => Self::parse_int_array,
            12 => Self::parse_long_array,
            _ => {
                println!("Other type");
                panic!("Wrong type");
            }
        }
    }

    pub fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        let (remain, (id, name)) = Self::parse_header(bytes)?;

        let (remain, tag) = Self::tag_parse(id)(remain)?;

        Ok((remain, Self { name, tag }))
    }
}

#[derive(Debug)]
pub enum TagInner {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Array(Vec<u8>),
    String(String),
    List(Vec<TagInner>),
    Compound(Vec<Box<Tag>>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

crate::handler_number!(read_i8, i8, 1);
crate::handler_number!(read_u8, u8, 1);
crate::handler_number!(read_i16, i16, 2);
crate::handler_number!(read_u16, u16, 2);
crate::handler_number!(read_i32, i32, 4);
crate::handler_number!(read_u32, u32, 4);
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
    };
}
