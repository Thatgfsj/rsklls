//! rsklls - Universal Rust Skills Framework
//!
//! A lightweight framework for building reusable Rust capabilities
//! that can be integrated into various hosts (AI Agents, CLI, GUI, etc.)

pub mod config;
pub mod error;
pub mod skill;

#[cfg(feature = "pyo3")]
pub mod pyo3_bindings;

#[cfg(feature = "cxx")]
pub mod cxx_bindings;

#[cfg(feature = "gui")]
pub mod gui;

pub use config::Config;
pub use error::{Error, Result};
pub use skill::{Executor, Registry, Skill};

// Re-export commonly used types
pub use anyhow::Result as AnyhowResult;

/// Framework version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Framework name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Initialize the framework with default configuration
pub fn init() -> Result<Config> {
    log::info!("Initializing {} v{}", NAME, VERSION);
    Config::default()
}

/// Get framework info
pub fn info() -> FrameworkInfo {
    FrameworkInfo {
        name: NAME,
        version: VERSION,
    }
}

/// Framework information
#[derive(Debug, Clone)]
pub struct FrameworkInfo {
    pub name: &'static str,
    pub version: &'static str,
}

use std::fmt;

impl fmt::Display for FrameworkInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} v{}", self.name, self.version)
    }
}
