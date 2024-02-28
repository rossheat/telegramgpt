#!/bin/bash

set -e

cargo fmt -- --check
cargo clippy -- -D warnings
cargo test

echo "Local CI checks passed."
