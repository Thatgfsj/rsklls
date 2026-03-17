# rsklls - Rust Skills

<p align="center">
  <img src="https://img.shields.io/badge/Rust-Programming-blue?style=for-the-badge&logo=rust" alt="Badge">
  <img src="https://img.shields.io/badge/OpenClaw-Skill-green?style=for-the-badge&logo=open-source-initiative" alt="Badge">
  <img src="https://img.shields.io/badge/License-MIT-yellow?style=for-the-badge" alt="Badge">
</p>

<p align="center">
  🦞 A通用 Rust 技能框架 (Skill/Plugin Framework) - 跨语言开发、GUI 中文支持、模块化能力复用
</p>

---

## 📋 Overview

**rsklls** 是一个通用的 Rust 技能框架，旨在提供可复用的 Rust 开发能力。

### 🎯 Core Positioning

rsklls = **能力定义** + **调度** + **执行**

- ⚡ **通用 Skill 框架** - 非 OpenClaw 专属，可被任何 Rust 项目/AI Agent/CLI/GUI 集成
- 🔄 **跨运行时复用** - 能力模块化，一次编写，多处调用
- 🪶 **轻量无侵入** - 零运行时依赖，可嵌入任何环境
- 🌐 **多宿主适配** - OpenClaw、Tauri、其他 Agent、CLI 均可集成

### ✨ Core Value

| Value | Description |
|-------|-------------|
| **跨运行时复用** | 能力模块化，一次开发，多场景使用 |
| **轻量无侵入** | 零额外依赖，可嵌入任何 Rust 项目 |
| **AI Agent 集成** | 可被任何 AI Agent 调用作为技能 |
| **多宿主支持** | OpenClaw、Tauri、CLI、自有系统 |

### 📊 Use Cases

- 🤖 **AI Agent 技能** - 作为 Claude/OpenAI 等 AI Agent 的技能库
- 🔌 **插件系统** - 为应用构建可插拔的能力模块
- 📦 **模块化业务逻辑** - 跨项目复用通用能力
- 🖥️ **GUI 应用** - 快速构建支持中文的桌面应用

---

## 🚀 Quick Start

### Installation

```bash
# 添加到你的 Rust 项目
cargo add rsklls
```

### Minimal Example

```rust
use rsklls::skill;

#[skill]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[skill]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[skill]
fn greet_chinese(name: &str) -> String {
    format!("你好, {}!", name)
}
```

### Use in Python (PyO3)

```python
from my_rust_lib import greet, add, greet_chinese

print(greet("World"))           # Hello, World!
print(add(1, 2))               # 3
print(greet_chinese("张三"))    # 你好, 张三!
```

---

## 📦 Features

### 1. Rust Development Basics
- Environment setup (all platforms)
- Core concepts: Ownership, Borrowing, Lifetimes
- Common crates and patterns
- Error handling

### 2. Python-Rust Interop (PyO3)
- Building Python extensions in Rust
- Type conversions and error handling
- Performance optimization

### 3. Rust-Go Interop
- C FFI fundamentals
- cxx crate usage
- cgo integration
- Protobuf/gRPC patterns

### 4. GUI Development
- egui integration
- Tauri framework
- Chinese character display solutions
- Windows/Linux compatibility

### 5. Chinese Display Solutions
- Windows console UTF-8 configuration
- Windows GUI Chinese font rendering
- egui/Tauri Chinese font setup

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Host                                  │
│              (OpenClaw / Tauri / CLI / Agent)              │
└─────────────────────────┬───────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                      rsklls Framework                       │
├─────────────────────────────────────────────────────────────┤
│  Capability Layer                                           │
│  ├── Skills (decorated functions)                         │
│  ├── FFI Bindings (PyO3, cxx, cgo)                       │
│  └── GUI Components (egui, Tauri)                         │
├─────────────────────────────────────────────────────────────┤
│  Core Layer                                                │
│  ├── Skill Registry                                        │
│  ├── Execution Engine                                      │
│  └── Type Converters                                       │
└─────────────────────────────────────────────────────────────┘
```

---

## 🔧 Integration

### OpenClaw Integration

```yaml
# openclaw.json
{
  "skills": {
    "rsklls": {
      "enabled": true,
      "trigger_words": ["rust", "pyo3", "rust go"]
    }
  }
}
```

### Tauri Integration

```rust
// src-tauri/src/main.rs
use rsklls::ffi::pyo3;

#[tauri::command]
fn call_skill(skill_name: &str, args: &str) -> String {
    rsklls::execute(skill_name, args)
}
```

### Standalone CLI

```bash
# Build as CLI tool
cargo install rsklls-cli

# Use as command
rsklls greet --name "张三"
rsklls add --a 1 --b 2
```

---

## 📋 Compatibility

| Category | Status |
|----------|--------|
| **Rust Version** | 1.70+ |
| **no_std** | Supported (optional) |
| **Embedded** | Supported (no_alloc mode) |
| **WASM** | Planned |
| **Platforms** | Windows, Linux, macOS |

---

## 📁 Project Structure

```
rsklls/
├── SKILL.md              # Main skill documentation
├── README.md             # This file
├── README_zh.md          # Chinese version
├── LICENSE               # MIT License
├── Cargo.toml            # Workspace config
├── rustfmt.toml          # Code format config
├── clippy.toml          # Lint config
├── src/
│   ├── core/            # Core framework
│   ├── ffi/             # FFI bindings
│   │   ├── python/      # PyO3
│   │   └── go/          # cxx/cgo
│   └── gui/             # GUI components
└── examples/            # Usage examples
```

---

## 🗺️ Roadmap

- [ ] 更多宿主适配 (LangChain, AutoGen)
- [ ] FFI 扩展 (Node.js, Ruby)
- [ ] WASM 支持 (浏览器/无环境)
- [ ] 插件热加载
- [ ] 能力市场 (Capability Marketplace)

---

## 📝 License

MIT License - See [LICENSE](./LICENSE) for details.

---

## 🤝 Contributing

Welcome! Please submit issues and pull requests.

---

*🤖 Powered by OpenClaw Community*
*🐙 Created by Thatgfsj*
