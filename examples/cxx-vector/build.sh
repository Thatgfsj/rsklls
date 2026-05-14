#!/bin/bash
# build.sh - Build CXX example

set -e

echo "Building CXX Vector Demo..."

# Build Rust
cargo build --release

echo ""
echo "Build complete!"
echo ""
echo "C++ headers generated in:"
find target -name "*.h" -path "*cxx*" 2>/dev/null | head -3
echo ""
echo "See README.md for C++ integration instructions."