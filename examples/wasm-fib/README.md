# WASM Fibonacci Example

A demonstration of compiling Rust to WebAssembly for use in browsers or Node.js.

## Building

```bash
# Install wasm-pack if needed
cargo install wasm-pack

# Build for web
wasm-pack build --target web --release

# Build for Node.js
wasm-pack build --target nodejs --release
```

## Usage in Browser

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Rust WASM Demo</title>
</head>
<body>
    <script type="module">
        import init, { fibonacci, factorial, is_prime, gcd, Matrix } from './pkg/wasm_fib.js';

        async function run() {
            await init();

            console.log("fibonacci(10) =", fibonacci(10));
            console.log("factorial(10) =", factorial(10));
            console.log("is_prime(17) =", is_prime(17));
            console.log("gcd(48, 18) =", gcd(48, 18));

            const m = new Matrix(2, 3, 0);
            m.set(0, 0, 1);
            m.set(0, 1, 2);
            m.set(0, 2, 3);
            m.set(1, 0, 4);
            m.set(1, 1, 5);
            m.set(1, 2, 6);

            console.log("Matrix 2x3:", m.rows, "x", m.cols);
            console.log("Transposed:", m.transpose().rows, "x", m.transpose().cols);
        }

        run();
    </script>
</body>
</html>
```

## Usage in Node.js

```javascript
const { fibonacci, factorial, is_prime } = require('./pkg/wasm_fib_bg.wasm');

console.log(fibonacci(10));  // 55
console.log(factorial(10)); // 3628800
console.log(is_prime(17));  // true
```

## Files

- `Cargo.toml` - WASM-bindgen configuration
- `src/lib.rs` - Module with fibonacci, matrix operations
- `build.sh` - Build script