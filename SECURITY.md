# Security policy

## Reporting a vulnerability

If you discover a security vulnerability in aozora-flavored-markdown-epub — a panic on
untrusted Aozora Flavored Markdown input, a directory traversal in the OPF / NAV writer, an
unsafe-Rust block introduced by a dependency, or anything with
exploitative potential — **do not open a public issue**. Instead:

1. Preferred: open a private report via
   [GitHub Security Advisories](https://github.com/P4suta/aozora-flavored-markdown-epub/security/advisories/new).
2. Alternative: email the maintainer at
   `42543015+P4suta@users.noreply.github.com` with the subject
   `[aozora-flavored-markdown-epub security] <short summary>`.

Please include:

- The shortest input or reproduction steps that trigger the issue.
- The aozora-flavored-markdown-epub version, the aozora-flavored-markdown crate version, and the Rust
  toolchain version.
- Whether the issue is reachable when converting user-supplied Aozora Flavored Markdown.

## Response expectations

- We acknowledge reports within **7 days**.
- Triage, patch, and coordinated disclosure typically complete within
  **30–60 days** for high-severity issues, faster for critical ones.

## Scope

In scope:
- Crashes / panics on any UTF-8 or Shift_JIS input within 10 MiB.
- Path traversal in OPF, NAV, or container.xml writers.
- ZIP slip in the EPUB packaging step.
- HTML-escape regressions in spine XHTML output.

Out of scope:
- Vulnerabilities in Aozora Flavored Markdown itself — please report those at
  <https://github.com/P4suta/aozora-flavored-markdown/security/advisories/new>.

## Supported versions

aozora-flavored-markdown-epub is pre-1.0. Only the `main` branch is supported.

| Version | Supported |
|---|---|
| main  | ✅ |
| <1.0  | ❌ (use main) |
