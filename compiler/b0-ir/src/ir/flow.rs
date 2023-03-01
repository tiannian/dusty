use super::{Block, Statement};

pub enum Flow {
    If(If),
    Loop(Loop),
}

pub struct If {
    pub cond: Statement,
    pub t: Option<Block>,
    pub f: Option<Block>,
    pub case: Vec<Block>,
}

pub struct Loop {
    pub begin: Option<Statement>,
    pub end: Option<Statement>,
    pub step: Option<Statement>,
}
