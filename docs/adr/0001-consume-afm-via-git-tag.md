# 0001. Consume afm via git tag dep, not as a vendored copy

- Status: superseded by ADR-0003
- Date: 2026-04-28
- Deciders: @P4suta
- Tags: architecture, dependency

## Context

afm-epub needs the same parser stack that afm itself uses to translate
afm sources to HTML. Three obvious shapes:

1. **Vendor afm in-tree** as `upstream/afm/`. Mirrors what afm does
   with comrak (afm ADR-0001).
2. **Spawn the `afm` CLI** from afm-epub at run time. Mirrors what
   afm-pandoc / afm-zola / afm-hugo do.
3. **Cargo git dependency, pinned to an afm revision.** Pull afm's
   library crates straight from its GitHub repo at a pinned commit.

afm publishes its v0.1 public surface specifically for sibling
consumers (afm ADR-0009).

## Decision

Adopt option 3: depend on afm's library crates via a git dep pinned to
a specific afm revision. Iterate locally via the
`[patch."https://github.com/P4suta/afm"]` block at the workspace root.

## Consequences

Easier:
- **Library-grade access.** afm-epub can call the parser directly,
  reuse the AST, and customise serialisation per spine item — none of
  which a CLI subprocess gives us.
- **No CLI dependency at runtime.** End users install afm-epub alone
  and get a single binary.
- **No vendored fork to maintain.** afm's release cadence is the only
  thing we track; we bump the tag in this workspace's `Cargo.toml` and
  re-test.

Harder:
- **Tag bumps are coordinated.** A breaking change in afm's library
  surface forces an afm-epub follow-up. afm ADR-0009 explicitly
  commits to a v0.1 public surface for exactly this reason; we trust
  that contract until it bends.
- **Build-time clones.** Cargo fetches the afm tree once per
  toolchain cache; this is a one-shot cost.

## Alternatives considered

- **Vendor afm in-tree** — rejected. afm and afm-epub develop at
  different cadences; vendoring would mean shipping a large subtree and
  merging upstream by hand for no benefit here.
- **Spawn `afm` CLI** — rejected. We need library-level access: wrap
  each rendered chapter as an XHTML spine item, build the NAV from
  headings, and surface errors with `miette` spans against the afm
  source. That is library work, not CLI work.

## References

- afm ADR-0009 (authoring tools / public surface):
  <https://github.com/P4suta/afm/blob/main/docs/adr/0009-authoring-tools-live-in-sibling-repositories.md>
