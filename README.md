# rskills

Rust + 跨语言互操作专家技能，面向 AI 编程助手 (Claude Code, Cline, OpenClaw, OpenCode 等)。

## 项目定位

**面向 AI 编程助手的 Rust + FFI 技能包**

帮助 AI Agent 处理 Rust 开发及跨语言集成任务。

## 平台兼容

| 平台 | 状态 | 说明 |
|------|------|------|
| **Claude Code** | ✅ 完整支持 | 复制到 `~/.claude/skills/` |
| **Cline** | ✅ 完整支持 | 复制到 `~/.cline/skills/` |
| **OpenClaw** | ✅ 完整支持 | 沙箱安全部署、交叉编译 |
| **OpenCode** | ✅ 完整支持 | 适配 skill tool 调用格式 |
| **Generic Agent** | ✅ 兼容 | 标准 SKILL.md 格式 |

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

### 安装到 Claude Code / OpenCode

```bash
# 复制到 skills 目录
cp -r rskills ~/.claude/skills/
# 或
cp -r rskills ~/.opencode/skills/
```

### 安装到 Cline

```bash
# 复制到 Cline skills 目录
cp -r rskills ~/.cline/skills/
```

### 安装到 OpenClaw

```bash
# 使用 clawdhub 安装
clawdhub install rskills

# 或手动复制
cp -r rskills ~/.claw/skills/
```

### OpenClaw 部署示例

```bash
# 交叉编译 Rust 二进制
./scripts/build_for_openclaw.sh x86_64 my_rust_app

# 本地测试
./scripts/openclaw_test.sh ./dist/my_rust_app

# 部署到 OpenClaw 沙箱
# 参考 references/openclaw-integration.md
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

`rust`, `cargo`, `pyo3`, `maturin`, `python rust`, `rust python`, `rust go`, `go rust`, `cgo`, `cxx`, `jni`, `rust java`, `rust ffi`, `所有权`, `借用`, `生命周期`, `tokio`, `openclaw`, `claw`, `opencode`, `clawdbot`

## OpenClaw 专用能力

```bash
# 交叉编译 (x86_64 / aarch64)
./scripts/build_for_openclaw.sh x86_64 my_app
./scripts/build_for_openclaw.sh aarch64 my_app

# 本地沙箱测试
./scripts/openclaw_test.sh <binary_path> [timeout]

# FFI 安全检查
python scripts/check_ffi_safety.py <path>

# 生成 C++ 头文件
./scripts/generate_bindings.sh
```

## OpenCode / OpenClaw 集成说明

当 AI Agent 使用 skill tool 加载此技能时:
1. AI 首先读取精简的 `SKILL.md` 获取核心指令
2. 根据需要查阅 `references/` 下的详细文档
3. 可执行 `scripts/` 中的自动化脚本
4. 可运行 `tests/` 中的集成测试验证

**OpenClaw 安全特性:**
- 所有 unsafe 代码标注 `// SAFETY:` 注释
- 跨语言边界使用 `panic::catch_unwind` 保护
- 支持静态链接 (`x86_64-unknown-linux-musl`)
- 可打包为 distroless/scratch 镜像

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

### OpenCode / OpenClaw 兼容性要求

- SKILL.md 使用标准格式，支持 skill tool 解析
- references/ 目录结构支持渐进式加载
- scripts/ 使用 POSIX 兼容的 shell 脚本
- Python 脚本使用 Python 3.6+ 语法
- 所有 unsafe 块必须包含 SAFETY 注释

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