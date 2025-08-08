use std::fmt::{Display, Formatter};

type StdResult<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Generic(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Generic(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = StdResult<T>;

