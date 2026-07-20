//! Error type shared across the client.

/// Errors returned by the Prita client.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Missing or rejected credentials (HTTP 401, or no token available).
    #[error("not authenticated: set PRITA_TOKEN or run `prita auth login`")]
    Unauthenticated,

    /// Network / transport failure.
    #[error("network error: {0}")]
    Http(#[from] reqwest::Error),

    /// Local configuration problem (e.g. an unwritable config directory).
    #[error("configuration error: {0}")]
    Config(String),

    /// The GraphQL server returned a non-empty `errors` array.
    #[error("API error: {0}")]
    GraphQl(String),

    /// Filesystem error while reading or writing local state.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Malformed JSON in a response or the credentials file.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
