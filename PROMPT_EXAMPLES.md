# 示例提示词

本文档展示如何向 AI 提问以触发 rskills 的最佳用法。

---

## PyO3 相关

### "用 PyO3 为这个 Python 代码写一个 Rust 扩展模块"

```
我有一个 Python 计算密集型函数需要优化:

def matrix_multiply(A, B):
    result = [[0] * len(B[0]) for _ in range(len(A))]
    for i in range(len(A)):
        for j in range(len(B[0])):
            for k in range(len(B)):
                result[i][j] += A[i][k] * B[k][j]
    return result

请用 PyO3 将其重写为 Rust 扩展。
```

### "我的 PyO3 模块在导入时报错 'undefined symbol'"

```
错误信息:
ImportError: /path/to/my_rust_module.cpython-310-x86_64-linux-gnu.so:
undefined symbol: _ZN4core...

我正在使用 pyo3 0.22.0，Python 3.10，Linux x86_64。
请帮我诊断问题。
```

### "如何从 Rust 调用 Python 函数?"

```
我有一个训练好的 sklearn 模型需要从 Rust 调用。
模型在 model.pkl 文件中。
请展示如何在 Rust 中加载并调用这个模型的预测方法。
```

---

## cxx / C++ 互操作

### "我的 C++ 项目需要调用 Rust 函数，请生成 cxx 桥接代码"

```
C++ 代码:
std::vector<int> process_data(const std::string& input) {
    // 复杂的字符串处理逻辑
    return result;
}

请用 Rust 重写这个函数，并用 cxx 生成 C++ 绑定。
```

### "cxx 生成的头文件编译报错 'incomplete type'"

```
错误:
#include "my_bindings.h"
my_bindings.h:50: error: incomplete type 'std::vector<...>' not allowed

请帮我解决这个问题。
```

---

## JNI / Java 互操作

### "在 Android NDK 中使用 Rust JNI"

```
我需要在 Android 应用中调用 Rust 库。
包名: com.example.app
类名: RustProcessor
函数: processImage(byte[] data) -> byte[]

请提供完整的 Rust JNI 实现。
```

### "JNI 内存泄漏如何排查?"

```
我的 Java 调用 Rust 后内存持续增长。
每次调用返回新的 String 对象。
请检查以下代码是否有内存泄漏问题。
```

---

## WASM 相关

### "将这个 Rust 算法编译为 WASM 在浏览器运行"

```
Rust 代码:
pub fn compress(data: &[u8]) -> Vec<u8> {
    // 压缩算法
}

请用 wasm-bindgen 包装，并在 HTML 中展示如何调用。
```

### "WASM 模块如何与 JavaScript 传递大型数组?"

```
我需要处理 100MB 的图像数据。
当前使用 WebGL 但想用 Rust WASM 重写核心算法。
请展示如何在 JS 和 WASM 之间高效传递数据。
```

---

## OpenClaw 部署

### "在 OpenClaw 中部署这个 Rust WASM 模块"

```
我有一个 Rust 编写的图像处理库。
需要编译为 WASM 并部署到 OpenClaw 平台。
目标: x86_64 Linux，内存限制 512MB。
请提供构建和部署步骤。
```

### "交叉编译 Rust 到 ARM64 用于 OpenClaw"

```
需要在 macOS 上交叉编译 Rust 二进制到:
- 目标: aarch64-unknown-linux-musl
- 用途: OpenClaw ARM64 沙箱

请提供完整的工具链配置和构建步骤。
```

---

## Rust 核心问题

### "所有权和生命周期问题"

```
错误: cannot borrow `self` as mutable because it is also borrowed as immutable

代码:
impl Cache {
    fn get(&self, key: &str) -> Option<&Value> { ... }
    fn insert(&mut self, key: String, value: Value) { ... }
    fn update(&mut self, key: &str, value: Value) {
        let old = self.get(key); // 这里出错
        ...
    }
}

请解释问题并提供修复方案。
```

### "异步代码死锁"

```
使用 tokio 时偶尔出现死锁。
涉及 Mutex<T> 在多任务间共享。
请展示正确的 async 锁使用模式。
```

---

## FFI 安全检查

### "检查以下 Rust FFI 代码是否存在内存泄漏风险"

```rust
#[no_mangle]
pub extern "C" fn process_data(
    input: *const c_char,
    output: *mut c_char,
    len: usize,
) -> i32 {
    let s = unsafe { CStr::from_ptr(input).to_str().unwrap() };
    let result = do_process(s);
    // 疑问: 这里需要手动释放 output 吗?
    0
}
```

请检查并说明内存管理责任。

### "如何正确处理跨语言 panic"

```
Rust 代码可能在 FFI 边界 panic。
我希望 C++ 调用 Rust 时不会导致进程崩溃。
请展示如何使用 panic::catch_unwind。
```

---

## 一般性请求

### "审查我的 Cargo.toml 依赖版本"

```
请检查以下配置是否需要更新版本锁定:

[dependencies]
pyo3 = "0.22"
cxx = "1.0"
tokio = "1.35"

我担心 API 变动导致构建失败。
```

### "优化这个 Rust 程序的性能"

```
代码片段... (省略)

这是关键路径代码，需要最低延迟。
请建议性能优化方向。
```