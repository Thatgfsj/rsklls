#!/bin/bash
# build.sh - Build WASM example

set -e

echo "Building WASM Fibonacci Demo..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    cargo install wasm-pack
fi

# Build for web target
wasm-pack build --target web --release

echo ""
echo "Build complete!"
echo "Output in pkg/"
echo ""
echo "To test with a local server:"
echo "  cd pkg && python3 -m http.server 8080"
echo "  # Then open http://localhost:8080"