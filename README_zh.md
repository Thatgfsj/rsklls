# rsklls - Rust 技能

<p align="center">
  <img src="https://img.shields.io/badge/Rust-Programming-blue?style=for-the-badge&logo=rust" alt="Badge">
  <img src="https://img.shields.io/badge/OpenClaw-Skill-green?style=for-the-badge&logo=open-source-initiative" alt="Badge">
  <img src="https://img.shields.io/badge/License-MIT-yellow?style=for-the-badge" alt="Badge">
</p>

<p align="center">
  🦞 为 OpenClaw 打造的 Rust 开发技能 - 包含 Rust 编程、Python-Rust 互操作 (PyO3)、Rust-Go 互操作，以及 GUI 程序中文显示问题解决方案。
</p>

---

## 📋 简介

rsklls 是一个 OpenClaw/Claude Code 技能，帮助完成 Rust 程序开发和互操作任务。

### ✨ 功能

- 🔧 **Rust 开发基础** - 核心概念、所有权、借用、生命周期
- 🐍 **Python-Rust 互操作** - 使用 PyO3 构建 Python 扩展
- 🔄 **Rust-Go 互操作** - C FFI、cxx、共享内存方案
- 🖥️ **GUI 中文显示** - Windows 终端和 GUI 中文显示解决方案

---

## 📦 包含内容

| 主题 | 描述 |
|------|------|
| 环境配置 | 各平台 Rust 安装 |
| 核心概念 | 所有权、借用、生命周期 |
| PyO3 | 使用 Rust 构建 Python 扩展 |
| Rust-Go 互操作 | C FFI、cxx、cgo、IPC |
| 中文显示 | Windows 控制台和 GUI 中文解决方案 |

---

## 🚀 快速开始

### 触发词

```
rust, rust开发, rust编程, pyo3, rust python, rust go, rust互转, rust GUI, 中文显示
```

### 使用示例

```
User: 如何用 Python 调用 Rust?
AI: [使用 rsklls 技能提供 PyO3 教程...]
```

---

## 📖 主题

### 1. Rust 开发基础

```bash
# 创建新项目
cargo new hello_rust
cd hello_rust
cargo run
```

### 2. Python-Rust (PyO3)

```rust
use pyo3::prelude::*;

#[pyfunction]
fn greet(name: &str) -> String {
    format!("你好, {}!", name)
}
```

### 3. Rust-Go 互操作

使用 C FFI 或 cxx crate 实现互操作。

### 4. 中文显示

Windows 控制台和 GUI 应用程序正确显示中文的解决方案。

---

## 📝 License

MIT License - See [LICENSE](./LICENSE) for details.

---

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

---

*🤖 由 OpenClaw Community 维护*
