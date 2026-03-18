# 所有权 (Ownership)

## 目录

- [定义](#定义)
- [核心规则](#核心规则)
- [所有权移动 (Move)](#所有权移动-move)
- [克隆 (Clone)](#克隆-clone)
- [使用场景](#使用场景)
- [常见陷阱](#常见陷阱)
- [示例代码](#示例代码)

---

## 定义

所有权是 Rust 最核心的特性，它是一套管理内存的规则系统。通过所有权机制，Rust 在编译时就能保证内存安全，无需垃圾回收器（GC）或手动内存管理。

### 为什么需要所有权？

传统语言的内存管理方式：
- **C/C++**: 手动管理，容易造成内存泄漏、悬垂指针、重复释放
- **Java/Python**: 垃圾回收（GC），运行时开销大，不可预测的暂停

Rust 的所有权系统在**编译时**就能检测出内存安全问题，实现了零成本抽象。

---

## 核心规则

1. **每个值都有一个所有者**（owner）
2. **同一时刻只能有一个所有者**
3. **当所有者离开作用域时，值会被自动释放**

```rust
fn main() {
    {
        let s = String::from("hello"); // s 进入作用域
        // s 是字符串的所有者
    } // s 离开作用域，内存被释放
    // println!("{}", s); // 错误！s 已不存在
}
```

---

## 所有权移动 (Move)

### 基本移动

当一个值被赋值给另一个变量时，所有权会被转移：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 的所有权移动到 s2
    
    // println!("{}", s1);  // 错误！s1 已失效
    println!("{}", s2);     // 正确
}
```

### 函数传参与移动

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);  // s 的所有权移动到函数
    
    // println!("{}", s);  // 错误！s 已失效
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}  // some_string 离开作用域，内存被释放
```

### 函数返回值与移动

```rust
fn main() {
    let s1 = gives_ownership();  // 函数将所有权转移给 s1
    println!("{}", s1);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string  // 所有权被移动出去
}
```

### Copy 类型

某些类型实现了 `Copy` trait，赋值时会复制值而不是移动：

```rust
fn main() {
    let x = 5;
    let y = x;  // x 被复制，而不是移动
    
    println!("x = {}, y = {}", x, y);  // 两者都有效
}
```

**实现了 Copy 的类型：**
- 所有整数类型：`i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- 浮点类型：`f32`, `f64`
- 布尔类型：`bool`
- 字符类型：`char`
- 元组（当所有元素都是 Copy 类型时）：`(i32, i32)` 是 Copy，但 `(i32, String)` 不是

---

## 克隆 (Clone)

当需要显式地深拷贝数据时，使用 `clone` 方法：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // 深拷贝
    
    println!("s1 = {}, s2 = {}", s1, s2);  // 两者都有效
}
```

### Clone vs Copy

| 特性 | Copy | Clone |
|------|------|-------|
| 触发方式 | 自动（赋值、传参） | 显式调用 `clone()` |
| 复制类型 | 浅拷贝（按位复制） | 深拷贝 |
| 适用类型 | 简单类型（栈上数据） | 任意类型 |
| 性能 | 快速 | 可能有性能开销 |

---

## 使用场景

### 1. 资源管理

```rust
use std::fs::File;
use std::io::Read;

struct FileProcessor {
    file: File,
}

impl FileProcessor {
    fn new(path: &str) -> std::io::Result<Self> {
        let file = File::open(path)?;
        Ok(FileProcessor { file })
    }
    
    fn read_content(&mut self) -> std::io::Result<String> {
        let mut content = String::new();
        self.file.read_to_string(&mut content)?;
        Ok(content)
    }
}
// 当 FileProcessor 被销毁时，File 会自动关闭
```

### 2. 线程间所有权转移

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    // data 的所有权移动到新线程
    let handle = thread::spawn(move || {
        println!("{:?}", data);
    });
    
    // println!("{:?}", data);  // 错误！data 已移动
    
    handle.join().unwrap();
}
```

### 3. 构建器模式

```rust
struct Server {
    host: String,
    port: u16,
}

impl Server {
    fn new(host: String) -> Self {
        Server { host, port: 80 }
    }
    
    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    
    fn build(self) -> Self {
        self
    }
}

fn main() {
    let server = Server::new("localhost".to_string())
        .port(8080)
        .build();
}
```

---

## 常见陷阱

### 陷阱 1: 使用已移动的值

```rust
// 错误示例
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1);  // 错误！value borrowed here after move
}

// 正确做法 1: 使用引用
fn main() {
    let s1 = String::from("hello");
    let s2 = &s1;  // 借用
    println!("{}", s1);  // 正确
}

// 正确做法 2: 克隆
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{} {}", s1, s2);  // 正确
}
```

### 陷阱 2: 集合元素移动

```rust
// 错误示例
fn main() {
    let v = vec![String::from("a"), String::from("b")];
    let first = v[0];  // 错误！cannot move out of index
}

// 正确做法
fn main() {
    let v = vec![String::from("a"), String::from("b")];
    let first = &v[0];  // 使用引用
    println!("{}", first);
}
```

### 陷阱 3: 部分移动

```rust
fn main() {
    let s = String::from("hello");
    let v = vec![1, 2, 3];
    
    let tuple = (s, v);
    
    // 部分移动
    let (s2, _) = tuple;
    
    // println!("{}", tuple.0);  // 错误！.0 已移动
    println!("{:?}", tuple.1);   // 正确，.1 仍有效
}
```

### 陷阱 4: 在循环中移动

```rust
// 错误示例
fn main() {
    let data = Some(String::from("hello"));
    
    for _ in 0..2 {
        if let Some(s) = data.clone() {  // 需要克隆
            println!("{}", s);
        }
    }
}
```

---

## 示例代码

### 完整示例：实现 RAII 模式

```rust
use std::ops::Drop;

struct DatabaseConnection {
    url: String,
    connected: bool,
}

impl DatabaseConnection {
    fn new(url: &str) -> Self {
        println!("正在连接数据库: {}", url);
        DatabaseConnection {
            url: url.to_string(),
            connected: true,
        }
    }
    
    fn query(&self, sql: &str) {
        if self.connected {
            println!("执行查询: {}", sql);
        }
    }
}

impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        println!("关闭数据库连接: {}", self.url);
        self.connected = false;
    }
}

fn main() {
    let db = DatabaseConnection::new("localhost:5432");
    db.query("SELECT * FROM users");
    // 离开作用域时自动调用 drop
}
```

### 完整示例：所有权与错误处理

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;  // ? 操作符可能提前返回错误
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)  // 所有权转移给调用者
}

fn process_file(path: &str) -> io::Result<()> {
    let content = read_file(path)?;  // 获得内容的所有权
    println!("文件内容长度: {}", content.len());
    Ok(())
}

fn main() {
    match process_file("example.txt") {
        Ok(_) => println!("处理成功"),
        Err(e) => eprintln!("处理失败: {}", e),
    }
}
```

---

## 参考资源

- [The Rust Book - 所有权](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Rust Reference - Ownership](https://doc.rust-lang.org/reference/ownership.html)

---

*Last updated: 2026-03-18*
