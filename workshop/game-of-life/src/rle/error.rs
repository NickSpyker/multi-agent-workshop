use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum ParseError {
    MissingHeader,
    InvalidHeader(String),
    InvalidPattern(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingHeader => write!(f, "Missing header line (x = ..., y = ...)"),
            Self::InvalidHeader(msg) => write!(f, "Invalid header: {msg}"),
            Self::InvalidPattern(msg) => write!(f, "Invalid pattern: {msg}"),
        }
    }
}

impl Error for ParseError {}
