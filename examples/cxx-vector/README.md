# CXX Vector Example

Demonstrates Rust-C++ interop via cxx, showing how vectors and strings are passed.

## Building

```bash
# Build Rust library
cargo build --release

# C++ headers are generated in target/debug/build/*-out/
```

## C++ Usage

```cpp
#include "cxx-vector/cxx-vector.h"
#include <iostream>
#include <vector>

int main() {
    // Vector sum
    std::vector<int64_t> nums = {1, 2, 3, 4, 5};
    int64_t sum = vector_sum(nums);
    std::cout << "Sum: " << sum << std::endl;

    // Vector sort
    std::vector<int32_t> sorted = {3, 1, 4, 1, 5, 9, 2, 6};
    vector_sort(sorted);
    // sorted is now {1, 1, 2, 3, 4, 5, 6, 9}

    // String conversion
    auto chars = string_to_chars("Hello");
    auto str = char_to_string(chars);
}
```

## Key Features

- `Vec<T>` ↔ `std::vector<T>` automatic conversion
- `String` ↔ `std::string` automatic conversion
- `&str` ↔ `std::string_view` conversion
- Zero-copy passing where possible