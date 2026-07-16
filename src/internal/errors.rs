use hyper::Error as HyperError;
use hyper::http::uri::InvalidUri;
use hyper_util::client::legacy::Error as RequestError;
use log::SetLoggerError;
use serde_ini::de::Error as ParseError;
use serde_ini::ser::Error as SerializeIniError;
use serde_json::Error as SerializeJsonError;
use std::io::Error as IoError;
use tokio::task::JoinError;
use toml::de::Error as TomlError;

#[derive(Debug)]
pub enum AppError {
    Log(SetLoggerError),
    Uri(InvalidUri),
    Hyper(HyperError),
    Network(RequestError),
    Io(IoError),
    Parse(ParseError),
    SerializeINI(SerializeIniError),
    SerializeJSON(SerializeJsonError),
    Toml(TomlError),
    TaskJoin(JoinError),
    EmptyConfig,
}

impl From<SetLoggerError> for AppError {
    fn from(e: SetLoggerError) -> Self {
        AppError::Log(e)
    }
}

impl From<InvalidUri> for AppError {
    fn from(e: InvalidUri) -> Self {
        AppError::Uri(e)
    }
}

impl From<HyperError> for AppError {
    fn from(e: HyperError) -> Self {
        AppError::Hyper(e)
    }
}

impl From<RequestError> for AppError {
    fn from(e: RequestError) -> Self {
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

impl From<JoinError> for AppError {
    fn from(e: JoinError) -> Self {
        AppError::TaskJoin(e)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::Log(e) => write!(f, "Logger error: {}", e),
            AppError::Uri(e) => write!(f, "Uri syntax error: {}", e),
            AppError::Hyper(e) => write!(f, "Hyper error: {}", e),
            AppError::Network(e) => write!(f, "Network error: {}", e),
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
            AppError::SerializeINI(e) => write!(f, "Serialize INI error: {}", e),
            AppError::SerializeJSON(e) => write!(f, "Serialize JSON error: {}", e),
            AppError::Toml(e) => write!(f, "TOML error: {}", e),
            AppError::EmptyConfig => write!(
                f,
                "Config file not found. Use CLI or create 'config.toml' file manually."
            ),
            AppError::TaskJoin(e) => write!(f, "Tokio task join error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}
