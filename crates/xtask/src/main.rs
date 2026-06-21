//! Developer automation. Mirrors the role of `aozora-flavored-markdown`'s `xtask`:
//! generate ADR scaffolding, run release-prep checks, drive
//! end-to-end pipeline tests against `examples/`.

use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "xtask", version, about)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// Scaffold a new ADR under `docs/adr/` from the 0000 template,
    /// auto-incrementing the sequence number.
    NewAdr {
        /// Slug used in the file name (`NNNN-<slug>.md`). The title
        /// inside the file is the `--title`, or the slug if absent.
        slug: String,
        #[arg(long)]
        title: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::NewAdr { slug, title } => new_adr(&slug, title.as_deref()),
    }
}

fn new_adr(slug: &str, title: Option<&str>) -> anyhow::Result<()> {
    let title = title.unwrap_or(slug);
    let adr_dir = PathBuf::from("docs/adr");
    let template_path = adr_dir.join("0000-template.md");
    if !template_path.exists() {
        anyhow::bail!(
            "ADR template not found at {} — expected `docs/adr/0000-template.md`",
            template_path.display()
        );
    }
    let template = fs::read_to_string(&template_path)?;

    let next = next_adr_number(&adr_dir)?;
    let dest = adr_dir.join(format!("{next:04}-{slug}.md"));
    if dest.exists() {
        anyhow::bail!("destination already exists: {}", dest.display());
    }

    let body = template
        .replace("# 0000. Template", &format!("# {next:04}. {title}"))
        .replace("- Status: superseded by …\n", "- Status: proposed\n");
    fs::write(&dest, body)?;
    println!("created {}", dest.display());
    Ok(())
}

fn next_adr_number(adr_dir: &std::path::Path) -> anyhow::Result<u32> {
    let mut highest: u32 = 0;
    for entry in fs::read_dir(adr_dir)? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if let Some(prefix) = name.split('-').next()
            && let Ok(n) = prefix.parse::<u32>()
            && n > highest
        {
            highest = n;
        }
    }
    Ok(highest + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_adr_number_increments_from_zero() {
        let dir = tempdir();
        std::fs::write(dir.path().join("0000-template.md"), "stub").unwrap();
        std::fs::write(dir.path().join("0003-aaa.md"), "stub").unwrap();
        std::fs::write(dir.path().join("0001-bbb.md"), "stub").unwrap();
        assert_eq!(next_adr_number(dir.path()).unwrap(), 4);
    }

    #[test]
    fn next_adr_number_handles_empty_dir() {
        let dir = tempdir();
        assert_eq!(next_adr_number(dir.path()).unwrap(), 1);
    }

    #[test]
    fn next_adr_number_skips_non_numeric_files() {
        let dir = tempdir();
        std::fs::write(dir.path().join("README.md"), "stub").unwrap();
        std::fs::write(dir.path().join("0002-xxx.md"), "stub").unwrap();
        assert_eq!(next_adr_number(dir.path()).unwrap(), 3);
    }

    #[test]
    fn new_adr_success_and_missing_template() {
        // Single test that manipulates the process-global cwd to avoid races.
        let dir = tempdir();
        let adr_dir = dir.path().join("docs/adr");
        std::fs::create_dir_all(&adr_dir).unwrap();

        let original = std::env::current_dir().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();

        // Error case: template absent.
        let missing = new_adr("my-slug", None);

        // Success case: write the template, then scaffold a new ADR.
        let template = "# 0000. Template\n- Status: superseded by …\nbody\n";
        std::fs::write(adr_dir.join("0000-template.md"), template).unwrap();
        let created = new_adr("my-slug", Some("My Title"));

        std::env::set_current_dir(&original).unwrap();

        assert!(missing.is_err());
        created.unwrap();

        let dest = adr_dir.join("0001-my-slug.md");
        assert!(dest.exists());
        let body = std::fs::read_to_string(&dest).unwrap();
        assert!(body.contains("# 0001. My Title"));
        assert!(body.contains("- Status: proposed\n"));
        assert!(!body.contains("# 0000. Template"));
        assert!(!body.contains("superseded by"));
    }

    fn tempdir() -> tempfile::TempDir {
        tempfile::tempdir().expect("tempdir")
    }
}
