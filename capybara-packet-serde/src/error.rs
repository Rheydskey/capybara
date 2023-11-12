use std::{
    fmt::{self, Display},
    num::TryFromIntError,
};

use capybara_packet_parser::winnow::error::{ContextError, ErrMode};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Eof,
    Message(String),
    WinnowError(ErrMode<ContextError>),
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Message(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Message(msg) => formatter.write_str(msg),
            Self::Eof => formatter.write_str("unexpected end of input"),
            Self::WinnowError(e) => formatter.write_str(&e.to_string()),
        }
    }
}

impl std::error::Error for Error {}

impl From<ErrMode<capybara_packet_parser::winnow::error::ContextError>> for Error {
    fn from(value: ErrMode<capybara_packet_parser::winnow::error::ContextError>) -> Self {
        Self::WinnowError(value)
    }
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self::Message(value.to_string())
    }
}

impl From<TryFromIntError> for Error {
    fn from(value: TryFromIntError) -> Self {
        Self::Message(value.to_string())
    }
}
