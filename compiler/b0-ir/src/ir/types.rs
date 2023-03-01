use super::ident::Ident;

pub enum Types {
    Type(Ident),
    Array(Ident, u32),
    TypeRef(Ident),
    Slice(Ident),
}
