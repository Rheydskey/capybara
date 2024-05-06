use std::io::Read;

use flate2::bufread::ZlibDecoder;
use winnow::{
    binary::{be_i32, be_u32, be_u8, length_repeat},
    error::{AddContext, ContextError},
    stream::Stream,
    PResult, Parser,
};

use crate::RootCompound;

#[derive(Debug)]
pub struct Location {
    offset: u32,
    size: u32,
}

impl Location {
    pub fn parse(bytes: &mut &[u8]) -> PResult<Self> {
        let offset = be_u32(bytes)?;

        let result = Self {
            offset: ((offset >> 8) & 0x00FF_FFFF) * 4096,
            size: (offset & 0xFF) * 4096,
        };

        Ok(result)
    }
}

#[derive(Debug)]
pub struct Timestamps(u32);

impl Timestamps {
    pub fn parse(bytes: &mut &[u8]) -> PResult<Self> {
        let timestamp = be_u32(bytes)?;

        Ok(Self(timestamp))
    }

    pub fn get_timestamps(&self) -> u32 {
        self.0
    }
}

#[derive(Debug)]
pub struct Region {
    pub locations: Vec<Location>,
    pub timestamps: Vec<Timestamps>,
    pub chunks: Vec<Chunk>,
}

impl Region {
    pub fn parse(bytes: &mut &[u8]) -> PResult<Self> {
        let locations: Vec<Location> =
            length_repeat(|_: &mut _| -> PResult<usize> { Ok(1024) }, Location::parse)
                .parse_next(bytes)?;

        let timestamps = length_repeat(
            |_: &mut _| -> PResult<usize> { Ok(1024) },
            Timestamps::parse,
        )
        .parse_next(bytes)?;

        let mut chunks = Vec::new();
        for Location { offset, size } in &locations {
            if *size == 0 {
                chunks.push(Chunk {
                    lenght: 0,
                    compression_type: 2,
                    data: RootCompound::empty(),
                });
                continue;
            }

            let mut chunk_data = &bytes[(offset - 8192) as usize..(offset + size - 8192) as usize];

            chunks.push(Chunk::parse(&mut chunk_data)?);
        }

        Ok(Self {
            locations,
            timestamps,
            chunks,
        })
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub lenght: i32,
    pub compression_type: u8,
    pub data: RootCompound,
}

impl Chunk {
    pub fn parse(bytes: &mut &[u8]) -> PResult<Self> {
        let lenght = be_i32(bytes)?;
        let compression_type = be_u8(bytes)?;
        let data = winnow::token::take::<u32, &[u8], ContextError>(lenght.unsigned_abs())
            .parse_next(bytes)?;

        let mut zlib_decoder = ZlibDecoder::new(data);
        let mut chunk_data = Vec::new();

        if zlib_decoder.read_to_end(&mut chunk_data).is_err() {
            return Err(winnow::error::ErrMode::Cut(
                ContextError::new().add_context(
                    bytes,
                    &data.checkpoint(),
                    winnow::error::StrContext::Label("Correct zlib arraybytes"),
                ),
            ));
        }

        let mut slice_data = chunk_data.as_slice();

        let data = RootCompound::parse(&mut slice_data)?;

        Ok(Self {
            lenght,
            compression_type,
            data,
        })
    }
}
