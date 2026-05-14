# PyO3 完整代码示例

## 项目配置

### Cargo.toml
```toml
[package]
name = "my_rust_module"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "=0.22.0", features = ["extension-module"] }
anyhow = "=1.0.86"
thiserror = "=1.0.60"

[build-dependencies]
maturin = "=1.7.4"
```

### maturin.toml (可选)
```toml
[build]
args = ["--release"]

[target.x86_64-apple-darwin]
args = ["--release", "--target", "x86_64-apple-darwin"]

[target.x86_64-pc-windows-msvc]
args = ["--release"]
```

---

## 基础函数导出

```rust
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

/// 简单函数
#[pyfunction]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// 带错误处理
#[pyfunction]
fn divide(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        Err(pyo3::exceptions::PyZeroDivisionError::new_err("division by zero"))
    } else {
        Ok(a / b)
    }
}

/// 使用 anyhow 错误
#[pyfunction]
fn read_file(path: &str) -> PyResult<String> {
    Ok(anyhow::Context::context(
        std::fs::read_to_string(path),
        || format!("Failed to read file: {}", path),
    )?)
}
```

---

## 类导出

```rust
#[pyclass]
struct Point {
    #[pyo3(get, set)]
    x: f64,
    #[pyo3(get, set)]
    y: f64,
}

#[pymethods]
impl Point {
    #[new]
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    fn distance(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn __repr__(&self) -> String {
        format!("Point({}, {})", self.x, self.y)
    }

    #[staticmethod]
    fn origin() -> Self {
        Point { x: 0.0, y: 0.0 }
    }

    #[classmethod]
    fn from_tuple(cls: &PyType, tuple: (f64, f64)) -> PyResult<Self> {
        Ok(Point { x: tuple.0, y: tuple.1 })
    }
}
```

---

## Python 类型处理

```rust
/// 处理字典
#[pyfunction]
fn sum_dict_values(dict: &PyDict) -> PyResult<i64> {
    let mut sum = 0i64;
    for (key, value) in dict.iter() {
        if let Ok(num) = value.extract::<i64>() {
            sum += num;
        }
    }
    Ok(sum)
}

/// 处理列表
#[pyfunction]
fn process_list(list: &PyList) -> PyResult<Vec<String>> {
    list.iter()
        .map(|item| item.extract::<String>())
        .collect()
}

/// 返回 Python 对象
#[pyfunction]
fn create_nested_dict(py: Python) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    dict.set_item("nested", {
        let inner = PyDict::new(py);
        inner.set_item("key", "value")?;
        inner
    })?;
    Ok(dict.into())
}

/// 接受任意 Python 对象
#[pyfunction]
fn inspect_object(obj: &PyAny) -> PyResult<String> {
    let type_name = obj.get_type().name()?;
    Ok(format!("Type: {}", type_name))
}
```

---

## 模块定义

```rust
#[pymodule]
fn my_rust_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(divide, m)?)?;
    m.add_function(wrap_pyfunction!(read_file, m)?)?;
    m.add_function(wrap_pyfunction!(sum_dict_values, m)?)?;
    m.add_function(wrap_pyfunction!(process_list, m)?)?;
    m.add_function(wrap_pyfunction!(create_nested_dict, m)?)?;
    m.add_function(wrap_pyfunction!(inspect_object, m)?)?;
    m.add_class::<Point>()?;
    Ok(())
}
```

---

## 初始化函数 (可选)

用于复杂初始化（如加载配置、连接数据库）：

```rust
#[pymodule]
fn my_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // 模块级初始化
    println!("模块加载中...");
    Ok(())
}
```

---

## Python 调用示例

```python
from my_rust_module import add, divide, Point, sum_dict_values

# 基础函数
print(add(1, 2))  # 3
print(divide(10, 2))  # 5.0
print(divide(10, 0))  # Raises ZeroDivisionError

# 类
p = Point(3.0, 4.0)
print(p.distance())  # 5.0
print(Point.origin())  # Point(0.0, 0.0)

# 复杂类型
result = sum_dict_values({"a": 1, "b": 2, "c": 3})
print(result)  # 6
```

---

## 构建命令

```bash
# 开发模式 - 自动安装到当前 Python 环境
maturin develop

# 指定 Python 版本
maturin develop --interpreter python3.11

# 发布构建
maturin build --release

# 生成特定平台的 wheel
maturin build --release --target x86_64-apple-darwin
maturin build --release --target x86_64-pc-windows-msvc

# 使用 existingIRTUALENV
maturin develop -m .venv
```

---

## 常见问题

### 导入失败
```bash
# 检查架构匹配
python -c "import platform; print(platform.machine())"
rustc --print cfg | grep target_arch

# 检查 Python 版本
python --version
```

### 类型转换错误
```rust
// 使用 extract 代替直接转换
let num: i64 = value.extract()?;  // 正确
// let num: i64 = value.extract::<i64>()?;  // 同样正确
```

### GIL 释放 (用于 CPU 密集型任务)
```rust
#[pyfunction]
fn cpu_intensive() -> PyResult<()> {
    let gil = Python::acquire_gil();
    // 在 GIL 下执行
    drop(gil);  // 释放 GIL
    
    // 长时间计算
    let result = compute_heavy();
    
    Ok(result)
}
```
