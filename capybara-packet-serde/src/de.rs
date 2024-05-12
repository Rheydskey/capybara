use crate::Error;
use capybara_packet_parser::winnow::binary;
use capybara_packet_parser::{PacketBytes, PacketString, Parsable, VarInt};
use serde::de::{MapAccess, SeqAccess};
use serde::Deserialize;

pub struct Deserializer<'de> {
    input: &'de [u8],
    id: u32,
}

impl<'de> Deserializer<'de> {
    #[must_use]
    pub const fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer { input, id: 0 }
    }

    pub fn parse_bool(&mut self) -> crate::Result<bool> {
        Ok(capybara_packet_parser::PacketBool::parse(&mut self.input)?)
    }

    crate::impl_num!(parse_i8, i8, binary::be_i8);
    crate::impl_num!(parse_i16, i16, binary::be_i16);
    crate::impl_num!(parse_i32, i32, binary::be_i32);
    crate::impl_num!(parse_i64, i64, binary::be_i64);
    crate::impl_num!(parse_u8, u8, binary::be_u8);
    crate::impl_num!(parse_u16, u16, binary::be_u16);
    crate::impl_num!(parse_u32, u32, binary::be_u32);
    crate::impl_num!(parse_u64, u64, binary::be_u64);
    crate::impl_num!(parse_f32, f32, binary::be_f32);
    crate::impl_num!(parse_f64, f64, binary::be_f64);

    pub fn parse_str(&mut self) -> crate::Result<String> {
        Ok(PacketString::parse(&mut self.input)?)
    }

    pub fn parse_bytes(&mut self) -> crate::Result<Vec<u8>> {
        Ok(PacketBytes::parse(&mut self.input)?)
    }

    pub fn parse_varint_with_read(&mut self) -> crate::Result<(&[u8], i32)> {
        let v = self.input;
        let a = VarInt::parse(&mut self.input)?;

        Ok((&v[0..(v.len() - self.input.len())], a))
    }

    pub fn parse_read_uuid(&mut self) -> crate::Result<&[u8]> {
        let a = Ok(&self.input[0..16]);
        self.input = &self.input[16..];
        a
    }
    pub fn parse_varint(&mut self) -> crate::Result<i32> {
        Ok(VarInt::parse(&mut self.input)?)
    }
}

#[macro_export]
macro_rules! impl_num {
    ($name:tt, $type:ty, $winnow_type:expr ) => {
        pub fn $name(&mut self) -> $crate::Result<$type> {
            Ok($winnow_type(&mut self.input)?)
        }
    };
}

pub fn from_bytes<'a, T>(bytes: &'a [u8]) -> crate::Result<T>
where
    T: Deserialize<'a> + std::fmt::Debug,
{
    T::deserialize(&mut Deserializer::from_bytes(bytes))
}

impl<'de, 'a> serde::de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i8(self.parse_i8()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i16(self.parse_i16()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i32(self.parse_i32()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i64(self.parse_i64()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u8(self.parse_u8()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u16(self.parse_u16()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u32(self.parse_u32()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u64(self.parse_u64()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f32(self.parse_f32()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f64(self.parse_f64()?)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_string(self.parse_str()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_string(self.parse_str()?)
    }

    fn deserialize_bytes<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!();
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.parse_bool()? {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(self, _: &'static str, v: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        v.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let size = self.parse_varint()?;
        visitor.visit_seq(&mut PacketHelper::new(
            self,
            Some(size.unsigned_abs() as usize),
        ))
    }

    fn deserialize_tuple<V>(self, _: usize, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _: &'static str,
        _: usize,
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(&mut PacketHelper::new(self, None))
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        _field: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if name == "VarInt" {
            return visitor.visit_bytes(self.parse_varint_with_read()?.0);
        }

        if name == "uuid" {
            return visitor.visit_bytes(self.parse_read_uuid()?);
        }

        Err(Error::Message(format!("Unknown variant {}", name)))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let id = self.id;
        self.id += 1;
        visitor.visit_u32(id)
    }

    fn deserialize_ignored_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        println!("{:?}", self.input);
        Err(Error::Message("Ignored".to_string()))
    }
}

// https://gitlab.com/mcrs/serde_mcpacket/-/blob/main/src/de.rs
struct PacketHelper<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    size: Option<usize>,
}

impl<'a, 'de> PacketHelper<'a, 'de> {
    pub fn new(de: &'a mut Deserializer<'de>, size: Option<usize>) -> Self {
        Self { de, size }
    }
}

impl<'de, 'a> SeqAccess<'de> for PacketHelper<'a, 'de> {
    type Error = crate::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.de.input.is_empty() {
            return Ok(None);
        }
        if let Some(remaining) = self.size {
            if remaining == 0 {
                return Ok(None);
            }

            let a = seed.deserialize(&mut *self.de).map(Some);
            if a.is_ok() {
                self.size = Some(remaining - 1);
            }

            return a;
        }

        Ok(None)
    }
}

impl<'de, 'a> MapAccess<'de> for PacketHelper<'a, 'de> {
    type Error = crate::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.de.input.is_empty() {
            return Ok(None);
        }

        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}
