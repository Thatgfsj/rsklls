# Changelog

All notable changes to this project will be documented in this file.

## [1.1.0] - 2025-01-14

### Added
- **目录重构**: 遵循"渐进式披露"原则
  - 精简 SKILL.md 至 150 行以内
  - 创建 `references/` 目录拆分详细内容
- **references/**: 9 个详细参考文档
  - `ownership-borrowing.md` - Rust 所有权/借用/生命周期
  - `ffi-overview.md` - FFI 方案对比 (PyO3/cxx/JNI/WASM)
  - `pyo3-examples.md` - PyO3 完整代码示例
  - `cxx-bridge.md` - cxx 跨语言调用示例
  - `java-jni.md` - JNI 绑定示例
  - `wasm-guide.md` - Rust → WASM 指南
  - `testing-tips.md` - 测试技巧
  - `common-pitfalls.md` - 常见陷阱与解决方案
  - `openclaw-integration.md` - OpenClaw 集成专用内容
- **scripts/**: 5 个自动化脚本
  - `init_rust_project.sh` - 初始化 FFI 项目
  - `check_ffi_safety.py` - FFI 代码安全检查
  - `generate_bindings.sh` - 生成 C++ 头文件
  - `build_for_openclaw.sh` - OpenClaw 交叉编译
  - `openclaw_test.sh` - OpenClaw 本地测试
- **tests/**: AI 可执行测试
  - `test_pyo3/` - PyO3 测试项目
  - `test_cxx/` - cxx 测试项目
  - `run_all_tests.sh` - 测试运行脚本
- **examples/**: 3 个完整示例项目
  - `pyo3-calc/` - Python-Rust 计算器
  - `cxx-vector/` - C++-Rust 向量操作
  - `wasm-fib/` - WASM 斐波那契
- **PROMPT_EXAMPLES.md**: 示例提示词集合
- **.github/workflows/ci.yml**: GitHub Actions CI
- **版本锁定建议**: 所有 Cargo.toml 示例使用精确版本

### Changed
- SKILL.md: 从 664 行精简至 ~150 行
- 所有示例添加 `// SAFETY:` 注释说明
- 更新触发词列表 (增加 `openclaw`, `claw`)

## [1.0.0] - 2025-01-13

### Added
- 初始 SKILL.md (664 行)
- 基本的 Rust/FFI 文档
- PyO3, cxx, JNI 示例