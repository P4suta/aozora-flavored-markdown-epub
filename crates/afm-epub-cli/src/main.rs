//! `afm-epub` CLI — thin clap wrapper over the `afm_epub` library.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "afm-epub", version, about, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// Build an EPUB3 from a manuscript directory.
    Build {
        /// Input directory or single afm file.
        #[arg(long)]
        input: PathBuf,
        /// `book.toml` metadata path.
        #[arg(long)]
        metadata: PathBuf,
        /// Output `.epub` path.
        #[arg(long, short = 'o')]
        output: PathBuf,
    },
}

fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Build {
            input,
            metadata,
            output,
        } => afm_epub::build(&afm_epub::BuildOptions {
            input: &input,
            metadata: &metadata,
            output: &output,
        })?,
    }
    Ok(())
}
