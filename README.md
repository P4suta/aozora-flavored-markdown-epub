# afm-epub

EPUB3 generator for [Aozora Flavored Markdown
(afm)](https://github.com/P4suta/afm). Takes one or more `.md` files
(plain CommonMark + GFM, or afm with ruby / bouten / 縦中横 / 字下げ /
gaiji / accent decomposition), and produces a spec-compliant
**EPUB 3.3** package suitable for any current reading system —
Apple Books, Kobo, Calibre, Vivliostyle, Thorium.

## Status

Pre-alpha scaffolding. Cargo workspace + crate skeleton. No real
EPUB writing yet.

## What this gives you

```sh
# Render afm sources as a single EPUB3:
afm-epub build \
    --input ./manuscripts/ \
    --metadata ./book.toml \
    --output ./out/book.epub
```

- `book.toml` declares dc:title / dc:creator / dc:identifier /
  dc:language plus afm-specific options (writing mode, gaiji policy).
- Every `.md` under `--input` becomes one XHTML spine item.
- Bundled CSS themes (horizontal / vertical) cover every renderer-emitted
  class out of the box.
- `--validate` runs [epubcheck](https://github.com/w3c/epubcheck) over
  the produced file via the bundled wrapper.

## Sibling projects

| Repo | Role |
|---|---|
| [`P4suta/aozora`](https://github.com/P4suta/aozora) | Aozora Bunko notation parser |
| [`P4suta/afm`](https://github.com/P4suta/afm) | Aozora Flavored Markdown |
| [`P4suta/aozora-tools`](https://github.com/P4suta/aozora-tools) | Editor tooling |
| `P4suta/afm-hugo` | Hugo module |
| `P4suta/afm-zola` | Zola theme |
| `P4suta/afm-obsidian` | Obsidian plugin |
| `P4suta/afm-logseq` | Logseq plugin |
| `P4suta/afm-pandoc` | Pandoc Lua filter |
| `P4suta/afm-typst` | Typst package |
| **`P4suta/afm-epub`** (this repo) | **EPUB3 generator** |

## Compatibility

- Rust ≥ 1.95 (2024 edition).
- afm ≥ 0.2 (consumed via git tag dep — see ADR-0001).
- EPUB Reading Systems implementing **EPUB 3.3** (recent Apple Books,
  Kobo, Calibre 7+, Thorium 3+, Vivliostyle Reader 2025+).

## Development

This workspace is **Docker-only** (ADR-0002). Every cargo / test /
lint invocation goes through `docker compose run` via `just`:

```sh
docker compose build dev
just test
just build
just example
```

## Licence

Dual-licensed under [Apache-2.0](./LICENSE-APACHE) OR [MIT](./LICENSE-MIT)
at the user's option. See [`NOTICE`](./NOTICE) for third-party material.
