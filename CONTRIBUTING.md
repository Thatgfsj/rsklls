# 贡献指南

感谢你考虑为 rsklls 做贡献！

## 如何贡献

### 报告问题

如果你发现了 bug 或有功能建议：

1. 在 [Issues](https://github.com/Thatgfsj/rsklls/issues) 中搜索是否已有相关问题
2. 如果没有，创建新的 Issue，包含：
   - 清晰的标题
   - 问题描述
   - 复现步骤（如果是 bug）
   - 期望行为
   - 实际行为
   - 环境信息（Rust 版本、操作系统等）

### 提交代码

#### 1. Fork 并克隆仓库

```bash
git clone https://github.com/YOUR_USERNAME/rsklls.git
cd rsklls
```

#### 2. 创建分支

```bash
git checkout -b feature/your-feature-name
# 或
git checkout -b fix/your-bug-fix
```

#### 3. 进行修改

- 遵循代码风格规范
- 添加必要的测试
- 更新相关文档

#### 4. 运行检查

```bash
# 格式化
cargo fmt

# Lint 检查
cargo clippy -- -D warnings

# 运行测试
cargo test

# 检查文档
cargo doc --no-deps
```

#### 5. 提交更改

使用清晰的提交信息：

```
类型: 简短描述

详细描述（可选）

相关 Issue: #123
```

类型：
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式（不影响功能）
- `refactor`: 重构
- `test`: 测试相关
- `chore`: 构建/工具相关

示例：
```
feat: 添加异步数据库连接支持

- 支持 tokio 异步运行时
- 添加连接池配置
- 添加重连机制

Closes #42
```

#### 6. 推送并创建 PR

```bash
git push origin feature/your-feature-name
```

然后在 GitHub 上创建 Pull Request。

## 代码规范

### 命名

- 类型：PascalCase
- 函数/变量：snake_case
- 常量：SCREAMING_SNAKE_CASE

### 文档注释

公开 API 必须有文档注释：

```rust
/// 简短描述。
///
/// 详细描述。
///
/// # Examples
///
/// ```
/// use rsklls::some_function;
/// assert_eq!(some_function(1, 2), 3);
/// ```
///
/// # Panics
///
/// 描述何时会 panic。
///
/// # Errors
///
/// 描述可能的错误。
pub fn some_function(a: i32, b: i32) -> i32 {
    a + b
}
```

### 测试

- 新功能必须有测试
- Bug 修复应包含回归测试
- 运行 `cargo test` 确保所有测试通过

## 开发环境设置

### 要求

- Rust 1.70+
- rustfmt
- clippy

### 推荐 VSCode 扩展

- rust-analyzer
- CodeLLDB
- Better TOML

## 行为准则

- 尊重所有贡献者
- 接受建设性批评
- 关注对社区最有利的事情

## 许可证

通过提交代码，你同意你的贡献将根据 MIT 许可证授权。

---

*Last updated: 2026-03-18*
