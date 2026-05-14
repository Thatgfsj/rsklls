# OpenClaw 集成指南

## 概述

OpenClaw 是 AI Agent 的沙箱执行环境。本指南说明如何在 OpenClaw 中安全高效地部署 Rust 程序。

---

## OpenClaw 沙箱模型与 Rust unsafe

### 安全模型

OpenClaw 提供了进程级隔离：
- 独立的文件系统视图
- CPU/内存限制
- 网络访问控制

Rust `unsafe` 代码在 OpenClaw 中需要额外注意：

1. **限制 unsafe 块范围**: 只在真正需要时使用 unsafe
2. **文档化 SAFETY 注释**: 每个 unsafe 块必须说明理由
3. **避免裸指针操作**: 使用引用代替裸指针
4. **正确处理 panic**: 跨语言边界捕获 panic

```rust
// OpenClaw 安全的 FFI 示例
#[no_mangle]
pub extern "C" fn safe_rust_function(data: *const c_char) -> *mut c_char {
    // SAFETY: OpenClaw 限制下，数据总是有效的 UTF-8
    // 调用者保证 data 非空且以 null 结尾
    let s = unsafe { CStr::from_ptr(data).to_str().unwrap_or("") };
    let result = process(s);
    CString::new(result).unwrap().into_raw()
}
```

---

## 资源限制

### 配置 Cargo.toml

```toml
[package]
name = "openclaw-rust"
version = "1.0.0"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"  # OpenClaw 建议：避免 panic 展开
```

### 内存限制

Rust 程序在 OpenClaw 中应避免：
- 大规模内存分配
- 内存泄漏
- 无限增长的数据结构

```rust
// 预分配限制
const MAX_BUFFER_SIZE: usize = 1024 * 1024;  // 1MB

fn process_with_limit(data: &[u8]) -> Result<Vec<u8>, &'static str> {
    if data.len() > MAX_BUFFER_SIZE {
        return Err("Data exceeds limit");
    }
    Ok(data.to_vec())
}
```

---

## 跨架构编译

### 支持的目标

OpenClaw 支持：
- `x86_64-unknown-linux-musl` (最常见)
- `aarch64-unknown-linux-musl` (ARM64)
- `x86_64-pc-windows-gnu`

### 交叉编译配置

```toml
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"

[target.aarch64-unknown-linux-musl]
linker = "aarch64-linux-musl-gcc"
```

### 安装交叉编译工具

```bash
# Linux x86_64
rustup target add x86_64-unknown-linux-musl

# ARM64
rustup target add aarch64-unknown-linux-musl

# 安装 musl-gcc
# Debian/Ubuntu: apt install musl-dev
# macOS: brew install FiloSottile/musl-cross/musl-cross
```

---

## Docker 镜像打包

### 使用 scratch 镜像 (零基础镜像)

```dockerfile
# stage 1: build
FROM rust:1.77 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# stage 2: package
FROM alpine:3.19
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/my_app /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/my_app"]
```

### 使用 distroless

```dockerfile
FROM rust:1.77 as builder
WORKDIR /app
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM gcr.io/distroless/cc
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/my_app /my_app
ENTRYPOINT ["/my_app"]
```

### 多架构打包脚本

```bash
#!/bin/bash
# scripts/build_for_openclaw.sh

TARGET=$1
PROJECT_NAME=${2:-my_rust_app}

case "$TARGET" in
    x86_64)
        TARGET_TRIPLE="x86_64-unknown-linux-musl"
        ;;
    aarch64)
        TARGET_TRIPLE="aarch64-unknown-linux-musl"
        ;;
    *)
        echo "Usage: $0 <x86_64|aarch64> <project_name>"
        exit 1
        ;;
esac

echo "Building for $TARGET_TRIPLE..."

# 构建
cargo build --release --target "$TARGET_TRIPLE"

# 创建 tar 包
mkdir -p "dist/${TARGET}"
cp "target/${TARGET_TRIPLE}/release/${PROJECT_NAME}" "dist/${TARGET}/"

tar -czvf "dist/${PROJECT_NAME}-${TARGET}.tar.gz" -C "dist/${TARGET}" .

echo "Done: dist/${PROJECT_NAME}-${TARGET}.tar.gz"
```

---

## OpenClaw 本地测试

### 测试脚本

```bash
#!/bin/bash
# scripts/openclaw_test.sh

BINARY=$1
TIMEOUT=${2:-5}

if [ ! -f "$BINARY" ]; then
    echo "Binary not found: $BINARY"
    exit 1
fi

echo "Testing binary: $BINARY"
echo "Timeout: ${TIMEOUT}s"

# 模拟 OpenClaw 限制环境
ulimit -t "$TIMEOUT" 2>/dev/null || true
ulimit -m 1048576 2>/dev/null || true  # 1GB memory limit

# 运行
timeout "$TIMEOUT" "$BINARY" || exit_code=$?

if [ $? -eq 124 ]; then
    echo "TIMEOUT: Binary exceeded ${TIMEOUT}s limit"
    exit 1
fi

echo "Binary executed successfully"
```

---

## OpenClaw API 调用示例

```rust
// 使用 reqwest 调用 OpenClaw API (如果需要)
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ExecuteRequest {
    command: String,
    timeout_ms: u32,
}

#[derive(Deserialize)]
struct ExecuteResponse {
    stdout: String,
    stderr: String,
    exit_code: i32,
}

async fn execute_in_openclaw(
    base_url: &str,
    api_key: &str,
    binary_path: &str,
) -> Result<ExecuteResponse, Box<dyn std::error::Error>> {
    let client = Client::new();

    let req = ExecuteRequest {
        command: binary_path.to_string(),
        timeout_ms: 5000,
    };

    let resp = client
        .post(&format!("{}/api/execute", base_url))
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&req)
        .send()
        .await?;

    Ok(resp.json().await?)
}
```

---

## 最佳实践

### 1. 静态链接

```toml
[target.x86_64-unknown-linux-musl]
rustflags = ["-C", "target-feature=-crt-static"]
```

### 2. 去除调试信息

```toml
[profile.release]
strip = true  # 去除符号表
```

### 3. 验证二进制

```bash
# 检查是否静态链接
file my_app
ldd my_app  # 应该输出 "not a dynamic executable"

# 检查去除的符号
nm my_app | head  # 应该几乎没有符号
```

---

## 故障排查

### 问题：段错误
- 检查指针是否有效
- 验证内存分配/释放配对
- 确保使用 `panic::catch_unwind`

### 问题：超时
- 减少程序启动时间
- 检查是否有无限循环
- 增加 OpenClaw 超时限制

### 问题：内存超限
- 减少堆内存使用
- 使用 `#[global_allocator]` 限制
- 检查内存泄漏