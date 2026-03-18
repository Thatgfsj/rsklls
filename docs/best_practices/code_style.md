# 代码风格规范

## 目录

- [命名规范](#命名规范)
- [代码格式化](#代码格式化)
- [注释规范](#注释规范)
- [模块拆分原则](#模块拆分原则)
- [Clippy 规则](#clippy-规则)
- [常见反例与修正](#常见反例与修正)

---

## 命名规范

### 类型命名

```rust
// 结构体：PascalCase（大驼峰）
struct UserProfile {
    name: String,
}

// 枚举：PascalCase
enum HttpStatus {
    Ok,
    NotFound,
    InternalError,
}

// trait：PascalCase
trait Serialize {
    fn serialize(&self) -> String;
}
```

### 函数和变量命名

```rust
// 函数：snake_case（小写下划线）
fn calculate_total_price(items: &[Item]) -> f64 {
    // ...
}

// 变量：snake_case
let user_count = 100;
let is_active = true;

// 常量：SCREAMING_SNAKE_CASE
const MAX_CONNECTIONS: usize = 100;
static DEFAULT_PORT: u16 = 8080;

// 静态变量：SCREAMING_SNAKE_CASE
static mut COUNTER: u32 = 0;
```

### 模块命名

```rust
// 模块：snake_case
mod user_service;
mod database_connection;

// use 时可以重命名为更合适的名称
use std::io::Result as IoResult;
```

### 生命周期命名

```rust
// 简短生命周期：单个小写字母
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str;

// 复杂场景：有意义的名称
fn parse<'input, 'output>(input: &'input str) -> &'output str;
```

---

## 代码格式化

### rustfmt 配置

```toml
# rustfmt.toml
max_width = 100              # 最大行宽
tab_spaces = 4               # 缩进空格数
edition = "2021"             # Rust 版本
use_small_heuristics = "Max" # 紧凑格式
imports_granularity = "Module" # import 分组
group_imports = "StdExternalCrate" # import 排序
```

### 基本格式

```rust
// 函数定义
fn function_name(arg1: Type1, arg2: Type2) -> ReturnType {
    // 函数体
}

// 长参数换行
fn long_function(
    arg1: LongType1,
    arg2: LongType2,
    arg3: LongType3,
) -> LongReturnType {
    // ...
}

// 链式调用
let result = some_value
    .method1()
    .method2()
    .method3();

// match 分支
match value {
    Pattern1 => {
        // 多行代码块
        do_something();
    }
    Pattern2 => single_expression(),  // 单行可以省略大括号
}
```

### import 排序

```rust
// 标准库
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

// 外部 crate
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

// 本地模块
use crate::config::Config;
use crate::error::Error;
use super::utils;

// 父模块
use super::*;
```

---

## 注释规范

### 文档注释

```rust
/// 计算两个数的和。
///
/// # Examples
///
/// ```
/// use my_lib::add;
/// assert_eq!(add(1, 2), 3);
/// ```
///
/// # Panics
///
/// 当结果溢出时可能 panic。
///
/// # Errors
///
/// 此函数不返回错误。
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 用户信息结构体。
///
/// 存储用户的基本信息，包括用户名和年龄。
///
/// # Examples
///
/// ```
/// let user = User::new("Alice", 30);
/// println!("{}", user.name);
/// ```
pub struct User {
    /// 用户名，不能为空
    pub name: String,
    /// 用户年龄，必须为正数
    pub age: u32,
}
```

### 代码注释

```rust
fn complex_function() {
    // 单行注释：解释为什么这样做
    
    /*
     * 多行注释：
     * 用于较长的解释
     * 可以跨越多行
     */
    
    // TODO: 待实现的功能
    // FIXME: 需要修复的问题
    // NOTE: 重要的注意事项
    // SAFETY: unsafe 代码的安全保证
}
```

---

## 模块拆分原则

### 目录结构

```
my-project/
├── Cargo.toml
├── src/
│   ├── main.rs        # 二进制入口
│   ├── lib.rs         # 库入口
│   ├── config.rs      # 配置模块
│   ├── error.rs       # 错误定义
│   ├── models/        # 模型子模块
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── product.rs
│   └── services/      # 服务子模块
│       ├── mod.rs
│       ├── auth.rs
│       └── database.rs
```

### 模块组织

```rust
// src/lib.rs
pub mod config;
pub mod error;
pub mod models;
pub mod services;

// 重新导出常用类型
pub use error::{Error, Result};
pub use models::{User, Product};
```

```rust
// src/models/mod.rs
mod user;
mod product;

pub use user::User;
pub use product::Product;
```

### 可见性控制

```rust
pub struct PublicStruct {
    pub public_field: i32,
    pub(crate) crate_visible: i32,
    pub(super) parent_visible: i32,
    private_field: i32,  // 默认私有
}

pub fn public_function() {}

pub(crate) fn crate_only_function() {}

fn private_function() {}
```

---

## Clippy 规则

### 常用配置

```toml
# clippy.toml
msrv = "1.70"                    # 最低 Rust 版本
cognitive-complexity-threshold = 25  # 认知复杂度阈值
```

### .clippy.toml 或代码中配置

```rust
// 允许特定 lint
#[allow(clippy::too_many_arguments)]
fn many_args(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) {
}

// 警告级别
#[warn(clippy::pedantic)]
fn strict_function() {}
```

### 推荐启用的 lint

```toml
# .cargo/config.toml 或命令行
[lints.clippy]
pedantic = "warn"
nursery = "allow"
```

---

## 常见反例与修正

### 反例 1: 过长函数

```rust
// 错误：函数过长，难以理解
fn process_user(input: &str) -> Result<User, Error> {
    let parts: Vec<&str> = input.split(',').collect();
    let name = parts.get(0).ok_or(Error::InvalidInput)?;
    let age = parts.get(1).ok_or(Error::InvalidInput)?.parse()?;
    let email = parts.get(2).ok_or(Error::InvalidInput)?;
    let city = parts.get(3).ok_or(Error::InvalidInput)?;
    // ... 更多处理
    Ok(User { name: name.to_string(), age, email: email.to_string(), city: city.to_string() })
}

// 正确：拆分为小函数
fn parse_user(input: &str) -> Result<User, Error> {
    let parts: Vec<&str> = input.split(',').collect();
    
    Ok(User {
        name: parse_name(&parts)?,
        age: parse_age(&parts)?,
        email: parse_email(&parts)?,
        city: parse_city(&parts)?,
    })
}

fn parse_name(parts: &[&str]) -> Result<String, Error> {
    parts.get(0)
        .ok_or(Error::InvalidInput)
        .map(|s| s.to_string())
}

// ... 其他 parse_* 函数
```

### 反例 2: 过度嵌套

```rust
// 错误：嵌套太深
fn process(data: Option<Vec<i32>>) -> i32 {
    if let Some(v) = data {
        if !v.is_empty() {
            if v[0] > 0 {
                if v[0] < 100 {
                    return v[0] * 2;
                }
            }
        }
    }
    0
}

// 正确：提前返回
fn process(data: Option<Vec<i32>>) -> i32 {
    let v = match data {
        Some(v) => v,
        None => return 0,
    };
    
    if v.is_empty() || v[0] <= 0 || v[0] >= 100 {
        return 0;
    }
    
    v[0] * 2
}
```

### 反例 3: 不必要的 clone

```rust
// 错误：不必要的克隆
fn greet(name: &str) -> String {
    let owned = name.to_string();  // 不必要
    format!("Hello, {}", owned)
}

// 正确：直接使用引用
fn greet(name: &str) -> String {
    format!("Hello, {}", name)
}
```

### 反例 4: 不当的错误处理

```rust
// 错误：吞掉错误
fn read_config() -> Config {
    let content = std::fs::read_to_string("config.toml").unwrap_or_default();
    parse_config(&content).unwrap_or(Config::default())
}

// 正确：传播错误
fn read_config() -> Result<Config, Error> {
    let content = std::fs::read_to_string("config.toml")
        .map_err(|e| Error::Io(e))?;
    parse_config(&content)
        .map_err(|e| Error::Parse(e))
}
```

### 反例 5: 魔法数字

```rust
// 错误：魔法数字
fn calculate_discount(price: f64) -> f64 {
    price * 0.85  // 什么是 0.85？
}

// 正确：使用常量
const DISCOUNT_RATE: f64 = 0.85;  // 15% 折扣

fn calculate_discount(price: f64) -> f64 {
    price * DISCOUNT_RATE
}
```

---

## 参考资源

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/index.html)

---

*Last updated: 2026-03-18*
