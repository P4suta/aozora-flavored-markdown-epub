//! Phase 1 — discover.
//!
//! Walks the input directory, collects the afm sources in spine order
//! (lexicographic for now; the metadata file may override), and parses
//! the `book.toml` into a structured [`Metadata`] value.

use std::path::PathBuf;

use serde::Deserialize;

use crate::{BuildOptions, Error, Result};

/// One book's worth of inputs after discovery.
#[derive(Debug, Clone)]
pub struct Manuscript {
    pub metadata: Metadata,
    pub sources: Vec<SourceFile>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub creator: String,
    pub language: String,
    #[serde(default)]
    pub identifier: Option<String>,
    #[serde(default = "default_mode")]
    pub writing_mode: WritingMode,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WritingMode {
    Horizontal,
    Vertical,
}

const fn default_mode() -> WritingMode {
    WritingMode::Horizontal
}

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: PathBuf,
    pub bytes: Vec<u8>,
}

pub fn collect(opts: &BuildOptions<'_>) -> Result<Manuscript> {
    let metadata_text =
        std::fs::read_to_string(opts.metadata).map_err(|source| Error::DiscoverIo {
            path: opts.metadata.to_path_buf(),
            source,
        })?;
    let metadata: Metadata =
        toml::from_str(&metadata_text).map_err(|source| Error::MetadataParse {
            path: opts.metadata.to_path_buf(),
            source,
        })?;

    let mut sources = Vec::new();
    if opts.input.is_file() {
        sources.push(read_source(opts.input)?);
    } else {
        let entries = std::fs::read_dir(opts.input).map_err(|source| Error::DiscoverIo {
            path: opts.input.to_path_buf(),
            source,
        })?;
        let mut paths: Vec<_> = entries
            .filter_map(std::result::Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|ext| ext == "md"))
            .collect();
        paths.sort();
        for p in paths {
            sources.push(read_source(&p)?);
        }
    }

    Ok(Manuscript { metadata, sources })
}

fn read_source(path: &std::path::Path) -> Result<SourceFile> {
    let bytes = std::fs::read(path).map_err(|source| Error::DiscoverIo {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(SourceFile {
        path: path.to_path_buf(),
        bytes,
    })
}
