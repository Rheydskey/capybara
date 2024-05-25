pub mod mcregion;

use winnow::{
    binary::{
        be_f32, be_f64, be_i16, be_i32, be_i64, be_i8, be_u16, be_u8, length_repeat, length_take,
    },
    combinator::trace,
    error::ContextError,
    stream::{AsBytes, Stream, StreamIsPartial},
    token::take,
    PResult, Parser,
};

#[derive(Debug)]
pub struct RootCompound {
    pub tags: Vec<Tag>,
}

impl RootCompound {
    const fn empty() -> Self {
        Self { tags: Vec::new() }
    }
    fn parse_compound(bytes: &mut &[u8]) -> PResult<Self> {
        let mut tags = Vec::new();
        while !bytes.is_empty() {
            let tag = Tag::parse(bytes)?;
            tags.push(tag);
        }

        Ok(Self { tags })
    }

    pub fn parse(bytes: &mut &[u8]) -> PResult<Self> {
        Self::parse_compound(bytes)
    }
}

#[derive(Debug)]
pub struct Tag {
    pub name: String,
    pub tag: TagInner,
}

impl Tag {
    fn parse_header<I>(bytes: &mut I) -> PResult<(u8, String)>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        trace("Parse tag header", |bytes: &mut I| {
            let id = be_u8(bytes)?;

            if id == 0 {
                return Ok((id, String::new()));
            }

            let name = length_take(be_u16)
                .try_map(move |taken_bytes: <I as Stream>::Slice| {
                    String::from_utf8(taken_bytes.as_bytes().to_vec())
                })
                .parse_next(bytes)?;

            Ok((id, name))
        })
        .parse_next(bytes)
    }

    fn parse_byte<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        trace("Parse byte", |bytes: &mut I| {
            let value = be_i8.parse_next(bytes)?;
            Ok(TagInner::Byte(value))
        })
        .parse_next(bytes)
    }

    fn parse_short<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        trace("Parse short", |bytes: &mut I| {
            be_i16.map(TagInner::Short).parse_next(bytes)
        })
        .parse_next(bytes)
    }

    fn parse_int<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        trace("Parse int", |bytes: &mut I| {
            be_i32.map(TagInner::Int).parse_next(bytes)
        })
        .parse_next(bytes)
    }

    fn parse_long<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        trace("Parse long", |bytes: &mut I| {
            be_i64.map(TagInner::Long).parse_next(bytes)
        })
        .parse_next(bytes)
    }

    fn parse_float<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        trace("Parse float", |bytes: &mut I| {
            be_f32.map(TagInner::Float).parse_next(bytes)
        })
        .parse_next(bytes)
    }

    fn parse_double<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        trace("Parse double", |bytes: &mut I| {
            be_f64.map(TagInner::Double).parse_next(bytes)
        })
        .parse_next(bytes)
    }

    fn parse_byte_array<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        trace("Parse byte array", |bytes: &mut I| {
            length_take(be_i32.try_map(|f: i32| -> Result<u32, _> { f.try_into() }))
                .map(|f: <I as Stream>::Slice| TagInner::Array(f.as_bytes().to_vec()))
                .parse_next(bytes)
        })
        .parse_next(bytes)
    }

    fn parse_string<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        trace("Parse string", |bytes: &mut I| -> PResult<TagInner> {
            let value = be_u16.parse_next(bytes)?;

            take(value)
                .try_map(move |taken_bytes: <I as Stream>::Slice| {
                    String::from_utf8(taken_bytes.as_bytes().to_vec())
                })
                .map(TagInner::String)
                .parse_next(bytes)
        })
        .parse_next(bytes)
    }

    fn parse_list<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        trace("Parse list", |bytes: &mut I| {
            let id = be_u8(bytes)?;
            let parser = Self::tag_parse(id);

            length_repeat(Self::get_length, parser)
                .map(TagInner::List)
                .parse_next(bytes)
        })
        .parse_next(bytes)
    }

    #[allow(clippy::unnecessary_wraps)]
    fn parse_end<I>(_: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        Ok(TagInner::End)
    }

    fn parse_compound<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        let mut children = Vec::new();

        loop {
            let child = Self::parse(bytes)?;
            if matches!(child.tag, TagInner::End) {
                break;
            }

            children.push(Box::new(child));
        }

        Ok(TagInner::Compound(children))
    }

    fn parse_int_array<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        trace("Parse array of int", |bytes: &mut I| {
            let elements = length_repeat(Self::get_length, be_i32).parse_next(bytes)?;
            Ok(TagInner::IntArray(elements))
        })
        .parse_next(bytes)
    }

    fn parse_long_array<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        trace("Parse array of long", |bytes: &mut I| {
            let elements = length_repeat(Self::get_length, be_i64).parse_next(bytes)?;
            Ok(TagInner::LongArray(elements))
        })
        .parse_next(bytes)
    }

    pub fn get_length<I>(bytes: &mut I) -> PResult<usize>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        be_i32.map(|f| f as usize).parse_next(bytes)
    }

    #[must_use]
    /// Return parser of tag from id
    /// # Panics
    /// Will panic if an id don't have a parser
    pub fn tag_parse<I>(id: u8) -> impl Parser<I, TagInner, ContextError>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
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
                panic!("Wrong type");
            }
        }
    }

    pub fn parse<I>(bytes: &mut I) -> PResult<Self>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        trace("Parse tag", |i: &mut I| {
            let (id, name) = Self::parse_header(i)?;

            let tag = Self::tag_parse(id).parse_next(i)?;

            Ok(Self { name, tag })
        })
        .parse_next(bytes)
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
