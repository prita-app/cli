//! Command dispatch.

mod articles;
mod auth;
mod tags;
mod view;

use prita_client::{PritaClient, config};

use crate::cli::{Cli, Command};
use crate::error::CliError;
use crate::output::Format;

/// Route a parsed CLI invocation to its handler.
pub async fn dispatch(cli: Cli, format: Format) -> Result<(), CliError> {
    match cli.command {
        Command::Auth { command } => auth::run(command, format).await,
        Command::List(args) => articles::list(args, format).await,
        Command::Get(args) => articles::get(args, format).await,
        Command::Save(args) => articles::save(args, format).await,
        Command::Rm(args) => articles::rm(args, format).await,
        Command::Tag(args) => articles::tag(args, format).await,
        Command::Untag(args) => articles::untag(args, format).await,
        Command::Progress(args) => articles::progress(args, format).await,
        Command::Tags(args) => tags::run(args, format).await,
    }
}

/// Build an authenticated client, or fail if no token is available.
pub(crate) fn client() -> Result<PritaClient, CliError> {
    let (token, _source) =
        config::resolve_token().ok_or(prita_client::Error::Unauthenticated)?;
    Ok(PritaClient::new(token, config::api_url())?)
}
