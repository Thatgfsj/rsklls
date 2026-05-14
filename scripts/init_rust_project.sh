#!/usr/bin/env bash
# init_rust_project.sh - Initialize a Rust FFI project with common dependencies
# Usage: ./init_rust_project.sh <project_name> <ffi_type>
# ffi_type: pyo3 | cxx | jni | wasm | ffi

set -e

PROJECT_NAME="${1:-my_rust_ffi}"
FFI_TYPE="${2:-pyo3}"

echo "Creating Rust project: $PROJECT_NAME with FFI type: $FFI_TYPE"

# Create project
cargo new "$PROJECT_NAME" --lib
cd "$PROJECT_NAME"

# Configure based on FFI type
case "$FFI_TYPE" in
    pyo3)
        cat >> Cargo.toml << 'EOF'

[lib]
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "=0.22.0", features = ["extension-module"] }
anyhow = "=1.0.86"
thiserror = "=1.0.60"
EOF

        cat > src/lib.rs << 'EOF'
use pyo3::prelude::*;

#[pyfunction]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[pymodule]
fn my_rust_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
EOF

        echo "PyO3 project created. Build with: maturin develop"
        ;;

    cxx)
        cat >> Cargo.toml << 'EOF'

[dependencies]
cxx = "=1.0.146"

[build-dependencies]
cxx-build = "=1.0.146"
EOF

        cat > src/lib.rs << 'EOF'
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn rust_process(data: &str) -> String;
    }
}

fn rust_process(data: &str) -> String {
    format!("Processed: {}", data)
}
EOF

        cat > build.rs << 'EOF'
fn main() {
    cxx_build::bridge("src/lib.rs").compile("cxx-demo");
}
EOF

        echo "cxx project created. Build with: cargo build"
        ;;

    jni)
        cat >> Cargo.toml << 'EOF'

[lib]
crate-type = ["cdylib"]

[dependencies]
jni = "=0.21.1"
anyhow = "=1.0.86"
EOF

        cat > src/lib.rs << 'EOF'
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jint, jstring};

#[no_mangle]
pub extern "system" fn Java_com_example_RustLib_add(
    _env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    a + b
}
EOF

        echo "JNI project created. Build with: cargo build --release"
        ;;

    wasm)
        cat >> Cargo.toml << 'EOF'

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "=0.2.92"
js-sys = "=0.3.69"

[profile.release]
opt-level = "s"
lto = true
EOF

        cat > src/lib.rs << 'EOF'
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
EOF

        echo "WASM project created. Build with: wasm-pack build --target web"
        ;;

    ffi)
        cat >> Cargo.toml << 'EOF'

[lib]
crate-type = ["staticlib", "cdylib"]

[dependencies]
libc = "=0.2.155"
anyhow = "=1.0.86"
EOF

        cat > src/lib.rs << 'EOF'
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn rust_greet(name: *const c_char) -> *mut c_char {
    // SAFETY: caller guarantees name is valid null-terminated string
    let name_str = unsafe { CStr::from_ptr(name).to_str().unwrap_or("World") };
    let greeting = format!("Hello, {}!", name_str);
    CString::new(greeting).unwrap().into_raw()
}
EOF

        echo "FFI project created. Build with: cargo build --release"
        ;;

    *)
        echo "Unknown FFI type: $FFI_TYPE"
        echo "Valid types: pyo3 | cxx | jni | wasm | ffi"
        exit 1
        ;;
esac

echo ""
echo "Project initialized: $PROJECT_NAME"
echo "Next steps:"
echo "  cd $PROJECT_NAME"
echo "  cargo build"