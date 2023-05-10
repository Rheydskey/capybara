#[derive(Debug, Clone)]
pub enum FType {
    NonGeneric(String),
    Generic(String, Vec<String>),
}

#[derive(Debug, Clone)]
pub struct Field {
    pub ident: String,
    pub field_type: FType,
    pub attribute_type: String,
}
