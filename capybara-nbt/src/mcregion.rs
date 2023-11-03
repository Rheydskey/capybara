use std::io::Read;

use flate2::bufread::ZlibDecoder;
use nom::{bytes::complete::take, multi::count, IResult};

use crate::{read_i32, read_u32, read_u64, read_u8, RootCompound};

#[derive(Debug)]
pub struct Location {
    offset: u32,
    size: u32,
}

impl Location {
    pub fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        let (remain, offset) = read_u32(bytes)?;

        let result = Self {
            offset: ((offset >> 8) & 0xFFFFFF) * 4096,
            size: (offset & 0xFF) * 4096,
        };

        Ok((remain, result))
    }
}

#[derive(Debug)]
pub struct Timestamps(u32);

impl Timestamps {
    pub fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        let (remain, timestamp) = read_u32(bytes)?;

        Ok((remain, Self(timestamp)))
    }
}

#[derive(Debug)]
pub struct Region {
    locations: Vec<Location>,
    timestamps: Vec<Timestamps>,
    chunks: Vec<Chunk>,
}

impl Region {
    pub fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        let (remain, locations) = count(Location::parse, 1024)(bytes)?;
        let (remain, timestamps) = count(Timestamps::parse, 1024)(remain)?;

        let mut chunks = Vec::new();
        for location in &locations {
            let Location { offset, size } = location;
            let chunk_data = &bytes[*offset as usize..(*offset + *size) as usize];

            if *size == 0 {
                chunks.push(Chunk {
                    lenght: 0,
                    compression_type: 2,
                    data: RootCompound::empty(),
                });
                continue;
            }

            let (_, a) = Chunk::parse(chunk_data)?;

            chunks.push(a);
        }

        Ok((
            remain,
            Self {
                locations,
                timestamps,
                chunks,
            },
        ))
    }
}

#[derive(Debug)]
pub struct Chunk {
    lenght: i32,
    compression_type: u8,
    data: RootCompound,
}

impl Chunk {
    pub fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        let (remain, lenght) = read_i32(bytes)?;

        let (remain, compression_type) = read_u8(remain)?;
        let take_compressed_data = take::<u32, &[u8], ()>(lenght.unsigned_abs());

        let (remain, data) = take_compressed_data(remain).unwrap();
        let mut zlib_decoder = ZlibDecoder::new(data);
        let mut chunk_data = Vec::new();

        zlib_decoder.read_to_end(&mut chunk_data).unwrap();

        let data = RootCompound::parse(&chunk_data);

        Ok((
            remain,
            Self {
                lenght,
                compression_type,
                data,
            },
        ))
    }
}
