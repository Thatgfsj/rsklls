# rsklls - Rust Skills

<p align="center">
  <img src="https://img.shields.io/badge/Rust-Programming-blue?style=for-the-badge&logo=rust" alt="Badge">
  <img src="https://img.shields.io/badge/License-MIT-yellow?style=for-the-badge" alt="Badge">
  <img src="https://img.shields.io/badge/Rust-1.70+-brightgreen?style=for-the-badge" alt="Badge">
</p>

<p align="center">
  A universal Rust skills framework for cross-language development, GUI with Chinese support, and modular capability reuse.
</p>

---

## Overview

**rsklls** (Rust Skills) is a universal Rust framework providing reusable Rust development capabilities.

### Core Positioning

rsklls = **Capability Definition** + **Dispatch** + **Execution**

- Universal skill framework for any Rust project/AI Agent/CLI/GUI
- Cross-runtime reuse - write once, use everywhere
- Lightweight with zero runtime dependencies
- Multi-host support: OpenClaw, Tauri, CLI, custom systems

### Use Cases

- AI Agent skills - use as capability library for Claude, OpenAI, etc.
- Plugin systems - build pluggable capability modules
- Modular business logic - reuse capabilities across projects
- GUI applications - quickly build desktop apps with Chinese support

---

## Quick Start

### Installation

```bash
# Add to your Rust project
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

### Python Usage (PyO3)

```python
from my_rust_lib import greet, add, greet_chinese

print(greet("World"))           # Hello, World!
print(add(1, 2))               # 3
print(greet_chinese("张三"))    # 你好, 张三!
```

---

## Features

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

## Architecture

```
Host (OpenClaw / Tauri / CLI / Agent)
         │
         ▼
┌─────────────────────────────┐
│      rsklls Framework       │
├─────────────────────────────┤
│  Capability Layer           │
│  ├── Skills                 │
│  ├── FFI Bindings          │
│  └── GUI Components         │
├─────────────────────────────┤
│  Core Layer                 │
│  ├── Skill Registry         │
│  ├── Execution Engine       │
│  └── Type Converters        │
└─────────────────────────────┘
```

---

## Integration

### OpenClaw

```json
{
  "skills": {
    "rsklls": {
      "enabled": true,
      "trigger_words": ["rust", "pyo3", "rust go"]
    }
  }
}
```

### Tauri

```rust
use rsklls::ffi::pyo3;

#[tauri::command]
fn call_skill(skill_name: &str, args: &str) -> String {
    rsklls::execute(skill_name, args)
}
```

---

## Compatibility

| Category | Status |
|----------|--------|
| Rust Version | 1.70+ |
| no_std | Supported (optional) |
| Embedded | Supported |
| WASM | Planned |
| Platforms | Windows, Linux, macOS |

---

## Project Structure

```
rsklls/
├── src/
│   ├── cli.rs           # CLI argument parsing
│   ├── config.rs        # Configuration loading
│   ├── skill/
│   │   ├── mod.rs       # Skill core logic
│   │   ├── registry.rs  # Skill registration
│   │   └── executor.rs  # Execution engine
│   ├── ffi/             # FFI bindings
│   └── gui/             # GUI components
├── tests/               # Integration tests
├── benches/             # Benchmarks
├── Cargo.toml
├── rustfmt.toml
├── clippy.toml
└── .github/
    └── workflows/
        └── ci.yml
```

---

## Development

### Requirements

- Rust 1.70+
- Python 3.8+ (for PyO3 examples)
- Go 1.18+ (for cgo examples)

### Build

```bash
# Debug
cargo build

# Release
cargo build --release

# Run tests
cargo test

# Run clippy
cargo clippy -- -D warnings

# Format
cargo fmt
```

---

## Roadmap

- [ ] More host adapters (LangChain, AutoGen)
- [ ] FFI extensions (Node.js, Ruby)
- [ ] WASM support
- [ ] Hot reload for plugins
- [ ] Capability marketplace

---

## License

MIT License - See [LICENSE](./LICENSE) for details.

---

## Contributing

Contributions are welcome! Please submit issues and pull requests.
