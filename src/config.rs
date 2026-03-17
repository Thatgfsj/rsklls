//! Configuration for rsklls framework

use serde::{Deserialize, Serialize};

/// Framework configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Log level
    pub log_level: LogLevel,
    /// Enable skill registry
    pub enable_registry: bool,
    /// Enable executor
    pub enable_executor: bool,
    /// Skill timeout in seconds
    pub skill_timeout: u64,
    /// Max concurrent skills
    pub max_concurrent: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: LogLevel::Info,
            enable_registry: true,
            enable_executor: true,
            skill_timeout: 30,
            max_concurrent: 4,
        }
    }
}

impl Config {
    /// Create a new config with custom log level
    pub fn with_log_level(mut self, level: LogLevel) -> Self {
        self.log_level = level;
        self
    }
    
    /// Load config from file
    pub fn from_file(path: &str) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }
    
    /// Save config to file
    pub fn save(&self, path: &str) -> crate::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// Log level
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    #[default]
    Info,
    Debug,
    Warn,
    Error,
}

impl LogLevel {
    /// Convert to log crate level
    pub fn to_log_level(&self) -> log::LevelFilter {
        match self {
            LogLevel::Info => log::LevelFilter::Info,
            LogLevel::Debug => log::LevelFilter::Debug,
            LogLevel::Warn => log::LevelFilter::Warn,
            LogLevel::Error => log::LevelFilter::Error,
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "info"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}
