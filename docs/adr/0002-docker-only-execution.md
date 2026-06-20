# 0002. Docker-only execution

- Status: accepted
- Date: 2026-04-28
- Deciders: @P4suta
- Tags: infra, build

## Context

afm-epub needs a Rust toolchain, afm's git-pulled crates, and the
Java-based `epubcheck` validator. Installing all three on the host
invites "works on my machine" failures, especially around epubcheck's
JRE. Sibling repos (afm, aozora) use the same Docker-only pattern.

## Decision

Every build / test / lint / coverage / validate invocation flows
through `docker compose run --rm dev <…>` via Justfile targets. Host
toolchain invocations are forbidden in automation.

The `dev` image bakes in:

- Rust 1.95 + workspace cargo extensions (`cargo-nextest`,
  `cargo-llvm-cov`, `cargo-deny`, `cargo-audit`, `sccache`).
- `typos`, `just`.
- `epubcheck` (and its JRE).

## Consequences

Easier:
- **Zero-friction onboarding.** `docker compose build dev && just test`
  is the entire bootstrap.
- **Reproducible CI.** The same image runs locally and in
  `.github/workflows/ci.yml`.
- **Stable epubcheck.** Pinning the version in the Dockerfile means
  validator drift cannot silently change CI verdicts.

Harder:
- **First build is slow.** The image takes a few minutes to build
  cold; mitigated by Docker layer cache + `sccache` for Cargo.
- **Host shell completions / IDE integration** require a separate
  rust-toolchain on the host for the editor to read. We accept this
  cost; editors only need to *read* the source — they never *run*
  cargo against it.

## Alternatives considered

- **Host-installed toolchain** — rejected. epubcheck plus its Java
  dependency makes "install everything on the host" too noisy.
- **Devcontainer only (no docker-compose)** — kept compatible.
  `docker-compose.yml`'s service definition can also feed a
  `.devcontainer/` config later; we are not exclusive.

## References

- afm ADR-0002:
  <https://github.com/P4suta/afm/blob/main/docs/adr/0002-docker-only-execution.md>
