use std::string::FromUtf8Error;

pub type SdResult<T> = Result<T, ShadowError>;

#[derive(Debug)]
pub enum ShadowError {
    String(String),
}

impl From<std::string::FromUtf8Error> for ShadowError {
    fn from(e: FromUtf8Error) -> Self {
        ShadowError::String(e.to_string())
    }
}

impl std::convert::From<std::io::Error> for ShadowError {
    fn from(e: std::io::Error) -> Self {
        ShadowError::String(e.to_string())
    }
}

impl std::convert::From<git2::Error> for ShadowError {
    fn from(e: git2::Error) -> Self {
        ShadowError::String(e.to_string())
    }
}

impl std::convert::From<std::env::VarError> for ShadowError {
    fn from(e: std::env::VarError) -> Self {
        ShadowError::String(e.to_string())
    }
}