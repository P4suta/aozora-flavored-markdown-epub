# aozora-flavored-markdown-epub dev image — single-stage, mirrors Aozora Flavored Markdown's pattern.
# Pinned base for reproducibility; the rust toolchain is pinned through
# rust-toolchain.toml so the image's installed channel is just the
# bootstrap.

FROM rust:1.96-bookworm AS dev

ENV CARGO_TERM_COLOR=always \
    CARGO_NET_RETRY=10      \
    CARGO_INCREMENTAL=0     \
    RUSTC_WRAPPER=          \
    SCCACHE_DIR=/sccache    \
    DEBIAN_FRONTEND=noninteractive

# System tools we use during dev (zip for sanity-checking EPUB output,
# bsdtar for some test fixtures, just for the task runner).
RUN apt-get update                                                              \
 && apt-get install -y --no-install-recommends                                  \
        ca-certificates curl unzip zip libarchive-tools                         \
 && rm -rf /var/lib/apt/lists/*

# just task runner
RUN curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh        \
    | bash -s -- --to /usr/local/bin

# Cargo extensions for the dev workflow.
RUN cargo install --locked cargo-nextest cargo-llvm-cov cargo-deny cargo-audit  \
        sccache typos-cli

ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache

# epubcheck for `just validate`.
ARG EPUBCHECK_VERSION=5.2.0
RUN apt-get update                                                              \
 && apt-get install -y --no-install-recommends default-jre-headless             \
 && rm -rf /var/lib/apt/lists/*                                                 \
 && curl -fsSL                                                                  \
        https://github.com/w3c/epubcheck/releases/download/v${EPUBCHECK_VERSION}/epubcheck-${EPUBCHECK_VERSION}.zip \
        -o /tmp/epubcheck.zip                                                   \
 && unzip /tmp/epubcheck.zip -d /opt                                            \
 && ln -s /opt/epubcheck-${EPUBCHECK_VERSION}/epubcheck.jar /opt/epubcheck.jar  \
 && rm /tmp/epubcheck.zip                                                       \
 && printf '#!/usr/bin/env bash\nexec java -jar /opt/epubcheck.jar "$@"\n'      \
        > /usr/local/bin/epubcheck                                              \
 && chmod +x /usr/local/bin/epubcheck

WORKDIR /workspace
