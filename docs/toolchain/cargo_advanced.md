# Cargo 高级用法

## 目录

- [工作区 (Workspace)](#工作区-workspace)
- [自定义构建脚本](#自定义构建脚本)
- [依赖管理](#依赖管理)
- [发布流程](#发布流程)
- [性能优化配置](#性能优化配置)

---

## 工作区 (Workspace)

工作区允许管理多个相关的包，共享一个 `Cargo.lock` 和目标目录。

### 创建工作区

```toml
# Cargo.toml (根目录)
[workspace]
members = [
    "crates/core",
    "crates/api",
    "crates/cli",
]
resolver = "2"
```

### 目录结构

```
my-workspace/
├── Cargo.toml          # 工作区配置
├── Cargo.lock          # 统一的锁文件
├── crates/
│   ├── core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   ├── api/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   └── cli/
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
└── target/             # 共享的构建目录
```

### 包间依赖

```toml
# crates/api/Cargo.toml
[package]
name = "api"
version = "0.1.0"

[dependencies]
core = { path = "../core" }  # 工作区内依赖
```

### 工作区命令

```bash
# 构建所有包
cargo build --workspace

# 运行特定包
cargo run -p api

# 测试特定包
cargo test -p core

# 检查所有包
cargo check --workspace
```

---

## 自定义构建脚本

### build.rs 基础

在项目根目录放置 `build.rs` 文件：

```rust
// build.rs
fn main() {
    // 在构建前执行
    println!("cargo:rerun-if-changed=src/schema.sql");
    
    // 生成代码
    generate_code();
}

fn generate_code() {
    // 代码生成逻辑
}
```

### 常用构建脚本功能

#### 1. 生成代码

```rust
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");
    
    fs::write(&dest_path, r#"
        pub fn generated_function() -> &'static str {
            "Hello from generated code"
        }
    "#).unwrap();
    
    println!("cargo:rerun-if-changed=build.rs");
}
```

使用生成的代码：
```rust
// src/main.rs
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

fn main() {
    println!("{}", generated_function());
}
```

#### 2. 编译 C 代码

```rust
fn main() {
    // 编译 C 库
    cc::Build::new()
        .file("src/native/mylib.c")
        .include("src/native")
        .compile("mylib");
    
    // 链接
    println!("cargo:rustc-link-lib=static=mylib");
    println!("cargo:rerun-if-changed=src/native/mylib.c");
}
```

#### 3. 条件编译

```rust
fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=dylib=user32");
    }
    
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=dylib=dl");
    }
}
```

---

## 依赖管理

### 版本指定

```toml
[dependencies]
serde = "1.0"                    # 语义化版本
tokio = { version = "1.0", features = ["full"] }  # 指定特性
my-lib = { git = "https://github.com/user/my-lib", branch = "main" }  # Git 依赖
my-local = { path = "../my-local" }  # 本地依赖
```

### 特性 (Features)

```toml
[package]
name = "my-lib"

[features]
default = ["std"]
std = []
async = ["tokio"]
full = ["std", "async"]

[dependencies]
tokio = { version = "1.0", optional = true }
```

使用特性：
```bash
cargo build --features async
cargo build --no-default-features
cargo build --all-features
```

### 依赖覆盖

```toml
# Cargo.toml
[patch.crates-io]
serde = { git = "https://github.com/serde-rs/serde" }

[replace]
"serde:1.0.0" = { git = "https://github.com/serde-rs/serde" }
```

### 可选依赖

```toml
[dependencies]
serde = { version = "1.0", optional = true }
tokio = { version = "1.0", optional = true }

[features]
serialization = ["serde"]
async-runtime = ["tokio"]
```

---

## 发布流程

### 发布前检查

```bash
# 1. 更新版本号
# 编辑 Cargo.toml 中的 version

# 2. 运行测试
cargo test --all-features

# 3. 运行 clippy
cargo clippy -- -D warnings

# 4. 检查文档
cargo doc --no-deps

# 5. 检查发布包内容
cargo package --list

# 6. 本地打包测试
cargo package
```

### 发布到 crates.io

```bash
# 登录
cargo login <api-token>

# 发布
cargo publish

# 发布特定包（工作区）
cargo publish -p my-lib
```

### 发布配置

```toml
# Cargo.toml
[package]
name = "my-lib"
version = "0.1.0"
edition = "2021"
license = "MIT"
description = "A brief description"
repository = "https://github.com/user/my-lib"
readme = "README.md"
keywords = ["rust", "library"]
categories = ["development-tools"]

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

---

## 性能优化配置

### Profile 配置

```toml
# Cargo.toml

# Release 配置
[profile.release]
opt-level = 3          # 优化等级 0-3
lto = true             # 链接时优化
codegen-units = 1      # 减少并行编译单元
strip = true           # 移除符号信息
panic = "abort"        # panic 时直接终止

# 开发配置
[profile.dev]
opt-level = 0
debug = true

# 自定义 profile
[profile.release-fast]
inherits = "release"
opt-level = 2
lto = false
```

### 编译时间优化

```toml
# 使用更快的链接器
# 在 .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# 使用 sccache 缓存
# 设置环境变量 RUSTC_WRAPPER=sccache
```

### 增量编译

```toml
[profile.dev]
incremental = true

[profile.release]
incremental = true
```

---

## 常用命令速查

| 命令 | 描述 |
|------|------|
| `cargo new` | 创建新项目 |
| `cargo init` | 在当前目录初始化 |
| `cargo build` | 构建 |
| `cargo build --release` | Release 构建 |
| `cargo run` | 运行 |
| `cargo test` | 测试 |
| `cargo bench` | 基准测试 |
| `cargo doc` | 生成文档 |
| `cargo doc --open` | 生成并打开文档 |
| `cargo check` | 快速检查 |
| `cargo clippy` | Lint 检查 |
| `cargo fmt` | 格式化 |
| `cargo clean` | 清理构建 |
| `cargo tree` | 查看依赖树 |
| `cargo outdated` | 检查过时依赖 |
| `cargo audit` | 安全审计 |
| `cargo install` | 安装二进制 |
| `cargo uninstall` | 卸载二进制 |

---

## 参考资源

- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Cargo Reference](https://doc.rust-lang.org/cargo/reference/)

---

*Last updated: 2026-03-18*
