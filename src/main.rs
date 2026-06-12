//! `muster-mcp` — live-session census as a read-only MCP server.
//!
//! Exposes `sessions_census` and `sessions_verdict` over MCP stdio.
//! The `reap` verb is intentionally absent from this server.
//!
//! # Usage
//!
//! ```text
//! muster-mcp serve [--muster-bin <path>]
//! ```

mod backend;
mod tools;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use mcp_core::serve_stdio;

use backend::MusterCli;
use tools::{SessionsCensus, SessionsVerdict};

// ---------------------------------------------------------------------------
// CLI definition
// ---------------------------------------------------------------------------

/// Live-session census as a read-only MCP server.
#[derive(Debug, Parser)]
#[command(name = "muster-mcp", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Start the MCP stdio server.
    Serve(ServeArgs),
}

/// Arguments for the `serve` subcommand.
#[derive(Debug, Parser)]
struct ServeArgs {
    /// Path to the `muster` binary.
    ///
    /// Defaults to `muster` resolved via `$PATH`. Can also be set via the
    /// `MUSTER_BIN` environment variable.
    #[arg(long, env = "MUSTER_BIN", value_name = "PATH")]
    muster_bin: Option<PathBuf>,
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() -> Result<()> {
    // SIGPIPE reset must be first — per wintermute scaffold convention.
    // Prevents broken-pipe panics when stdout is closed by the client.
    sigpipe::reset();

    let cli = Cli::parse();

    match cli.command {
        Commands::Serve(args) => {
            let muster_cli = args
                .muster_bin
                .map_or_else(MusterCli::from_path, MusterCli::new);

            let tools: Vec<Box<dyn mcp_core::Tool>> = vec![
                Box::new(SessionsCensus::new(muster_cli.clone())),
                Box::new(SessionsVerdict::new(muster_cli)),
            ];

            serve_stdio(tools, "muster-mcp", env!("CARGO_PKG_VERSION"))?;
        }
    }

    Ok(())
}
