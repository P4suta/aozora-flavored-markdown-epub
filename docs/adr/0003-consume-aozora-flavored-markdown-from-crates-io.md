# 0003. Consume aozora-flavored-markdown from crates.io

- Status: accepted
- Date: 2026-06-21
- Deciders: @P4suta
- Tags: architecture, dependency

## Context

ADR-0001 pinned this crate's parser stack to a git revision of upstream
because the parser had no published release. Iterating meant carrying a
git dep, a `[patch]` block at the workspace root, and a cargo-deny git
allow-list to keep the supply-chain audit happy.

Upstream has since renamed `afm-markdown` to `aozora-flavored-markdown`
and published it — together with its `aozora-encoding` companion crate —
to crates.io at 0.4.1. With a real semver release available, the git
pin and its supporting machinery are no longer necessary. The published
crate also raises the minimum supported Rust version (MSRV) to 1.96.0.

## Decision

Depend on `aozora-flavored-markdown = "0.4.1"` and
`aozora-encoding = "0.4.1"` from crates.io. Drop the git pins, the
workspace `[patch]` block, and the cargo-deny git allow-list. Set the
MSRV to 1.96.0, as required by the crate.

## Consequences

Easier:
- **Simpler reproducible builds.** No git fetch at build time and no
  workspace-level `[patch]` to keep in sync.
- **Semver-tracked dependencies.** Version bumps are explicit Cargo
  version changes, and cargo-audit / cargo-deny reason about them like
  any other registry crate.
- **No git allow-list.** cargo-deny no longer needs a bespoke allow
  entry for the upstream GitHub source.

Harder / accepted:
- **MSRV floor of 1.96.0.** Our toolchain must satisfy the crate's
  requirement; older Rust is no longer supported.

This supersedes ADR-0001.

## Alternatives considered

- **Keep the git pin (ADR-0001 status quo)** — rejected. Now that
  upstream publishes to crates.io, the git dep, `[patch]` block, and
  cargo-deny allow-list are pure overhead with no benefit.
- **Vendor the published crate in-tree** — rejected for the same
  reasons as in ADR-0001: divergent cadences and a large subtree to
  merge by hand for no gain.

## References

- ADR-0001 (superseded): [0001-consume-afm-via-git-tag.md](./0001-consume-afm-via-git-tag.md)
- `aozora-flavored-markdown` on crates.io:
  <https://crates.io/crates/aozora-flavored-markdown>
- `aozora-encoding` on crates.io:
  <https://crates.io/crates/aozora-encoding>
