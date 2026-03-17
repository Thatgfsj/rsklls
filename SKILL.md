# Rust Skills (rsklls)

A comprehensive skill for Rust program development, Python-Rust interoperability, and Rust-Go interoperability.

## Trigger Words

"rust", "rust开发", "rust编程", "pyo3", "rust python", "rust go", "rust互转", "rust GUI", "中文显示"

---

## 📦 目录

1. [Rust 程序开发基础](#rust-程序开发基础)
2. [Python 与 Rust 互操作 (PyO3)](#python-与-rust-互操作-pyo3)
3. [Rust 与 Go 互操作](#rust-与-go-互操作)
4. [GUI 程序中文显示问题](#gui-程序中文显示问题)

---

## Rust 程序开发基础

### 环境安装

```bash
# macOS / Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows (使用 winget)
winget install Rustlang.Rust.MSVC

# 验证安装
rustc --version
cargo --version
```

### 快速开始

```bash
# 创建新项目
cargo new hello_rust
cd hello_rust

# 编译运行
cargo run

# Debug 模式
cargo build

# Release 模式 (优化)
cargo build --release
```

### 核心概念

#### 所有权 (Ownership)

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 移动到 s2
    
    // println!("{}", s1); // 错误! s1 已失效
    println!("{}", s2); // 正确
    
    // 克隆
    let s3 = s1.clone();
    println!("{} {}", s2, s3); // 两者都有效
}
```

#### 借用 (Borrowing)

```rust
fn main() {
    let s = String::from("hello");
    
    // 不可变借用
    let len = calculate_length(&s);
    println!("{}", len); // 5
    
    // 可变借用
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s); // "hello world"
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" world");
}
```

#### 生命周期

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### 常用 Crate

| Crate | 用途 |
|-------|------|
| `serde` | 序列化/反序列化 |
| `tokio` | 异步运行时 |
| `reqwest` | HTTP 客户端 |
| `actix-web` | Web 框架 |
| `egui` | GUI 框架 |
| `winit` | 窗口管理 |
| `windows-rs` | Windows API |

---

## Python 与 Rust 互操作 (PyO3)

### 环境准备

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 maturin (用于构建 Python 扩展)
pip install maturin

# 或者安装 pyo3-build-config
pip install pyo3-build-config
```

### 项目创建

```bash
# 方式 1: 使用 maturin 创建
maturin new rust_lib
cd rust_lib

# 方式 2: 手动创建
cargo new --lib my_rust_lib
```

### 配置 Cargo.toml

```toml
[package]
name = "my_rust_lib"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.22", features = ["extension-module"] }
```

### 编写 Rust 代码

```rust
use pyo3::prelude::*;

#[pyfunction]
fn say_hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[pyfunction]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[pyclass]
struct Person {
    name: String,
    age: u32,
}

#[pymethods]
impl Person {
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
    
    fn greet(&self) -> String {
        format!("你好, 我叫{}，今年{}岁!", self.name, self.age)
    }
    
    fn get_age(&self) -> u32 {
        self.age
    }
}

#[pymodule]
fn my_rust_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_class::<Person>()?;
    Ok(())
}
```

### 构建与使用

```bash
# 开发模式 (实时重载)
maturin develop

# 生产构建
maturin build --release
```

### Python 中使用

```python
# 安装构建好的包
pip install my_rust_lib

# 使用
from my_rust_lib import say_hello, add, Person

print(say_hello("World"))  # Hello, World!
print(add(1, 2))           # 3

person = Person("张三", 25)
print(person.greet())      # 你好, 我叫张三，今年25岁!
print(person.get_age())    # 25
```

### 高级特性

#### 返回 Python 对象

```rust
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyfunction]
fn get_dict() -> PyResult<Py<PyDict>> {
    Python::with_gil(|py| {
        let dict = PyDict::new(py);
        dict.set_item("name", "张三")?;
        dict.set_item("age", 25)?;
        Ok(dict.into())
    })
}
```

#### 异常处理

```rust
use pyo3::prelude::*;

#[pyfunction]
fn divide(a: i32, b: i32) -> PyResult<i32> {
    if b == 0 {
        Err(pyo3::exceptions::PyValueError::new_err("除数不能为零"))
    } else {
        Ok(a / b)
    }
}
```

---

## Rust 与 Go 互操作

### 方式 1: C FFI

Rust 和 Go 都可以通过 C ABI 互操作。

#### Rust 端 (build.rs + cxx)

```toml
# Cargo.toml
[dependencies]
cxx = "1.0"

[build-depends]
cxx-build = "1.0"
```

```rust
// src/lib.rs
use cxx::bridge;

#[bridge]
mod ffi {
    extern "Rust" {
        fn greet(name: &str) -> String;
    }
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

#### Go 端

```go
package main

// #include "rust_lib.h"
import "C"

func main() {
    name := C.CString("World")
    defer C.free(unsafe.Pointer(name))
    
    result := C.greet(name)
    println(result)
}
```

### 方式 2: cgo 通过 C 库

#### Rust 编译为静态库

```toml
[lib]
crate-type = ["staticlib"]
```

```rust
// src/lib.rs
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}
```

编译:
```bash
cargo build --release
```

#### Go 调用

```go
package main

