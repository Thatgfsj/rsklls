//! Skill module - core capability definition and execution

pub mod executor;
pub mod registry;

pub use executor::Executor;
pub use registry::{Registry, SkillInput, SkillOutput, Skill};

use serde::{Deserialize, Serialize};

/// Input data for skill execution
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillInput {
    /// Input data as key-value pairs
    pub data: std::collections::HashMap<String, String>,
    /// Positional arguments
    pub args: Vec<String>,
    /// Named arguments
    #[serde(default)]
    pub kwargs: std::collections::HashMap<String, String>,
}

impl SkillInput {
    /// Create new input with data
    pub fn new(data: std::collections::HashMap<String, String>) -> Self {
        Self {
            data,
            args: vec![],
            kwargs: std::collections::HashMap::new(),
        }
    }
    
    /// Create from a single string
    pub fn from_string(s: impl Into<String>) -> Self {
        let mut data = std::collections::HashMap::new();
        data.insert("input".to_string(), s.into());
        Self::new(data)
    }
    
    /// Get argument by index
    pub fn arg(&self, index: usize) -> Option<&str> {
        self.args.get(index).map(|s| s.as_str())
    }
    
    /// Get named argument
    pub fn kwarg(&self, key: &str) -> Option<&str> {
        self.kwargs.get(key).map(|s| s.as_str())
    }
}

/// Output data from skill execution
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillOutput {
    /// Output data as key-value pairs
    pub data: std::collections::HashMap<String, String>,
    /// Success flag
    pub success: bool,
    /// Error message if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl SkillOutput {
    /// Create successful output
    pub fn success(data: std::collections::HashMap<String, String>) -> Self {
        Self {
            data,
            success: true,
            error: None,
        }
    }
    
    /// Create successful output from string
    pub fn from_string(s: impl Into<String>) -> Self {
        let mut data = std::collections::HashMap::new();
        data.insert("output".to_string(), s.into());
        Self::success(data)
    }
    
    /// Create error output
    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            data: std::collections::HashMap::new(),
            success: false,
            error: Some(msg.into()),
        }
    }
}

/// Skill trait - define custom skills
pub trait SkillTrait: Send + Sync {
    /// Execute the skill with input
    fn execute(&self, input: &SkillInput) -> crate::Result<SkillOutput>;
    
    /// Get skill name
    fn name(&self) -> &str;
    
    /// Get skill description
    fn description(&self) -> &str;
}

/// Metadata for a skill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMeta {
    pub name: String,
    pub description: String,
    pub version: String,
    pub tags: Vec<String>,
}

impl SkillMeta {
    /// Create new metadata
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            version: "0.1.0".to_string(),
            tags: vec![],
        }
    }
    
    /// Add a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
    
    /// Set version
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }
}

/// Macro to define a skill function
#[macro_export]
macro_rules! skill {
    (fn $name:ident($($arg:ident: $ty:ty),*) -> $ret:ty $body:block) => {
        #[allow(non_snake_case)]
        pub fn $name($($arg: $ty),*) -> $ret $body
    };
}
