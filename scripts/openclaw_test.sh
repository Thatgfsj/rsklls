#!/usr/bin/env bash
# openclaw_test.sh - 在模拟 OpenClaw 环境中测试 Rust 程序

set -e

BINARY="$1"
TIMEOUT="${2:-5}"

if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found: $BINARY"
    echo "Usage: $0 <binary_path> [timeout_seconds]"
    exit 1
fi

echo "Testing binary in simulated OpenClaw environment"
echo "Binary: $BINARY"
echo "Timeout: ${TIMEOUT}s"
echo ""

# 检查文件类型
echo "Binary info:"
file "$BINARY"
echo ""

# 检查是否静态链接
echo "Dynamic dependencies:"
ldd "$BINARY" 2>/dev/null || echo "  (static binary or not dynamic executable)"
echo ""

# 设置资源限制 (如果可用)
echo "Running with resource limits..."

# 使用 timeout 限制执行时间
if command -v timeout &> /dev/null; then
    if timeout "${TIMEOUT}" "$BINARY"; then
        echo ""
        echo "✓ Binary executed successfully within ${TIMEOUT}s"
    else
        exit_code=$?
        if [ $exit_code -eq 124 ]; then
            echo ""
            echo "✗ TIMEOUT: Binary exceeded ${TIMEOUT}s limit"
            exit 1
        else
            echo ""
            echo "✗ Binary exited with code: $exit_code"
            exit $exit_code
        fi
    fi
else
    # Windows 或无 timeout 命令时直接运行
    "$BINARY"
    echo ""
    echo "✓ Binary executed successfully"
fi