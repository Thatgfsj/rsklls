# rsklls - Rust 技能框架

<p align="center">
  <img src="https://img.shields.io/badge/Rust-Programming-blue?style=for-the-badge&logo=rust" alt="Badge">
  <img src="https://img.shields.io/badge/License-MIT-yellow?style=for-the-badge" alt="Badge">
  <img src="https://img.shields.io/badge/Rust-1.70+-brightgreen?style=for-the-badge" alt="Badge">
</p>

<p align="center">
  通用 Rust 技能框架 - 跨语言开发、GUI 中文支持、模块化能力复用
</p>

---

## 概览

**rsklls** (Rust Skills) 是一个通用的 Rust 框架，提供可复用的 Rust 开发能力。

### 核心定位

rsklls = 能力定义 + 调度 + 执行

- 通用技能框架 - 可被任何 Rust 项目/AI Agent/CLI/GUI 集成
- 跨运行时复用 - 能力模块化，一次编写，多处调用
- 轻量无侵入 - 零运行时依赖，可嵌入任何环境
- 多宿主适配 - OpenClaw、Tauri、其他 Agent、CLI 均可集成

### 使用场景

- AI Agent 技能 - 作为 Claude、OpenAI 等 AI Agent 的技能库
- 插件系统 - 为应用构建可插拔的能力模块
- 模块化业务逻辑 - 跨项目复用通用能力
- GUI 应用 - 快速构建支持中文的桌面应用

---

## 快速开始

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

## 功能模块

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

## 架构

```
宿主 (OpenClaw / Tauri / CLI / Agent)
         │
         ▼
┌─────────────────────────────┐
│       rsklls 框架           │
├─────────────────────────────┤
│  能力层                     │
│  ├── Skills                 │
│  ├── FFI 绑定              │
│  └── GUI 组件              │
├─────────────────────────────┤
│  核心层                     │
│  ├── Skill 注册器           │
│  ├── 执行引擎               │
│  └── 类型转换器             │
└─────────────────────────────┘
```

---

## 集成

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

## 兼容性

| 类别 | 状态 |
|------|------|
| Rust 版本 | 1.70+ |
| no_std | 支持（可选） |
| 嵌入式 | 支持 |
| WASM | 计划中 |
| 平台 | Windows, Linux, macOS |

---

## 项目结构

```
rsklls/
├── src/
│   ├── cli.rs           # CLI 命令行解析
│   ├── config.rs        # 配置加载
│   ├── skill/
│   │   ├── mod.rs       # 技能核心逻辑
│   │   ├── registry.rs  # 技能注册器
│   │   └── executor.rs  # 执行引擎
│   ├── ffi/             # FFI 绑定
│   └── gui/             # GUI 组件
├── tests/               # 集成测试
├── benches/             # 性能测试
├── Cargo.toml
├── rustfmt.toml
├── clippy.toml
└── .github/
    └── workflows/
        └── ci.yml
```

---

## 开发

### 要求

- Rust 1.70+
- Python 3.8+ (用于 PyO3 示例)
- Go 1.18+ (用于 cgo 示例)

### 构建

```bash
# Debug
cargo build

# Release
cargo build --release

# 测试
cargo test

# Clippy 检查
cargo clippy -- -D warnings

# 格式化
cargo fmt
```

---

## 路线图

- [ ] 更多宿主适配 (LangChain, AutoGen)
- [ ] FFI 扩展 (Node.js, Ruby)
- [ ] WASM 支持
- [ ] 插件热加载
- [ ] 能力市场

---

## 许可证

MIT License - 详见 [LICENSE](./LICENSE)

---

## 贡献

欢迎提交 Issue 和 Pull Request！
