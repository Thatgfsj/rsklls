# Rust 学习路径

## 目录

- [学习路径概览](#学习路径概览)
- [入门阶段](#入门阶段)
- [进阶阶段](#进阶阶段)
- [高级阶段](#高级阶段)
- [学习资源](#学习资源)

---

## 学习路径概览

```
入门阶段 ──────► 进阶阶段 ──────► 高级阶段
(1-2个月)       (2-4个月)        (持续学习)
    │               │                │
    ▼               ▼                ▼
基础语法         异步编程         Unsafe Rust
所有权系统       并发模型         FFI绑定
借用与生命周期   宏编程           性能优化
错误处理         测试体系         编译器插件
```

---

## 入门阶段

### 目标

能够编写基本的 Rust 程序，理解核心概念，处理常见编译错误。

### 知识点清单

#### 1. 环境搭建

- [ ] 安装 Rust (rustup)
- [ ] 配置 IDE (VSCode + rust-analyzer)
- [ ] 理解 Cargo 基本命令
- [ ] 了解项目结构

```bash
# 安装
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 常用命令
cargo new project_name    # 创建项目
cargo build               # 构建
cargo run                 # 运行
cargo test                # 测试
cargo clippy              # 代码检查
cargo fmt                 # 格式化
```

#### 2. 基础语法

- [ ] 变量与可变性 (`let`, `mut`)
- [ ] 数据类型（标量、复合）
- [ ] 函数定义与调用
- [ ] 控制流（`if`, `loop`, `while`, `for`）
- [ ] 注释规范

```rust
fn main() {
    // 变量
    let x = 5;           // 不可变
    let mut y = 10;      // 可变
    
    // 类型注解
    let z: i32 = 100;
    
    // 控制流
    if x > y {
        println!("x > y");
    }
    
    for i in 0..10 {
        println!("{}", i);
    }
}
```

#### 3. 所有权系统 ⭐

- [ ] 所有权规则
- [ ] 移动语义
- [ ] 克隆 (Clone)
- [ ] Copy 类型

**必读文档**: [所有权](./basic_concepts/ownership.md)

#### 4. 借用与引用 ⭐

- [ ] 不可变引用 (`&T`)
- [ ] 可变引用 (`&mut T`)
- [ ] 引用规则（同时只能有一个可变引用或多个不可变引用）
- [ ] 悬垂引用预防

**必读文档**: [借用](./basic_concepts/borrowing.md)

#### 5. 生命周期 ⭐

- [ ] 生命周期标注语法
- [ ] 生命周期省略规则
- [ ] 结构体中的生命周期
- [ ] 静态生命周期

**必读文档**: [生命周期](./basic_concepts/lifetimes.md)

#### 6. 复合类型

- [ ] 结构体（命名、元组、单元）
- [ ] 枚举定义与使用
- [ ] `Option` 和 `Result`
- [ ] 数组与切片
- [ ] 向量 (`Vec`)

```rust
// 结构体
struct User {
    name: String,
    age: u32,
}

// 枚举
enum WebEvent {
    Click { x: i64, y: i64 },
    KeyPress(char),
    Paste(String),
}

// 使用
fn main() {
    let user = User { name: "Alice".into(), age: 30 };
    let event = WebEvent::Click { x: 100, y: 200 };
}
```

#### 7. 模式匹配

- [ ] `match` 表达式
- [ ] `if let` 表达式
- [ ] `while let` 表达式
- [ ] 解构模式

**必读文档**: [模式匹配](./basic_concepts/pattern_matching.md)

#### 8. 错误处理

- [ ] `panic!` 与不可恢复错误
- [ ] `Result<T, E>` 处理可恢复错误
- [ ] `?` 操作符
- [ ] 自定义错误类型

**必读文档**: [错误处理](./basic_concepts/error_handling.md)

#### 9. 泛型与 Trait

- [ ] 泛型函数、结构体、枚举
- [ ] Trait 定义与实现
- [ ] Trait Bounds
- [ ] 关联类型

**必读文档**: [泛型与Trait](./basic_concepts/generics_traits.md)

### 实践项目

1. **猜数字游戏**
   - 使用 `rand` crate
   - 处理用户输入
   - 比较和循环

2. **命令行工具**
   - 解析参数
   - 读写文件
   - 错误处理

3. **简单 CRUD 服务**
   - 结构体定义
   - 方法定义
   - 数据存储

---

## 进阶阶段

### 目标

能够编写并发程序、理解高级特性、使用常用生态系统库。

### 知识点清单

#### 1. 模块系统

- [ ] 模块定义 (`mod`)
- [ ] 可见性 (`pub`)
- [ ] `use` 引入
- [ ] 文件组织
- [ ] 工作空间 (Workspace)

#### 2. 标准库深入

- [ ] 集合类型 (`HashMap`, `HashSet`, `BTreeMap`)
- [ ] 字符串处理 (`String`, `&str`)
- [ ] 迭代器方法链
- [ ] 智能指针 (`Box`, `Rc`, `Arc`)
- [ ] `Cell` 和 `RefCell`

#### 3. 异步编程 ⭐

- [ ] `Future` trait
- [ ] `async/await` 语法
- [ ] `tokio` 运行时
- [ ] 异步 I/O
- [ ] 异步通道

**必读文档**: [异步编程](./advanced_features/async_programming.md)

#### 4. 并发编程 ⭐

- [ ] 线程创建 (`thread::spawn`)
- [ ] 消息传递 (`mpsc`)
- [ ] 共享状态 (`Mutex`, `RwLock`)
- [ ] 原子类型
- [ ] `Send` 和 `Sync` trait

**必读文档**: [并发编程](./advanced_features/concurrency.md)

#### 5. 宏编程

- [ ] 声明式宏 (`macro_rules!`)
- [ ] 过程宏 (派生宏、属性宏、函数式宏)
- [ ] 宏的卫生性

**必读文档**: [宏编程](./advanced_features/macros.md)

#### 6. 测试

- [ ] 单元测试
- [ ] 集成测试
- [ ] 文档测试
- [ ] 属性测试 (`proptest`)
- [ ] Mock 测试 (`mockall`)

**必读文档**: [测试体系](./best_practices/testing.md)

#### 7. 常用生态系统

| 领域 | 库 |
|------|-----|
| Web 框架 | `actix-web`, `axum`, `rocket` |
| 序列化 | `serde`, `serde_json` |
| 数据库 | `sqlx`, `diesel`, `sea-orm` |
| HTTP 客户端 | `reqwest` |
| 日志 | `tracing`, `log` |
| CLI | `clap` |
| GUI | `egui`, `iced`, `tauri` |

**必读文档**: [生态系统](./best_practices/ecosystem.md)

### 实践项目

1. **Web API 服务**
   - 使用 `axum` 或 `actix-web`
   - 数据库集成
   - JWT 认证
   - 错误处理

2. **异步爬虫**
   - `tokio` + `reqwest`
   - 并发控制
   - 数据解析

3. **CLI 工具**
   - `clap` 参数解析
   - 配置文件
   - 进度显示

---

## 高级阶段

### 目标

掌握 unsafe Rust、FFI、性能优化、底层原理。

### 知识点清单

#### 1. Unsafe Rust ⭐

- [ ] unsafe 块与函数
- [ ] 解引用原始指针
- [ ] 调用 unsafe 函数
- [ ] 访问可变静态变量
- [ ] 实现 unsafe trait

**必读文档**: [Unsafe Rust](./advanced_features/unsafe_rust.md)

#### 2. 内存布局

- [ ] 类型的内存表示
- [ ] `repr` 属性
- [ ] 零大小类型
- [ ] 对齐与填充

#### 3. FFI 与跨语言调用

- [ ] C FFI 基础
- [ ] `bindgen` 工具
- [ ] `cxx` crate
- [ ] Python 绑定 (`pyo3`)
- [ ] JavaScript 绑定 (`wasm-bindgen`)

**必读文档**: [FFI绑定](./advanced_features/ffi.md)

#### 4. 性能优化 ⭐

- [ ] 基准测试 (`criterion`)
- [ ] 性能分析工具
- [ ] 编译优化选项
- [ ] 内存优化
- [ ] 零成本抽象

**必读文档**: [性能优化](./performance_optimization/)

#### 5. 高级并发模式

- [ ] Actor 模型
- [ ] 无锁数据结构
- [ ] 协程与绿色线程
- [ ] 异步运行时原理

#### 6. 编译器内部

- [ ] 借用检查器原理
- [ ] 类型推断
- [ ] 编译器插件
- [ ] 过程宏深入

### 实践项目

1. **高性能服务**
   - 零拷贝优化
   - 连接池
   - 背压控制

2. **系统工具**
   - 内存映射
   - 文件系统操作
   - 进程管理

3. **嵌入式项目**
   - `no_std` 开发
   - 硬件抽象层
   - 实时操作系统

---

## 学习资源

### 官方资源

| 资源 | 链接 | 描述 |
|------|------|------|
| The Rust Book | https://doc.rust-lang.org/book/ | 官方教程 |
| Rust by Example | https://doc.rust-lang.org/rust-by-example/ | 示例学习 |
| Rustlings | https://github.com/rust-lang/rustlings | 练习题 |
| 标准库文档 | https://doc.rust-lang.org/std/ | API 文档 |

### 进阶书籍

| 书籍 | 描述 |
|------|------|
| 《Programming Rust》 | 全面深入 |
| 《Rust for Rustaceans》 | 高级主题 |
| 《Zero To Production In Rust》 | Web 后端开发 |
| Rustonomicon | Unsafe Rust 指南 |

### 社区资源

- [Rust 中文社区](https://rustcc.cn/)
- [Rust 官方论坛](https://users.rust-lang.org/)
- [Reddit r/rust](https://www.reddit.com/r/rust/)
- [Discord Rust 社区](https://discord.gg/rust-lang)

---

## 学习建议

1. **边学边练**：每个概念都要写代码验证
2. **阅读标准库**：学习最佳实践
3. **参与社区**：提问、回答、贡献代码
4. **写项目**：实际应用巩固知识
5. **关注错误**：编译器是老师，理解每条错误信息

---

*Last updated: 2026-03-18*
