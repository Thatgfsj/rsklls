# FFI 方案对比：PyO3 vs cxx vs JNI vs WASM

## 方案总览

| 方案 | 适用场景 | 优点 | 缺点 | 推荐度 |
|------|---------|------|------|--------|
| **PyO3** | Python 调用 Rust | 原生 Python 集成，成熟稳定 | 仅限 Python | ⭐⭐⭐⭐⭐ |
| **cxx** | Rust + C++ 互操作 | 类型安全，代码生成 | 需要双方使用 cxx | ⭐⭐⭐⭐⭐ |
| **JNI** | Java 调用 Rust | 标准 Java 方案 | 复杂，性能开销大 | ⭐⭐⭐ |
| **wasm-pack** | 浏览器/Node.js | 跨平台，沙箱安全 | WASM 生态限制 | ⭐⭐⭐⭐ |
| **raw FFI** | 通用 C 接口 | 最高灵活性 | 需要手动管理内存安全 | ⭐⭐⭐ |

---

## PyO3 (Python ↔ Rust)

### 适用场景
- 为 Python 编写高性能扩展模块
- 将 Python 项目关键路径用 Rust 重写
- 利用 Rust 生态库

### 关键依赖
```toml
pyo3 = { version = "=0.22.0", features = ["extension-module"] }
maturin = "1.0"  # 构建工具
```

### 版本锁定建议
```toml
pyo3 = "=0.22.0"  # 固定版本避免 API 变动
```

### 构建流程
```bash
# 开发
maturin develop

# 发布
maturin build --release --interpreter python3.10

# 或使用 maturin.toml 配置
```

---

## cxx (Rust ↔ C++)

### 适用场景
- Rust 与 C++ 项目互操作
- 需要类型安全的跨语言调用
- 双方都是系统级语言

### 关键依赖
```toml
cxx = "=1.0.146"
[build-dependencies]
cxx-build = "=1.0.146"
```

### 特点
- 生成 C++ 头文件和 Rust 绑定代码
- 支持 `Vec<T>`, `String`, `HashMap` 等复杂类型
- 零成本抽象

### 代码生成
```rust
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn rust_process(data: &str) -> String;
    }
    extern "C++" {
        include!("wrapper.h");
        fn cpp_callback(result: &str);
    }
}
```

---

## JNI (Java ↔ Rust)

### 适用场景
- Android NDK 开发
- Java 后端需要调用 Rust
- 游戏引擎 (JVM 集成)

### 关键依赖
```toml
jni = "=0.21.1"
```

### 缺点
- 函数签名复杂 (`Java_com_example_RustLib_function`)
- 容易引入内存问题
- 性能开销较大

### 替代方案
考虑使用 **[jniify](https://github.com/ivansrc/jniify)** 或 **[jawt](https://github.com/terzl/jawt)** 简化 JNI 开发。

---

## WASM (浏览器/Node.js)

### 适用场景
- Web 前端性能关键代码
- 边缘计算 (Cloudflare Workers 等)
- 插件系统

### 关键依赖
```toml
wasm-bindgen = "=0.2.92"
wasm-pack = "0.12"  # 构建工具
```

### 构建命令
```bash
wasm-pack build --target web --release
wasm-pack build --target nodejs --release
```

### 限制
- 无法直接调用系统 API
- 内存模型不同 (线性内存)
- 浮点数精度差异

---

## Raw FFI (通用 C 接口)

### 适用场景
- 与遗留 C 代码交互
- 需要最大灵活性
- 嵌入式系统

### 安全准则
1. 所有 unsafe 块必须写 `// SAFETY:` 注释
2. 使用 `#[repr(C)]` 确保结构体布局
3. 使用 `std::ffi` 类型 (`CString`, `CStr`, `c_char` 等)
4. 使用 `panic::catch_unwind` 保护跨语言边界

### 示例
```rust
#[no_mangle]
pub extern "C" fn rust_function(data: *const c_char) -> *mut c_char {
    unsafe {
        // SAFETY: 调用者保证 data 非空且以 null 结尾
        let s = CStr::from_ptr(data).to_str().unwrap();
        let result = process(s);
        CString::new(result).unwrap().into_raw()
    }
}
```

---

## 选择决策树

```
需要与什么语言互操作？
├── Python → PyO3
├── C++ → cxx (推荐) 或 raw FFI
├── Java/Android → JNI (或考虑 jniify)
├── Web/Node.js → WASM
└── 多种语言 → raw FFI 或各自专用方案
```

---

## 跨语言类型映射表

| Rust 类型 | C 类型 | Python 类型 | Java 类型 | WASM 类型 |
|-----------|--------|-------------|-----------|-----------|
| `i32` | `int32_t` | `int` | `int` | `i32` |
| `i64` | `int64_t` | `int` | `long` | `i64` |
| `f64` | `double` | `float` | `double` | `f64` |
| `bool` | `bool` | `bool` | `boolean` | `i32` (0/1) |
| `&str` | `const char*` | `str` | `String` | JS String |
| `&[u8]` | `uint8_t* + size_t` | `bytes` | `byte[]` | Uint8Array |
| `Vec<T>` | `T* + size_t` | `list` | `Object[]` | JS Array |
