//! `prita auth`: manage the stored application token.

use std::io::Write;

use prita_client::config::{self, TokenSource};
use serde::Serialize;

use crate::cli::AuthCommand;
use crate::error::CliError;
use crate::output::{Format, Render, emit};

pub async fn run(command: AuthCommand, format: Format) -> Result<(), CliError> {
    match command {
        AuthCommand::Login { token } => login(token, format),
        AuthCommand::Status => status(format),
        AuthCommand::Logout => logout(format),
    }
}

fn login(token: Option<String>, format: Format) -> Result<(), CliError> {
    let token = match token {
        Some(token) => token,
        None => prompt_for_token()?,
    };
    let token = token.trim().to_string();
    if token.is_empty() {
        return Err(CliError::new("empty_token", "no token was provided"));
    }
    if !token.starts_with(config::TOKEN_PREFIX) {
        // Warn but continue; the server validates the token on the next request.
        eprintln!(
            "warning: token does not start with `{}`, storing it anyway",
            config::TOKEN_PREFIX
        );
    }

    let path = config::save_token(&token)?;
    emit(
        format,
        &LoginResult {
            status: "logged_in",
            stored_at: path.display().to_string(),
        },
    );
    Ok(())
}

/// Open the token page and read a token from stdin. Prompts are written to
/// stderr so stdout stays JSON-only.
fn prompt_for_token() -> Result<String, CliError> {
    let url = config::token_page_url();
    eprintln!("Opening the Prita token page:\n  {url}");
    if webbrowser::open(&url).is_err() {
        eprintln!("(couldn't open a browser automatically; open the URL above yourself)");
    }
    eprint!("Paste your application token: ");
    std::io::stderr().flush().ok();

    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .map_err(|e| CliError::new("stdin", e.to_string()))?;
    Ok(line)
}

fn status(format: Format) -> Result<(), CliError> {
    let (authenticated, source) = match config::resolve_token() {
        Some((_, source)) => (true, Some(source)),
        None => (false, None),
    };
    emit(
        format,
        &AuthStatus {
            authenticated,
            source,
            api_url: config::api_url(),
        },
    );
    Ok(())
}

fn logout(format: Format) -> Result<(), CliError> {
    let removed = config::clear_token()?;
    emit(
        format,
        &LogoutResult {
            status: if removed { "logged_out" } else { "not_logged_in" },
        },
    );
    Ok(())
}

#[derive(Serialize)]
struct LoginResult {
    status: &'static str,
    stored_at: String,
}

impl Render for LoginResult {
    fn plain(&self) -> String {
        format!("Logged in. Token stored at {}.", self.stored_at)
    }
}

#[derive(Serialize)]
struct AuthStatus {
    authenticated: bool,
    source: Option<TokenSource>,
    api_url: String,
}

impl Render for AuthStatus {
    fn plain(&self) -> String {
        if self.authenticated {
            format!("Authenticated ({}).", self.api_url)
        } else {
            "Not authenticated. Run `prita auth login` or set PRITA_TOKEN.".to_string()
        }
    }
}

#[derive(Serialize)]
struct LogoutResult {
    status: &'static str,
}

impl Render for LogoutResult {
    fn plain(&self) -> String {
        match self.status {
            "logged_out" => "Logged out. Stored token removed.".to_string(),
            _ => "No stored token to remove.".to_string(),
        }
    }
}
