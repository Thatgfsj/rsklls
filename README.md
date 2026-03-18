# rsklls - Rust 技能框架

<p align="center">
  <img src="https://img.shields.io/badge/Rust-Programming-orange?style=for-the-badge&logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/License-MIT-yellow?style=for-the-badge" alt="License">
  <img src="https://img.shields.io/badge/Rust-1.70+-brightgreen?style=for-the-badge" alt="Rust Version">
</p>

<p align="center">
  <strong>全面的 Rust 开发技能框架</strong><br>
  从入门到精通的完整学习路径
</p>

---

## 📖 目录

- [概述](#概述)
- [学习路径](#学习路径)
- [文档目录](#文档目录)
- [快速开始](#快速开始)
- [核心特性](#核心特性)
- [生态系统](#生态系统)
- [贡献指南](#贡献指南)

---

## 概述

**rsklls** (Rust Skills) 是一个全面的 Rust 开发技能框架，提供：

- **系统化学习路径**：从入门到高级的完整学习规划
- **核心知识体系**：所有权、借用、生命周期等核心概念深度解析
- **实战项目案例**：真实项目场景的最佳实践
- **工程化指南**：Cargo、CI/CD、测试等工程实践
- **问题排查手册**：常见错误及解决方案

---

## 学习路径

### 🟢 入门阶段（1-2个月）

| 模块 | 内容 | 文档 |
|------|------|------|
| 环境搭建 | rustup、Cargo、IDE配置 | [学习路径](docs/learning_path.md) |
| 基础语法 | 变量、类型、控制流 | [学习路径](docs/learning_path.md) |
| 所有权系统 ⭐ | 所有权规则、移动、克隆 | [所有权](docs/basic_concepts/ownership.md) |
| 借用与引用 ⭐ | 引用规则、借用检查 | [借用](docs/basic_concepts/borrowing.md) |
| 生命周期 ⭐ | 生命周期标注、省略规则 | [生命周期](docs/basic_concepts/lifetimes.md) |
| 模式匹配 | match、if let、解构 | [模式匹配](docs/basic_concepts/pattern_matching.md) |
| 错误处理 | Result、Option、? | [错误处理](docs/basic_concepts/error_handling.md) |
| 泛型与Trait | 泛型、trait bounds | [泛型与Trait](docs/basic_concepts/generics_traits.md) |

### 🟡 进阶阶段（2-4个月）

| 模块 | 内容 | 文档 |
|------|------|------|
| 模块系统 | mod、pub、use | [学习路径](docs/learning_path.md) |
| 异步编程 ⭐ | Future、async/await、tokio | [异步编程](docs/advanced_features/async_programming.md) |
| 并发编程 ⭐ | 线程、通道、锁 | [并发编程](docs/advanced_features/concurrency.md) |
| 测试体系 | 单元测试、集成测试、mock | [测试体系](docs/best_practices/testing.md) |
| 生态系统 | 常用 crate 介绍 | [学习路径](docs/learning_path.md) |

### 🔴 高级阶段（持续学习）

| 模块 | 内容 | 文档 |
|------|------|------|
| Unsafe Rust | 原始指针、unsafe块 | [学习路径](docs/learning_path.md) |
| FFI | C绑定、pyo3 | [学习路径](docs/learning_path.md) |
| 性能优化 | 基准测试、火焰图 | [性能优化](docs/performance_optimization/index.md) |

---

## 文档目录

```
docs/
├── learning_path.md           # 学习路径总览
├── basic_concepts/            # 基础概念
│   ├── ownership.md          # 所有权
│   ├── borrowing.md          # 借用
│   ├── lifetimes.md          # 生命周期
│   ├── pattern_matching.md   # 模式匹配
│   ├── error_handling.md     # 错误处理
│   └── generics_traits.md    # 泛型与Trait
├── advanced_features/         # 高级特性
│   ├── async_programming.md  # 异步编程
│   └── concurrency.md        # 并发编程
├── best_practices/           # 最佳实践
│   ├── code_style.md         # 代码规范
│   └── testing.md            # 测试体系
├── performance_optimization/ # 性能优化
│   └── index.md
├── project_practice/         # 项目实战
│   └── cli_tool.md          # CLI工具案例
├── toolchain/               # 工具链
│   ├── cargo_advanced.md    # Cargo高级用法
│   └── ci_cd.md             # CI/CD配置
└── troubleshooting/         # 问题排查
    └── index.md
```

---

## 快速开始

### 安装

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 验证安装
rustc --version
cargo --version
```

### 创建项目

```bash
cargo new my_project
cd my_project
cargo run
```

### 最小示例

```rust
fn main() {
    // 所有权示例
    let s1 = String::from("hello");
    let s2 = s1;  // s1 移动到 s2
    
    // 借用示例
    let s3 = String::from("world");
    let len = calculate_length(&s3);  // 借用
    println!("{} 的长度是 {}", s3, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

---

## 核心特性

### 1. 所有权系统

Rust 的核心创新，在编译时保证内存安全：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // 所有权转移
    
    // println!("{}", s1);  // 错误！s1 已失效
    println!("{}", s2);     // 正确
}
```

详见：[所有权文档](docs/basic_concepts/ownership.md)

### 2. 借用检查器

编译时检查引用有效性：

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;      // 不可变借用
    let r2 = &s;      // 可以有多个不可变借用
    println!("{} {}", r1, r2);
    
    let r3 = &mut s;  // 可变借用（在 r1, r2 不再使用后）
    r3.push_str(" world");
}
```

详见：[借用文档](docs/basic_concepts/borrowing.md)

### 3. 零成本抽象

高级抽象没有运行时开销：

```rust
// 迭代器链 - 编译后与手写循环一样高效
let sum: i32 = (1..=100)
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .sum();
```

### 4. 错误处理

类型安全的错误处理：

```rust
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
```

详见：[错误处理文档](docs/basic_concepts/error_handling.md)

---

## 生态系统

### 常用 Crate

| 领域 | Crate | 用途 |
|------|-------|------|
| 异步运行时 | tokio, async-std | 异步编程 |
| Web框架 | axum, actix-web | 后端服务 |
| 序列化 | serde, serde_json | JSON/数据序列化 |
| 数据库 | sqlx, diesel | 数据库访问 |
| HTTP客户端 | reqwest | HTTP请求 |
| CLI | clap | 命令行参数 |
| 日志 | tracing, log | 日志记录 |
| 错误处理 | thiserror, anyhow | 错误定义/处理 |
| 测试 | proptest, mockall | 属性测试/Mock |

---

## 项目结构

```
rsklls/
├── docs/                    # 文档目录
├── src/                     # 源代码
│   ├── lib.rs              # 库入口
│   ├── main.rs             # CLI入口
│   └── skill/              # 技能模块
├── tests/                   # 集成测试
├── examples/               # 示例代码
├── Cargo.toml              # 项目配置
├── SKILL.md                # 技能说明文档
├── CHANGELOG.md            # 更新日志
└── CONTRIBUTING.md         # 贡献指南
```

---

## 开发

### 要求

- Rust 1.70+
- rustfmt
- clippy

### 构建

```bash
# 开发构建
cargo build

# Release 构建
cargo build --release

# 运行测试
cargo test

# 代码检查
cargo clippy -- -D warnings

# 格式化
cargo fmt
```

---

## 贡献指南

欢迎贡献！请参阅 [CONTRIBUTING.md](CONTRIBUTING.md)

### 贡献方式

1. Fork 仓库
2. 创建功能分支
3. 提交更改
4. 创建 Pull Request

---

## 许可证

MIT License - 详见 [LICENSE](LICENSE)

---

## 参考资源

- [The Rust Book](https://doc.rust-lang.org/book/) - 官方教程
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 示例学习
- [Rustlings](https://github.com/rust-lang/rustlings) - 练习题
- [标准库文档](https://doc.rust-lang.org/std/) - API 参考

---

<p align="center">
  <strong>Happy Coding with Rust! 🦀</strong>
</p>