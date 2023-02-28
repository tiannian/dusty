use super::{ident::Ident, literal::Literal};

pub enum Statement {
    Bound,
    Move,
    FunctionCall,
}

pub enum RightValue {
    Literal(Literal),
    Ident(Ident),
}

pub struct Bound {
    pub left: Ident,
    pub ty: Ident,
    pub right: RightValue,
}

pub struct Move {}

pub struct FunctionCall {}
