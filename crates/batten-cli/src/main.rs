//! Main CLI binary entrypoint for `batten`.

#![forbid(unsafe_code)]

mod args;
mod handlers;

use args::{Cli, Command};
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.verbose {
        tracing_subscriber::fmt()
            .with_writer(std::io::stderr)
            .init();
    }

    // One span per invocation (`GB-339`). Opened after logging is configured and wrapped around every fallible step,
    // so a failing compile records `error` rather than leaving the span running.
    let command = std::env::args().nth(1).filter(|arg| !arg.starts_with('-'));
    let invocation = gaugeblock_workspace::InvocationSpan::start("batten", command.as_deref());

    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        match cli.command {
            Command::Compile {
                scope,
                hole,
                format,
                max_depth,
                output,
                json,
            } => {
                handlers::handle_compile(
                    &scope,
                    &hole,
                    format,
                    max_depth,
                    output.as_deref(),
                    json,
                )?;
            }
            Command::Validate {
                candidate,
                constraint,
                allow_literals,
                max_tokens,
                json,
            } => {
                handlers::handle_validate(
                    &candidate,
                    &constraint,
                    allow_literals,
                    max_tokens,
                    json,
                )?;
            }
            Command::Inspect { scope, hole, json } => {
                handlers::handle_inspect(&scope, &hole, json)?;
            }
            Command::Mcp => {
                batten_mcp::run_stdio_server()?;
            }
        }

        Ok(())
    })();

    invocation.finish(result.is_ok());
    result
}
