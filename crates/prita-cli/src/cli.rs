//! Command-line surface (clap).

use clap::{Args, Parser, Subcommand};

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
    after_help = HELP_EXAMPLES
)]
pub struct Cli {
    /// Print human-readable output instead of the default JSON.
    #[arg(long, global = true)]
    pub plain: bool,

    #[command(subcommand)]
    pub command: Command,
}

const HELP_EXAMPLES: &str = "\
Examples:
  prita auth login prita_xxxx            Store an application token
  prita list                             Your saved articles, newest first
  prita list --tag <tag-id> -n 50        Articles under a tag
  prita save https://example.com/post    Save a URL to your library
  prita get <article-id>                 Read an article (text + tags)
  prita tag <article-id> <tag-id>        Add a tag to an article
  prita progress <article-id> 100        Mark an article fully read
  prita tags                             Show your whole tag tree
  prita tags create Essays --color blue  Create a tag
  prita tags path Reading 2026 Essays    Create a nested tag path (mkdir -p)
  prita rm <article-id>                  Delete an article

Output is JSON by default; add --plain for human-readable text. Set PRITA_TOKEN
to authenticate without a stored login (best for agents and CI).";

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Authenticate the CLI with a Prita application token.
    Auth {
        #[command(subcommand)]
        command: AuthCommand,
    },
    /// List saved articles, newest first.
    List(ListArgs),
    /// Show one article, with its readable text and tags.
    Get(GetArgs),
    /// Save an article from a URL.
    Save(SaveArgs),
    /// Delete an article.
    Rm(RmArgs),
    /// Add a tag to an article.
    Tag(TagArgs),
    /// Remove a tag from an article.
    Untag(TagArgs),
    /// Set reading progress (0-100) for an article.
    Progress(ProgressArgs),
    /// List and manage tags.
    Tags(TagsArgs),
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

#[derive(Args, Debug)]
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

#[derive(Args, Debug)]
pub struct GetArgs {
    /// The article id.
    pub id: String,
}

#[derive(Args, Debug)]
pub struct SaveArgs {
    /// URL of the page to save.
    pub url: String,

    /// Optional pre-fetched HTML content (skips server-side fetching).
    #[arg(long)]
    pub content: Option<String>,
}

#[derive(Args, Debug)]
pub struct RmArgs {
    /// The article id.
    pub id: String,
}

#[derive(Args, Debug)]
pub struct TagArgs {
    /// The article id.
    pub article_id: String,

    /// The tag id.
    pub tag_id: String,
}

#[derive(Args, Debug)]
pub struct ProgressArgs {
    /// The article id.
    pub article_id: String,

    /// Reading progress as a percentage (0-100).
    pub percent: i32,

    /// Optional anchor index for a precise position.
    #[arg(long)]
    pub anchor: Option<i32>,
}

#[derive(Args, Debug)]
pub struct TagsArgs {
    #[command(subcommand)]
    pub command: Option<TagsCommand>,
}

#[derive(Subcommand, Debug)]
pub enum TagsCommand {
    /// List the whole tag tree (the default when no subcommand is given).
    List,
    /// Create a tag.
    Create {
        /// The tag name.
        name: String,
        /// Parent tag id. Omit for a root tag.
        #[arg(long)]
        parent: Option<String>,
        /// Color: tomato, crimson, plum, violet, indigo, blue, teal, grass, amber, or brown.
        #[arg(long)]
        color: Option<String>,
    },
    /// Rename a tag.
    Rename {
        /// The tag id.
        id: String,
        /// The new name.
        name: String,
    },
    /// Reparent a tag; its subtree moves with it. Omit --parent to move it to the root.
    Move {
        /// The tag id.
        id: String,
        /// New parent tag id. Omit to move to the root.
        #[arg(long)]
        parent: Option<String>,
    },
    /// Set or clear a tag's color. Pass `none` to clear.
    Color {
        /// The tag id.
        id: String,
        /// A color name, or `none` to clear.
        color: String,
    },
    /// Delete a tag and its whole subtree. Articles are only untagged, never deleted.
    Delete {
        /// The tag id.
        id: String,
    },
    /// Create a whole path of tags at once (an atomic `mkdir -p`).
    Path {
        /// Path segments, root to leaf (e.g. `Reading 2026 Essays`).
        #[arg(required = true)]
        segments: Vec<String>,
    },
    /// Pin a tag into the navigation.
    Pin {
        /// The tag id.
        id: String,
    },
    /// Unpin a tag.
    Unpin {
        /// The tag id.
        id: String,
    },
}
