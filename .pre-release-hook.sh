#!/bin/sh

cargo fmt
cargo clippy --workspace --all-features
cargo test --workspace --all-features
cargo readme --output README.md
