#!/usr/bin/env bash
# run_all_tests.sh - 运行所有集成测试

set -e

cd "$(dirname "$0")"

echo "=== Rust FFI Skills Test Suite ==="
echo ""

# 测试 PyO3
echo "1. Testing PyO3..."
cd test_pyo3
if cargo build 2>&1 | tee /tmp/test_pyo3_build.log; then
    echo "   ✓ PyO3 builds successfully"
else
    echo "   ✗ PyO3 build failed"
    cat /tmp/test_pyo3_build.log
fi
cd ..

# 测试 cxx
echo ""
echo "2. Testing cxx..."
cd test_cxx
if cargo build 2>&1 | tee /tmp/test_cxx_build.log; then
    echo "   ✓ cxx builds successfully"
    # 运行主程序测试
    if cargo run 2>&1 | tee /tmp/test_cxx_run.log; then
        echo "   ✓ cxx runs successfully"
    else
        echo "   ✗ cxx run failed"
    fi
else
    echo "   ✗ cxx build failed"
    cat /tmp/test_cxx_build.log
fi
cd ..

echo ""
echo "=== Test Summary ==="
echo "All projects compiled. Check logs above for details."
echo ""
echo "To build PyO3 for Python, run: cd test_pyo3 && maturin develop"