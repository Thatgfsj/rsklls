# 错误处理 (Error Handling)

## 目录

- [概述](#概述)
- [panic! 不可恢复错误](#panic-不可恢复错误)
- [Result<T, E>](#resultt-e)
- [Option<T>](#optiont)
- [? 操作符](#-操作符)
- [自定义错误类型](#自定义错误类型)
- [错误处理最佳实践](#错误处理最佳实践)
- [常见陷阱](#常见陷阱)

---

## 概述

Rust 将错误分为两类：

| 类型 | 处理方式 | 示例 |
|------|----------|------|
| **可恢复错误** | `Result<T, E>` | 文件不存在、网络超时 |
| **不可恢复错误** | `panic!` | 数组越界、空指针解引用 |

---

## panic! 不可恢复错误

### 何时使用 panic

- 程序处于无法恢复的状态
- 继续执行会导致更多问题
- 原型开发阶段的快速失败

### 基本用法

```rust
fn main() {
    panic!("程序崩溃了！");
}
```

### 自动触发 panic

```rust
fn main() {
    let v = vec![1, 2, 3];
    // v[99];  // panic: index out of bounds
    
    // 使用 get 更安全
    match v.get(99) {
        Some(n) => println!("{}", n),
        None => println!("索引越界"),
    }
}
```

### unwrap 和 expect

```rust
use std::fs::File;

fn main() {
    // unwrap: 失败时 panic
    let file = File::open("config.txt").unwrap();
    
    // expect: 失败时 panic 并显示自定义消息
    let file = File::open("config.txt")
        .expect("无法打开配置文件");
}
```

---

## Result<T, E>

### 定义

```rust
enum Result<T, E> {
    Ok(T),   // 成功，包含值
    Err(E),  // 失败，包含错误
}
```

### 基本用法

```rust
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    match read_file("hello.txt") {
        Ok(content) => println!("文件内容: {}", content),
        Err(e) => eprintln!("读取失败: {}", e),
    }
}
```

### 常用方法

```rust
use std::fs::File;

fn main() -> std::io::Result<()> {
    // ok: 转换为 Option<T>
    let result: Result<i32, &str> = Ok(5);
    let option: Option<i32> = result.ok();
    
    // err: 转换为 Option<E>
    let result: Result<i32, &str> = Err("error");
    let option: Option<&str> = result.err();
    
    // unwrap_or: 失败时返回默认值
    let value = result.unwrap_or(0);
    
    // unwrap_or_else: 失败时调用闭包
    let value = result.unwrap_or_else(|e| {
        println!("错误: {}", e);
        0
    });
    
    // map: 转换成功值
    let result: Result<i32, &str> = Ok(5);
    let doubled = result.map(|x| x * 2);  // Ok(10)
    
    // map_err: 转换错误值
    let result: Result<i32, &str> = Err("error");
    let mapped = result.map_err(|e| e.to_string());  // Err("error".to_string())
    
    // and_then: 链式操作
    let result: Result<i32, &str> = Ok(5);
    let chained = result.and_then(|x| {
        if x > 0 { Ok(x * 2) }
        else { Err("negative") }
    });
    
    Ok(())
}
```

---

## Option<T>

### 定义

```rust
enum Option<T> {
    Some(T),  // 有值
    None,     // 无值
}
```

### 基本用法

```rust
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

fn main() {
    match find_user(1) {
        Some(name) => println!("找到用户: {}", name),
        None => println!("用户不存在"),
    }
}
```

### 常用方法

```rust
fn main() {
    let some = Some(5);
    let none: Option<i32> = None;
    
    // unwrap: 获取值，None 时 panic
    // let value = none.unwrap();  // panic!
    
    // unwrap_or: 提供默认值
    let value = none.unwrap_or(0);  // 0
    
    // unwrap_or_else: 使用闭包
    let value = none.unwrap_or_else(|| {
        println!("计算默认值");
        10
    });
    
    // map: 转换值
    let doubled = some.map(|x| x * 2);  // Some(10)
    
    // and_then: 链式操作
    let result = some.and_then(|x| {
        if x > 0 { Some(x * 2) }
        else { None }
    });
    
    // or: 提供备选
    let value = none.or(Some(5));  // Some(5)
    
    // ok_or: 转换为 Result
    let result = none.ok_or("没有值");  // Err("没有值")
    
    // and: 返回另一个 Option
    let combined = some.and(Some(10));  // Some(10)
    let combined = none.and(Some(10));  // None
    
    // filter: 条件过滤
    let filtered = some.filter(|&x| x > 3);  // Some(5)
    let filtered = some.filter(|&x| x > 10);  // None
}
```

---

## ? 操作符

### 基本用法

`?` 操作符简化错误传播：
- 如果是 `Ok(v)` 或 `Some(v)`，返回 `v`
- 如果是 `Err(e)` 或 `None`，提前返回错误

```rust
use std::fs::File;
use std::io::{self, Read};

// 不使用 ?
fn read_file_old(path: &str) -> Result<String, io::Error> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }
    
    Ok(content)
}

// 使用 ?
fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
```

### 链式调用

```rust
use std::fs::File;
use std::io::Read;

fn read_config() -> Result<String, std::io::Error> {
    let mut content = String::new();
    File::open("config.toml")?.read_to_string(&mut content)?;
    Ok(content)
}
```

### 在 Option 上使用

```rust
fn get_first_char(s: Option<&str>) -> Option<char> {
    let s = s?;
    s.chars().next()
}

fn main() {
    let result = get_first_char(Some("hello"));  // Some('h')
    let result = get_first_char(None);           // None
}
```

### try! 宏（已废弃）

```rust
// 旧写法
let file = try!(File::open("hello.txt"));

// 新写法
let file = File::open("hello.txt")?;
```

---

## 自定义错误类型

### 基本实现

```rust
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct AppError {
    pub kind: String,
    pub message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError {
            kind: "IO".to_string(),
            message: err.to_string(),
        }
    }
}
```

### 使用 thiserror 简化

```rust
// Cargo.toml
// [dependencies]
// thiserror = "1.0"

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("数据读取失败")]
    ReadError(#[from] std::io::Error),
    
    #[error("解析错误: {0}")]
    ParseError(String),
    
    #[error("无效的配置: {key}")]
    ConfigError { key: String },
    
    #[error("未知错误")]
    Unknown,
}

fn parse_data(input: &str) -> Result<i32, DataStoreError> {
    input.parse()
        .map_err(|e: std::num::ParseIntError| DataStoreError::ParseError(e.to_string()))
}
```

### 使用 anyhow 处理应用错误

```rust
// Cargo.toml
// [dependencies]
// anyhow = "1.0"

use anyhow::{Context, Result};

fn read_config(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .context(format!("无法读取配置文件: {}", path))
}

fn main() -> Result<()> {
    let config = read_config("config.toml")?;
    println!("配置: {}", config);
    Ok(())
}
```

---

## 错误处理最佳实践

### 1. 库代码使用具体错误类型

```rust
// 库代码
#[derive(Debug)]
pub struct ParseError {
    pub position: usize,
    pub expected: &'static str,
    pub found: String,
}

pub fn parse(input: &str) -> Result<Vec<Token>, ParseError> {
    // ...
}
```

### 2. 应用代码使用 anyhow

```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let config = load_config()
        .context("加载配置失败")?;
    
    let data = fetch_data(&config)
        .context("获取数据失败")?;
    
    process_data(&data)
        .context("处理数据失败")?;
    
    Ok(())
}
```

### 3. 错误转换

```rust
use std::io;

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Parse(String),
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::Parse(err.to_string())
    }
}
```

### 4. 错误链追踪

```rust
use anyhow::{Context, Result};

fn deep_function() -> Result<()> {
    Err(anyhow::anyhow!("底层错误"))
}

fn middle_function() -> Result<()> {
    deep_function().context("中间层错误")
}

fn top_function() -> Result<()> {
    middle_function().context("顶层错误")
}

fn main() {
    match top_function() {
        Ok(_) => println!("成功"),
        Err(e) => {
            eprintln!("错误: {}", e);
            for cause in e.chain() {
                eprintln!("  原因: {}", cause);
            }
        }
    }
}
```

---

## 常见陷阱

### 陷阱 1: 忽略错误

```rust
// 错误示例
fn main() {
    let _ = std::fs::remove_file("temp.txt");  // 忽略错误
}

// 正确做法
fn main() {
    if let Err(e) = std::fs::remove_file("temp.txt") {
        eprintln!("删除失败: {}", e);
    }
}
```

### 陷阱 2: 过度使用 unwrap

```rust
// 错误示例
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = args.get(1).unwrap();  // 可能 panic
}

// 正确做法
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = args.get(1)
        .expect("请提供文件名参数");
}
```

### 陷阱 3: 错误类型不兼容

```rust
// 错误示例
fn mixed_errors() -> Result<String, std::io::Error> {
    let n: i32 = "abc".parse()?;  // ParseIntError 不兼容
    Ok(n.to_string())
}

// 正确做法 1: 使用 Box<dyn Error>
fn mixed_errors() -> Result<String, Box<dyn std::error::Error>> {
    let n: i32 = "abc".parse()?;
    Ok(n.to_string())
}

// 正确做法 2: 使用 anyhow
fn mixed_errors() -> anyhow::Result<String> {
    let n: i32 = "abc".parse()?;
    Ok(n.to_string())
}
```

### 陷阱 4: Option 和 Result 混用

```rust
// 不好的做法
fn find_and_parse(input: &str) -> Result<i32, String> {
    let s = input.find("number:")
        .ok_or("未找到 number")?;
    
    let num = input[s.len()..].parse()
        .map_err(|e| format!("解析失败: {}", e))?;
    
    Ok(num)
}
```

---

## 示例：完整的错误处理流程

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("连接失败: {0}")]
    ConnectionError(String),
    
    #[error("查询错误: {0}")]
    QueryError(String),
    
    #[error("记录未找到: {table}/{id}")]
    NotFound { table: String, id: u64 },
}

pub struct Database {
    // ...
}

impl Database {
    pub fn connect(url: &str) -> Result<Self, DatabaseError> {
        if url.is_empty() {
            return Err(DatabaseError::ConnectionError("URL 为空".into()));
        }
        Ok(Database { /* ... */ })
    }
    
    pub fn find_user(&self, id: u64) -> Result<User, DatabaseError> {
        // 模拟查询
        if id == 0 {
            return Err(DatabaseError::NotFound {
                table: "users".into(),
                id,
            });
        }
        Ok(User { id, name: "Test".into() })
    }
}

pub struct User {
    pub id: u64,
    pub name: String,
}

fn main() {
    match Database::connect("localhost:5432") {
        Ok(db) => {
            match db.find_user(1) {
                Ok(user) => println!("用户: {:?}", user),
                Err(e) => eprintln!("查询错误: {}", e),
            }
        }
        Err(e) => eprintln!("连接错误: {}", e),
    }
}
```

---

## 参考资源

- [The Rust Book - 错误处理](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [thiserror 文档](https://docs.rs/thiserror/)
- [anyhow 文档](https://docs.rs/anyhow/)

---

*Last updated: 2026-03-18*
