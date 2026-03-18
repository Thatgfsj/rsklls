# 实战项目：命令行工具

## 项目概述

构建一个功能完整的命令行工具 `grep-lite`，实现类似 grep 的文件搜索功能。

## 功能需求

- 搜索文件中的匹配文本
- 支持正则表达式
- 支持忽略大小写
- 支持递归搜索目录
- 彩色输出
- 行号显示

---

## 项目结构

```
grep-lite/
├── Cargo.toml
├── src/
│   ├── main.rs        # 入口点
│   ├── lib.rs         # 库入口
│   ├── cli.rs         # CLI 参数解析
│   ├── search.rs      # 搜索逻辑
│   ├── output.rs      # 输出格式化
│   └── error.rs       # 错误定义
├── tests/
│   └── integration.rs
└── README.md
```

---

## 第一步：项目初始化

```bash
cargo new grep-lite
cd grep-lite
```

### Cargo.toml

```toml
[package]
name = "grep-lite"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your@email.com>"]
description = "A simplified grep implementation in Rust"

[dependencies]
clap = { version = "4.0", features = ["derive"] }
regex = "1.10"
colored = "2.0"
walkdir = "2.4"
thiserror = "1.0"
anyhow = "1.0"

[dev-dependencies]
tempfile = "3.0"
assert_cmd = "2.0"
predicates = "3.0"
```

---

## 第二步：错误定义

```rust
// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrepError {
    #[error("文件读取失败: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("无效的正则表达式: {0}")]
    RegexError(#[from] regex::Error),
    
    #[error("没有找到匹配")]
    NoMatch,
}

pub type Result<T> = std::result::Result<T, GrepError>;
```

---

## 第三步：CLI 参数解析

```rust
// src/cli.rs
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "grep-lite")]
#[command(about = "A simplified grep implementation", long_about = None)]
pub struct Args {
    /// 要搜索的模式
    #[arg(required = true)]
    pub pattern: String,
    
    /// 要搜索的文件或目录
    #[arg(required = true)]
    pub path: String,
    
    /// 忽略大小写
    #[arg(short = 'i', long)]
    pub ignore_case: bool,
    
    /// 递归搜索目录
    #[arg(short = 'r', long)]
    pub recursive: bool,
    
    /// 显示行号
    #[arg(short = 'n', long)]
    pub line_numbers: bool,
    
    /// 只显示文件名
    #[arg(short = 'l', long)]
    pub files_with_matches: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
```

---

## 第四步：搜索逻辑

```rust
// src/search.rs
use crate::error::{GrepError, Result};
use regex::{Regex, RegexBuilder};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

pub struct Match {
    pub file: String,
    pub line_number: usize,
    pub line: String,
    pub start: usize,
    pub end: usize,
}

pub struct Searcher {
    pattern: Regex,
    show_line_numbers: bool,
    files_only: bool,
}

impl Searcher {
    pub fn new(pattern: &str, ignore_case: bool) -> Result<Self> {
        let regex = RegexBuilder::new(pattern)
            .case_insensitive(ignore_case)
            .build()?;
        
        Ok(Searcher {
            pattern: regex,
            show_line_numbers: true,
            files_only: false,
        })
    }
    
    pub fn with_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }
    
    pub fn files_only(mut self, files_only: bool) -> Self {
        self.files_only = files_only;
        self
    }
    
    pub fn search_file(&self, path: &Path) -> Result<Vec<Match>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut matches = Vec::new();
        let file_name = path.display().to_string();
        
        for (line_number, line_result) in reader.lines().enumerate() {
            let line = line_result?;
            let line_number = line_number + 1;  // 1-based
            
            for cap in self.pattern.find_iter(&line) {
                matches.push(Match {
                    file: file_name.clone(),
                    line_number,
                    line: line.clone(),
                    start: cap.start(),
                    end: cap.end(),
                });
                
                if self.files_only {
                    return Ok(matches);
                }
            }
        }
        
        Ok(matches)
    }
    
    pub fn search_dir(&self, path: &Path) -> Result<Vec<Match>> {
        let mut all_matches = Vec::new();
        
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Ok(m) = self.search_file(entry.path()) {
                    all_matches.extend(m);
                }
            }
        }
        
        Ok(all_matches)
    }
}
```

---

## 第五步：输出格式化

