#!/bin/bash

echo "Deleting old build artifacts..."
rm -rf ./target

echo "Building Lambda package..."
cargo lambda build --release --arm64 --output-format zip