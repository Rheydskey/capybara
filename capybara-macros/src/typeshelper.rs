use proc_macro2::{Delimiter, Group, Span};
use quote::quote;

use crate::Field;

pub struct VarInt;

impl VarInt {
    pub fn decode() -> Group {
        Group::new(
            Delimiter::None,
            quote!(VarInt::new().read_from_iter(&mut bytes).unwrap()),
        )
    }

    pub fn encode(field: &Field) -> Group {
        let ident = proc_macro2::Ident::new(&field.ident, Span::call_site());
        Self::encode_group(Group::new(Delimiter::None, quote!(self.#ident)))
    }

    pub fn encode_group(group: Group) -> Group {
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

        let lenght = VarInt::encode_group(Group::new(
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

        println!("{bytes:?}");

        (lenght, bytes)
    }
}
