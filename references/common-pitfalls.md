# 常见陷阱与解决方案

## 内存安全

### 1. use-after-free

**问题**: 在值被释放后继续使用其引用

```rust
// 错误示例
let s = String::from("hello");
let r = &s;
drop(s);  // 释放 s
println!("{}", r);  // 悬垂引用！
```

**解决**: 确保引用存活期间所有者不被 drop

```rust
// 正确
let s = String::from("hello");
{
    let r = &s;
    println!("{}", r);  // r 在作用域内有效
}
// r 已离开，但 s 仍有效
drop(s);  // 现在可以 drop s
```

---

### 2. 双重释放

**问题**: 同一块内存被释放两次

**场景**: C 和 Rust 都尝试释放同一块内存

```rust
// 错误：Rust 释放了，但 C 也可能释放
#[no_mangle]
pub extern "C" fn create_and_return() -> *mut c_char {
    let s = CString::new("hello").unwrap();
    s.into_raw()  // 转移所有权，不 drop
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    // SAFETY: 调用者保证 s 是我们的
    unsafe {
        let _ = CString::from_raw(s);  // drop
    }
}
```

**解决**: 明确所有权协议，文档化谁负责释放

```rust
// 明确约定：分配方释放
// Rust 侧
#[no_mangle]
pub extern "C" fn rust_alloc(size: usize) -> *mut c_void {
    libc::malloc(size)
}

#[no_mangle]
pub extern "C" fn rust_free(ptr: *mut c_void) {
    // SAFETY: 调用者保证 ptr 来自 rust_alloc
    unsafe {
        libc::free(ptr);
    }
}
```

---

### 3. 数据竞争

**问题**: 多个线程同时访问可变数据

```rust
// 错误示例
use std::thread;
let mut data = vec![1, 2, 3];

let handles: Vec<_> = (0..3).map(|i| {
    thread::spawn(move || {
        data.push(i);  // 数据竞争！
    })
}).collect();

for h in handles { h.join().unwrap(); }
```

**解决**: 使用 `Mutex`, `RwLock`, 或 `Arc`

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let data = Arc::new(Mutex::new(vec![1, 2, 3]));
let handles: Vec<_> = (0..3).map(|i| {
    let data = Arc::clone(&data);
    thread::spawn(move || {
        let mut d = data.lock().unwrap();
        d.push(i);
    })
}).collect();

for h in handles { h.join().unwrap(); }
```

---

## FFI 特定问题

### 4. 类型布局不匹配

**问题**: Rust 结构体与 C 结构体布局不同

```rust
// 问题：C 可能期望 packed 结构体
#[repr(C)]
struct MyStruct {
    a: u8,
    b: u32,  // C 中可能是 4 字节对齐，但 Rust 可能不同
    c: u8,
}
```

**解决**: 使用 `#[repr(C, packed)]` 或手动控制对齐

```rust
#[repr(C, packed)]
struct PackedStruct {
    a: u8,
    b: u32,
    c: u8,
}
```

---

### 5. 字符串编码问题

**问题**: UTF-8 和系统编码不一致

```rust
// 错误：在非 UTF-8 系统上可能失败
fn get_error() -> *const c_char {
    let msg = "错误信息".to_string();
    CString::new(msg).unwrap().into_raw()
}
```

**解决**: 使用 `CString::new()` 并处理可能的错误

```rust
fn safe_error(msg: &str) -> *mut c_char {
    match CString::new(msg) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => CString::new("Unknown error").unwrap().into_raw(),
    }
}
```

---

### 6. 生命周期错配

**问题**: 返回的引用指向已释放的数据

```rust
// 错误：返回指向本地数据的引用
#[no_mangle]
pub extern "C" fn bad_function() -> *const c_char {
    let s = String::from("temp");
    s.as_ptr()  // 悬垂指针！
}
```

**解决**: 返回所有权 (String/CString) 或使用静态生命周期

```rust
// 正确：返回 C 字符串所有权
#[no_mangle]
pub extern "C" fn good_function() -> *mut c_char {
    CString::new("temp").unwrap().into_raw()
    // 调用者负责释放
}

// 或使用静态生命周期
static MESSAGE: &str = "static message";

#[no_mangle]
pub extern "C" fn get_message() -> *const c_char {
    MESSAGE.as_ptr()  // OK，静态生命周期
}
```

---

## 异步问题

### 7. 阻塞异步运行时

**问题**: 在异步上下文中使用阻塞调用

```rust
// 错误示例
async fn bad_async() {
    let data = std::fs::read("file.txt").unwrap();  // 阻塞！
}
```

**解决**: 使用 `tokio::fs` 或 `tokio::task::spawn_blocking`

```rust
// 正确
async fn good_async() {
    let data = tokio::fs::read("file.txt").await.unwrap();
}

// 或使用 spawn_blocking 包装阻塞代码
async fn wrapper() {
    let result = tokio::task::spawn_blocking(|| {
        // 阻塞操作
        std::fs::read("file.txt").unwrap()
    }).await.unwrap();
}
```

---

### 8. 异步上下文中的阻塞锁

**问题**: 在 async 中使用 `Mutex` 导致死锁

```rust
// 可能的问题
async fn deadlock_risk(m: std::sync::Mutex<u32>) {
    let _ = m.lock().unwrap();  // 持有锁时 .await 会导致问题
}
```

**解决**: 使用 `tokio::sync::Mutex` (异步锁)

```rust
async fn correct_async(m: tokio::sync::Mutex<u32>) {
    let _ = m.lock().await;  // 异步锁
}
```

---

## 常见编译器错误

### 9. 未使用的导入/变量

```rust
#[allow(unused_imports)]
use std::collections::HashMap;  // 如果不用

let _x = 5;  // 下划线抑制警告
```

### 10. 迭代器失效

```rust
let mut v = vec![1, 2, 3];
for i in &v {
    v.push(*i);  // 错误：迭代同时修改
}
```

**解决**: 收集后修改或使用索引

```rust
let items: Vec<_> = v.iter().collect();
for i in items {
    v.push(*i);
}
```

---

## PyO3 特定问题

### 11. GIL 释放后对象被回收

```rust
#[pyfunction]
fn bad_example(py: Python) -> i32 {
    drop(py);  // 释放 GIL
    // Python 对象可能被 GC
    42
}
```

**解决**: 在需要持有 GIL 的操作中使用 `py`

```rust
#[pyfunction]
fn correct_example(py: Python) -> i32 {
    // 使用 py 确保对象有效
    let _dict = PyDict::new(py);
    42
}
```

---

### 12. 类型提取失败

```rust
#[pyfunction]
fn extract_example(value: &PyAny) -> PyResult<String> {
    // 尝试不同类型
    if let Ok(s) = value.extract::<String>() {
        Ok(s)
    } else if let Ok(i) = value.extract::<i64>() {
        Ok(format!("{}", i))
    } else {
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Unsupported type"
        ))
    }
}
```

---

## 调试清单

- [ ] 所有 `unsafe` 块都有 `// SAFETY:` 注释
- [ ] 跨 FFI 边界的类型使用 `#[repr(C)]`
- [ ] 字符串转换使用 `CString`/`CStr`
- [ ] 裸指针在 `unsafe` 块中使用
- [ ] `Vec` 传递时同时传递长度
- [ ] 所有权协议在文档中明确
- [ ] 错误使用 `panic::catch_unwind` 捕获
- [ ] 异步代码使用 `tokio::sync` 类型的锁