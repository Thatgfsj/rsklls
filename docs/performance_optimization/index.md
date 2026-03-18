# 性能优化

## 目录

- [基准测试 (Criterion)](#基准测试-criterion)
- [性能分析工具](#性能分析工具)
- [编译优化选项](#编译优化选项)
- [内存优化](#内存优化)
- [常见优化技巧](#常见优化技巧)

---

## 基准测试 (Criterion)

### 安装

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "my_benchmark"
harness = false
```

### 基本用法

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### 比较基准

```rust
use criterion::{BenchmarkId, Criterion};

fn bench_solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");
    
    for size in [10, 20, 30].iter() {
        group.bench_with_input(BenchmarkId::new("recursive", size), size, 
            |b, &size| b.iter(|| fibonacci(size)));
        
        group.bench_with_input(BenchmarkId::new("iterative", size), size,
            |b, &size| b.iter(|| fibonacci_iterative(size)));
    }
    
    group.finish();
}
```

### 运行基准测试

```bash
cargo bench

# 输出到 HTML
cargo bench -- --save-baseline new

# 与基准比较
cargo bench -- --baseline old
```

---

## 性能分析工具

### 火焰图 (Flamegraph)

```bash
# 安装
cargo install flamegraph

# 运行并生成火焰图
cargo flamegraph --bin my-app

# 或指定要分析的命令
cargo flamegraph --root -- my-app --arg1 value
```

### perf (Linux)

```bash
# 记录性能数据
perf record -g ./target/release/my-app

# 查看报告
perf report

# 生成火焰图
perf script | stackcollapse-perf.pl | flamegraph.pl > flame.svg
```

###samply (跨平台)

```bash
# 安装
cargo install samply

# 采样
samply record ./target/release/my-app
```

### 内存分析

```rust
// 使用 jemalloc 进行内存分析
// Cargo.toml
// [dependencies]
// tikv-jemallocator = { version = "0.5", features = ["profiling", "unprefixed_malloc_on_supported_platforms"] }

use tikv_jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    // 设置环境变量
    // MALLOC_CONF="prof:true,prof_prefix:jeprof.out"
    
    // 运行后分析
    // jeprof --svg ./target/release/my-app jeprof.out.*.heap > profile.svg
}
```

---

## 编译优化选项

### Profile 配置

```toml
# Cargo.toml

[profile.release]
opt-level = 3        # 最高优化
lto = true          # 链接时优化
codegen-units = 1   # 单个代码生成单元（更慢但更优）
strip = true        # 移除符号
panic = "abort"     # panic 时终止（更小更快）

[profile.release-fast]
inherits = "release"
opt-level = 2
lto = false         # 禁用 LTO 加快编译

[profile.dev]
opt-level = 0
debug = true
incremental = true
```

### opt-level 说明

| 级别 | 描述 |
|------|------|
| 0 | 无优化 |
| 1 | 基本优化 |
| 2 | 标准优化 |
| 3 | 最高优化 |
| "s" | 优化大小 |
| "z" | 更激进的大小优化 |

### 目标 CPU 优化

```toml
# .cargo/config.toml
[build]
rustflags = ["-C", "target-cpu=native"]
```

---

## 内存优化

### 减少内存分配

```rust
// 不好：每次迭代都分配
fn process(items: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for &item in items {
        let mut temp = Vec::new();  // 重复分配
        temp.push(item);
        result.extend(temp);
    }
    result
}

// 好：预分配，复用缓冲区
fn process_optimized(items: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(items.len());
    let mut temp = Vec::with_capacity(1);  // 复用
    
    for &item in items {
        temp.clear();
        temp.push(item);
        result.extend(&temp);
    }
    result
}
```

### 使用 Cow (Copy-on-Write)

```rust
use std::borrow::Cow;

fn process_string(input: &str) -> Cow<str> {
    if input.contains("placeholder") {
        // 只有需要修改时才分配
        Cow::Owned(input.replace("placeholder", "value"))
    } else {
        // 不修改则借用
        Cow::Borrowed(input)
    }
}
```

### 小字符串优化

```rust
// 使用 smallvec/smallstr
// Cargo.toml
// [dependencies]
// smallvec = "1.0"
// smallstr = "0.3"

use smallstr::SmallString;

fn main() {
    // 栈上存储最多 8 字节
    let small: SmallString<[u8; 8]> = SmallString::from("hello");
    
    // 超过限制则堆分配
    let large: SmallString<[u8; 8]> = SmallString::from("hello world this is long");
}
```

---

## 常见优化技巧

### 1. 避免不必要的 Clone

```rust
// 不好
fn process(data: Vec<i32>) -> Vec<i32> {
    let mut data = data.clone();  // 不必要的克隆
    data.push(1);
    data
}

// 好
fn process(mut data: Vec<i32>) -> Vec<i32> {
    data.push(1);
    data
}
```

### 2. 使用迭代器

```rust
// 不好
fn sum_squares(data: &[i32]) -> i32 {
    let mut result = 0;
    for i in 0..data.len() {
        result += data[i] * data[i];
    }
    result
}

// 好
fn sum_squares_iter(data: &[i32]) -> i32 {
    data.iter().map(|&x| x * x).sum()
}
```

### 3. 预计算和缓存

```rust
use std::collections::HashMap;
use std::hash::Hash;

struct Cache<K, V> {
    data: HashMap<K, V>,
}

impl<K: Eq + Hash + Clone, V: Clone> Cache<K, V> {
    fn get_or_compute<F: FnOnce() -> V>(&mut self, key: K, f: F) -> &V {
        self.data.entry(key).or_insert_with(f)
    }
}
```

### 4. 并行处理

```rust
// 使用 rayon 并行处理
// Cargo.toml
// [dependencies]
// rayon = "1.8"

use rayon::prelude::*;

fn process_parallel(data: &[i32]) -> i32 {
    data.par_iter()  // 并行迭代
        .map(|&x| expensive_computation(x))
        .sum()
}

fn expensive_computation(x: i32) -> i32 {
    // 模拟复杂计算
    (0..1000).fold(x, |acc, i| acc.wrapping_mul(i).wrapping_add(1))
}
```

### 5. 避免边界检查

```rust
// 使用迭代器避免边界检查
fn sum(data: &[i32]) -> i32 {
    data.iter().sum()  // 无边界检查
}

// 或使用 unsafe（谨慎使用）
fn sum_unsafe(data: &[i32]) -> i32 {
    let mut sum = 0;
    let ptr = data.as_ptr();
    for i in 0..data.len() {
        unsafe {
            sum += *ptr.add(i);  // 无边界检查
        }
    }
    sum
}
```

---

## 性能清单

- [ ] 使用 Release 模式构建
- [ ] 启用 LTO 和优化选项
- [ ] 使用 `criterion` 进行基准测试
- [ ] 使用火焰图定位热点
- [ ] 预分配集合容量
- [ ] 避免不必要的克隆
- [ ] 使用迭代器代替索引访问
- [ ] 考虑并行处理（rayon）
- [ ] 分析内存分配
- [ ] 使用合适的算法和数据结构

---

## 参考资源

- [Criterion 文档](https://docs.rs/criterion/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [rayon 文档](https://docs.rs/rayon/)

---

*Last updated: 2026-03-18*
