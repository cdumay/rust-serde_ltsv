use std::error::Error;
use std::fmt;

/// structure to store errors
#[derive(Debug)]
pub enum LtsvError {
    InvalidInput(String),
    DeserializerError(serde_value::DeserializerError),
    SerializerError(serde_value::SerializerError),
    FromUtf8Error(std::string::FromUtf8Error),
}

impl fmt::Display for LtsvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LtsvError::InvalidInput(data) => write!(f, "{}", data),
            LtsvError::DeserializerError(err) => write!(f, "{}", err),
            LtsvError::SerializerError(err) => write!(f, "{}", err),
            LtsvError::FromUtf8Error(err) => write!(f, "{}", err),
        }
    }
}

impl From<std::string::FromUtf8Error> for LtsvError {
    fn from(err: std::string::FromUtf8Error) -> LtsvError {
        LtsvError::FromUtf8Error(err)
    }
}

impl From<serde_value::SerializerError> for LtsvError {
    fn from(err: serde_value::SerializerError) -> LtsvError {
        LtsvError::SerializerError(err)
    }
}

impl From<serde_value::DeserializerError> for LtsvError {
    fn from(err: serde_value::DeserializerError) -> LtsvError {
        LtsvError::DeserializerError(err)
    }
}

impl Error for LtsvError {}

pub type LtsvResult<T> = Result<T, LtsvError>;

