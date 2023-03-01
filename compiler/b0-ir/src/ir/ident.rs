use crate::error::{Error, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct Ident {
    pub ident: String,
}

impl Ident {
    pub fn new(s: &str) -> Result<Self> {
        Ok(Self {
            ident: String::from(s),
        })
    }

    pub fn push(&mut self, c: char) -> Result<()> {
        if c.is_alphanumeric() {
            self.ident.push(c);
            Ok(())
        } else {
            Err(Error::unexpect_token(c, "alphanumeric"))
        }
    }
}
