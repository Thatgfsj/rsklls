# 借用 (Borrowing)

## 目录

- [定义](#定义)
- [核心规则](#核心规则)
- [不可变借用](#不可变借用)
- [可变借用](#可变借用)
- [借用规则详解](#借用规则详解)
- [使用场景](#使用场景)
- [常见陷阱](#常见陷阱)
- [示例代码](#示例代码)

---

## 定义

借用（Borrowing）是 Rust 中允许在不获取所有权的情况下访问数据的机制。通过引用（`&`）实现，分为：
- **不可变借用**（`&T`）：只读访问
- **可变借用**（`&mut T`）：可读可写访问

---

## 核心规则

1. **可以有多个不可变借用，或一个可变借用，但不能同时存在**
2. **引用必须始终有效**（不能悬垂引用）
3. **引用的作用域从引入开始，到最后一次使用结束**

```rust
fn main() {
    let mut s = String::from("hello");
    
    // 规则 1: 多个不可变借用
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);  // 正确
    
    // 规则 1: 一个可变借用
    let r3 = &mut s;
    r3.push_str(" world");
    println!("{}", r3);  // 正确
}
```

---

## 不可变借用

### 基本语法

```rust
fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);  // 借用 s
    println!("'{}' 的长度是 {}", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s 离开作用域，但由于它只是引用，原始值不会被释放
```

### 多个不可变借用

```rust
fn main() {
    let s = String::from("hello");
    
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    
    println!("{} {} {}", r1, r2, r3);  // 正确：多个不可变借用可以共存
}
```

### 只读访问

```rust
fn main() {
    let s = String::from("hello");
    let r = &s;
    
    // r.push_str(" world");  // 错误！不可变引用不能修改数据
    println!("{}", r);
}
```

---

## 可变借用

### 基本语法

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);  // "hello world"
}

fn change(s: &mut String) {
    s.push_str(" world");
}
```

### 唯一性保证

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &mut s;
    r1.push_str(" world");
    
    // let r2 = &mut s;  // 错误！已有可变借用 r1 存在
    // println!("{} {}", r1, r2);
    
    println!("{}", r1);  // 正确
}
```

### 可变借用与不可变借用互斥

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s;  // 错误！已有不可变借用存在
    
    println!("{} {}", r1, r2);
    // 此处 r1, r2 不再使用，作用域结束
    
    let r3 = &mut s;  // 正确！之前的借用已结束
    r3.push_str(" world");
    println!("{}", r3);
}
```

---

## 借用规则详解

### 作用域与 Non-Lexical Lifetimes (NLL)

Rust 2018 引入了 NLL，引用的作用域到**最后一次使用**为止：

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;
    println!("{}", r1);  // r1 最后一次使用
    
    // r1 的作用域已结束，可以创建可变借用
    let r2 = &mut s;
    r2.push_str(" world");
    println!("{}", r2);
}
```

### 借用检查器工作原理

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;           // r1 作用域开始
    let r2 = &s;           // r2 作用域开始
    println!("{} {}", r1, r2);  // r1, r2 最后使用
    
    // r1, r2 作用域在此结束
    
    let r3 = &mut s;       // r3 作用域开始
    r3.push_str(" world");
    println!("{}", r3);    // r3 最后使用
}
```

### 函数中的借用

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    
    let result = longest(&s1, &s2);
    println!("最长的字符串是: {}", result);
}

// 返回的引用生命周期与较短的那个参数相同
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

## 使用场景

### 1. 避免所有权转移

```rust
fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    let sum = calculate_sum(&data);
    let avg = calculate_average(&data);
    
    println!("总和: {}, 平均值: {}", sum, avg);
}

fn calculate_sum(data: &Vec<i32>) -> i32 {
    data.iter().sum()
}

fn calculate_average(data: &Vec<i32>) -> f64 {
    if data.is_empty() { return 0.0; }
    calculate_sum(data) as f64 / data.len() as f64
}
```

### 2. 迭代器模式

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // 不可变迭代
    for item in &v {
        println!("{}", item);
    }
    
    // 可变迭代
    let mut v2 = vec![1, 2, 3];
    for item in &mut v2 {
        *item *= 2;
    }
    println!("{:?}", v2);  // [2, 4, 6]
}
```

### 3. 结构体方法

```rust
struct User {
    name: String,
    age: u32,
}

impl User {
    // 不可变借用 self
    fn greet(&self) -> String {
        format!("你好，我是{}，今年{}岁", self.name, self.age)
    }
    
    // 可变借用 self
    fn have_birthday(&mut self) {
        self.age += 1;
    }
    
    // 获取引用
    fn name(&self) -> &str {
        &self.name
    }
}

fn main() {
    let mut user = User {
        name: String::from("张三"),
        age: 25,
    };
    
    println!("{}", user.greet());
    user.have_birthday();
    println!("新年龄: {}", user.age);
    println!("名字: {}", user.name());
}
```

### 4. 回调函数

```rust
fn apply_to_string(s: &mut String, f: fn(&mut String)) {
    f(s);
}

fn add_exclamation(s: &mut String) {
    s.push('!');
}

fn add_prefix(s: &mut String) {
    s.insert_str(0, "Hello, ");
}

fn main() {
    let mut s = String::from("World");
    
    apply_to_string(&mut s, add_prefix);
    apply_to_string(&mut s, add_exclamation);
    
    println!("{}", s);  // "Hello, World!"
}
```

---

## 常见陷阱

### 陷阱 1: 同时持有可变和不可变引用

```rust
// 错误示例
fn main() {
    let mut v = vec![1, 2, 3];
    
    let first = &v[0];      // 不可变借用
    v.push(4);              // 错误！push 需要可变借用
    
    println!("{}", first);
}

// 正确做法
fn main() {
    let mut v = vec![1, 2, 3];
    
    let first = v[0];  // 复制值而不是借用
    v.push(4);
    
    println!("{}", first);
}
```

### 陷阱 2: 返回局部变量的引用

```rust
// 错误示例
fn dangling_reference() -> &String {
    let s = String::from("hello");
    &s  // 错误！返回了局部变量的引用
}  // s 在此处被释放

// 正确做法 1: 返回所有权
fn return_ownership() -> String {
    let s = String::from("hello");
    s
}

// 正确做法 2: 使用输入引用
fn return_input<'a>(s: &'a String) -> &'a String {
    s
}
```

### 陷阱 3: 在循环中创建多个可变引用

```rust
// 错误示例
fn main() {
    let mut v = vec![1, 2, 3];
    
    for i in 0..v.len() {
        let first = &v[0];  // 不可变借用
        // v[i] += first;   // 错误！需要可变借用
    }
}

// 正确做法
fn main() {
    let mut v = vec![1, 2, 3];
    
    let first = v[0];
    for i in 0..v.len() {
        v[i] += first;
    }
}
```

### 陷阱 4: 可变引用的生命周期

```rust
// 错误示例
fn main() {
    let mut s = String::from("hello");
    
    let r = &mut s;
    let r2 = &mut s;  // 错误！r 仍然有效
    
    println!("{}", r);
}

// 正确做法
fn main() {
    let mut s = String::from("hello");
    
    {
        let r = &mut s;
        r.push_str(" world");
    }  // r 作用域结束
    
    let r2 = &mut s;
    r2.push_str("!");
    
    println!("{}", r2);
}
```

### 陷阱 5: 结构体中的自引用

```rust
// 错误示例 - 自引用结构
struct SelfRef {
    data: String,
    reference: &String,  // 需要生命周期标注
}

// 正确做法：使用生命周期
struct SelfRef<'a> {
    data: String,
    reference: &'a String,
}

// 或使用 Pin 处理自引用
use std::pin::Pin;

struct SelfRefPin {
    data: String,
    pointer: *const String,  // 原始指针，需要 unsafe
}
```

---

## 示例代码

### 完整示例：实现安全的缓存

```rust
use std::collections::HashMap;

struct Cache<'a> {
    data: HashMap<String, String>,
    source: &'a dyn DataSource,
}

trait DataSource {
    fn fetch(&self, key: &str) -> Option<String>;
}

impl<'a> Cache<'a> {
    fn new(source: &'a dyn DataSource) -> Self {
        Cache {
            data: HashMap::new(),
            source,
        }
    }
    
    fn get(&mut self, key: &str) -> Option<&String> {
        if self.data.contains_key(key) {
            return self.data.get(key);
        }
        
        if let Some(value) = self.source.fetch(key) {
            self.data.insert(key.to_string(), value);
            self.data.get(key)
        } else {
            None
        }
    }
}

struct MockDataSource;

impl DataSource for MockDataSource {
    fn fetch(&self, key: &str) -> Option<String> {
        Some(format!("value_for_{}", key))
    }
}

fn main() {
    let source = MockDataSource;
    let mut cache = Cache::new(&source);
    
    println!("{:?}", cache.get("key1"));
    println!("{:?}", cache.get("key1"));  // 第二次从缓存获取
}
```

### 完整示例：借用与并发

```rust
use std::sync::Mutex;

struct SharedData {
    data: Mutex<Vec<i32>>,
}

impl SharedData {
    fn new() -> Self {
        SharedData {
            data: Mutex::new(Vec::new()),
        }
    }
    
    fn add(&self, value: i32) {
        let mut data = self.data.lock().unwrap();
        data.push(value);
    }
    
    fn get_sum(&self) -> i32 {
        let data = self.data.lock().unwrap();
        data.iter().sum()
    }
}

fn main() {
    let shared = SharedData::new();
    
    shared.add(1);
    shared.add(2);
    shared.add(3);
    
    println!("总和: {}", shared.get_sum());
}
```

---

## 参考资源

- [The Rust Book - 引用与借用](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust Reference - Borrowing](https://doc.rust-lang.org/reference/expressions/operator-expr.html#borrowing-operators)

---

*Last updated: 2026-03-18*
