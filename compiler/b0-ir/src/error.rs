use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    StdIo(#[from] std::io::Error),

    #[error("Wrong state")]
    WrongState,

    #[error("Unexpect token: {0}, expect: {1}")]
    UnexpectToken(char, String),
}

impl Error {
    pub fn unexpect_token(c: char, expect: &str) -> Self {
        Self::UnexpectToken(c, String::from(expect))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
