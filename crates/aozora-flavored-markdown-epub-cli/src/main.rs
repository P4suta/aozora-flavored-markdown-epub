//! `aozora-flavored-markdown-epub` CLI — thin clap wrapper over the
//! `aozora_flavored_markdown_epub` library.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "aozora-flavored-markdown-epub",
    version,
    about,
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// Build an EPUB3 from a manuscript directory.
    Build {
        /// Input directory or single Aozora Flavored Markdown file.
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
        } => aozora_flavored_markdown_epub::build(&aozora_flavored_markdown_epub::BuildOptions {
            input: &input,
            metadata: &metadata,
            output: &output,
        })?,
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
        Cli::try_parse_from(
            std::iter::once("aozora-flavored-markdown-epub").chain(args.iter().copied()),
        )
    }

    #[test]
    fn parses_build_with_all_paths() {
        let cli = parse(&[
            "build",
            "--input",
            "m",
            "--metadata",
            "b.toml",
            "--output",
            "o.epub",
        ])
        .expect("parses");
        match cli.cmd {
            Cmd::Build {
                input,
                metadata,
                output,
            } => {
                assert_eq!(input, PathBuf::from("m"));
                assert_eq!(metadata, PathBuf::from("b.toml"));
                assert_eq!(output, PathBuf::from("o.epub"));
            },
        }
    }

    #[test]
    fn output_has_a_short_flag() {
        let cli = parse(&[
            "build",
            "--input",
            "m",
            "--metadata",
            "b.toml",
            "-o",
            "o.epub",
        ])
        .expect("parses");
        match cli.cmd {
            Cmd::Build { output, .. } => assert_eq!(output, PathBuf::from("o.epub")),
        }
    }

    #[test]
    fn build_requires_output() {
        assert!(parse(&["build", "--input", "m", "--metadata", "b.toml"]).is_err());
    }

    #[test]
    fn unknown_subcommand_is_rejected() {
        assert!(parse(&["frobnicate"]).is_err());
    }
}