// #cgo LDFLAGS: -L./target/release -lrust_lib
// #include <stdlib.h>
import "C"
import "fmt"

func main() {
    result := C.rust_add(C.int(1), C.int(2))
    fmt.Printf("Result: %d\n", result)
}
```

### 方式 3: 共享内存 + IPC

对于复杂场景，使用 Protobuf + gRPC:

```protobuf
// api.proto
syntax = "proto3";

service RustService {
    rpc Process(Data) returns (Result);
}

message Data {
    string content = 1;
}

message Result {
    bool success = 1;
    string message = 2;
}
```

---

## GUI 程序中文显示问题

### Windows 终端中文显示

#### 方法 1: 设置代码页

```rust
fn main() {
    // 在 Windows 上启用 UTF-8
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        let _ = std::process::Command::new("cmd")
            .args(&["/C", "chcp 65001 > nul"])
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .spawn();
    }
    
    println!("你好，世界!");
}
```

#### 方法 2: 使用 winapi 设置控制台模式

```rust
#[cfg(windows)]
fn enable_utf8() {
    use std::mem::MaybeUninit;
    
    #[link(name = "kernel32")]
    extern "system" {
        fn SetConsoleOutputCP(codepage: u32) -> Bool;
        fn SetConsoleCP(codepage: u32) -> Bool;
    }
    
    unsafe {
        SetConsoleOutputCP(65001);
        SetConsoleCP(65001);
    }
}
```

### Windows GUI (winapi 直接绘制)

```rust
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

fn main() {
    #[cfg(windows)]
    {
        use std::ptr::null_mut;
        
        #[link(name = "user32")]
        extern "system" {
            fn MessageBoxW(hwnd: *mut std::ffi::c_void, text: *const u16, caption: *const u16, utype: u32) -> i32;
        }
        
        let text = to_wide("你好，世界!");
        let caption = to_wide("Rust 消息框");
        
        unsafe {
            MessageBoxW(null_mut(), text.as_ptr(), caption.as_ptr(), 0);
        }
    }
}
```

### 使用 winit + egui 显示中文

```rust
use egui::CtxRef;

fn demo_ui(ctx: &CtxRef) {
    egui::Window::new("中文测试").show(ctx, |ui| {
        ui.heading("你好，世界!");
        ui.label("这是中文标签");
        ui.button("按钮");
    });
}
```

**注意**: 确保系统安装了支持中文的字体。

### Windows 字体配置

```rust
// 在 egui 中设置中文字体
fn setup_chinese_fonts(ctx: &CtxRef) {
    let mut fonts = egui::FontDefinitions::default();
    
    // 添加系统自带的中文字体
    fonts.font_data.insert(
        "microsoft_yahei".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "C:\\Windows\\Fonts\\msyh.ttc"
        ))),
    );
    
    // 设置为默认字体
    fonts
        .fonts_for_family
        .get_mut(&egui::FontFamily::Proportional)
        .insert(0, "microsoft_yahei".to_owned());
    
    ctx.set_fonts(fonts);
}
```

### 跨平台控制台输出

```rust
use std::io::Write;

fn print_chinese(s: &str) {
    #[cfg(windows)]
    {
        // Windows: 使用 Write::write_all 直接输出
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        let _ = handle.write_all(s.as_bytes());
        let _ = handle.flush();
    }
    
    #[cfg(not(windows))]
    {
        println!("{}", s);
    }
}

fn main() {
    print_chinese("你好，世界!\n");
}
```

### 使用 crossterm 跨平台终端

```rust
use crossterm::{
    style::{Color, Print, Stylize},
    ExecutableCommand,
    terminal,
};

fn main() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();
    
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(Print("你好，世界!".with(Color::Green)))?
        .execute(Print("\n"))?;
    
    Ok(())
}
```

Cargo.toml 添加:
```toml
crossterm = "0.27"
```

---

## 常见问题

### Q: Rust 编译速度太慢?

```bash
# 使用 cargo watch 自动重载
cargo install cargo-watch
cargo watch -x build

# 使用 sccache 缓存编译结果
cargo install sccache
export RUSTC_WRAPPER=sccache
```

### Q: PyO3 导入失败?

```bash
# 确保 Python 版本匹配
python --version
rustc --version

# 检查 PYTHONPATH
echo $PYTHONPATH
```

### Q: Windows 编译报错?

```bash
# 安装 Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools

# 或者使用 MSYS2
pacman -S mingw-w64-x86_64-gcc
```

### Q: 中文显示为方块?

1. 确保系统安装了中文字体
2. Windows: 检查控制台字体设置
3. GUI: 在代码中指定中文字体路径

---

## 参考资源

- [The Rust Book](https://doc.rust-lang.org/book/)
- [PyO3 文档](https://pyo3.rs/)
- [Rust FFI 指南](https://doc.rust-lang.org/nomicon/)
- [cxx 文档](https://cxx.rs/)
- [egui 文档](https://docs.rs/egui/)

---

*Last updated: 2026-03-18*
*Skill: rsklls - Rust Development Assistant*
