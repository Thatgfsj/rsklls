# 测试体系

## 目录

- [单元测试](#单元测试)
- [集成测试](#集成测试)
- [文档测试](#文档测试)
- [属性测试 (Proptest)](#属性测试-proptest)
- [Mock 测试 (Mockall)](#mock-测试-mockall)
- [测试驱动开发 (TDD)](#测试驱动开发-tdd)

---

## 单元测试

### 基本结构

```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]  // 只在测试时编译
mod tests {
    use super::*;

    #[test]  // 标记为测试函数
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, -2), -3);
    }
}
```

### 断言宏

```rust
#[test]
fn test_assertions() {
    // 相等断言
    assert_eq!(1 + 1, 2);
    assert_ne!(1 + 1, 3);
    
    // 布尔断言
    assert!(true);
    assert!(!false);
    
    // 带自定义消息
    assert_eq!(
        1 + 1, 2,
        "数学定律应该成立"
    );
    
    // panic 测试
    // assert!(false, "这会失败");
}
```

### 测试 panic

```rust
#[test]
#[should_panic]  // 期望发生 panic
fn test_panic() {
    panic!("这是一个预期的 panic");
}

#[test]
#[should_panic(expected = "division by zero")]  // 匹配 panic 消息
fn test_divide_by_zero() {
    divide(1, 0);
}

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }
    a / b
}
```

### 测试 Result

```rust
#[test]
fn test_result() -> Result<(), String> {
    let result = parse("42")?;
    assert_eq!(result, 42);
    Ok(())
}

fn parse(s: &str) -> Result<i32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
```

### 测试私有函数

```rust
// 私有函数可以直接在同一个模块测试
fn internal_helper() -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_internal() {
        assert_eq!(internal_helper(), 42);
    }
}
```

---

## 集成测试

### 目录结构

```
my-project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/          # 集成测试目录
    ├── common/
    │   └── mod.rs  # 公共测试工具
    └── integration_test.rs
```

### 基本集成测试

```rust
// tests/integration_test.rs
use my_project::MyStruct;

#[test]
fn test_integration() {
    let instance = MyStruct::new();
    assert!(instance.is_valid());
}
```

### 公共测试模块

```rust
// tests/common/mod.rs
pub fn setup() -> TestContext {
    TestContext {
        // 测试配置
    }
}

pub struct TestContext {
    // ...
}

impl Drop for TestContext {
    fn drop(&mut self) {
        // 清理逻辑
    }
}
```

```rust
// tests/integration_test.rs
mod common;

#[test]
fn test_with_setup() {
    let ctx = common::setup();
    // 使用 ctx 进行测试
}
```

---

## 文档测试

### 基本用法

```rust
/// 将两个数相加
/// 
/// # Examples
/// 
/// ```
/// use my_lib::add;
/// assert_eq!(add(1, 2), 3);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 忽略部分代码

```rust
/// ```rust
/// use my_lib::complex_setup;
/// # // 这行不会显示在文档中，但会执行
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let result = complex_setup()?;
/// # Ok(())
/// # }
/// ```
pub fn complex_setup() -> Result<i32, Box<dyn std::error::Error>> {
    Ok(42)
}
```

### 编译失败测试

```rust
/// ```compile_fail
/// use my_lib::unsafe_function;
/// unsafe_function(); // 这应该编译失败
/// ```
pub fn unsafe_function() {
    // ...
}
```

### 忽略测试

```rust
/// ```rust,ignore
/// // 这个测试会被忽略
/// assert!(false);
/// ```
pub fn some_function() {}
```

---

## 属性测试 (Proptest)

### 安装

```toml
[dev-dependencies]
proptest = "1.0"
```

### 基本用法

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_add_commutative(a: i32, b: i32) {
        // 交换律
        prop_assert_eq!(a + b, b + a);
    }
    
    #[test]
    fn test_string_reverse(s: String) {
        // 两次反转应恢复原值
        let reversed: String = s.chars().rev().collect();
        let double_reversed: String = reversed.chars().rev().collect();
        prop_assert_eq!(s, double_reversed);
    }
}
```

### 自定义策略

```rust
proptest! {
    #[test]
    fn test_custom_strategy(
        a: i32,
        b in 0..100i32,  // 范围
        s in "[a-z]{1,10}",  // 正则表达式字符串
        vec in prop::collection::vec(0..100u32, 0..10),  // 向量
    ) {
        prop_assert!(b >= 0 && b < 100);
        prop_assert!(s.len() >= 1 && s.len() <= 10);
    }
}
```

---

## Mock 测试 (Mockall)

### 安装

```toml
[dev-dependencies]
mockall = "0.12"
```

### Mock Trait

```rust
use mockall::automock;

#[automock]  // 自动生成 Mock
pub trait Database {
    fn get_user(&self, id: u32) -> Option<String>;
    fn save_user(&mut self, id: u32, name: &str) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mock() {
        let mut mock_db = MockDatabase::new();
        
        // 设置期望
        mock_db
            .expect_get_user()
            .with(1u32)
            .times(1)
            .returning(|_| Some("Alice".to_string()));
        
        // 使用 mock
        let result = mock_db.get_user(1);
        assert_eq!(result, Some("Alice".to_string()));
    }
}
```

### Mock 方法行为

```rust
mock_db
    .expect_save_user()
    .with(mockall::predicate::eq(2u32), mockall::predicate::always())
    .returning(|_, _| true);

mock_db
    .expect_get_user()
    .returning(|id| {
        if id == 1 { Some("Alice".to_string()) }
        else { None }
    });
```

---

## 测试驱动开发 (TDD)

### TDD 流程

1. **Red**: 先写失败的测试
2. **Green**: 写最少代码让测试通过
3. **Refactor**: 重构代码

### 示例

```rust
// 第 1 步: Red - 写失败的测试
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculator_add() {
        let mut calc = Calculator::new();
        assert_eq!(calc.add(1, 2), 3);
    }
}

// 第 2 步: Green - 实现最少代码
pub struct Calculator;

impl Calculator {
    pub fn new() -> Self {
        Calculator
    }
    
    pub fn add(&mut self, a: i32, b: i32) -> i32 {
        a + b
    }
}

// 第 3 步: Refactor - 优化代码
// （如果需要的话）
```

### 测试命令

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_add

# 运行特定模块
cargo test tests::calculator

# 显示输出
cargo test -- --nocapture

# 并行测试
cargo test -- --test-threads=4

# 只运行被忽略的测试
cargo test -- --ignored
```

---

## 测试最佳实践

### 1. 使用 AAA 模式

```rust
#[test]
fn test_with_aaa_pattern() {
    // Arrange (准备)
    let mut service = Service::new();
    let input = "test";
    
    // Act (执行)
    let result = service.process(input);
    
    // Assert (断言)
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "processed: test");
}
```

### 2. 测试隔离

```rust
#[test]
fn test_isolated() {
    // 每个测试应该是独立的
    // 使用局部变量，不共享状态
    
    let mut temp_file = tempfile::NamedTempFile::new().unwrap();
    // 使用临时文件进行测试
}
```

### 3. 有意义的测试名称

```rust
// 好的命名
#[test]
fn test_add_returns_sum_of_two_numbers() {}

#[test]
fn test_parse_returns_error_for_invalid_input() {}

// 不好的命名
#[test]
fn test_add() {}

#[test]
fn test1() {}
```

---

## 参考资源

- [The Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Proptest 文档](https://docs.rs/proptest/)
- [Mockall 文档](https://docs.rs/mockall/)

---

*Last updated: 2026-03-18*
