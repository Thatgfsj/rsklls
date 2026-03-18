# 生命周期 (Lifetimes)

## 目录

- [定义](#定义)
- [核心概念](#核心概念)
- [生命周期标注语法](#生命周期标注语法)
- [生命周期省略规则](#生命周期省略规则)
- [静态生命周期](#静态生命周期)
- [使用场景](#使用场景)
- [常见陷阱](#常见陷阱)
- [示例代码](#示例代码)

---

## 定义

生命周期是 Rust 用于追踪引用有效范围的机制。它确保引用始终指向有效数据，防止悬垂引用（dangling references）。

### 为什么需要生命周期？

```rust
// 编译器需要知道返回的引用来自哪个参数
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
// 错误：missing lifetime specifier
```

生命周期标注告诉编译器引用之间的关系，使其能够在编译时验证引用的有效性。

---

## 核心概念

### 生命周期的本质

生命周期不是指变量"存活"的时间，而是指**引用保持有效的作用域范围**。

```rust
fn main() {
    let r;                // ----------+-- 'a
                          //           |
    {                     //           |
        let x = 5;        // -+-- 'b   |
        r = &x;           //  |        |
    }                     // -+        |
                          //           |
    println!("r: {}", r); // ----------+
    // 错误！x 在 'b 结束时被释放，但 r 在 'a 中仍然有效
}
```

### 生命周期标注的目标

确保**引用的生命周期不会超过它所引用的数据的生命周期**。

---

## 生命周期标注语法

### 基本语法

生命周期参数以单引号开头，如 `'a`、`'b`、`'static`。

```rust
&i32        // 普通引用
&'a i32     // 带有显式生命周期的引用
&'a mut i32 // 带有显式生命周期的可变引用
```

### 函数中的生命周期

```rust
// 标注：返回的引用与两个参数中较短的生命周期相同
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("long string");
    let result;
    
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        println!("最长的: {}", result);  // 正确
    }
    
    // println!("最长的: {}", result);  // 锣误！string2 已被释放
}
```

### 结构体中的生命周期

```自引用结构体需要标注生命周期```：

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("{}", excerpt.part);
}
```

### impl 块中的生命周期

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("公告: {}", announcement);
        self.part
    }
}
```

---

## 生命周期省略规则

编译器在以下情况下可以自动推断生命周期：

### 三条省略规则

1. **每个引用参数都获得一个独立的生命周期**
   ```rust
   fn foo(x: &i32, y: &i32)  // 等价于 fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
   ```

2. **如果只有一个输入生命周期，它被赋给所有输出生命周期**
   ```rust
   fn foo(x: &str) -> &str   // 等价于 fn foo<'a>(x: &'a str) -> &'a str
   ```

3. **如果有多个输入生命周期但其中一个来自 `&self` 或 `&mut self`，`self` 的生命周期赋给所有输出生命周期**
   ```rust
   fn method(&self, x: &str) -> &str  // 返回值继承 self 的生命周期
   ```

### 省略示例

```rust
// 编译器可以推断的情况
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// 需要显式标注的情况
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

## 静态生命周期

### `'static` 含义

`'static` 表示引用在整个程序运行期间都有效。

```rust
// 字符串字面量具有 'static 生命周期
let s: &'static str = "I have a static lifetime.";
```

### 使用场景

```rust
// 1. 常量
const MAX_POINTS: u32 = 100_000;

// 2. 静态变量
static HELLO_WORLD: &str = "Hello, world!";

// 3. 存储在二进制文件中的数据
fn main() {
    let static_str: &'static str = "This lives forever";
}
```

### 注意事项

```rust
// 错误：不要滥用 'static 来"解决"生命周期问题
fn get_str() -> &'static str {
    let s = String::from("hello");
    // &s  // 错误！s 不是静态的
    Box::leak(s.into_boxed_str())  // 这会内存泄漏，但返回 'static 引用
}
```

---

## 使用场景

### 1. 构建文本解析器

```rust
struct Parser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, position: 0 }
    }
    
    fn peek(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }
    
    fn consume(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.position += ch.len_utf8();
        Some(ch)
    }
    
    fn remaining(&self) -> &'a str {
        &self.input[self.position..]
    }
}

fn main() {
    let input = "hello world";
    let mut parser = Parser::new(input);
    
    while let Some(ch) = parser.consume() {
        println!("消耗字符: {}", ch);
    }
}
```

### 2. 构建迭代器

```rust
struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: char,
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.remainder.is_empty() {
            return None;
        }
        
        match self.remainder.find(self.delimiter) {
            Some(index) => {
                let (result, remainder) = self.remainder.split_at(index);
                self.remainder = &remainder[1..];  // 跳过分隔符
                Some(result)
            }
            None => {
                let result = self.remainder;
                self.remainder = "";
                Some(result)
            }
        }
    }
}

fn split(s: &str, delimiter: char) -> StrSplit {
    StrSplit {
        remainder: s,
        delimiter,
    }
}

fn main() {
    for part in split("a,b,c", ',') {
        println!("{}", part);
    }
}
```

### 3. 回调函数与生命周期

```rust
struct EventSystem<'a> {
    handlers: Vec<Box<dyn Fn(&str) + 'a>>,
}

impl<'a> EventSystem<'a> {
    fn new() -> Self {
        EventSystem { handlers: Vec::new() }
    }
    
    fn register<F: Fn(&str) + 'a>(&mut self, handler: F) {
        self.handlers.push(Box::new(handler));
    }
    
    fn trigger(&self, event: &str) {
        for handler in &self.handlers {
            handler(event);
        }
    }
}

fn main() {
    let mut system = EventSystem::new();
    
    let prefix = "Event: ".to_string();
    system.register(move |e| {
        println!("{}{}", prefix, e);
    });
    
    system.trigger("UserLoggedIn");
}
```

---

## 常见陷阱

### 陷阱 1: 返回局部变量的引用

```rust
// 错误示例
fn get_reference() -> &'static str {
    let s = String::from("hello");
    &s  // 错误！s 在函数结束时被释放
}

// 正确做法
fn get_string() -> String {
    String::from("hello")  // 返回所有权
}

// 或者返回静态字符串
fn get_static() -> &'static str {
    "hello"  // 字符串字面量
}
```

### 陷阱 2: 结构体生命周期不匹配

```rust
// 错误示例
struct Container<'a, 'b> {
    a: &'a str,
    b: &'b str,
}

fn combine<'a>(container: Container<'a, 'a>) -> &'a str {
    if container.a.len() > container.b.len() {
        container.a
    } else {
        container.b
    }
}

// 正确使用
fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    
    let container = Container {
        a: &s1,
        b: &s2,
    };
    
    // 这里两个引用的生命周期必须相同
}
```

### 陷阱 3: 过度约束生命周期

```rust
// 过度约束 - 所有生命周期都使用 'a
fn over_constrained<'a>(x: &'a str, y: &'a str) -> (&'a str, &'a str) {
    (x, y)
}

// 更灵活 - 使用不同的生命周期
fn flexible<'a, 'b>(x: &'a str, y: &'b str) -> (&'a str, &'b str) {
    (x, y)
}
```

### 陷阱 4: 循环引用

```rust
use std::rc::Rc;
use std::cell::RefCell;

// 可能导致内存泄漏的循环引用
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

// 正确做法：使用 Weak 引用打破循环
use std::rc::Weak;

struct SafeNode {
    value: i32,
    next: Option<Rc<RefCell<SafeNode>>>,
    prev: Option<Weak<RefCell<SafeNode>>>,  // 弱引用
}
```

### 陷阱 5: 与闭包交互

```rust
// 错误示例
fn create_closure<'a>() -> Box<dyn Fn() -> &'a str> {
    let s = String::from("hello");
    Box::new(move || &s)  // 错误！s 的所有权被移动到闭包
}

// 正确做法
fn create_closure() -> Box<dyn Fn() -> String> {
    let s = String::from("hello");
    Box::new(move || s.clone())
}

// 或者使用 'static
fn create_static_closure() -> Box<dyn Fn() -> &'static str> {
    Box::new(|| "hello")
}
```

---

## 示例代码

### 完整示例：实现字符串缓存

```rust
use std::collections::HashMap;

pub struct StringCache<'a> {
    cache: HashMap<&'a str, &'a str>,
}

impl<'a> StringCache<'a> {
    pub fn new() -> Self {
        StringCache {
            cache: HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, key: &'a str, value: &'a str) {
        self.cache.insert(key, value);
    }
    
    pub fn get(&self, key: &str) -> Option<&&'a str> {
        self.cache.get(key)
    }
    
    pub fn contains(&self, key: &str) -> bool {
        self.cache.contains_key(key)
    }
}

fn main() {
    let key1 = "name";
    let value1 = "Alice";
    let key2 = "city";
    let value2 = "Beijing";
    
    let mut cache = StringCache::new();
    cache.insert(key1, value1);
    cache.insert(key2, value2);
    
    if let Some(&value) = cache.get("name") {
        println!("name: {}", value);
    }
}
```

### 完整示例：生命周期与 trait

```rust
trait Processor<'a> {
    fn process(&self, input: &'a str) -> &'a str;
}

struct UpperCase;
struct LowerCase;

impl<'a> Processor<'a> for UpperCase {
    fn process(&self, input: &'a str) -> &'a str {
        // 注意：这里需要外部确保已转换
        input  // 简化示例
    }
}

impl<'a> Processor<'a> for LowerCase {
    fn process(&self, input: &'a str) -> &'a str {
        input
    }
}

fn run_processor<'a>(processor: &dyn Processor<'a>, input: &'a str) -> &'a str {
    processor.process(input)
}

fn main() {
    let input = "Hello World";
    let upper = UpperCase;
    
    let result = run_processor(&upper, input);
    println!("{}", result);
}
```

---

## 高级主题：生命周期子类型化

```rust
// 'static: 'a 意味着 'static 的生命周期比 'a 长
fn longest_with_static<'a>(x: &'a str, y: &'static str) -> &'a str {
    x  // 可以返回 x，因为它的生命周期已知
}

// 协变与逆变
fn covariance_example<'a, 'b: 'a>(x: &'a str, y: &'b str) {
    let _: &'a str = y;  // 'b: 'a 允许将 &'b 赋给 &'a
}
```

---

## 参考资源

- [The Rust Book - 生命周期](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Rust Reference - Lifetimes](https://doc.rust-lang.org/reference/lifetime-elision.html)
- [Rustonomicon - Lifetimes](https://doc.rust-lang.org/nomicon/lifetimes.html)

---

*Last updated: 2026-03-18*
