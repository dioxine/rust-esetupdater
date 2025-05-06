use reqwest::Error as ReqwestError;
use serde_ini::de::Error as ParseError;
use serde_ini::ser::Error as SerializeError;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum AppError {
    Network(ReqwestError),
    Io(IoError),
    Parse(ParseError),
    Serialize(SerializeError),
}

impl From<ReqwestError> for AppError {
    fn from(e: ReqwestError) -> Self {
        AppError::Network(e)
    }
}

impl From<IoError> for AppError {
    fn from(e: IoError) -> Self {
        AppError::Io(e)
    }
}

impl From<ParseError> for AppError {
    fn from(e: ParseError) -> Self {
        AppError::Parse(e)
    }
}

impl From<SerializeError> for AppError {
    fn from(e: SerializeError) -> Self {
        AppError::Serialize(e)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::Network(e) => write!(f, "Network error: {}", e),
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
            AppError::Serialize(e) => write!(f, "Serialize error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}
