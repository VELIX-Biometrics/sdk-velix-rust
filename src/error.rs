use thiserror::Error;

#[derive(Debug, Error)]
pub enum VelixError {
    #[error("HTTP error {status}: {message}")]
    Http { status: u16, message: String },

    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("Rate limit exceeded — retry after {retry_after_secs}s")]
    RateLimit { retry_after_secs: u64 },

    #[error("Biometric error: {0}")]
    Biometric(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
