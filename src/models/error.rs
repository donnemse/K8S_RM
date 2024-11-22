
use std::io;

#[derive(Debug)]
pub enum AppError {
    IoError(String),
    KubeError(String),
    ParseError(String),
}

impl std::error::Error for AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO Error: {}", e),
            AppError::KubeError(e) => write!(f, "Kubernetes Error: {}", e),
            AppError::ParseError(e) => write!(f, "Parse Error: {}", e),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::IoError(err.to_string())
    }
}