```rust
// src/output.rs
use crate::search::Match;
use colored::Colorize;

pub struct Printer {
    show_line_numbers: bool,
    files_only: bool,
}

impl Printer {
    pub fn new(show_line_numbers: bool, files_only: bool) -> Self {
        Printer {
            show_line_numbers,
            files_only,
        }
    }
    
    pub fn print_matches(&self, matches: &[Match]) {
        if self.files_only {
            let mut printed_files = std::collections::HashSet::new();
            for m in matches {
                if printed_files.insert(&m.file) {
                    println!("{}", m.file);
                }
            }
            return;
        }
        
        for m in matches {
            self.print_match(m);
        }
    }
    
    fn print_match(&self, m: &Match) {
        let file = m.file.cyan();
        
        if self.show_line_numbers {
            let line_num = format!("{}:", m.line_number).yellow();
            
            // 高亮匹配部分
            let highlighted = self.highlight_match(&m.line, m.start, m.end);
            
            println!("{}:{}:{}", file, line_num, highlighted);
        } else {
            let highlighted = self.highlight_match(&m.line, m.start, m.end);
            println!("{}:{}", file, highlighted);
        }
    }
    
    fn highlight_match(&self, line: &str, start: usize, end: usize) -> String {
        let before = &line[..start];
        let matched = &line[start..end];
        let after = &line[end..];
        
        format!("{}{}{}", before, matched.red().bold(), after)
    }
}
```

---

## 第六步：库入口

```rust
// src/lib.rs
pub mod cli;
pub mod error;
pub mod output;
pub mod search;

pub use cli::Args;
pub use error::{GrepError, Result};
pub use output::Printer;
pub use search::{Match, Searcher};
```

---

## 第七步：主程序

```rust
// src/main.rs
use clap::Parser;
use grep_lite::{
    Args, Printer, Searcher,
};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    let searcher = Searcher::new(&args.pattern, args.ignore_case)?
        .with_line_numbers(args.line_numbers)
        .files_only(args.files_with_matches);
    
    let path = Path::new(&args.path);
    let matches = if args.recursive && path.is_dir() {
        searcher.search_dir(path)?
    } else {
        searcher.search_file(path)?
    };
    
    if matches.is_empty() {
        eprintln!("没有找到匹配");
        std::process::exit(1);
    }
    
    let printer = Printer::new(args.line_numbers, args.files_with_matches);
    printer.print_matches(&matches);
    
    Ok(())
}
```

---

## 第八步：集成测试

```rust
// tests/integration.rs
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

fn setup_test_files() -> TempDir {
    let dir = TempDir::new().unwrap();
    
    fs::write(
        dir.path().join("test.txt"),
        "hello world\nfoo bar\nhello rust\n",
    ).unwrap();
    
    fs::write(
        dir.path().join("other.txt"),
        "other content\nhello there\n",
    ).unwrap();
    
    dir
}

#[test]
fn test_basic_search() {
    let dir = setup_test_files();
    let file = dir.path().join("test.txt");
    
    let mut cmd = Command::cargo_bin("grep-lite").unwrap();
    cmd.arg("hello")
        .arg(file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("hello world"))
        .stdout(predicate::str::contains("hello rust"));
}

#[test]
fn test_ignore_case() {
    let dir = setup_test_files();
    let file = dir.path().join("test.txt");
    
    let mut cmd = Command::cargo_bin("grep-lite").unwrap();
    cmd.arg("HELLO")
        .arg("-i")
        .arg(file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"));
}

#[test]
fn test_recursive() {
    let dir = setup_test_files();
    
    let mut cmd = Command::cargo_bin("grep-lite").unwrap();
    cmd.arg("hello")
        .arg("-r")
        .arg(dir.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("test.txt"))
        .stdout(predicate::str::contains("other.txt"));
}
```

---

## 使用示例

```bash
# 基本搜索
grep-lite "pattern" file.txt

# 忽略大小写
grep-lite -i "pattern" file.txt

# 显示行号
grep-lite -n "pattern" file.txt

# 递归搜索目录
grep-lite -r "pattern" ./src

# 只显示文件名
grep-lite -l "pattern" ./src

# 组合使用
grep-lite -r -i -n "pattern" ./src
```

---

## 扩展建议

1. **添加更多选项**
   - `-A` / `-B` / `-C`: 显示上下文行
   - `-v`: 反向匹配
   - `-c`: 统计匹配行数

2. **性能优化**
   - 使用多线程并行搜索
   - 内存映射大文件

3. **输出格式**
   - JSON 输出
   - 输出到文件

---

*Last updated: 2026-03-18*
