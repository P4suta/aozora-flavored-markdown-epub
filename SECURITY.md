# Security policy

## Reporting a vulnerability

If you discover a security vulnerability in afm-epub — a panic on
untrusted afm input, a directory traversal in the OPF / NAV writer, an
unsafe-Rust block introduced by a dependency, or anything with
exploitative potential — **do not open a public issue**. Instead:

1. Preferred: open a private report via
   [GitHub Security Advisories](https://github.com/P4suta/afm-epub/security/advisories/new).
2. Alternative: email the maintainer at
   `42543015+P4suta@users.noreply.github.com` with the subject
   `[afm-epub security] <short summary>`.

Please include:

- The shortest input or reproduction steps that trigger the issue.
- The afm-epub version, the afm crate version pin, and the Rust
  toolchain version.
- Whether the issue is reachable when converting user-supplied afm.

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
- Vulnerabilities in afm itself — please report those at
  <https://github.com/P4suta/afm/security/advisories/new>.

## Supported versions

afm-epub is pre-1.0. Only the `main` branch is supported.

| Version | Supported |
|---|---|
| main  | ✅ |
| <1.0  | ❌ (use main) |
