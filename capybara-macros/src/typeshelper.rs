use proc_macro2::{Delimiter, Group, Ident, Span};
use quote::quote;

use crate::Field;

pub struct VarInt;

impl VarInt {
    pub fn decode() -> Group {
        Group::new(
            Delimiter::None,
            quote!(VarInt::new().read_from_cursor(&mut bytes).unwrap()),
        )
    }

    pub fn encode(field: &Field) -> Group {
        let ident = proc_macro2::Ident::new(&field.ident, Span::call_site());
        Self::encode_group(&Group::new(Delimiter::None, quote!(self.#ident)))
    }

    pub fn encode_group(group: &Group) -> Group {
        Group::new(
            Delimiter::None,
            quote!(
                VarInt::encode(i32::try_from(#group).unwrap())
            ),
        )
    }
}

pub struct ArrayBytes;

impl ArrayBytes {
    pub fn decode() -> Group {
        Group::new(
            Delimiter::None,
            quote!(PacketBytes::from_cursor(&mut bytes).unwrap()),
        )
    }

    pub fn encode(field: &Field) -> (Group, Group) {
        let ident = proc_macro2::Ident::new(&field.ident, Span::call_site());

        let lenght = VarInt::encode_group(&Group::new(
            Delimiter::None,
            quote!(
                self.#ident.0.len()
            ),
        ));

        let bytes = Group::new(
            Delimiter::None,
            quote!(
                self.#ident.0
            ),
        );

        (lenght, bytes)
    }
}

pub struct StringHelper;

impl StringHelper {
    pub fn decode() -> Group {
        Group::new(
            Delimiter::None,
            quote!(PacketString::from_cursor(&mut bytes).unwrap().to_string()),
        )
    }

    pub fn encode(field: &Field) -> Group {
        let ident = Ident::new(&field.ident, Span::call_site());
        Group::new(Delimiter::None, quote!(PacketString::to_bytes(self.#ident)))
    }
}

pub struct VarLong;

impl VarLong {
    pub fn decode() -> Group {
        Group::new(
            Delimiter::None,
            quote!(VarLong::new().read_from_iter(&mut bytes).unwrap()),
        )
    }

    pub fn encode(field: &Field) -> Group {
        let ident = proc_macro2::Ident::new(&field.ident, Span::call_site());
        Self::encode_group(&Group::new(Delimiter::None, quote!(self.#ident)))
    }

    pub fn encode_group(group: &Group) -> Group {
        Group::new(
            Delimiter::None,
            quote!(
                VarLong::encode(i64::try_from(#group).unwrap())
            ),
        )
    }
}

pub struct U8helper;

impl U8helper {
    pub fn decode() -> Group {
        Group::new(Delimiter::None, quote!(bytes.get_u8()))
    }

    pub fn encode(field: &Field) -> Group {
        let ident = Ident::new(&field.ident, Span::call_site());
        Group::new(Delimiter::None, quote!(self.#ident))
    }
}

pub struct U16helper;

impl U16helper {
    pub fn decode() -> Group {
        Group::new(
            Delimiter::None,
            quote!(((u16::from(bytes.get_u8())) << 8) | u16::from(bytes.get_u8())),
        )
    }

    pub fn encode(field: &Field) -> Group {
        let ident = Ident::new(&field.ident, Span::call_site());
        Group::new(Delimiter::None, quote!(self.#ident.to_be_bytes()))
    }
}

pub struct BoolHelper;

impl BoolHelper {
    pub fn decode() -> Group {
        Group::new(
            Delimiter::None,
            quote!(*PacketBool::from_cursor(&mut bytes).unwrap()),
        )
    }

    pub fn encode(field: &Field) -> Group {
        let ident = Ident::new(&field.ident, Span::call_site());
        Group::new(Delimiter::None, quote!(u8::from(self.#ident)))
    }
}

pub struct UuidHelper;

impl UuidHelper {
    pub fn decode() -> Group {
        Group::new(
            Delimiter::None,
            quote!(PacketUUID::from_cursor(&mut bytes).to_uuid()),
        )
    }

    pub fn encode(field: &Field) -> Group {
        let ident = Ident::new(&field.ident, Span::call_site());
        Group::new(Delimiter::None, quote!(self.#ident.to_u128_le()))
    }
}
