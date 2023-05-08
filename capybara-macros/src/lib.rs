use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenTree};
use syn::{
    parse_macro_input, AngleBracketedGenericArguments, DeriveInput, GenericArgument, PathArguments,
    Type,
};
extern crate proc_macro;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug)]
struct SimpleToken {
    token: proc_macro2::TokenStream,
}

impl SimpleToken {
    pub fn new() -> Self {
        Self {
            token: proc_macro2::TokenStream::new(),
        }
    }

    pub fn double_colon(self) -> Self {
        self.punct_joint(':').punct_joint(':')
    }

    pub fn ident(mut self, ident: &str, span: Span) -> Self {
        self.token.append(Ident::new(ident, span));
        self
    }

    pub fn ident_callsite(self, ident: &str) -> Self {
        self.ident(ident, Span::call_site())
    }

    pub fn punct_joint(mut self, punct: char) -> Self {
        self.token.append(Punct::new(punct, Spacing::Joint));
        self
    }

    pub fn create_group<F>(self, fnonce: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        println!("{self:?}");
        self.append_group(fnonce(Self::new()).into_group())
    }

    pub fn create_group_demiliter<F>(self, fnonce: F, delimiter: Delimiter) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        println!("{self:?}");
        self.append_group(fnonce(Self::new()).into_group_delimiter(delimiter))
    }

    pub fn append_group(mut self, group: Group) -> Self {
        self.token.append(group);
        self
    }

    pub fn into_group_delimiter(self, delimiter: Delimiter) -> Group {
        Group::new(delimiter, self.token)
    }

    pub fn into_group(self) -> Group {
        self.into_group_delimiter(Delimiter::Parenthesis)
    }

    pub fn append_vec<T>(mut self, vec: Vec<T>) -> Self
    where
        T: ToTokens,
    {
        for v in vec {
            v.to_tokens(&mut self.token);
        }

        self
    }

    pub fn append<U>(mut self, tt: U) -> Self
    where
        U: Into<TokenTree>,
    {
        self.token.append(tt);
        self
    }
}

struct VarInt;

impl VarInt {
    pub fn decode() -> Group {
        SimpleToken::new()
            .ident_callsite("VarInt")
            .double_colon()
            .ident_callsite("new")
            .create_group(|st| st)
            .punct_joint('.')
            .ident_callsite("read_from_iter")
            .create_group(|st| {
                st.create_group_demiliter(
                    |st| st.punct_joint('&').ident_callsite("mut"),
                    Delimiter::None,
                )
                .create_group_demiliter(|st| st.ident_callsite("bytes"), Delimiter::None)
            })
            .punct_joint('.')
            .ident_callsite("unwrap")
            .create_group(|st| st)
            .into_group_delimiter(Delimiter::None)
    }

    pub fn encode(field: &Field) -> Group {
        Self::encode_group(
            SimpleToken::new()
                .ident_callsite("self")
                .punct_joint('.')
                .ident_callsite(&field.ident)
                .into_group_delimiter(Delimiter::None),
        )
    }

    pub fn encode_group(group: Group) -> Group {
        SimpleToken::new()
            .ident_callsite("VarInt")
            .punct_joint(':')
            .punct_joint(':')
            .ident_callsite("encode")
            .create_group(move |st| {
                st.ident_callsite("i32")
                    .double_colon()
                    .ident_callsite("try_from")
                    .append_group(group)
                    .punct_joint('.')
                    .ident_callsite("unwrap")
                    .create_group(|st| st)
            })
            .into_group_delimiter(Delimiter::None)
    }
}

struct ArrayBytes;

impl ArrayBytes {
    pub fn decode() -> Group {
        SimpleToken::new()
            .ident_callsite("PacketBytes")
            .double_colon()
            .ident_callsite("from_iterator")
            .create_group(|st| {
                st.create_group_demiliter(
                    |st| st.punct_joint('&').ident_callsite("mut"),
                    Delimiter::None,
                )
                .ident_callsite("bytes")
            })
            .punct_joint('.')
            .ident_callsite("unwrap")
            .create_group_demiliter(|st| st, Delimiter::Parenthesis)
            .into_group_delimiter(Delimiter::None)
    }

    pub fn encode(field: &Field) -> (Group, Group) {
        let lenght = VarInt::encode_group(
            SimpleToken::new()
                .ident_callsite("self")
                .punct_joint('.')
                .ident_callsite(&field.ident)
                .punct_joint('.')
                .append(proc_macro2::Literal::usize_unsuffixed(0))
                .punct_joint('.')
                .ident_callsite("len")
                .create_group(|st| st)
                .into_group(),
        );

        let bytes = SimpleToken::new()
            .ident_callsite("self")
            .punct_joint('.')
            .ident_callsite(&field.ident)
            .punct_joint('.')
            .append(proc_macro2::Literal::usize_unsuffixed(0))
            .into_group();

        (lenght, bytes)
    }
}

