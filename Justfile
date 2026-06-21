# aozora-flavored-markdown-epub Justfile
# Docker-only execution (ADR-0002). 全てのコマンドは
# `docker compose run --rm dev <…>` を経由する。

set shell := ["bash", "-uc"]

# 開発コンテナ shorthand
_dev := "docker compose run --rm dev"

default:
    @just --list --unsorted

build:
    {{_dev}} cargo build --workspace --all-targets

test:
    {{_dev}} cargo nextest run --workspace

# fmt + clippy + typos + workspace lint
lint:
    {{_dev}} cargo fmt --all -- --check
    {{_dev}} cargo clippy --workspace --all-targets -- -D warnings
    {{_dev}} typos

# Line-coverage gate. Branch coverage needs a nightly toolchain
# (`-Z coverage-options=branch`); tracked as a separate follow-up.
coverage:
    {{_dev}} cargo llvm-cov --workspace --fail-under-lines 88

# example: ローカル fixture から EPUB を生成
example:
    {{_dev}} cargo run --release -p aozora-flavored-markdown-epub-cli -- \
        build --input examples/sample/manuscript --metadata examples/sample/book.toml --output out/sample.epub

# 生成済み EPUB を epubcheck で検証
validate path:
    {{_dev}} epubcheck {{path}}

# epubcheck（警告もエラー扱い）— CI ゲートで使用
validate-strict path:
    {{_dev}} epubcheck --failonwarnings {{path}}

# CI のフルパイプラインを再現
ci: lint test coverage example
    {{_dev}} epubcheck --failonwarnings out/sample.epub

# lefthook フック設定 (host にインストール済みの lefthook を使う)
hooks:
    lefthook install

# lefthook git hook stub を削除
hooks-uninstall:
    lefthook uninstall

# 開発ツール一式を mise でプロビジョニング (host inner loop / commit-msg hook)
setup:
    mise install

# Dependency advisory + license check
deny:
    {{_dev}} cargo deny check
