use super::ident::Ident;

pub struct BuiltinMacroCall {
    pub name: Ident,
    pub input: String,
}
