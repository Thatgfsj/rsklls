# PyO3 Calculator Example

A simple calculator module demonstrating Rust-Python interop via PyO3.

## Building

```bash
# Development mode (installs to current Python)
maturin develop

# Release build
maturin build --release
```

## Usage

```python
from pyo3_calc import basic_add, basic_divide, Calculator

# Simple functions
print(basic_add(3.0, 4.0))  # 7.0
print(basic_divide(10.0, 2.0))  # 5.0

# Class-based calculator
calc = Calculator(10.0)
calc.add(5.0)      # 15.0
calc.multiply(2.0) # 30.0
calc.divide(3.0)   # 10.0
print(calc)        # Calculator(value=10.0)
calc.reset()
print(calc.value)  # 0.0

# Error handling
try:
    calc.divide(0.0)
except ZeroDivisionError as e:
    print(f"Caught: {e}")
```

## Files

- `Cargo.toml` - Project configuration with PyO3
- `src/lib.rs` - Module implementation
- `build.sh` - Build script