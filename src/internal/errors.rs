use reqwest::Error as ReqwestError;
use serde_ini::de::Error as ParseError;
use serde_ini::ser::Error as SerializeIniError;
use serde_json::Error as SerializeJsonError;
use std::io::Error as IoError;
use toml::de::Error as TomlError;

#[derive(Debug)]
pub enum AppError {
    Network(ReqwestError),
    Io(IoError),
    Parse(ParseError),
    SerializeINI(SerializeIniError),
    SerializeJSON(SerializeJsonError),
    Toml(TomlError),
    EmptyConfig
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

impl From<SerializeIniError> for AppError {
    fn from(e: SerializeIniError) -> Self {
        AppError::SerializeINI(e)
    }
}

impl From<SerializeJsonError> for AppError {
    fn from(e: SerializeJsonError) -> Self {
        AppError::SerializeJSON(e)
    }
}

impl From<TomlError> for AppError {
    fn from(e: TomlError) -> Self {
        AppError::Toml(e)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::Network(e) => write!(f, "Network error: {}", e),
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
            AppError::SerializeINI(e) => write!(f, "Serialize INI error: {}", e),
            AppError::SerializeJSON(e) => write!(f, "Serialize JSON error: {}", e),
            AppError::Toml(e) => write!(f, "TOML error: {}", e),
            AppError::EmptyConfig => write!(f, "Config file not found. Consider using CLI or creating 'config.toml' file manually."),
        }
    }
}

impl std::error::Error for AppError {}
