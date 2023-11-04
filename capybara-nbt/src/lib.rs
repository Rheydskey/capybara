pub mod mcregion;

use winnow::{
    binary::{be_f32, be_f64, be_i16, be_i32, be_i64, be_i8, be_u16, be_u8},
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
        let id = be_u8(bytes)?;

        if id == 0 {
            return Ok((id, String::new()));
        }

        let lenght = be_u16(bytes)?;
        let name = take(lenght).parse_next(bytes)?;
        let name = String::from_utf8(name.as_bytes().to_vec()).unwrap();

        Ok((id, name))
    }

    fn parse_byte<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        let value = be_i8.parse_next(bytes)?;
        Ok(TagInner::Byte(value))
    }

    fn parse_short<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        let value = be_i16.parse_next(bytes)?;
        Ok(TagInner::Short(value))
    }

    fn parse_int<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        let value = be_i32.parse_next(bytes)?;
        Ok(TagInner::Int(value))
    }

    fn parse_long<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        let value = be_i64.parse_next(bytes)?;
        Ok(TagInner::Long(value))
    }

    fn parse_float<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        let value = be_f32.parse_next(bytes)?;
        Ok(TagInner::Float(value))
    }

    fn parse_double<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        let value = be_f64.parse_next(bytes)?;
        Ok(TagInner::Double(value))
    }

    fn parse_byte_array<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        let value = be_i32.parse_next(bytes)?;
        let bytes = take(value.unsigned_abs()).parse_next(bytes)?;
        Ok(TagInner::Array(bytes.as_bytes().to_vec()))
    }

    fn parse_string<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        let value = be_u16.parse_next(bytes)?;
        let bytes = take(value).parse_next(bytes)?;

        Ok(TagInner::String(
            String::from_utf8(bytes.as_bytes().to_vec()).unwrap(),
        ))
    }

    fn parse_array_of<I, O, P, S>(
        mut size: S,
        mut parse: P,
    ) -> impl FnMut(&mut I) -> PResult<Vec<O>>
    where
        S: Parser<I, i32, ContextError>,
        P: Parser<I, O, ContextError>,
    {
        move |b: &mut I| -> PResult<Vec<O>> {
            let size = size.parse_next(b)?.unsigned_abs() as usize;
            let mut result = Vec::new();
            for _ in 0..size {
                let o = parse.parse_next(b)?;
                result.push(o);
            }

            Ok(result)
        }
    }

    fn parse_list<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        let id = be_u8(bytes)?;
        let parser = Self::tag_parse(id);
        let elements = Self::parse_array_of(be_i32, parser)(bytes)?;
        Ok(TagInner::List(elements))
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
        let elements = Self::parse_array_of(be_i32, be_i32)(bytes)?;
        Ok(TagInner::IntArray(elements))
    }

    fn parse_long_array<I>(bytes: &mut I) -> PResult<TagInner>
    where
        I: StreamIsPartial + Stream<Token = u8>,
        <I as Stream>::Slice: AsBytes,
    {
        let elements = Self::parse_array_of(be_i32, be_i64)(bytes)?;
        Ok(TagInner::LongArray(elements))
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
        let (id, name) = Self::parse_header(bytes)?;

        let tag = Self::tag_parse(id).parse_next(bytes)?;

        Ok(Self { name, tag })
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
