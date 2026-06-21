# aozora-flavored-markdown-epub

EPUB3 generator for [Aozora Flavored Markdown
(Aozora Flavored Markdown)](https://github.com/P4suta/aozora-flavored-markdown). Takes one or more `.md` files
(plain CommonMark + GFM, or Aozora Flavored Markdown with ruby / bouten / 縦中横 / 字下げ /
gaiji / accent decomposition), and produces a spec-compliant
**EPUB 3.3** package suitable for any current reading system —
Apple Books, Kobo, Calibre, Vivliostyle, Thorium.

## Usage

```sh
aozora-flavored-markdown-epub build \
    --input ./manuscripts/ \
    --metadata ./book.toml \
    --output ./out/book.epub
```

- `book.toml` declares dc:title / dc:creator / dc:identifier /
  dc:language plus Aozora Flavored Markdown-specific options (writing mode, gaiji policy).
- Every `.md` under `--input` becomes one XHTML spine item.
- A bundled stylesheet styles Aozora Flavored Markdown's output (ruby, bouten, 縦中横, indent).

Validate a produced file with `just validate out/book.epub`
([epubcheck](https://github.com/w3c/epubcheck)).

## Sibling projects

- [`P4suta/aozora`](https://github.com/P4suta/aozora) — Aozora Bunko notation parser
- [`P4suta/aozora-flavored-markdown`](https://github.com/P4suta/aozora-flavored-markdown) — Aozora Flavored Markdown (this crate's input)

## Compatibility

- Rust ≥ 1.96 (2024 edition).
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
