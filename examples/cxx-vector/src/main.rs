use std::collections::HashMap;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn vector_sum(v: &[i64]) -> i64;
        fn vector_product(v: &[f64]) -> f64;
        fn vector_sort(v: &mut Vec<i32>);
        fn string_to_chars(s: &str) -> Vec<u8>;
        fn char_to_string(chars: Vec<u8>) -> String;
        fn map_operation(key: &str, value: i64) -> String;
    }
}

fn vector_sum(v: &[i64]) -> i64 {
    v.iter().sum()
}

fn vector_product(v: &[f64]) -> f64 {
    v.iter().fold(1.0, |acc, x| acc * x)
}

fn vector_sort(v: &mut Vec<i32>) {
    v.sort();
}

fn string_to_chars(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

fn char_to_string(chars: Vec<u8>) -> String {
    String::from_utf8_lossy(&chars).to_string()
}

fn map_operation(key: &str, value: i64) -> String {
    format!("{} -> {}", key, value)
}

fn main() {
    println!("=== CXX Vector Demo ===");

    // Vector operations
    let nums = vec![1_i64, 2, 3, 4, 5];
    println!("Sum of [1,2,3,4,5] = {}", vector_sum(&nums));

    let floats = vec![1.5, 2.0, 3.0];
    println!("Product of [1.5,2.0,3.0] = {}", vector_product(&floats));

    // Sort
    let mut sorted = vec![3, 1, 4, 1, 5, 9, 2, 6];
    vector_sort(&mut sorted);
    println!("Sorted [3,1,4,1,5,9,2,6] = {:?}", sorted);

    // String operations
    let chars = string_to_chars("Hello");
    println!("Chars of 'Hello' = {:?}", chars);
    println!("String from chars = {}", char_to_string(chars));

    // Map operation
    println!("Map op: {}", map_operation("answer", 42));
}