use field::Field;
use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Ident, Span};

use syn::{
    parse_macro_input, AngleBracketedGenericArguments, Attribute, DataStruct, DeriveInput, Fields,
    GenericArgument, PathArguments, Type,
};
extern crate proc_macro;
use quote::{quote, ToTokens, TokenStreamExt};
use typeshelper::{I64Helper, StringHelper, U16helper, U8helper, UuidHelper, VarLong};

use crate::{
    field::FType,
    typeshelper::{ArrayBytes, BoolHelper, VarInt},
};

mod field;
mod typeshelper;

struct IntoResponse(Field);

impl ToTokens for IntoResponse {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self.0.attribute_type.as_str() {
            "varint" => Self::put_bytes_ts(tokens, &VarInt::encode(&self.0)),
            "arraybytes" => {
                let (lenght, val) = ArrayBytes::encode(&self.0);

                Self::put_bytes_ts(tokens, &lenght);

                Self::put_bytes_ts(tokens, &val);
            }
            "varlong" => Self::put_bytes_ts(tokens, &VarLong::encode(&self.0)),
            "string" => Self::put_bytes_ts(tokens, &StringHelper::encode(&self.0)),
            "u8" => Self::put_u8_ts(tokens, &U8helper::encode(&self.0)),
            "u16" => Self::put_bytes_ts(tokens, &U16helper::encode(&self.0)),
            "bool" => Self::put_u8_ts(tokens, &BoolHelper::encode(&self.0)),
            "uuid" => Self::put_u128_ts(tokens, &UuidHelper::encode(&self.0)),
            "i64" => Self::put_i64_ts(tokens, &I64Helper::encode(&self.0)),
            _ => unimplemented!(),
        };
    }
}

