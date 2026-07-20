//! Command-line surface (clap).

use clap::{Parser, Subcommand};

/// Prita command-line interface: a read-it-later and knowledge library.
///
/// Save links, read them, tag them, and search them from the terminal.
///
/// Commands print JSON by default so output is easy to parse; pass --plain for
/// human-readable text. Authenticate with an application token using
/// `prita auth login <token>` or the PRITA_TOKEN environment variable.
#[derive(Parser, Debug)]
#[command(
    name = "prita",
    version,
    arg_required_else_help = true,
    after_help = "Examples:\n  \
        prita auth login prita_xxxx        Store an application token\n  \
        PRITA_TOKEN=prita_xxxx prita ...   Authenticate via environment (agents/CI)\n  \
        prita --plain auth status          Human-readable output\n\
        \nEnvironment:\n  \
        PRITA_TOKEN     Application token (overrides the stored token)\n  \
        PRITA_API_URL   GraphQL endpoint (default: https://api.prita.app/graphql)"
)]
pub struct Cli {
    /// Print human-readable output instead of the default JSON.
    #[arg(long, global = true)]
    pub plain: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Authenticate the CLI with a Prita application token.
    Auth {
        #[command(subcommand)]
        command: AuthCommand,
    },
    /// List saved articles, newest first.
    List(ListArgs),
}

#[derive(clap::Args, Debug)]
pub struct ListArgs {
    /// Only include articles under this tag id (rolls up descendants).
    #[arg(long)]
    pub tag: Option<String>,

    /// Maximum number of articles to return.
    #[arg(long, short = 'n', default_value_t = 20)]
    pub limit: i32,

    /// Pagination cursor: the end_cursor from a previous page.
    #[arg(long)]
    pub after: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum AuthCommand {
    /// Store an application token.
    ///
    /// With no token, opens the Prita token page and prompts for one. Pass the
    /// token directly (or set PRITA_TOKEN) for non-interactive use.
    Login {
        /// The application token (starts with `prita_`). Omit to be prompted.
        token: Option<String>,
    },
    /// Show whether the CLI is authenticated and where the token comes from.
    Status,
    /// Remove the stored application token.
    Logout,
}
