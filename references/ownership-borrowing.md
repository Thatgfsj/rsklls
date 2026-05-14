# Rust 所有权、借用与生命周期详解

## 所有权规则

### 三条核心规则

1. Rust 中每个值有且只有一个所有者 (owner)
2. 值在同一时间有且只有一个所有者
3. 当所有者离开作用域，值被 drop（内存释放）

### 所有权转移 (Move)

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 移动到 s2，s1 不再有效

// s1 现在无效，编译错误:
// println!("{}", s1);
```

### 克隆 (Clone)

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // 深拷贝，两个独立值

println!("{} {}", s1, s2);  // 都有效
```

### Copy 类型

栈上分配的类型（如 `i32`, `f64`, `bool`, 固定大小数组）自动 Copy：

```rust
let x = 5;
let y = x;  // 自动 Copy，x 仍有效
println!("{} {}", x, y);
```

---

## 借用 (Borrowing)

借用是对值的引用，不获得所有权。

### 不可变借用 `&T`

```rust
fn calculate_length(s: &String) -> usize {
    s.len()  // 借用 s，不获取所有权
}  // s 离开作用域但不 drop，因为只是借用

fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);  // 传入引用
    println!("'{}' 的长度是 {}", s, len);  // s 仍有效
}
```

### 可变借用 `&mut T`

```rust
fn change(s: &mut String) {
    s.push_str(" world");
}

fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);  // "hello world"
}
```

**关键限制**：同一时间只能有一个可变借用，或多个不可变借用，不能同时存在。

```rust
let mut s = String::from("hello");

let r1 = &s;      // OK
let r2 = &s;      // OK，多个不可变借用
// let r3 = &mut s;  // 错误！不能同时有可变借用

println!("{} {}", r1, r2);

// r1, r2 在这里之后才可以用 r3
let r3 = &mut s;  // OK，因为 r1, r2 已不再使用
```

---

## 生命周期 (Lifetime)

生命周期是引用有效的作用域。

### 生命周期标注 `'a`

```rust
// 返回引用时，Rust 需要知道输入引用的生命周期关系
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

`'a` 表示"返回值的生命周期与输入参数中较短的那个相同"。

### 结构体中的生命周期

结构体持有引用时必须标注生命周期：

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,  // 必须标注，说明 part 引用必须比结构体活得更久
}

impl<'a> ImportantExcerpt<'a> {
    fn announcement(&self) -> &str {
        self.part
    }
}
```

### 静态生命周期 `'static`

程序整个运行期间都有效的引用：

```rust
let s: &'static str = "我拥有静态生命周期";
```

字符串字面量默认是 `'static`。

### 生命周期省略

以下情况 Rust 自动推断生命周期：
- 输入生命周期：每个引用参数获得独立生命周期
- 输出生命周期：如果只有一个输入生命周期，返回值与之关联

```rust
// 省略前
fn first_word<'a>(s: &'a str) -> &'a str {

// 省略后（等价）
fn first_word(s: &str) -> &str {
```

---

## 常见错误与解决

### use after move

```rust
let s = String::from("hello");
// let len = s.len();  // 如果在这之前使用 s...
let s2 = s;  // s 移动到 s2
// println!("{}", s);  // 错误！s 已移动
```

**解决**：克隆 (`s.clone()`) 或在移动前提取需要的信息。

### 悬垂引用 (Dangling Reference)

```rust
// 错误示例
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // s 在函数结束时被 drop，返回悬垂引用
}

// 正确：返回所有权
fn dangle_correct() -> String {
    let s = String::from("hello");
    s  // 移动出去，所有权转移
}
```

### 多个可变引用

```rust
let mut v = vec![1, 2, 3];
let a = &mut v;
let b = &mut v;  // 错误！同时存在两个可变引用
// 解决：确保一个引用不再使用
```

---

## 与 FFI 相关的所有权要点

1. **跨语言传递字符串**：
   - Rust → C: 使用 `CString::new()` 转换，注意内存由谁释放
   - C → Rust: 使用 `CStr::from_ptr()` 读取，不拥有数据

2. **容器类型**：
   - `Vec<T>` 不能直接跨越 FFI 边界，使用 `Vec::as_ptr()` 获取指针
   - 接收方必须知道长度，调用 `Vec::from_raw_parts` 重建

3. **内存所有权协议**：
   - 在文档中明确谁负责分配和释放
   - 推荐：分配方释放，或使用唯一约定
