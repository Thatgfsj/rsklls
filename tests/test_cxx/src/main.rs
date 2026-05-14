#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn rust_add(a: i32, b: i32) -> i32;
        fn rust_multiply(a: i32, b: i32) -> i32;
        fn rust_fibonacci(n: u32) -> u64;
        fn rust_vector_sum(v: &[i64]) -> i64;
        fn rust_string_upper(s: &str) -> String;
    }
}

fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

fn rust_multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn rust_fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => {
            let mut a: u64 = 0;
            let mut b: u64 = 1;
            for _ in 2..=n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

fn rust_vector_sum(v: &[i64]) -> i64 {
    v.iter().sum()
}

fn rust_string_upper(s: &str) -> String {
    s.to_uppercase()
}

fn main() {
    println!("Rust FFI test");
    println!("  rust_add(3, 4) = {}", rust_add(3, 4));
    println!("  rust_multiply(3, 4) = {}", rust_multiply(3, 4));
    println!("  rust_fibonacci(10) = {}", rust_fibonacci(10));
    println!("  rust_vector_sum([1, 2, 3, 4, 5]) = {}", rust_vector_sum(&[1, 2, 3, 4, 5]));
    println!("  rust_string_upper(\"hello\") = {}", rust_string_upper("hello"));
}