impl IntoResponse {
    pub fn put_bytes_ts(tokens: &mut proc_macro2::TokenStream, group: &proc_macro2::Group) {
        let put_slice = Group::new(Delimiter::None, quote!(bytes.put_slice(&#group);));

        tokens.append(put_slice);
    }

    pub fn put_u8_ts(tokens: &mut proc_macro2::TokenStream, group: &proc_macro2::Group) {
        let put_slice = Group::new(Delimiter::None, quote!(bytes.put_u8(#group);));

        tokens.append(put_slice);
    }

    pub fn put_u128_ts(tokens: &mut proc_macro2::TokenStream, group: &proc_macro2::Group) {
        let put_slice = Group::new(Delimiter::None, quote!(bytes.put_u128(#group);));

        tokens.append(put_slice);
    }

    pub fn put_i64_ts(tokens: &mut proc_macro2::TokenStream, group: &proc_macro2::Group) {
        let put_slice = Group::new(Delimiter::None, quote!(bytes.put_i64(#group);));

        tokens.append(put_slice);
    }
}

struct SelfFromBytes(Vec<FromBytes>);

impl SelfFromBytes {
    pub fn new(fields: &CapyField) -> Self {
        Self(fields.0.iter().map(|f| FromBytes(f.clone())).collect())
    }
}

impl ToTokens for SelfFromBytes {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let frombytes = &self.0;
        tokens.append(Group::new(
            Delimiter::None,
            quote!(Ok(Self {
                #(#frombytes)*
            })),
        ));
    }
}

#[derive(Clone)]
struct FromBytes(Field);

impl ToTokens for FromBytes {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(self.to_frombytes());
    }
}

impl FromBytes {
    pub fn to_frombytes(&self) -> Group {
        let group = match self.0.attribute_type.as_str() {
            "arraybytes" => ArrayBytes::decode(),
            "varint" => VarInt::decode(),
            "varlong" => VarLong::decode(),
            "string" => StringHelper::decode(),
            "u8" => U8helper::decode(),
            "u16" => U16helper::decode(),
            "bool" => BoolHelper::decode(),
            "uuid" => UuidHelper::decode(),
            "i64" => I64Helper::decode(),
            _ => unimplemented!("{}", self.0.attribute_type.as_str()),
        };

        let ident = Ident::new(&self.0.ident, Span::call_site());

        Group::new(Delimiter::None, quote!(#ident: #group,))
    }
}

struct Id(String);

impl Id {
    pub fn get_from(
        struct_name: &Ident,
        attrs: &[Attribute],
    ) -> Result<Self, proc_macro2::TokenStream> {
        let ids = attrs
            .iter()
            .map(|e| e.parse_args::<proc_macro2::TokenStream>().unwrap())
            .filter_map(|token| token.into_iter().last())
            .map(|f| {
                let proc_macro2::TokenTree::Literal(l) = f else {
                    return None;
                };

                Some(l.to_string())
            })
            .flatten()
            .collect::<Vec<String>>();

        let Some(id) = ids.first().cloned() else {
            return Err(
                syn::Error::new(struct_name.span(), "No id for this packet").to_compile_error()
            );
        };

        Ok(Self(id))
    }
}

impl ToTokens for Id {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        syn::LitInt::new(&self.0, Span::call_site()).to_tokens(tokens);
    }
}

struct CapyField(Vec<Field>);

impl CapyField {
    pub fn gentype_contains_type(barket: &AngleBracketedGenericArguments) -> bool {
        for i in &barket.args {
            if let GenericArgument::Type(_) = i {
                return true;
            }
        }

        false
    }

    pub fn gentype_to_vec(barket: &AngleBracketedGenericArguments) -> Vec<String> {
        barket
            .args
            .iter()
            .filter_map(|f| {
                let GenericArgument::Type(Type::Path(seg)) = f else {
                    return None;
                };

                let Some(ident) = seg.path.get_ident() else {
                    return None;
                };

                Some(ident.to_string())
            })
            .collect::<Vec<String>>()
    }

    pub fn new(ident: &Ident, fields: Fields) -> Self {
        let methods = fields
            .into_iter()
            .filter_map(|f| {
                let field_name = f.ident.unwrap().to_string();
                let Type::Path(typath) = f.ty else {
                    return None;
                };

                let fieldtype;
                if let Some(ident) = typath.path.get_ident() {
                    fieldtype = FType::NonGeneric(ident.to_string());
                } else if let Some(segment) = typath.path.segments.last() {
                    let toptype = segment.ident.to_string();
                    fieldtype = match &segment.arguments {
                        PathArguments::None => FType::NonGeneric(toptype),
                        PathArguments::AngleBracketed(gentype) => {
                            if Self::gentype_contains_type(gentype) {
                                FType::Generic(toptype, Self::gentype_to_vec(gentype))
                            } else {
                                FType::NonGeneric(toptype)
                            }
                        }
                        PathArguments::Parenthesized(_) => {
                            unimplemented!()
                        }
                    }
                } else {
                    println!("Error on {ident} | {field_name}");
                    return None;
                }

                let attribute_name = f.attrs.first()?.meta.path().get_ident()?.to_string();

                Some(Field {
                    ident: field_name,
                    field_type: fieldtype,
                    attribute_type: attribute_name,
                })
            })
            .collect::<Vec<Field>>();

        Self(methods)
    }
}

struct IntoResponses(Vec<IntoResponse>);

impl IntoResponses {
    pub fn new(fields: &CapyField) -> Self {
        let intoresponses = fields
            .0
            .iter()
            .map(|f| IntoResponse(f.clone()))
            .collect::<Vec<IntoResponse>>();

        Self(intoresponses)
    }
}

impl ToTokens for IntoResponses {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let to_res = &self.0;
        tokens.append(Group::new(
            Delimiter::None,
            quote!(
                    #(#to_res;)*
            ),
        ))
    }
}

/// # Panics
/// Panic when invalid data
#[proc_macro_derive(
    packet,
    attributes(varint, varlong, arraybytes, string, u8, u16, bool, uuid, i64, id)
)]
pub fn derive_packet(item: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(item);

    let id = match Id::get_from(&ident, &attrs) {
        Ok(value) => value,
        Err(err) => return err.into(),
    };

    let syn::Data::Struct(DataStruct { fields, .. }) = data else {
        unimplemented!("Derive macro work only on struct")
    };
    let capyfield = CapyField::new(&ident, fields);
    let to_res: IntoResponses = IntoResponses::new(&capyfield);
    let from_bytes: SelfFromBytes = SelfFromBytes::new(&capyfield);

    let output = quote! {
        #[automatically_derived]
        impl IntoResponse for #ident {
            fn id(&self) -> usize {
                return #id;
            }

            fn to_response(self, packet: &Packet) -> ::anyhow::Result<Bytes> {
                let mut bytes = ::bytes::BytesMut::new();

                #to_res

                Ok(bytes.freeze())
            }
        }

        #[automatically_derived]
        impl crate::PacketTrait for #ident {
            fn from_bytes(bytes: &::bytes::Bytes) -> ::anyhow::Result<Self> {
                let mut bytes = ::std::io::Cursor::new(&bytes[..]);

                #from_bytes
            }
        }
    };

    output.into()
}
