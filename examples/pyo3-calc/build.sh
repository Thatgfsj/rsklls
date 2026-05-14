#!/bin/bash
# build.sh - Build PyO3 calculator

set -e

echo "Building PyO3 Calculator..."

# Clean
cargo clean

# Build release
maturin build --release

echo ""
echo "Build complete! Wheels in target/wheels/"
echo ""
echo "To install for development:"
echo "  maturin develop"