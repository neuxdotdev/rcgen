use thiserror::Error;
#[derive(Debug, Error)]
pub enum RcgenError {
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid date format: {0}")]
    DateParse(String),
    #[error("Invalid regex: {0}")]
    Regex(#[from] regex::Error),
    #[error("Invalid repository path: {0}")]
    InvalidPath(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("No commits found")]
    NoCommits,
    #[error("Invalid revision: {0}")]
    InvalidRevision(String),
}
pub type Result<T> = std::result::Result<T, RcgenError>;
