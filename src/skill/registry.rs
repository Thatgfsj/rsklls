//! Skill registry - manages registered skills

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::{Error, Result, SkillMeta, SkillOutput, SkillTrait, SkillInput};

/// Skill registry - stores and manages skills
#[derive(Default)]
pub struct Registry {
    skills: HashMap<String, Arc<dyn SkillTrait>>,
    metadata: HashMap<String, SkillMeta>,
}

impl Registry {
    /// Create a new registry
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Register a skill
    pub fn register<S: SkillTrait + 'static>(&mut self, skill: S) -> Result<()> {
        let name = skill.name().to_string();
        
        if self.skills.contains_key(&name) {
            return Err(Error::skill_not_found(format!(
                "Skill '{}' already registered",
                name
            )));
        }
        
        let meta = SkillMeta::new(&name, skill.description());
        self.metadata.insert(name.clone(), meta);
        self.skills.insert(name, Arc::new(skill));
        
        Ok(())
    }
    
    /// Get a skill by name
    pub fn get(&self, name: &str) -> Option<Arc<dyn SkillTrait>> {
        self.skills.get(name).cloned()
    }
    
    /// Check if skill exists
    pub fn contains(&self, name: &str) -> bool {
        self.skills.contains_key(name)
    }
    
    /// List all skill names
    pub fn list(&self) -> Vec<String> {
        self.skills.keys().cloned().collect()
    }
    
    /// Get skill metadata
    pub fn metadata(&self, name: &str) -> Option<&SkillMeta> {
        self.metadata.get(name)
    }
    
    /// Get all metadata
    pub fn all_metadata(&self) -> Vec<&SkillMeta> {
        self.metadata.values().collect()
    }
    
    /// Unregister a skill
    pub fn unregister(&mut self, name: &str) -> Result<()> {
        if self.skills.remove(name).is_none() {
            return Err(Error::skill_not_found(name));
        }
        self.metadata.remove(name);
        Ok(())
    }
    
    /// Clear all skills
    pub fn clear(&mut self) {
        self.skills.clear();
        self.metadata.clear();
    }
}

/// Thread-safe registry
pub type SharedRegistry = Arc<RwLock<Registry>>;

/// Create a new shared registry
pub fn new_registry() -> SharedRegistry {
    Arc::new(RwLock::new(Registry::new()))
}
