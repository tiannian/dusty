use super::{ident::Ident, types::Types, Block};

pub struct FunctionDefineArgs {
    pub name: Ident,
    pub ty: Types,
}

pub struct FunctionDefine {
    pub name: Ident,
    pub args: Vec<FunctionDefineArgs>,
    pub rets: Option<Types>,
    pub block: Block,
}
