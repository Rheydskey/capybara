use proc_macro2::{Delimiter, Group, Ident, Span};
use quote::quote;

use crate::Field;

pub struct VarInt;

impl VarInt {
    pub fn encode(field: &Field) -> Group {
        let ident = proc_macro2::Ident::new(&field.ident, Span::call_site());
        Self::encode_group(&Group::new(Delimiter::None, quote!(self.#ident)))
    }

    pub fn encode_group(group: &Group) -> Group {
        Group::new(
            Delimiter::None,
            quote!(
                VarInt::encode(i32::try_from(#group)?)
            ),
        )
    }
}

pub struct ArrayBytes;

impl ArrayBytes {
    pub fn encode(field: &Field) -> (Group, Group) {
        let ident = proc_macro2::Ident::new(&field.ident, Span::call_site());

        let lenght = VarInt::encode_group(&Group::new(
            Delimiter::None,
            quote!(
                self.#ident.len()
            ),
        ));

        let bytes = Group::new(
            Delimiter::None,
            quote!(
                self.#ident
            ),
        );

        (lenght, bytes)
    }
}

pub struct StringHelper;

impl StringHelper {
    pub fn encode(field: &Field) -> Group {
        let ident = Ident::new(&field.ident, Span::call_site());
        Group::new(
            Delimiter::None,
            quote!(::nom_mcpacket::PacketString::encode(self.#ident)),
        )
    }
}

pub struct VarLong;

impl VarLong {
    pub fn encode(field: &Field) -> Group {
        let ident = proc_macro2::Ident::new(&field.ident, Span::call_site());
        Self::encode_group(&Group::new(Delimiter::None, quote!(self.#ident)))
    }

    pub fn encode_group(group: &Group) -> Group {
        Group::new(
            Delimiter::None,
            quote!(
                VarLong::encode(i64::try_from(#group)?)
            ),
        )
    }
}

pub struct U8helper;

impl U8helper {
    pub fn encode(field: &Field) -> Group {
        let ident = Ident::new(&field.ident, Span::call_site());
        Group::new(Delimiter::None, quote!(self.#ident))
    }
}

pub struct U16helper;

impl U16helper {
    pub fn encode(field: &Field) -> Group {
        let ident = Ident::new(&field.ident, Span::call_site());
        Group::new(Delimiter::None, quote!(self.#ident.to_be_bytes()))
    }
}

pub struct BoolHelper;

impl BoolHelper {
    pub fn encode(field: &Field) -> Group {
        let ident = Ident::new(&field.ident, Span::call_site());
        Group::new(Delimiter::None, quote!(u8::from(self.#ident)))
    }
}

pub struct UuidHelper;

impl UuidHelper {
    pub fn encode(field: &Field) -> Group {
        let ident = Ident::new(&field.ident, Span::call_site());
        Group::new(Delimiter::None, quote!(self.#ident.to_u128_le()))
    }
}

pub struct I64Helper;

impl I64Helper {
    pub fn encode(field: &Field) -> Group {
        let ident = Ident::new(&field.ident, Span::call_site());
        Group::new(Delimiter::None, quote!(self.#ident))
    }
}
