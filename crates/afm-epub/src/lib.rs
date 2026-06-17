//! `afm-epub` — library API for converting Aozora Flavored Markdown
//! sources into an EPUB 3.3 package.
//!
//! ## Authoritative specifications
//!
//! Every byte of the package this crate produces is anchored in a
//! published specification. New phases / new metadata fields / new
//! manifest items must cite the relevant section before they land.
//!
//! - [EPUB 3.3 — Authoring & Interchange (W3C, 2023)](https://www.w3.org/TR/epub-33/)
//! - [EPUB 3.3 — Open Packaging Format (W3C, 2023)](https://www.w3.org/TR/epub-33/#sec-package-doc)
//! - [EPUB 3 OCF (Open Container Format)](https://www.w3.org/TR/epub-33/#sec-ocf)
//! - [EPUB 3 Navigation Document](https://www.w3.org/TR/epub-33/#sec-nav)
//! - [Dublin Core Metadata Element Set v1.1](https://www.dublincore.org/specifications/dublin-core/dces/)
//! - [BCP 47 — Tags for Identifying Languages (RFC 5646)](https://datatracker.ietf.org/doc/html/rfc5646)
//! - [W3C XML 1.0 (Fifth Edition)](https://www.w3.org/TR/xml/)
//! - [ZIP File Format Specification (PKWARE APPNOTE 6.3.10)](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT)
//!
//! ## Pipeline (mirroring `afm`'s ADR-0008 phase split)
//!
//! 1. **Discover** ([`discover`]) — collect the input afm sources and
//!    the `book.toml` metadata into an internal [`Manuscript`].
//! 2. **Render** ([`render`]) — turn every afm source into an XHTML
//!    spine item via [`afm_markdown::render_to_string`]. Output is
//!    already HTML-escaped by afm.
//! 3. **Compose** ([`compose`]) — synthesise the OPF Package Document
//!    (§3 of EPUB 3.3 OPF), the Navigation Document (§5 of EPUB 3.3
//!    Nav), the OCF `META-INF/container.xml` (§3 of EPUB 3 OCF), and
//!    the literal `mimetype` byte sequence (§3.3 OCF).
//! 4. **Package** ([`package`]) — pack the artefacts into the `.epub`
//!    ZIP container per OCF §3.4 (mimetype stored, no compression;
//!    everything else deflated).
//!
//! The crate exposes [`build`] as the single entry point; each phase
//! module is private and consumed only via that orchestrator.

#![doc(html_logo_url = "https://github.com/P4suta/afm-epub")]
// Transitive dependency duplicates pulled in by the aozora / comrak
// pipeline (winnow 0.7 + 1.0, hashbrown 0.15 + 0.17, unicode-width
// 0.1 + 0.2, wit-bindgen 0.51 + 0.57). These resolve only when the
// upstream crates align — not something afm-epub can fix locally.
// Tracked under CHANGELOG[Unreleased] for re-enablement once
// upstream lands the bumps.
#![allow(
    clippy::multiple_crate_versions,
    reason = "transitive duplicates from aozora / comrak pipeline; \
              see crate-level note above"
)]

mod compose;
mod discover;
mod error;
mod package;
mod render;

pub use error::{Error, Result};

/// Build configuration for one EPUB output.
#[derive(Debug, Clone)]
pub struct BuildOptions<'a> {
    /// Directory or single file containing afm sources.
    pub input: &'a std::path::Path,
    /// Path to `book.toml` metadata.
    pub metadata: &'a std::path::Path,
    /// Output `.epub` path.
    pub output: &'a std::path::Path,
}

/// Convert an afm manuscript into an EPUB 3.3 file.
///
/// # Errors
///
/// Returns [`Error`] if any phase fails. All errors carry source
/// spans where applicable (`miette::Report`).
pub fn build(opts: &BuildOptions<'_>) -> Result<()> {
    let manuscript = discover::collect(opts)?;
    let rendered = render::render_all(&manuscript)?;
    let bundle = compose::compose(&manuscript, &rendered)?;
    package::write(opts.output, &bundle)
}
