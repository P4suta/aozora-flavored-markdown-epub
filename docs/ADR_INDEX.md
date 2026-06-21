# Architecture Decision Records

This directory holds [MADR 4.0](https://adr.github.io/madr/) Architecture
Decision Records. Each file documents one decision; once accepted an ADR is
never edited — it is *superseded* by a later ADR that links back.

| ADR                                          | Title                                          | Status   |
| -------------------------------------------- | ---------------------------------------------- | -------- |
| [0001](./adr/0001-consume-afm-via-git-tag.md) | Consume afm via git tag dep, not a vendored copy | superseded by ADR-0003 |
| [0002](./adr/0002-docker-only-execution.md)  | Docker-only execution                          | accepted |
| [0003](./adr/0003-consume-aozora-flavored-markdown-from-crates-io.md) | Consume aozora-flavored-markdown from crates.io | accepted |

## Authoring a new ADR

1. Scaffold with `cargo xtask new-adr 'my new decision'` (copies
   `adr/0000-template.md` to the next sequential number).
2. Fill in the sections; keep paragraphs short and action-oriented.
3. Add a row to the table above.
4. Reference the ADR in the commit body and open a PR. ADRs are normally
   accepted on merge; controversial ones land as `proposed` and flip to
   `accepted` once the discussion concludes.
