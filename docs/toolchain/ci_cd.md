# CI/CD 配置

## GitHub Actions 配置

### 基本工作流

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main, master]
  pull_request:
    branches: [main, master]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  test:
    name: Test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        
      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-
            
      - name: Run tests
        run: cargo test --all-features
        
      - name: Run tests with minimal versions
        run: cargo test -Z minimal-versions

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
          
      - name: Run clippy
        run: cargo clippy --all-features -- -D warnings

  fmt:
    name: Format
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
          
      - name: Check formatting
        run: cargo fmt --all -- --check

  docs:
    name: Documentation
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        
      - name: Build documentation
        run: cargo doc --no-deps --all-features
        
      - name: Deploy to GitHub Pages
        if: github.event_name == 'push' && github.ref == 'refs/heads/main'
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./target/doc

  security:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        
      - name: Install cargo-audit
        run: cargo install cargo-audit
        
      - name: Run security audit
        run: cargo audit

  coverage:
    name: Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        
      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin
        
      - name: Generate coverage
        run: cargo tarpaulin --out Xml
        
      - name: Upload to codecov
        uses: codecov/codecov-action@v3
        with:
          files: cobertura.xml
```

---

## 多平台构建

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    name: Build ${{ matrix.target }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
            archive: tar.gz
          - target: x86_64-apple-darwin
            os: macos-latest
            archive: tar.gz
          - target: x86_64-pc-windows-msvc
            os: windows-latest
            archive: zip
          - target: aarch64-unknown-linux-gnu
            os: ubuntu-latest
            archive: tar.gz
          - target: aarch64-apple-darwin
            os: macos-latest
            archive: tar.gz

    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
          
      - name: Build
        run: cargo build --release --target ${{ matrix.target }}
        
      - name: Package (Unix)
        if: matrix.os != 'windows-latest'
        run: |
          cd target/${{ matrix.target }}/release
          tar -czvf ../../../${{ matrix.target }}.tar.gz <binary_name>
          
      - name: Package (Windows)
        if: matrix.os == 'windows-latest'
        run: |
          cd target/${{ matrix.target }}/release
          7z a ../../../${{ matrix.target }}.zip <binary_name>.exe
          
      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: ${{ matrix.target }}
          path: ${{ matrix.target }}.${{ matrix.archive }}

  release:
    name: Create Release
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Download artifacts
        uses: actions/download-artifact@v3
        
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            x86_64-unknown-linux-gnu/*
            x86_64-apple-darwin/*
            x86_64-pc-windows-msvc/*
            aarch64-unknown-linux-gnu/*
            aarch64-apple-darwin/*
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

---

## 发布到 crates.io

```yaml
# .github/workflows/publish.yml
name: Publish

on:
  push:
    tags:
      - 'v*'

jobs:
  publish:
    name: Publish to crates.io
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        
      - name: Publish
        run: cargo publish --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

---

## Docker 构建

```dockerfile
# Dockerfile
FROM rust:1.70 as builder

WORKDIR /app

# 复制依赖文件
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# 构建
RUN cargo build --release

# 运行时镜像
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/my-app /usr/local/bin/

CMD ["my-app"]
```

```yaml
# .github/workflows/docker.yml
name: Docker

on:
  push:
    branches: [main]
    tags: ['v*']

jobs:
  docker:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v2
        
      - name: Login to Docker Hub
        uses: docker/login-action@v2
        with:
          username: ${{ secrets.DOCKERHUB_USERNAME }}
          password: ${{ secrets.DOCKERHUB_TOKEN }}
          
      - name: Build and push
        uses: docker/build-push-action@v4
        with:
          push: true
          tags: user/my-app:latest
```

---

## 缓存优化

```yaml
# 高效缓存配置
- name: Cache cargo
  uses: Swatinem/rust-cache@v2
  with:
    key: ${{ matrix.target }}
    cache-on-failure: true
```

---

## 常用 Actions

| Action | 用途 |
|--------|------|
| `dtolnay/rust-toolchain` | 安装 Rust 工具链 |
| `Swatinem/rust-cache` | Rust 构建缓存 |
| `actions/cache` | 通用缓存 |
| `codecov/codecov-action` | 代码覆盖率上传 |
| `softprops/action-gh-release` | 创建 GitHub Release |
| `docker/build-push-action` | Docker 构建 |

---

*Last updated: 2026-03-18*
