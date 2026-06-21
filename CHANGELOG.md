# Changelog

All notable changes to aozora-flavored-markdown-epub are recorded in this
file. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/);
the project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **Breaking:** renamed the project to `aozora-flavored-markdown-epub`
  (library `aozora-flavored-markdown-epub`, binary
  `aozora-flavored-markdown-epub-cli`, CLI/binary name
  `aozora-flavored-markdown-epub`).
- **Breaking:** renamed the generated EPUB CSS classes to the
  `aozora-md-*` prefix.
- Migrated dependencies to crates.io semver deps, replacing the previous
  git-tag / SHA-pin approach.

### Added

- Initial repository scaffolding: Cargo workspace with
  `aozora-flavored-markdown-epub` library and
  `aozora-flavored-markdown-epub-cli` binary, Docker-only execution
  surface, ADR-0001 (consume Aozora Flavored Markdown from crates.io as a
  semver dep) and ADR-0002 (Docker-only execution).

[Unreleased]: https://github.com/P4suta/aozora-flavored-markdown-epub/compare/main...HEAD
