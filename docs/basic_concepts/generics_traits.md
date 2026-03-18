# 泛型、Trait 与关联类型

## 目录

- [泛型 (Generics)](#泛型-generics)
- [Trait 定义与实现](#trait-定义与实现)
- [Trait Bounds](#trait-bounds)
- [关联类型](#关联类型)
- [默认泛型参数](#默认泛型参数)
- [常见陷阱](#常见陷阱)

---

## 泛型 (Generics)

### 函数泛型

```rust
// 泛型函数
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 12, 100, 65];
    println!("最大数字: {}", largest(&numbers));
    
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("最大字符: {}", largest(&chars));
}
```

### 结构体泛型

```rust
struct Point<T> {
    x: T,
    y: T,
}

// 多个泛型参数
struct Point2D<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 特定类型的实现
impl Point<f32> {
    fn distance_from_origin(&self) -> f64 {
        ((self.x.powi(2) + self.y.powi(2)) as f64).sqrt()
    }
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    let mixed_point = Point2D { x: 1, y: 2.0 };
}
```

### 枚举泛型

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 实际应用
enum ApiResponse<T> {
    Success { data: T },
    Error { code: u32, message: String },
    Loading,
}
```

### 泛型性能

Rust 的泛型是**静态分发**（monomorphization），编译时会为每个具体类型生成专门代码，没有运行时开销。

---

## Trait 定义与实现

### 定义 Trait

```rust
pub trait Summary {
    fn summarize(&self) -> String;
    
    // 默认实现
    fn author(&self) -> String {
        String::from("未知作者")
    }
}
```

### 实现 Trait

```rust
pub struct Article {
    pub title: String,
    pub content: String,
    pub author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} - {}", self.title, self.author)
    }
    
    fn author(&self) -> String {
        self.author.clone()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}
```

### 孤儿规则

只能在**定义 trait 的 crate** 或**定义类型的 crate** 中实现 trait。

```rust
// 正确：在定义 Vec 的 crate 中实现自定义 trait
impl Summary for Vec<String> {
    fn summarize(&self) -> String {
        self.join(", ")
    }
}

// 错误：不能为外部类型实现外部 trait
// impl Display for Vec<String> {}  // 编译错误！
```

### Trait 作为参数

```rust
// impl Trait 语法
fn print_summary(item: &impl Summary) {
    println!("{}", item.summarize());
}

// Trait Bound 语法
fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// 多个 trait
fn process<T: Summary + Display>(item: &T) {
    println!("{}", item);
    println!("{}", item.summarize());
}

// where 子句
fn complex_function<T, U>(t: &T, u: &U) -> String
where
    T: Summary + Clone,
    U: Summary + Display,
{
    format!("{} {}", t.summarize(), u.summarize())
}
```

---

## Trait Bounds

### 条件实现

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// 所有 Pair<T> 都有 new 方法
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 只有 T 实现了 Display + PartialOrd 的 Pair 才有 cmp_display
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x >= y: {} >= {}", self.x, self.y);
        } else {
            println!("x < y: {} < {}", self.x, self.y);
        }
    }
}
```

### Blanket Implementation（覆盖实现）

```rust
// 标准库示例：为所有实现了 Display 的类型实现 ToString
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        // ...
    }
}
```

---

## 关联类型

### 基本用法

```rust
trait Container {
    type Item;  // 关联类型
    
    fn get(&self) -> Option<&Self::Item>;
    fn insert(&mut self, item: Self::Item);
}

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Container for Stack<T> {
    type Item = T;  // 指定关联类型
    
    fn get(&self) -> Option<&Self::Item> {
        self.items.last()
    }
    
    fn insert(&mut self, item: Self::Item) {
        self.items.push(item);
    }
}
```

### 关联类型 vs 泛型参数

```rust
// 使用泛型：可以有多个实现
trait Graph<N, E> {
    fn node(&self) -> N;
    fn edge(&self) -> E;
}

// 使用关联类型：每个类型只能有一个实现
trait Graph {
    type Node;
    type Edge;
    
    fn node(&self) -> Self::Node;
    fn edge(&self) -> Self::Edge;
}
```

### 标准库示例：Iterator

```rust
trait Iterator {
    type Item;  // 关联类型
    
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: usize,
    max: usize,
}

impl Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

### 关联类型约束

```rust
trait Graph {
    type Node: Clone + Eq;  // 关联类型约束
    type Edge;
    
    fn nodes(&self) -> Vec<Self::Node>;
}

trait Widget {
    type Layout: Layout;
    
    fn layout(&self) -> Self::Layout;
}
```

---

## 默认泛型参数

### 运算符重载

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// Add trait 定义: trait Add<Rhs=Self> { ... }
impl Add for Point {
    type Output = Point;  // 关联类型
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 自定义 Rhs
impl Add<i32> for Point {
    type Output = Point;
    
    fn add(self, rhs: i32) -> Point {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("{:?}", p3);  // Point { x: 4, y: 6 }
}
```

---

## 常见陷阱

### 陷阱 1: Trait Object 大小未知

```rust
// 错误示例
fn returns_trait() -> impl Summary {
    Article { /* ... */ }
}

// trait object 需要使用 dyn
fn returns_trait() -> Box<dyn Summary> {
    Box::new(Article { /* ... */ })
}
```

### 陷阱 2: 泛型参数过多

```rust
// 不好的设计
trait Graph<N, E, M, P> {
    // 太多泛型参数
}

// 更好的设计：使用关联类型
trait Graph {
    type Node;
    type Edge;
    type Metadata;
    // ...
}
```

### 陷阱 3: 关联类型与默认值

```rust
// 错误：关联类型不能有默认值（Rust 目前不支持）
trait Bad {
    type Item = i32;  // 错误！
}

// 正确：在实现中指定
trait Good {
    type Item;
}

impl Good for MyType {
    type Item = i32;
}
```

### 陷阱 4: 生命周期与泛型

```rust
// 需要同时指定生命周期和泛型
trait Processor<'a, T> {
    fn process(&'a self, input: T) -> &'a str;
}

// 或使用关联类型
trait Processor {
    type Input;
    type Output<'a> where Self: 'a;
    
    fn process(&self, input: Self::Input) -> Self::Output<'_>;
}
```

---

## 实战示例：构建插件系统

```rust
// 定义插件 trait
pub trait Plugin {
    type Config;
    type Error;
    
    fn name(&self) -> &str;
    fn initialize(&mut self, config: Self::Config) -> Result<(), Self::Error>;
    fn execute(&self, input: &str) -> Result<String, Self::Error>;
    fn shutdown(&mut self) -> Result<(), Self::Error>;
}

// 日志插件
pub struct LoggerPlugin {
    prefix: String,
}

impl Plugin for LoggerPlugin {
    type Config = String;
    type Error = std::io::Error;
    
    fn name(&self) -> &str {
        "LoggerPlugin"
    }
    
    fn initialize(&mut self, config: Self::Config) -> Result<(), Self::Error> {
        self.prefix = config;
        Ok(())
    }
    
    fn execute(&self, input: &str) -> Result<String, Self::Error> {
        Ok(format!("[{}] {}", self.prefix, input))
    }
    
    fn shutdown(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

// 插件管理器
pub struct PluginManager {
    plugins: Vec<Box<dyn PluginHandler>>,
}

trait PluginHandler {
    fn name(&self) -> &str;
    fn execute(&self, input: &str) -> Result<String, Box<dyn std::error::Error>>;
}

impl<P: Plugin + 'static> PluginHandler for P
where
    P::Error: std::error::Error + 'static,
{
    fn name(&self) -> &str {
        Plugin::name(self)
    }
    
    fn execute(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        Plugin::execute(self, input).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager { plugins: Vec::new() }
    }
    
    pub fn register<P: PluginHandler + 'static>(&mut self, plugin: P) {
        self.plugins.push(Box::new(plugin));
    }
    
    pub fn execute_all(&self, input: &str) -> Vec<Result<String, Box<dyn std::error::Error>>> {
        self.plugins.iter()
            .map(|p| p.execute(input))
            .collect()
    }
}
```

---

## 参考资源

- [The Rust Book - 泛型](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [The Rust Book - Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust Reference - Traits](https://doc.rust-lang.org/reference/items/traits.html)

---

*Last updated: 2026-03-18*
