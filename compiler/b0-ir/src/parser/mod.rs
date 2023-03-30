use crate::{
    error::{Error, Result},
    ir::{Ident, ImportPath, Item, NewTypeDefineType, Types},
};

pub enum Command {}

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Begin,
    FunctionOrNewType,
    MaybeImport,
    Import(ImportState),
    NewType(Ident),
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

                if c.is_alphanumeric() || c == '_' {
                    *s = State::FunctionOrNewType;
                    buff.push(c)
                } else if c == '=' {
                    *s = State::MaybeImport;
                } else {
                    return Err(Error::unexpect_token(c, "idnet or ="));
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
                    if c.is_alphanumeric() || c == ':' || c == '_' {
                        buff.push(c);
                        *ss = ImportState::Path;
                    } else {
                        return Err(Error::unexpect_token(c, "ident or ="));
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

            State::FunctionOrNewType => {
                if c.is_alphanumeric() || c == '_' {
                    buff.push(c)
                } else if c == '{' {
                    *s = State::NewType(Ident::new(&buff)?);
                } else if c == '(' {
                    // function
                } else {
                    return Err(Error::unexpect_token(c, "ident, { or ("));
                }
            }
            State::NewType(ident) => {
                let patten = "_:!&$()";
                if c.is_alphanumeric() || patten.contains(c) {
                    buff.push(c);
                } else if c == '}' {
                    // End new type
                } else {
                    return Err(Error::unexpect_token(c, "ident , or }"));
                }
            }
        }

        Ok(Item::Empty)
    }
}

// fn build_new_type_t(s: &str) -> Result<NewTypeDefineType> {
//     #[derive(Debug, PartialEq, Eq)]
//     enum State {
//         Begin,
//         Macro,
//         RefMut,
//         Ref,
//     }
//
//     let mut buff = String::new();
//     let mut state = State::Begin;
//
//     for c in s.chars() {
//         if c.is_whitespace() {
//             continue;
//         }
//
//         if state == State::Begin && (c.is_alphanumeric() || c == '_') {
//             buff.push(c);
//         } else if c == '!' {
//             state = State::Macro;
//         } else if c == '$' {
//             state = State::RefMut;
//         } else if c == '&' {
//             state = State::Ref;
//         } else {
//             return Err(Error::unexpect_token(c, ""));
//         }
//     }
//
//     Ok(())
// }
