//! cxx bindings for rsklls

#[cfg(feature = "cxx")]
pub mod bindings {
    cxx::bridge! {
        unsafe extern "Rust" {
            fn rsklls_version() -> String;
            fn rsklls_name() -> String;
        }
    }
}

/// Get version string
#[cfg(feature = "cxx")]
pub fn rsklls_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Get framework name
#[cfg(feature = "cxx")]
pub fn rsklls_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}
