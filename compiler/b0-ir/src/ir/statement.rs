use super::{
    ident::Ident, literal::Literal, macros::BuiltinMacroCall, new_type::NewTypeInit, types::Types,
};

pub struct Block {
    pub stmts: Vec<Statement>,
}

pub enum Statement {
    Bound(Bound),
    Move(Move),
    FunctionCall(FunctionCall),
    MacroCall(BuiltinMacroCall),
}

pub enum RightValue {
    Literal(Literal),
    Ident(Ident),
    NewTypeInit(NewTypeInit),
}

pub struct Bound {
    pub left: Ident,
    pub ty: Types,
    pub right: RightValue,
}

pub struct Move {
    pub left: Ident,
    pub right: RightValue,
}

pub struct FunctionCall {
    pub name: Ident,
    pub args: Vec<RightValue>,
}
