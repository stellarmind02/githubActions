#!/bin/sh
set -eu
cd "$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
if cargo fmt --version >/dev/null 2>&1; then
    cargo fmt --all -- --check
else
    # Support system Rust installations without a configured rustup toolchain.
    find src tests -name '*.rs' -exec rustfmt --edition 2024 --check {} +
fi
cargo test --locked --all-targets
cargo test --locked --doc
