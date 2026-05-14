# Rust → WASM 指南

## 概述

将 Rust 代码编译为 WebAssembly (WASM)，可在浏览器、Node.js 或边缘计算环境运行。

---

## 环境配置

### 安装工具
```bash
# wasm-pack (编译工具)
cargo install wasm-pack

# wasm-bindgen (JS 互操作)
cargo install wasm-bindgen-cli
```

### 推荐的 Cargo.toml 配置
```toml
[package]
name = "rust-wasm"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "=0.2.92"
js-sys = "=0.3.69"
web-sys = { version = "=0.3.69", features = ["console"] }

[profile.release]
opt-level = "s"  # 优化体积
lto = true       # 链接时优化
```

---

## 基础示例

### src/lib.rs
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! (from WASM)", name)
}

// 导出结构体
#[wasm_bindgen]
pub struct Point {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn distance(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, val: f64) {
        self.x = val;
    }
}
```

---

## 构建命令

### Web (浏览器)
```bash
wasm-pack build --target web --release
```

输出：`pkg/rust_wasm.js`, `pkg/rust_wasm_bg.wasm`

### Node.js
```bash
wasm-pack build --target nodejs --release
```

### Web Worker
```bash
wasm-pack build --target web --release
```

---

## 浏览器使用示例

### index.html
```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Rust WASM Demo</title>
</head>
<body>
    <script type="module">
        import init, { add, fibonacci, greet, Point } from './pkg/rust_wasm.js';

        async function run() {
            await init();

            // 基础函数
            console.log(add(1, 2));  // 3

            // 斐波那契 (注意：这是低效的递归实现)
            console.log(fibonacci(10));  // 55

            // 字符串
            console.log(greet("World"));  // "Hello, World! (from WASM)"

            // 结构体
            const p = new Point(3, 4);
            console.log(`Point(${p.x}, ${p.y}), distance = ${p.distance()}`);
        }

        run();
    </script>
</body>
</html>
```

### 使用 webpack (简化加载)
```bash
npm install wasm-loader webpack webpack-cli html-webpack-plugin
```

webpack.config.js:
```javascript
module.exports = {
    entry: './src/index.js',
    output: {
        path: __dirname + '/dist',
        filename: 'bundle.js',
    },
    module: {
        rules: [{
            test: /\.wasm$/,
            type: 'asset/resource',
        }],
    },
    experiments: {
        asyncWebAssembly: true,
    },
};
```

---

## Node.js 使用示例

```javascript
const { add, fibonacci, greet, Point } = require('./pkg/rust_wasm');

console.log(add(1, 2));  // 3
console.log(fibonacci(10));  // 55
console.log(greet("Node"));  // "Hello, Node! (from WASM)"

const p = new Point(3, 4);
console.log(p.distance());  // 5
```

---

## 异步操作 (JavaScript 回调)

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn fetch_data(url: &str) -> String {
    // 注意：WASM 中不能直接发起 HTTP 请求
    // 需要通过 JavaScript 回调实现
    format!("Fetch: {}", url)
}

// 使用 JsFuture 处理 Promise
#[wasm_bindgen]
pub async fn wait_ms(ms: u32) {
    let promise = js_sys::Promise::new(&|resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &wasm_bindgen::closure::Closure::once_into_js(resolve),
                ms as i32,
            )
            .unwrap();
    });
    wasm_bindgen_futures::JsFuture::from(promise).await;
}
```

---

## 内存管理

WASM 使用线性内存模型：

```rust
use wasm_bindgen::prelude::*;

// 传递字节数组
#[wasm_bindgen]
pub fn process_bytes(data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| b.wrapping_add(1)).collect()
}

// 从 JavaScript 获取指针
#[wasm_bindgen]
pub fn get_data_ptr() -> *const u8 {
    static DATA: [u8; 5] = [1, 2, 3, 4, 5];
    DATA.as_ptr()
}
```

JavaScript 访问内存:
```javascript
const memory = wasm.instance.exports.memory;
const ptr = wasm.get_data_ptr();
const view = new Uint8Array(memory.buffer, ptr, 5);
console.log(Array.from(view));  // [1, 2, 3, 4, 5]
```

---

## 限制与注意事项

1. **无直接系统 API**: 不能直接访问文件系统、网络等
2. **浮点精度**: WASM 使用 IEE 754，与 JavaScript 一致
3. **无多线程**: (除非启用 threading feature)
4. **体积优化**: 使用 `wasm-opt` 进一步压缩

```bash
# 安装 wasm-opt (来自 binaryen)
# 然后在构建后优化
wasm-opt -Oz -o output.wasm input.wasm
```

---

## 调试技巧

1. 使用 `console_error_panic_hook` 捕获 panic:
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}
```

2. 使用 `web-sys` 的 `console.log`:
```rust
web_sys::console::log_1(&"Debug message".into());
```