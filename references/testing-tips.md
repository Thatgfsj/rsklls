# 测试技巧：单元测试、集成测试与 FFI 测试

## 单元测试

### Rust 单元测试 (标准模式)

```rust
// src/calculator.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("division by zero")
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(9, 3), Ok(3));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10, 0), Err("division by zero"));
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_divide_panic() {
        divide(10, 0).unwrap();
    }
}
```

### 所有权测试

```rust
#[cfg(test)]
mod ownership_tests {
    #[test]
    fn test_move_semantics() {
        let s1 = String::from("hello");
        let s2 = s1;  // 移动
        // s1 不再可用，编译通过说明移动语义正确
        assert_eq!(s2, "hello");
    }

    #[test]
    fn test_borrow_immutable() {
        let s = String::from("hello");
        let r1 = &s;
        let r2 = &s;  // 多个不可变借用 OK
        assert_eq!(r1, &s);
        assert_eq!(r2, &s);
    }

    #[test]
    fn test_borrow_mutable() {
        let mut s = String::from("hello");
        let r = &mut s;
        r.push_str(" world");
        assert_eq!(r, "hello world");
    }
}
```

---

## 集成测试

### tests/ 目录下的集成测试

```rust
// tests/integration_calculator.rs

use rust_calculator:: {add, divide};

#[test]
fn test_add_integration() {
    assert_eq!(add(100, 200), 300);
}

#[test]
fn test_divide_integration() {
    assert_eq!(divide(20, 4), Ok(5));
}
```

### 跨 FFI 边界测试

```rust
// tests/ffi_integration.rs

use std::ffi::{CString, CStr};
use std::ptr;

#[test]
fn test_c_string_round_trip() {
    let original = "Hello FFI";
    let c_string = CString::new(original).unwrap();

    // SAFETY: 有效的 C 字符串
    let ptr = c_string.as_ptr();
    let recovered = unsafe {
        CStr::from_ptr(ptr).to_str().unwrap()
    };

    assert_eq!(recovered, original);
}

#[test]
fn test_raw_pointer_conversion() {
    let data: Vec<i32> = vec![1, 2, 3, 4, 5];
    let ptr = data.as_ptr();

    // 重建 Vec (不获取所有权，只是检查数据)
    let recovered: &[i32] = unsafe {
        std::slice::from_raw_parts(ptr, 5)
    };

    assert_eq!(recovered, &[1, 2, 3, 4, 5]);
}
```

---

## FFI 测试策略

### 1. C 兼容结构体测试

```rust
#[repr(C)]
struct Point {
    x: f64,
    y: f64,
}

#[test]
fn test_struct_layout() {
    let p = Point { x: 1.0, y: 2.0 };
    let size = std::mem::size_of::<Point>();
    // C 兼容结构体大小应为 16 字节 (2 * f64)
    assert_eq!(size, 16);
}

#[test]
fn test_struct_packing() {
    #[repr(C)]
    struct Packed {
        a: u8,
        b: u32,
        c: u8,
    }
    // 验证没有额外的对齐填充
    assert_eq!(std::mem::size_of::<Packed>(), 6);
}
```

### 2. Panic 恢复测试

```rust
use std::panic;

#[test]
fn test_panic_catch() {
    let result = panic::catch_unwind(|| {
        // 模拟可能 panic 的 FFI 调用
        panic!("simulated panic");
    });

    assert!(result.is_err());
}

#[no_mangle]
pub extern "C" fn safe_divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }
    a / b
}

#[test]
fn test_ffi_panic_recovery() {
    // SAFETY: 调用模拟的 FFI 函数
    let result = unsafe {
        panic::catch_unwind(|| {
            safe_divide(10, 0);
        })
    };

    // 验证 panic 被捕获
    assert!(result.is_err());
}
```

---

## Mock 测试 (用于 FFI)

```rust
#[cfg(test)]
mod mock_tests {
    extern "C" {
        fn external_callback(data: *const c_char) -> i32;
    }

    // 模拟外部函数
    mod mock {
        use std::sync::atomic::{AtomicI32, Ordering};

        static LAST_CALL_DATA: AtomicI32 = AtomicI32::new(0);

        pub fn set_last_call(data: i32) {
            LAST_CALL_DATA.store(data, Ordering::SeqCst);
        }

        pub fn get_last_call() -> i32 {
            LAST_CALL_DATA.load(Ordering::SeqCst)
        }
    }

    #[test]
    fn test_callback_integration() {
        mock::set_last_call(42);
        // 验证回调正确传递数据
        assert_eq!(mock::get_last_call(), 42);
    }
}
```

---

## 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_add

# 运行集成测试
cargo test --test integration

# 运行带日志的测试
RUST_LOG=debug cargo test

# 运行带覆盖率
cargo install cargo-tarpaulin
cargo tarpaulin --output-formats=html
```

---

## 测试模式总结

| 测试类型 | 位置 | 用途 |
|---------|------|-----|
| 单元测试 | `src/lib.rs` 内 | 测试单个函数/结构体 |
| 集成测试 | `tests/*.rs` | 测试模块间交互 |
| FFI 测试 | `tests/ffi_*.rs` | 验证跨语言边界 |
| 性能测试 | `benches/*.rs` | 基准测试 |
| 模糊测试 | `fuzz/*.rs` | 使用 cargo-fuzz |

---

## 持续集成中的测试

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features
      - run: cargo clippy -- -D warnings
      - run: cargo fmt -- --check
```