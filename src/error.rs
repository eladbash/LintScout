use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum LintScoutError {
    #[error("failed to read file {path}: {source}")]
    FileRead {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("invalid regex pattern '{pattern}': {source}")]
    InvalidPattern {
        pattern: String,
        source: regex::Error,
    },

    #[error("configuration error: {0}")]
    Config(String),

    #[error("failed to load config from {path}: {source}")]
    ConfigLoad {
        path: PathBuf,
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, LintScoutError>;
