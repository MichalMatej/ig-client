/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 12/5/25
******************************************************************************/
use reqwest::StatusCode;
use std::io;

/// Error type for fetch operations
#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    /// Network error from reqwest
    #[error("network error: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// Database error from sqlx
    #[error("db error: {0}")]
    Sqlx(#[from] sqlx::Error),
    /// Error during parsing
    #[error("parser error: {0}")]
    Parser(String),
}

/// Error type for authentication operations
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    /// Network error from reqwest
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    /// I/O error
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    /// JSON serialization or deserialization error
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    /// Other unspecified error
    #[error("other error: {0}")]
    Other(String),
    /// Invalid credentials error
    #[error("bad credentials")]
    BadCredentials,
    /// Unexpected HTTP status code
    #[error("unexpected http status: {0}")]
    Unexpected(StatusCode),
    /// Rate limit exceeded error
    #[error("rate limit exceeded")]
    RateLimitExceeded,
}

impl From<Box<dyn std::error::Error + Send + Sync>> for AuthError {
    #[cold]
    fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        match e.downcast::<reqwest::Error>() {
            Ok(req) => AuthError::Network(*req),
            Err(e) => match e.downcast::<serde_json::Error>() {
                Ok(js) => AuthError::Json(*js),
                Err(e) => match e.downcast::<std::io::Error>() {
                    Ok(ioe) => AuthError::Io(*ioe),
                    Err(other) => AuthError::Other(other.to_string()),
                },
            },
        }
    }
}

impl From<Box<dyn std::error::Error>> for AuthError {
    #[cold]
    fn from(e: Box<dyn std::error::Error>) -> Self {
        match e.downcast::<reqwest::Error>() {
            Ok(req) => AuthError::Network(*req),
            Err(e) => match e.downcast::<serde_json::Error>() {
                Ok(js) => AuthError::Json(*js),
                Err(e) => match e.downcast::<io::Error>() {
                    Ok(ioe) => AuthError::Io(*ioe),
                    Err(other) => AuthError::Other(other.to_string()),
                },
            },
        }
    }
}

impl From<AppError> for AuthError {
    #[cold]
    fn from(e: AppError) -> Self {
        match e {
            AppError::Network(e) => AuthError::Network(e),
            AppError::Io(e) => AuthError::Io(e),
            AppError::Json(e) => AuthError::Json(e),
            AppError::Unexpected(s) => AuthError::Unexpected(s),
            _ => AuthError::Other(e.to_string()),
        }
    }
}

/// General application error type
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// Network error from reqwest
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    /// I/O error
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    /// JSON serialization or deserialization error
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    /// Unexpected HTTP status code
    #[error("unexpected http status: {0}")]
    Unexpected(StatusCode),
    /// Database error from sqlx
    #[error("db error: {0}")]
    Db(#[from] sqlx::Error),
    /// Unauthorized access error
    #[error("unauthorized")]
    Unauthorized,
    /// OAuth token expired error (requires token refresh)
    #[error("oauth token expired")]
    OAuthTokenExpired,
    /// Resource not found error
    #[error("not found")]
    NotFound,
    /// API rate limit exceeded
    #[error("rate limit exceeded")]
    RateLimitExceeded,
    /// Historical data allowance exhausted (weekly quota of data points)
    ///
    /// The `allowance_expiry` field indicates the number of seconds
    /// until the allowance resets. Retrying before that is pointless.
    #[error("historical data allowance exceeded, resets in {allowance_expiry} seconds")]
    HistoricalDataAllowanceExceeded {
        /// Seconds until the weekly allowance resets
        allowance_expiry: u64,
    },
    /// Error during serialization or deserialization
    #[error("serialization error: {0}")]
    SerializationError(String),
    /// WebSocket communication error
    #[error("websocket error: {0}")]
    WebSocketError(String),
    /// Deserialization error with details
    #[error("deserialization error: {0}")]
    Deserialization(String),
    /// Invalid input error with a description of the constraint violated
    #[error("invalid input: {0}")]
    InvalidInput(String),
    /// Generic error for cases that don't fit other categories
    #[error("generic error: {0}")]
    Generic(String),
}

impl From<AuthError> for AppError {
    #[cold]
    fn from(e: AuthError) -> Self {
        match e {
            AuthError::Network(e) => AppError::Network(e),
            AuthError::Io(e) => AppError::Io(e),
            AuthError::Json(e) => AppError::Json(e),
            AuthError::BadCredentials => AppError::Unauthorized,
            AuthError::Unexpected(s) => AppError::Unexpected(s),
            _ => AppError::Unexpected(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    #[cold]
    fn from(e: Box<dyn std::error::Error>) -> Self {
        match e.downcast::<reqwest::Error>() {
            Ok(req) => AppError::Network(*req),
            Err(e) => match e.downcast::<serde_json::Error>() {
                Ok(js) => AppError::Json(*js),
                Err(e) => match e.downcast::<std::io::Error>() {
                    Ok(ioe) => AppError::Io(*ioe),
                    Err(_) => AppError::Unexpected(StatusCode::INTERNAL_SERVER_ERROR),
                },
            },
        }
    }
}

impl From<String> for AppError {
    #[cold]
    fn from(e: String) -> Self {
        AppError::Generic(e)
    }
}

impl From<lightstreamer_rs::utils::LightstreamerError> for AppError {
    #[cold]
    fn from(e: lightstreamer_rs::utils::LightstreamerError) -> Self {
        AppError::WebSocketError(e.to_string())
    }
}
