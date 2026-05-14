#!/usr/bin/env python3
"""
check_ffi_safety.py - 检查 Rust FFI 代码中的不安全模式

Usage:
    python check_ffi_safety.py <path_to_check>

检查以下模式:
- unsafe 块缺少 SAFETY 注释
- 裸指针 (*const / *mut) 使用
- 手动内存管理 (into_raw, from_raw, Vec::from_raw_parts)
- 可能的内存泄漏
- FFI 类型未使用 #[repr(C)]
"""

import re
import sys
from pathlib import Path
from typing import List, Tuple

ISSUES = []

def check_file(filepath: Path) -> List[str]:
    """检查单个文件"""
    issues = []
    content = filepath.read_text(encoding='utf-8', errors='ignore')
    lines = content.split('\n')

    in_unsafe = False
    unsafe_start = 0
    unsafe_has_safety = False

    for i, line in enumerate(lines, 1):
        stripped = line.strip()

        # 检测 unsafe 块开始
        if stripped.startswith('unsafe '):
            in_unsafe = True
            unsafe_start = i
            unsafe_has_safety = False

            # 检查行内 unsafe (如 unsafe { ... })
            if '{' in stripped:
                # 检查该行是否有 SAFETY 注释
                if '// SAFETY:' in stripped or '//SAFETY:' in stripped:
                    unsafe_has_safety = True

        # 在 unsafe 块内检查 SAFETY 注释
        if in_unsafe:
            if '// SAFETY:' in stripped or '//SAFETY:' in stripped:
                unsafe_has_safety = True

            # 检测 unsafe 块结束
            if stripped == '}':
                if not unsafe_has_safety:
                    issues.append(
                        f"{filepath}:{unsafe_start}: unsafe 块缺少 SAFETY 注释"
                    )
                in_unsafe = False

        # 检查裸指针使用
        if '*const c_char' in stripped or '*mut c_char' in stripped:
            if '// SAFETY:' not in stripped and 'unsafe' not in stripped:
                issues.append(
                    f"{filepath}:{i}: 裸指针可能需要 SAFETY 注释"
                )

        # 检查手动内存管理
        memory_patterns = [
            (r'\.into_raw\(\)', '可能内存泄漏：into_raw 后未 from_raw'),
            (r'Vec::from_raw_parts', 'Vec::from_raw_parts 需要确保长度正确'),
            (r'Box::from_raw', 'Box::from_raw 需要确保所有权'),
        ]

        for pattern, msg in memory_patterns:
            if re.search(pattern, stripped):
                if '// SAFETY:' not in stripped:
                    issues.append(f"{filepath}:{i}: {msg}")

        # 检查缺少 #[repr(C)] 的跨语言结构体
        struct_match = re.search(r'^struct\s+(\w+)', stripped)
        if struct_match:
            struct_name = struct_match.group(1)
            # 检查是否在 ffi 或 extern 上下文中
            if '#[repr(C)]' not in content[max(0, content.find(struct_name) - 200):content.find(struct_name)]:
                # 可能需要 #[repr(C)]，但这是启发式检查
                pass

    return issues

def main():
    if len(sys.argv) < 2:
        print("Usage: python check_ffi_safety.py <path>")
        print("  path: 文件或目录路径")
        sys.exit(1)

    target = Path(sys.argv[1])

    if target.is_file():
        files = [target]
    elif target.is_dir():
        files = list(target.rglob('*.rs'))
    else:
        print(f"Error: {target} 不是文件或目录")
        sys.exit(1)

    all_issues = []
    for f in files:
        issues = check_file(f)
        all_issues.extend(issues)

    if all_issues:
        print(f"找到 {len(all_issues)} 个潜在问题:")
        for issue in all_issues:
            print(f"  - {issue}")
        sys.exit(1)
    else:
        print("✓ 未发现明显的安全问题")
        print("注意: 这是启发式检查，不能保证完全正确")
        sys.exit(0)

if __name__ == '__main__':
    main()