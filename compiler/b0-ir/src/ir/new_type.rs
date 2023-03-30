use super::{ident::Ident, literal::Literal, macros::BuiltinMacroCall, types::Types};

pub enum NewTypeInitValue {
    Literal(Literal),
    Ident(Ident),
}

pub struct NewTypeInitKV {
    pub key: Ident,
    pub value: NewTypeInitValue,
}

pub struct NewTypeInit {
    pub kvs: Vec<NewTypeInitKV>,
}

pub enum NewTypeDefineType {
    Types(Types),
    MacroCall(BuiltinMacroCall),
}

pub struct NewTypeDefine {
    pub name: Option<Ident>,
    pub ty: Option<NewTypeDefineType>,
}
