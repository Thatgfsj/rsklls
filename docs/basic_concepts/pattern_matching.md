# 模式匹配 (Pattern Matching)

## 目录

- [概述](#概述)
- [match 表达式](#match-表达式)
- [if let 表达式](#if-let-表达式)
- [while let 表达式](#while-let-表达式)
- [解构模式](#解构模式)
- [守卫条件](#守卫条件)
- [绑定模式](#绑定模式)
- [使用场景](#使用场景)
- [常见陷阱](#常见陷阱)

---

## 概述

模式匹配是 Rust 中强大的控制流机制，允许根据数据的形状和内容进行分支匹配。模式匹配是**穷尽的**（exhaustive），编译器会确保所有可能的情况都被处理。

---

## match 表达式

### 基本语法

```rust
fn main() {
    let number = 13;
    
    match number {
        1 => println!("一"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("质数"),
        13..=19 => println!("青少年"),  // 不会执行，因为前面已匹配
        _ => println!("其他"),
    }
}
```

### 穷尽性检查

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_player(dir: Direction) {
    match dir {
        Direction::Up => println!("向上移动"),
        Direction::Down => println!("向下移动"),
        Direction::Left => println!("向左移动"),
        Direction::Right => println!("向右移动"),
        // 必须覆盖所有变体，否则编译错误
    }
}
```

### 匹配 Option

```rust
fn main() {
    let some_value = Some(5);
    
    match some_value {
        Some(1) => println!("值是 1"),
        Some(n @ 2..=10) => println!("值在 2-10 之间: {}", n),
        Some(_) => println!("其他值"),
        None => println!("没有值"),
    }
}
```

### 匹配 Result

```rust
use std::fs::File;
use std::io::Error;

fn main() {
    let result = File::open("hello.txt");
    
    match result {
        Ok(file) => println!("文件打开成功: {:?}", file),
        Err(Error { kind: std::io::ErrorKind::NotFound, .. }) => {
            println!("文件不存在");
        }
        Err(e) => println!("其他错误: {}", e),
    }
}
```

---

## if let 表达式

### 基本用法

```rust
fn main() {
    let some_value = Some(5);
    
    // 使用 match
    match some_value {
        Some(5) => println!("值是 5"),
        _ => (),
    }
    
    // 使用 if let（更简洁）
    if let Some(5) = some_value {
        println!("值是 5");
    }
}
```

### 结合 else

```rust
fn main() {
    let value = Some(10);
    
    if let Some(n) = value {
        println!("值是: {}", n);
    } else {
        println!("没有值");
    }
}
```

### 链式条件

```rust
fn main() {
    let x = Some(5);
    let y = 10;
    
    if let Some(n) = x {
        if n > y {
            println!("{} > {}", n, y);
        } else {
            println!("{} <= {}", n, y);
        }
    }
}
```

---

## while let 表达式

### 基本用法

```rust
fn main() {
    let mut stack = vec![1, 2, 3];
    
    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }
}
```

### 处理迭代器

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let mut iter = v.iter();
    
    while let Some(n) = iter.next() {
        if *n > 3 {
            break;
        }
        println!("值: {}", n);
    }
}
```

---

## 解构模式

### 解构结构体

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 10, y: 20 };
    
    // 解构所有字段
    let Point { x, y } = point;
    println!("x: {}, y: {}", x, y);
    
    // 解构并重命名
    let Point { x: a, y: b } = Point { x: 1, y: 2 };
    println!("a: {}, b: {}", a, b);
    
    // 部分解构
    let Point { x, .. } = point;
    println!("只取 x: {}", x);
    
    // 匹配特定值
    match point {
        Point { x: 0, y: 0 } => println!("原点"),
        Point { x: 0, y } => println!("y 轴上，y = {}", y),
        Point { x, y: 0 } => println!("x 轴上，x = {}", x),
        Point { x, y } => println!("坐标 ({}, {})", x, y),
    }
}
```

### 解构枚举

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(255, 0, 0);
    
    match msg {
        Message::Quit => println!("退出"),
        Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
        Message::Write(text) => println!("文本: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("颜色: R={}, G={}, B={}", r, g, b);
        }
    }
}
```

### 解构元组

```rust
fn main() {
    let tuple = (1, 2, 3, 4, 5);
    
    match tuple {
        (first, .., last) => {
            println!("第一个: {}, 最后一个: {}", first, last);
        }
    }
    
    // 解构嵌套
    let nested = ((1, 2), (3, 4));
    let ((a, b), (c, d)) = nested;
    println!("{}, {}, {}, {}", a, b, c, d);
}
```

### 解构数组

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    
    match arr {
        [1, 2, 3, 4, 5] => println!("完整匹配"),
        [first, .., last] => println!("首: {}, 尾: {}", first, last),
        [first, second, ..] => println!("前两个: {}, {}", first, second),
        _ => println!("其他情况"),
    }
    
    // 固定大小数组匹配
    let fixed: [i32; 3] = [1, 2, 3];
    match fixed {
        [1, ..] => println!("以 1 开头"),
        _ => println!("其他"),
    }
}
```

### 解构引用

```rust
fn main() {
    let point = Point { x: 10, y: 20 };
    let reference = &point;
    
    // 解构引用
    let &Point { x, y } = reference;
    println!("x: {}, y: {}", x, y);
    
    // 或使用 ref 模式
    match reference {
        &Point { x, y } => println!("x: {}, y: {}", x, y),
    }
    
    // ref 关键字
    let name = String::from("Rust");
    match name {
        ref n => println!("引用: {}", n),
    }
    println!("name 仍可用: {}", name);
}
```

---

## 守卫条件

### 基本用法

```rust
fn main() {
    let pair = (2, -2);
    
    match pair {
        (x, y) if x == y => println!("相等"),
        (x, y) if x + y == 0 => println!("和为零"),
        (x, _) if x % 2 == 0 => println!("x 是偶数"),
        _ => println!("其他"),
    }
}
```

### 结合枚举

```rust
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

fn main() {
    let temp = Temperature::Celsius(35.0);
    
    match temp {
        Temperature::Celsius(t) if t > 30.0 => println!("很热！{}°C", t),
        Temperature::Celsius(t) if t < 10.0 => println!("很冷！{}°C", t),
        Temperature::Celsius(t) => println!("舒适！{}°C", t),
        Temperature::Fahrenheit(t) if t > 86.0 => println!("很热！{}°F", t),
        Temperature::Fahrenheit(t) => println!("{}°F", t),
    }
}
```

---

## 绑定模式

### @ 绑定

```rust
fn main() {
    let age = 25;
    
    match age {
        n @ 0..=12 => println!("儿童: {} 岁", n),
        n @ 13..=19 => println!("青少年: {} 岁", n),
        n @ 20..=30 => println!("青年: {} 岁", n),
        n => println!("其他: {} 岁", n),
    }
}
```

### 绑定枚举变体

```rust
enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };
    
    match msg {
        Message::Hello { id: id @ 1..=10 } => {
            println!("找到 ID: {}", id);
        }
        Message::Hello { id } => {
            println!("其他 ID: {}", id);
        }
    }
}
```

---

## 使用场景

### 1. 状态机

```rust
enum State {
    Idle,
    Connecting,
    Connected { session_id: u64 },
    Disconnected { reason: String },
}

enum Event {
    Connect,
    Connected { session_id: u64 },
    Disconnect { reason: String },
    Timeout,
}

fn handle_event(state: State, event: Event) -> State {
    match (state, event) {
        (State::Idle, Event::Connect) => {
            println!("开始连接...");
            State::Connecting
        }
        (State::Connecting, Event::Connected { session_id }) => {
            println!("已连接，会话 ID: {}", session_id);
            State::Connected { session_id }
        }
        (State::Connecting, Event::Timeout) => {
            println!("连接超时");
            State::Disconnected { reason: "超时".to_string() }
        }
        (State::Connected { .. }, Event::Disconnect { reason }) => {
            println!("断开连接: {}", reason);
            State::Disconnected { reason }
        }
        _ => {
            println!("无效状态转换");
            state
        }
    }
}
```

### 2. AST 解析

```rust
enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

fn evaluate(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(a, b) => evaluate(a) + evaluate(b),
        Expr::Sub(a, b) => evaluate(a) - evaluate(b),
        Expr::Mul(a, b) => evaluate(a) * evaluate(b),
        Expr::Div(a, b) => evaluate(a) / evaluate(b),
    }
}

fn main() {
    // (1 + 2) * 3
    let expr = Expr::Mul(
        Box::new(Expr::Add(
            Box::new(Expr::Number(1)),
            Box::new(Expr::Number(2)),
        )),
        Box::new(Expr::Number(3)),
    );
    
    println!("结果: {}", evaluate(&expr));  // 9
}
```

### 3. 命令解析

```rust
enum Command {
    Help,
    List { all: bool },
    Get { id: u64 },
    Create { name: String, value: i32 },
    Delete { id: u64 },
}

fn parse_command(input: &str) -> Option<Command> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    match parts.as_slice() {
        ["help"] => Some(Command::Help),
        ["list"] => Some(Command::List { all: false }),
        ["list", "--all"] => Some(Command::List { all: true }),
        ["get", id] => id.parse().ok().map(|id| Command::Get { id }),
        ["create", name, value] => {
            value.parse().ok().map(|value| Command::Create {
                name: name.to_string(),
                value,
            })
        }
        ["delete", id] => id.parse().ok().map(|id| Command::Delete { id }),
        _ => None,
    }
}
```

---

## 常见陷阱

### 陷阱 1: 忘记处理所有情况

```rust
// 错误示例
fn main() {
    let value = Some(5);
    match value {
        Some(5) => println!("是 5"),
        // 错误！缺少 None 分支
    }
}

// 正确做法
fn main() {
    let value = Some(5);
    match value {
        Some(5) => println!("是 5"),
        Some(_) => println!("其他值"),
        None => println!("None"),
    }
}
```

### 陷阱 2: 变量遮蔽

```rust
// 错误示例
fn main() {
    let x = Some(5);
    let y = 10;
    
    match x {
        Some(y) => println!("匹配到: {}", y),  // 遮蔽了外部的 y
        None => println!("None"),
    }
    
    println!("外部 y: {}", y);  // 仍然是 10
}

// 正确做法：使用不同的变量名
fn main() {
    let x = Some(5);
    let y = 10;
    
    match x {
        Some(n) => println!("匹配到: {}", n),
        None => println!("None"),
    }
}
```

### 陷阱 3: 模式顺序

```rust
// 错误示例
fn main() {
    let x = 5;
    match x {
        n => println!("任何值: {}", n),  // 匹配所有
        5 => println!("五"),  // 永远不会执行！
    }
}

// 正确做法：特定模式在前
fn main() {
    let x = 5;
    match x {
        5 => println!("五"),
        n => println!("其他值: {}", n),
    }
}
```

### 陷阱 4: 移动语义

```rust
// 错误示例
fn main() {
    let s = Some(String::from("hello"));
    
    match s {
        Some(value) => println!("{}", value),  // value 获得所有权
        None => {}
    }
    
    // println!("{:?}", s);  // 错误！s 已被移动
}

// 正确做法：使用引用
fn main() {
    let s = Some(String::from("hello"));
    
    match &s {
        Some(value) => println!("{}", value),
        None => {}
    }
    
    println!("{:?}", s);  // 正确
}
```

---

## 参考资源

- [The Rust Book - 模式匹配](https://doc.rust-lang.org/book/ch18-00-patterns.html)
- [Rust Reference - Patterns](https://doc.rust-lang.org/reference/patterns.html)

---

*Last updated: 2026-03-18*
