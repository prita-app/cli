//! Local configuration: token storage and endpoint resolution.
//!
//! A token from `PRITA_TOKEN` takes precedence over the stored file, so an agent
//! can set the environment variable without writing any config.

use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::Error;

/// Default GraphQL endpoint. Overridable with [`ENV_API_URL`].
pub const DEFAULT_API_URL: &str = "https://api.prita.app/graphql";

/// Web page where a user creates an application token (Settings → Developer).
pub const DEFAULT_TOKEN_PAGE: &str = "https://web.prita.app/settings/developer";

/// Prefix every application token carries (`prita_<43 chars>`).
pub const TOKEN_PREFIX: &str = "prita_";

/// Environment variable holding the application token.
pub const ENV_TOKEN: &str = "PRITA_TOKEN";

/// Environment variable overriding the GraphQL endpoint.
pub const ENV_API_URL: &str = "PRITA_API_URL";

/// Where a resolved token came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenSource {
    /// From the `PRITA_TOKEN` environment variable.
    Env,
    /// From the stored credentials file.
    Config,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Credentials {
    #[serde(default)]
    token: Option<String>,
}

/// The GraphQL endpoint to use (env override or [`DEFAULT_API_URL`]).
pub fn api_url() -> String {
    std::env::var(ENV_API_URL)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_API_URL.to_string())
}

/// The URL a human visits to mint a token.
pub fn token_page_url() -> String {
    DEFAULT_TOKEN_PAGE.to_string()
}

/// Resolve the active token and where it came from, if any.
pub fn resolve_token() -> Option<(String, TokenSource)> {
    if let Ok(token) = std::env::var(ENV_TOKEN)
        && !token.trim().is_empty()
    {
        return Some((token, TokenSource::Env));
    }
    load_credentials()
        .ok()
        .and_then(|c| c.token)
        .filter(|t| !t.trim().is_empty())
        .map(|t| (t, TokenSource::Config))
}

/// Persist a token to the credentials file, returning its path.
pub fn save_token(token: &str) -> Result<PathBuf, Error> {
    let dir = config_dir()?;
    fs::create_dir_all(&dir)?;
    let path = dir.join("credentials.json");
    let body = serde_json::to_string_pretty(&Credentials {
        token: Some(token.to_string()),
    })?;
    fs::write(&path, body)?;
    restrict_permissions(&path);
    Ok(path)
}

/// Remove the stored token. Returns whether a file was actually deleted.
pub fn clear_token() -> Result<bool, Error> {
    let path = credentials_path()?;
    match fs::remove_file(&path) {
        Ok(()) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e.into()),
    }
}

fn config_dir() -> Result<PathBuf, Error> {
    let dirs = directories::ProjectDirs::from("app", "prita", "prita")
        .ok_or_else(|| Error::Config("could not determine a config directory".into()))?;
    Ok(dirs.config_dir().to_path_buf())
}

fn credentials_path() -> Result<PathBuf, Error> {
    Ok(config_dir()?.join("credentials.json"))
}

fn load_credentials() -> Result<Credentials, Error> {
    match fs::read_to_string(credentials_path()?) {
        Ok(s) => Ok(serde_json::from_str(&s)?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Credentials::default()),
        Err(e) => Err(e.into()),
    }
}

#[cfg(unix)]
fn restrict_permissions(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;
    // Best-effort: the token is a secret, keep it owner-only.
    let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o600));
}

#[cfg(not(unix))]
fn restrict_permissions(_path: &std::path::Path) {}
