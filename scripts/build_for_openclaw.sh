#!/usr/bin/env bash
# build_for_openclaw.sh - 交叉编译 Rust 二进制并打包为 OpenClaw 镜像

set -e

TARGET="${1:-x86_64}"
PROJECT_NAME="${2:-my_rust_app}"

case "$TARGET" in
    x86_64)
        TARGET_TRIPLE="x86_64-unknown-linux-musl"
        ;;
    aarch64)
        TARGET_TRIPLE="aarch64-unknown-linux-musl"
        ;;
    *)
        echo "Usage: $0 <x86_64|aarch64> [project_name]"
        exit 1
        ;;
esac

echo "Building for OpenClaw: $TARGET_TRIPLE"

# 添加 target 如果不存在
rustup target add "$TARGET_TRIPLE" 2>/dev/null || true

# 构建
cargo build --release --target "$TARGET_TRIPLE"

# 创建输出目录
mkdir -p "dist/${TARGET}"

# 复制二进制
cp "target/${TARGET_TRIPLE}/release/${PROJECT_NAME}" "dist/${TARGET}/"

# 创建 tar 包
tar -czvf "dist/${PROJECT_NAME}-${TARGET}.tar.gz" -C "dist/${TARGET}" .

echo ""
echo "Build complete!"
echo "Output: dist/${PROJECT_NAME}-${TARGET}.tar.gz"
echo ""
echo "Verify static linking:"
file "dist/${TARGET}/${PROJECT_NAME}"