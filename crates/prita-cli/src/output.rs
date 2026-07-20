//! Output formatting.
//!
//! Commands emit JSON by default; `--plain` switches to human-readable text.
//! This is the reverse of the usual `--json` opt-in, because the primary caller
//! is an agent.

use serde::Serialize;

use crate::error::CliError;

/// Which rendering a command should produce.
#[derive(Clone, Copy, Debug)]
pub enum Format {
    /// Machine-readable JSON (the default).
    Json,
    /// Human-readable text (`--plain`).
    Plain,
}

impl Format {
    pub fn resolve(plain: bool) -> Self {
        if plain { Format::Plain } else { Format::Json }
    }
}

/// Human-readable rendering for `--plain`. JSON output comes from [`Serialize`].
pub trait Render {
    fn plain(&self) -> String;
}

/// Print a successful result to stdout in the chosen format.
pub fn emit<T: Serialize + Render>(format: Format, value: &T) {
    match format {
        Format::Json => match serde_json::to_string_pretty(value) {
            Ok(json) => println!("{json}"),
            Err(e) => emit_error(format, &CliError::new("serialize", e.to_string())),
        },
        Format::Plain => println!("{}", value.plain()),
    }
}

/// Print an error to stderr in the chosen format.
pub fn emit_error(format: Format, err: &CliError) {
    match format {
        Format::Json => {
            #[derive(Serialize)]
            struct Envelope<'a> {
                error: &'a CliError,
            }
            let body = serde_json::to_string_pretty(&Envelope { error: err })
                .unwrap_or_else(|_| {
                    r#"{"error":{"code":"unknown","message":"unrenderable error"}}"#.to_string()
                });
            eprintln!("{body}");
        }
        Format::Plain => eprintln!("error: {}", err.message()),
    }
}
