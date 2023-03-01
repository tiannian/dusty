use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

use crate::{error::Result, ir::Item};

pub enum Command {}

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Begin,
    FunctionOrNewType,
    MaybeImport,
}

pub struct Parser {
    file: File,
}

impl Parser {
    pub fn parse(self) -> Result<Item> {
        let mut s = State::Begin;

        let reader = BufReader::new(self.file);

        let mut buff = String::new();

        for b in reader.lines() {
            let b = b?;

            for c in b.chars() {
                if s == State::Begin && c.is_alphanumeric() {
                    buff.push(c);
                    s = State::FunctionOrNewType;
                } else if s == State::FunctionOrNewType && c.is_alphanumeric() {
                    buff.push(c);
                    s = State::FunctionOrNewType;
                } else if s == State::Begin && c == '=' {
                    s = State::MaybeImport
                }
            }
        }

        Ok(Item::Empty)
    }
}
