mod cli;
mod commands;
mod error;
mod output;

use clap::Parser;

#[tokio::main]
async fn main() {
    let args = cli::Cli::parse();
    let format = output::Format::resolve(args.plain);

    if let Err(err) = commands::dispatch(args, format).await {
        output::emit_error(format, &err);
        std::process::exit(err.exit_code());
    }
}
