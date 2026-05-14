# rskills

Rust + 跨语言互操作专家技能，面向 AI 编程助手 (Claude Code, Cline, OpenClaw 等)。

## 项目定位

**面向 AI 编程助手的 Rust + FFI 技能包**

帮助 AI Agent 处理 Rust 开发及跨语言集成任务。

## 功能概览

| 模块 | 内容 |
|------|------|
| **Rust 核心** | 所有权、借用、生命周期、trait、异步 |
| **Python-Rust** | PyO3, maturin |
| **Go-Rust** | cgo, cxx, 共享库 |
| **Java-Rust** | JNI |
| **C/C++-Rust** | FFI, cxx |
| **WASM** | wasm-bindgen, wasm-pack |
| **OpenClaw** | 安全部署、交叉编译 |

## 快速开始

### 安装到 Claude Code

```bash
# 方法 1: 复制到 skills 目录
cp -r rskills ~/.claude/skills/

# 方法 2: 符号链接
ln -s $(pwd)/rskills ~/.claude/skills/
```

### 安装到 Cline

```bash
# 复制到 Cline skills 目录
cp -r rskills ~/.cline/skills/
```

### 初始化新 FFI 项目

```bash
# 创建 PyO3 项目
./scripts/init_rust_project.sh my_project pyo3

# 创建 cxx 项目
./scripts/init_rust_project.sh my_project cxx

# 创建 JNI 项目
./scripts/init_rust_project.sh my_project jni

# 创建 WASM 项目
./scripts/init_rust_project.sh my_project wasm
```

## 目录结构

```
rskills/
├── SKILL.md              # 技能入口 (精简后 <150 行)
├── PROMPT_EXAMPLES.md    # 示例提示词
├── README.md             # 本文档
├── CHANGELOG.md          # 变更日志
├── references/           # 详细参考文档
│   ├── ownership-borrowing.md
│   ├── ffi-overview.md
│   ├── pyo3-examples.md
│   ├── cxx-bridge.md
│   ├── java-jni.md
│   ├── wasm-guide.md
│   ├── testing-tips.md
│   ├── common-pitfalls.md
│   └── openclaw-integration.md
├── scripts/              # 自动化脚本
│   ├── init_rust_project.sh
│   ├── check_ffi_safety.py
│   ├── generate_bindings.sh
│   ├── build_for_openclaw.sh
│   └── openclaw_test.sh
├── tests/                # AI 可执行测试
│   ├── test_pyo3/
│   ├── test_cxx/
│   └── run_all_tests.sh
└── examples/             # 完整示例项目
    ├── pyo3-calc/
    ├── cxx-vector/
    └── wasm-fib/
```

## 触发词

在对话中使用以下词汇触发此技能:

`rust`, `cargo`, `pyo3`, `maturin`, `python rust`, `rust python`, `rust go`, `go rust`, `cgo`, `cxx`, `jni`, `rust java`, `rust ffi`, `所有权`, `借用`, `生命周期`, `tokio`, `openclaw`, `claw`

## 自动化命令

```bash
# 检查 FFI 代码安全性
python scripts/check_ffi_safety.py <path>

# 生成 C/C++ 绑定
./scripts/generate_bindings.sh

# OpenClaw 交叉编译
./scripts/build_for_openclaw.sh x86_64 my_app

# 运行集成测试
cd tests && ./run_all_tests.sh
```

## 示例

```python
# Python 调用 Rust (PyO3)
from pyo3_calc import Calculator, basic_add

calc = Calculator(10)
calc.add(5)       # 15
calc.multiply(2)  # 30
print(basic_add(3, 4))  # 7
```

```rust
// Rust 调用 C++ (cxx)
fn rust_process(data: &str) -> String {
    format!("Processed: {}", data)
}
```

## 贡献指南

### 添加新的 FFI 方案示例

1. 在 `examples/` 下创建新目录
2. 包含 `Cargo.toml`, `src/lib.rs`, `README.md`, `build.sh`
3. 更新 `references/ffi-overview.md` 添加方案对比

### 提交脚本

1. Shell 脚本使用 `shellcheck` 检查
2. Python 脚本包含类型注解和文档字符串
3. 更新 `SKILL.md` 的自动化能力章节

### 报告问题

创建 GitHub Issue 并提供:
- Rust 版本 (`rustc --version`)
- 目标平台 (`uname -a`)
- 最小可复现代码

## 许可证

MIT