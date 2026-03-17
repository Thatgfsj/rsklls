# rsklls - Rust 技能框架

<p align="center">
  <img src="https://img.shields.io/badge/Rust-Programming-blue?style=for-the-badge&logo=rust" alt="Badge">
  <img src="https://img.shields.io/badge/OpenClaw-Skill-green?style=for-the-badge&logo=open-source-initiative" alt="Badge">
  <img src="https://img.shields.io/badge/License-MIT-yellow?style=for-the-badge" alt="Badge">
</p>

<p align="center">
  🦞 通用 Rust 技能框架 (Skill/Plugin Framework) - 跨语言开发、GUI 中文支持、模块化能力复用
</p>

---

## 📋 概览

**rsklls** 是一个通用的 Rust 技能框架，旨在提供可复用的 Rust 开发能力。

### 🎯 核心定位

rsklls = **能力定义** + **调度** + **执行**

- ⚡ **通用 Skill 框架** - 非 OpenClaw 专属，可被任何 Rust 项目/AI Agent/CLI/GUI 集成
- 🔄 **跨运行时复用** - 能力模块化，一次编写，多处调用
- 🪶 **轻量无侵入** - 零运行时依赖，可嵌入任何环境
- 🌐 **多宿主适配** - OpenClaw、Tauri、其他 Agent、CLI 均可集成

### ✨ 核心价值

| 价值点 | 说明 |
|--------|------|
| **跨运行时复用** | 能力模块化，一次开发，多场景使用 |
| **轻量无侵入** | 零额外依赖，可嵌入任何 Rust 项目 |
| **AI Agent 集成** | 可被任何 AI Agent 调用作为技能 |
| **多宿主支持** | OpenClaw、Tauri、CLI、自有系统 |

### 📊 使用场景

- 🤖 **AI Agent 技能** - 作为 Claude/OpenAI 等 AI Agent 的技能库
- 🔌 **插件系统** - 为应用构建可插拔的能力模块
- 📦 **模块化业务逻辑** - 跨项目复用通用能力
- 🖥️ **GUI 应用** - 快速构建支持中文的桌面应用

---

## 🚀 快速开始

### 安装

```bash
# 添加到你的 Rust 项目
cargo add rsklls
```

### 最小示例

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

### Python 使用 (PyO3)

```python
from my_rust_lib import greet, add, greet_chinese

print(greet("World"))           # Hello, World!
print(add(1, 2))               # 3
print(greet_chinese("张三"))    # 你好, 张三!
```

---

## 📦 功能模块

### 1. Rust 开发基础
- 环境配置（各平台）
- 核心概念：所有权、借用、生命周期
- 常用 crate 和模式
- 错误处理

### 2. Python-Rust 互操作 (PyO3)
- 使用 Rust 构建 Python 扩展
- 类型转换和错误处理
- 性能优化

### 3. Rust-Go 互操作
- C FFI 基础
- cxx crate 使用
- cgo 集成
- Protobuf/gRPC 模式

### 4. GUI 开发
- egui 集成
- Tauri 框架
- 中文字符显示解决方案
- Windows/Linux 兼容性

### 5. 中文显示解决方案
- Windows 控制台 UTF-8 配置
- Windows GUI 中文字体渲染
- egui/Tauri 中文字体设置

---

## 🏗️ 架构

```
┌─────────────────────────────────────────────────────────────┐
│                        宿主                                   │
│              (OpenClaw / Tauri / CLI / Agent)              │
└─────────────────────────┬───────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                      rsklls 框架                            │
├─────────────────────────────────────────────────────────────┤
│  能力层                                                      │
│  ├── Skills (装饰器函数)                                    │
│  ├── FFI 绑定 (PyO3, cxx, cgo)                           │
│  └── GUI 组件 (egui, Tauri)                               │
├─────────────────────────────────────────────────────────────┤
│  核心层                                                      │
│  ├── Skill 注册器                                          │
│  ├── 执行引擎                                              │
│  └── 类型转换器                                            │
└─────────────────────────────────────────────────────────────┘
```

---

## 🔧 集成

### OpenClaw 集成

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

### Tauri 集成

```rust
// src-tauri/src/main.rs
use rsklls::ffi::pyo3;

#[tauri::command]
fn call_skill(skill_name: &str, args: &str) -> String {
    rsklls::execute(skill_name, args)
}
```

### 独立 CLI

```bash
# 构建为 CLI 工具
cargo install rsklls-cli

# 使用命令
rsklls greet --name "张三"
rsklls add --a 1 --b 2
```

---

## 📋 兼容性

| 类别 | 状态 |
|------|------|
| **Rust 版本** | 1.70+ |
| **no_std** | 支持（可选） |
| **嵌入式** | 支持 (no_alloc 模式) |
| **WASM** | 计划中 |
| **平台** | Windows, Linux, macOS |

---

## 📁 项目结构

```
rsklls/
├── SKILL.md              # 技能主文档
├── README.md             # 英文说明
├── README_zh.md          # 中文版本
├── LICENSE               # MIT 许可证
├── Cargo.toml            # 工作区配置
├── rustfmt.toml         # 代码格式配置
├── clippy.toml          # Lint 配置
├── src/
│   ├── core/            # 核心框架
│   ├── ffi/             # FFI 绑定
│   │   ├── python/      # PyO3
│   │   └── go/          # cxx/cgo
│   └── gui/             # GUI 组件
└── examples/            # 使用示例
```

---

## 🗺️ 路线图

- [ ] 更多宿主适配 (LangChain, AutoGen)
- [ ] FFI 扩展 (Node.js, Ruby)
- [ ] WASM 支持 (浏览器/无环境)
- [ ] 插件热加载
- [ ] 能力市场 (Capability Marketplace)

---

## 📝 许可证

MIT License - 详见 [LICENSE](./LICENSE)

---

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

---

*🤖 由 OpenClaw Community 维护*
*🐙 由 Thatgfsj 创建*
