# 问题排查

## 目录

- [常见编译错误](#常见编译错误)
- [所有权相关错误](#所有权相关错误)
- [生命周期错误](#生命周期错误)
- [并发问题](#并发问题)
- [调试技巧](#调试技巧)

---

## 常见编译错误

### E0382: 使用已移动的值

```
error[E0382]: use of moved value: `s`
 --> src/main.rs:4:20
  |
2 |     let s = String::from("hello");
3 |     let s2 = s;
  |             -- value moved here
4 |     println!("{}", s);
  |                    ^ value used here after move
```

**解决方案**：
```rust
// 方案 1: 使用引用
let s = String::from("hello");
let s2 = &s;
println!("{}", s);

// 方案 2: 克隆
let s = String::from("hello");
let s2 = s.clone();
println!("{}", s);
```

### E0425: 找不到值

```
error[E0425]: cannot find value `x` in this scope
 --> src/main.rs:2:5
  |
2 |     x
  |     ^ not found in this scope
```

**解决方案**：检查变量名拼写，确保变量在作用域内。

### E0308: 类型不匹配

```
error[E0308]: mismatched types
 --> src/main.rs:2:5
  |
2 |     let x: i32 = "hello";
  |     ^^^^^^^^^^   ------- expected `i32`, found `&str`
  |     |
  |     expected due to this
```

**解决方案**：确保类型匹配，必要时使用类型转换。

### E0597: 借用生命周期不够长

```
error[E0597]: `x` does not live long enough
 --> src/main.rs:3:5
  |
2 |     let r;
3 |     r = &x;
  |     ^^^^^^ borrowed value does not live long enough
4 | }
  | - `x` dropped here while still borrowed
```

**解决方案**：确保引用不会超过被引用值的生命周期。

---

## 所有权相关错误

### 错误: 返回局部变量的引用

```rust
// 错误
fn get_string() -> &str {
    let s = String::from("hello");
    &s  // s 在函数结束时被释放
}

// 解决方案 1: 返回所有权
fn get_string() -> String {
    String::from("hello")
}

// 解决方案 2: 返回静态字符串
fn get_string() -> &'static str {
    "hello"
}
```

### 错误: 不能可变地借用多次

```rust
// 错误
fn main() {
    let mut v = vec![1, 2, 3];
    let r1 = &mut v[0];
    let r2 = &mut v[1];  // 错误！
    *r1 += 1;
    *r2 += 1;
}

// 解决方案 1: 分开借用
fn main() {
    let mut v = vec![1, 2, 3];
    v[0] += 1;
    v[1] += 1;
}

// 解决方案 2: 使用索引
fn main() {
    let mut v = vec![1, 2, 3];
    for i in 0..v.len() {
        v[i] += 1;
    }
}
```

### 错误: 结构体字段借用冲突

```rust
struct Point {
    x: i32,
    y: i32,
}

// 错误
fn main() {
    let mut p = Point { x: 1, y: 2 };
    let x_ref = &mut p.x;
    let y_ref = &mut p.y;  // Rust 2018+ 允许这个！
    *x_ref += 1;
    *y_ref += 1;
}

// Rust 2018+ 支持 部分借用，上面的代码是正确的！
```

---

## 生命周期错误

### 错误: 生命周期标注不足

```rust
// 错误
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

// 解决方案: 添加生命周期标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### 错误: 结构体生命周期

```rust
// 错误
struct Parser {
    input: &str,  // 缺少生命周期
}

// 解决方案
struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input }
    }
}
```

### 错误: 静态生命周期要求

```rust
// 错误
fn static_example() -> &'static str {
    let s = String::from("hello");
    &s  // 局部变量不能有 'static 生命周期
}

// 解决方案
fn static_example() -> &'static str {
    "hello"  // 字符串字面量有 'static 生命周期
}
```

---

## 并发问题

### 错误: Rc 不是线程安全的

```rust
// 错误
use std::rc::Rc;
use std::thread;

fn main() {
    let data = Rc::new(5);
    let data2 = Rc::clone(&data);
    thread::spawn(move || {
        println!("{}", data2);  // 编译错误
    });
}

// 解决方案: 使用 Arc
use std::sync::Arc;

fn main() {
    let data = Arc::new(5);
    let data2 = Arc::clone(&data);
    thread::spawn(move || {
        println!("{}", data2);
    });
}
```

### 错误: RefCell 不是线程安全的

```rust
// 错误
use std::cell::RefCell;
use std::sync::Arc;

fn main() {
    let data = Arc::new(RefCell::new(5));
    // 编译错误：RefCell 不是 Sync
}

// 解决方案: 使用 Mutex
use std::sync::Mutex;

fn main() {
    let data = Arc::new(Mutex::new(5));
    let data2 = Arc::clone(&data);
    
    thread::spawn(move || {
        let mut guard = data2.lock().unwrap();
        *guard += 1;
    });
}
```

### 死锁排查

```rust
// 使用 backtrace 排查死锁
// 设置环境变量
// RUST_BACKTRACE=1

// 或使用工具
// cargo install deadlock-detection
```

---

## 调试技巧

### 1. 使用 dbg! 宏

```rust
fn main() {
    let x = 5;
    let y = dbg!(x * 2);
    // 输出: [src/main.rs:3] x * 2 = 10
    
    dbg!(y + 1);
    // 输出: [src/main.rs:6] y + 1 = 11
}
```

### 2. 使用 println! 调试

```rust
fn main() {
    let data = vec![1, 2, 3];
    println!("{:?}", data);       // Debug 格式
    println!("{:#?}", data);      // 美化格式
    println!("{:?}", data[0]);    // 单个元素
}
```

### 3. 使用 log crate

```rust
// Cargo.toml
// [dependencies]
// log = "0.4"
// env_logger = "0.10"

use log::{debug, info, warn, error};

fn main() {
    env_logger::init();
    
    debug!("调试信息");
    info!("普通信息");
    warn!("警告信息");
    error!("错误信息");
}

// 运行
// RUST_LOG=debug cargo run
```

### 4. 使用 IDE 调试器

- VSCode: 安装 `rust-analyzer` 和 `CodeLLDB`
- IntelliJ IDEA: 安装 Rust 插件

### 5. 检查编译器建议

```bash
# 详细错误信息
RUST_BACKTRACE=1 cargo build

# 更详细的解释
cargo explain E0382
```

### 6. 使用 clippy

```bash
# 安装
rustup component add clippy

# 运行
cargo clippy

# 作为错误
cargo clippy -- -D warnings
```

---

## 快速参考

| 错误代码 | 描述 | 常见原因 |
|----------|------|----------|
| E0382 | 使用已移动的值 | 变量被移动后再使用 |
| E0597 | 生命周期不够长 | 引用超过被引用值的生命周期 |
| E0308 | 类型不匹配 | 类型不一致 |
| E0425 | 找不到值 | 变量未定义或不在作用域 |
| E0277 | trait 约束未满足 | 类型未实现所需 trait |
| E0384 | 不可变变量重新赋值 | 对不可变变量使用 `=` |

---

## 参考资源

- [Rust Compiler Error Index](https://doc.rust-lang.org/error-index.html)
- [Rust 错误处理](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [clippy 文档](https://rust-lang.github.io/rust-clippy/)

---

*Last updated: 2026-03-18*
