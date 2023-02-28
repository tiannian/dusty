use crate::error::Result;

pub struct Ident {
    pub ident: String,
}

impl Ident {
    pub fn new(s: &str) -> Result<Self> {
        Ok(Self {
            ident: String::from(s),
        })
    }
}
