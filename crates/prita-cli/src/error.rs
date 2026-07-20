//! CLI-level error, rendered to the user as JSON (default) or plain text.

use serde::Serialize;

/// An error surfaced to the user. Serializes to `{ "code", "message" }`; the
/// exit code rides along but is not part of the payload.
#[derive(Debug, Serialize)]
pub struct CliError {
    #[serde(skip)]
    exit: i32,
    code: String,
    message: String,
}

impl CliError {
    /// A generic failure with exit code 1.
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            exit: 1,
            code: code.into(),
            message: message.into(),
        }
    }

    /// Override the process exit code.
    pub fn with_exit(mut self, exit: i32) -> Self {
        self.exit = exit;
        self
    }

    pub fn exit_code(&self) -> i32 {
        self.exit
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<prita_client::Error> for CliError {
    fn from(err: prita_client::Error) -> Self {
        use prita_client::Error as E;
        let (code, exit) = match &err {
            E::Unauthenticated => ("unauthenticated", 2),
            E::Http(_) => ("network", 1),
            E::Config(_) => ("config", 1),
            E::GraphQl(_) => ("graphql", 1),
            E::Io(_) => ("io", 1),
            E::Json(_) => ("invalid_response", 1),
        };
        CliError::new(code, err.to_string()).with_exit(exit)
    }
}
