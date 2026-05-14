# cxx 跨语言调用示例

## 项目结构

```
project/
├── Cargo.toml
├── build.rs
├── src/
│   └── lib.rs
├── cxx/
│   └── wrapper.h      # C++ 头文件 (自动生成)
└── cpp/
    └── main.cpp       # C++ 调用方
```

---

## Cargo.toml

```toml
[package]
name = "cxx-demo"
version = "0.1.0"
edition = "2021"

[dependencies]
cxx = "=1.0.146"
anyhow = "=1.0.86"

[build-dependencies]
cxx-build = "=1.0.146"

[lib]
crate-type = ["staticlib", "cdylib"]
```

---

## src/lib.rs

```rust
use std::collections::HashMap;
use std::ffi::CString;
use std::ptr;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn rust_process(data: &str) -> String;
        fn rust_calculate(x: i32, y: i32) -> i32;
        fn rust_vector_sum(v: &[i32]) -> i64;
        fn rust_string_map(input: Vec<String>) -> Vec<String>;
    }

    extern "C++" {
        include!("wrapper.h");
        fn cpp_callback(message: &str) -> i32;
    }
}

// --- Rust 函数实现 ---

fn rust_process(data: &str) -> String {
    format!("Processed: {}", data)
}

fn rust_calculate(x: i32, y: i32) -> i32 {
    x * x + y * y
}

fn rust_vector_sum(v: &[i32]) -> i64 {
    v.iter().map(|x| *x as i64).sum()
}

fn rust_string_map(input: Vec<String>) -> Vec<String> {
    input.into_iter().map(|s| s.to_uppercase()).collect()
}

// --- C++ 回调 (Rust 调用 C++) ---

#[no_mangle]
unsafe extern "C++" fn cpp_callback(message: *const c_char) -> i32 {
    // SAFETY: 调用者保证 message 是有效的 UTF-8 字符串
    let msg = std::ffi::CStr::from_ptr(message)
        .to_str()
        .unwrap_or("unknown");
    println!("C++ says: {}", msg);
    42
}
```

---

## build.rs

```rust
fn main() {
    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++17")
        .compile("cxx-demo");
}
```

---

## C++ 调用方 (main.cpp)

```cpp
#include "cxx/include/demo.h"  // cxx 生成的头文件

#include <iostream>
#include <vector>
#include <string>

int main() {
    // 调用 Rust 函数
    std::string result = rust_process("Hello from C++");
    std::cout << result << std::endl;

    int calc = rust_calculate(3, 4);
    std::cout << "3^2 + 4^2 = " << calc << std::endl;

    // 传递 vector
    std::vector<int> nums = {1, 2, 3, 4, 5};
    int64_t sum = rust_vector_sum(nums);
    std::cout << "Sum: " << sum << std::endl;

    // 传递字符串 vector
    std::vector<std::string> strings = {"hello", "world"};
    auto upper = rust_string_map(strings);
    for (const auto& s : upper) {
        std::cout << s << " ";
    }
    std::cout << std::endl;

    // 调用 C++ 回调 (Rust -> C++)
    int cb_result = cpp_callback("Hi from Rust!");
    std::cout << "Callback returned: " << cb_result << std::endl;

    return 0;
}
```

---

## 构建命令

```bash
# Rust 端编译
cargo build --release

# C++ 端编译 (需要链接 Rust 静态库)
# Linux/macOS
g++ -std=c++17 -I./cxx/include main.cpp -L./target/release -lrust_demo -o cpp_app

# Windows (MSVC)
cl /std:c++17 /EHsc /I./cxx/include main.cpp ./target/release/rust_demo.lib
```

---

## 复杂类型映射

cxx 自动处理以下类型：

| Rust 类型 | C++ 类型 |
|-----------|----------|
| `i32` | `int32_t` |
| `i64` | `int64_t` |
| `f64` | `double` |
| `bool` | `bool` |
| `String` | `std::string` |
| `&str` | `std::string_view` |
| `Vec<T>` | `std::vector<T>` |
| `HashMap<K,V>` | `std::unordered_map<K,V>` |
| `Option<T>` | `std::optional<T>` |
| `Result<T,E>` | `std::optional<T>` (失败返回 nullopt) |

---

## 错误处理

Rust `Result` 映射到 C++ `std::optional`：

```rust
// Rust
fn might_fail(input: &str) -> Result<i32, anyhow::Error> {
    if input.is_empty() {
        Err(anyhow::anyhow!("empty input"))
    } else {
        Ok(input.len() as i32)
    }
}
```

```cpp
// C++
auto result = might_fail("hello");
if (result.has_value()) {
    std::cout << "Result: " << result.value() << std::endl;
} else {
    std::cerr << "Call failed" << std::endl;
}
```

---

## 内存管理

- Rust 分配的内存由 Rust 管理
- C++ 分配的内存由 C++ 管理
- 字符串自动在边界转换
- Vec/String 在跨边界时复制
