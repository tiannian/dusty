use crate::{
    error::{Error, Result},
    ir::{Ident, ImportPath, Item},
};

pub enum Command {}

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Begin,
    FunctionOrNewType,
    MaybeImport,
    Import(ImportState),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ImportState {
    Begin,
    Path,
}

pub struct Parser {
    buff: String,
    state: State,
}

impl Parser {
    pub fn next(&mut self, c: char) -> Result<Item> {
        let s = &mut self.state;
        let buff = &mut self.buff;

        if c.is_whitespace() {
            return Ok(Item::Empty);
        }

        match s {
            State::Begin => {
                buff.clear();

                if c.is_alphanumeric() {
                    *s = State::FunctionOrNewType;
                    buff.push(c)
                } else if c == '=' {
                    *s = State::MaybeImport;
                } else {
                    return Err(Error::unexpect_token(c, "alphanumeric or ="));
                }
            }
            State::MaybeImport => {
                if c == '>' {
                    buff.clear();
                    *s = State::Import(ImportState::Begin);
                } else {
                    return Err(Error::unexpect_token(c, ">"));
                }
            }
            State::Import(ss) => match ss {
                ImportState::Begin => {
                    if c.is_alphanumeric() || c == ':' {
                        buff.push(c);
                        *ss = ImportState::Path;
                    } else {
                        return Err(Error::unexpect_token(c, "alphanumeric or ="));
                    }
                }
                ImportState::Path => {
                    if c == ';' {
                        *s = State::Begin;
                        let sp = buff.split("::");

                        let mut paths = Vec::new();
                        for path in sp {
                            let path = Ident::new(path)?;
                            paths.push(path);
                        }

                        return Ok(Item::Import(ImportPath { path: paths }));
                    }
                }
            },
            _ => return Err(Error::WrongState),
        }

        Ok(Item::Empty)
    }
}