#[derive(Debug, Clone)]
enum FieldType {
    NonGeneric(String),
    Generic(String, Vec<String>),
}

#[derive(Debug, Clone)]
struct Field {
    ident: String,
    field_type: FieldType,
    attr_type: String,
}

struct IntoResponse(Field);

impl ToTokens for IntoResponse {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self.0.attr_type.as_str() {
            "varint" => Self::put_bytes_ts(tokens, VarInt::encode(&self.0)),
            "arraybytes" => {
                let (lenght, val) = ArrayBytes::encode(&self.0);

                Self::put_bytes_ts(tokens, lenght);

                Self::put_bytes_ts(tokens, val);
            }
            _ => unimplemented!(),
        };

        println!("{tokens}");
    }
}

impl IntoResponse {
    pub fn put_bytes_ts(tokens: &mut proc_macro2::TokenStream, group: proc_macro2::Group) {
        let put_slice = SimpleToken::new()
            .ident_callsite("bytes")
            .punct_joint('.')
            .ident_callsite("put_slice")
            .create_group_demiliter(
                |st| st.punct_joint('&').append_group(group),
                Delimiter::Parenthesis,
            )
            .punct_joint(';')
            .into_group_delimiter(Delimiter::None);

        tokens.append(put_slice);
    }
}

struct SelfFromBytes(Vec<FromBytes>);

impl ToTokens for SelfFromBytes {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(
            SimpleToken::new()
                .ident_callsite("Ok")
                .create_group(|st| {
                    st.ident_callsite("Self").create_group_demiliter(
                        |st| st.append_vec(self.0.clone()),
                        Delimiter::Brace,
                    )
                })
                .into_group_delimiter(Delimiter::None),
        );
    }
}

#[derive(Clone)]
struct FromBytes(Field);

impl ToTokens for FromBytes {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(
            SimpleToken::new()
                .append_group(self.to_frombytes())
                .into_group_delimiter(Delimiter::None),
        );
    }
}

impl FromBytes {
    pub fn to_frombytes(&self) -> Group {
        SimpleToken::new()
            .ident_callsite(&self.0.ident)
            .punct_joint(':')
            .create_group_demiliter(
                |st| match self.0.attr_type.as_str() {
                    "arraybytes" => st.append_group(ArrayBytes::decode()),
                    "varint" => st.append_group(VarInt::decode()),
                    _ => unimplemented!("{}", self.0.attr_type.as_str()),
                },
                Delimiter::None,
            )
            .punct_joint(',')
            .into_group_delimiter(Delimiter::None)
    }
}

/// # Panics
/// Panic when invalid data
#[proc_macro_derive(packet, attributes(varint, varlong, arraybytes))]
pub fn derive_packet(item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(item);

    println!("{ident}");
    let gentype_contains_type = |barket: &AngleBracketedGenericArguments| -> bool {
        for i in &barket.args {
            if let GenericArgument::Type(_) = i {
                return true;
            }
        }

        false
    };

    let gentype_to_vec = |barket: &AngleBracketedGenericArguments| -> Vec<String> {
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
    };

    let syn::Data::Struct(ds) = data else {
      unimplemented!()
    };

    let methods: Vec<Field> = ds
        .fields
        .into_iter()
        .filter_map(|f| {
            let field_name = f.ident.unwrap().to_string();
            let Type::Path(typath) = f.ty else {
                    return None;
                };

            let fieldtype;
            if let Some(ident) = typath.path.get_ident() {
                fieldtype = FieldType::NonGeneric(ident.to_string());
            } else if let Some(segment) = typath.path.segments.last() {
                let toptype = segment.ident.to_string();
                fieldtype = match &segment.arguments {
                    PathArguments::None => FieldType::NonGeneric(toptype),
                    PathArguments::AngleBracketed(gentype) => {
                        if gentype_contains_type(gentype) {
                            FieldType::Generic(toptype, gentype_to_vec(gentype))
                        } else {
                            FieldType::NonGeneric(toptype)
                        }
                    }
                    PathArguments::Parenthesized(test) => {
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
                attr_type: attribute_name,
            })
        })
        .collect::<Vec<Field>>();
    let to_res: Vec<IntoResponse> = methods.iter().map(|f| IntoResponse(f.clone())).collect();
    let from_bytes: SelfFromBytes =
        SelfFromBytes(methods.iter().map(|f| FromBytes(f.clone())).collect());

    let output = quote! {
        #[automatically_derived]
        impl IntoResponse for #ident {
            fn to_response(self, state: &Arc<State>, packet: &Packet) -> Bytes {
                let mut bytes = BytesMut::new();
                #(#to_res;)*

                bytes.freeze()
            }
        }

        #[automatically_derived]
        impl crate::PacketTrait for #ident {
            fn from_bytes(bytes: &::bytes::Bytes) -> Result<Self, crate::PacketError> {
                let mut bytes = bytes.iter();

                #from_bytes
            }
        }
    };

    output.into()
}
