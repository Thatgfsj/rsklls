#!/usr/bin/env bash
# generate_bindings.sh - Generate C/C++ bindings from Rust

set -e

PROJECT_DIR="${1:-.}"

if [ ! -f "$PROJECT_DIR/Cargo.toml" ]; then
    echo "Error: Not a Rust project (no Cargo.toml)"
    exit 1
fi

cd "$PROJECT_DIR"

echo "Generating C++ bindings..."

# 使用 cbindgen 生成 C 头文件
if command -v cbindgen &> /dev/null; then
    echo "Using cbindgen..."
    cbindgen --config cbindgen.toml --crate my_rust_lib --output include/rust_lib.h 2>/dev/null || \
    cbindgen --crate my_rust_lib --output include/rust_lib.h || true
else
    echo "cbindgen not found, skipping..."
    echo "Install with: cargo install cbindgen"
fi

# 使用 cxx 生成 C++ 绑定
if [ -f "build.rs" ] && grep -q "cxx-build" build.rs; then
    echo "Using cxx..."
    cargo build
    # cxx 会自动在 target/debug/build/*-out/ 生成头文件
    find target -name "*.h" -path "*cxx*" 2>/dev/null | head -5
fi

# 如果有 cxx::bridge，生成 C++ 包装器
if grep -q "cxx::bridge" src/lib.rs 2>/dev/null; then
    echo "cxx bridge detected - C++ headers generated during build"
fi

echo ""
echo "Bindings generation complete!"
echo ""
echo "检查生成的目录:"
ls -la include/ 2>/dev/null || echo "  include/ 目录不存在"
echo ""
echo "提示："
echo "  - 使用 cbindgen 生成纯 C 头文件"
echo "  - 使用 cxx 生成 C++ 兼容绑定"
echo "  - C++ 头文件位于 target/debug/build/*-out/"