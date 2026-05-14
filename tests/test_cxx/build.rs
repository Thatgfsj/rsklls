fn main() {
    cxx_build::bridge("src/main.rs")
        .std("c++17")
        .compile("test_cxx");

    println!("cargo:rerun-if-changed=src/main.rs");
}