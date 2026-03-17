//! Error types for rsklls framework

use thiserror::Error;

/// Framework error type
#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Skill not found: {0}")]
    SkillNotFound(String),
    
    #[error("Execution error: {0}")]
    Execution(String),
    
    #[error("FFI error: {0}")]
    Ffi(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Create a configuration error
    pub fn config(msg: impl Into<String>) -> Self {
        Error::Config(msg.into())
    }
    
    /// Create a skill not found error
    pub fn skill_not_found(name: impl Into<String>) -> Self {
        Error::SkillNotFound(name.into())
    }
    
    /// Create an execution error
    pub fn execution(msg: impl Into<String>) -> Self {
        Error::Execution(msg.into())
    }
    
    /// Create an FFI error
    pub fn ffi(msg: impl Into<String>) -> Self {
        Error::Ffi(msg.into())
    }
    
    /// Create an invalid input error
    pub fn invalid_input(msg: impl Into<String>) -> Self {
        Error::InvalidInput(msg.into())
    }
}
