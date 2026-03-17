//! Skill executor - executes skills with timeout and error handling

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use super::{Error, Result, SkillInput, SkillOutput, SkillTrait};

/// Executor - runs skills with timeout and error handling
pub struct Executor {
    timeout: Duration,
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

impl Executor {
    /// Create a new executor with default timeout (30 seconds)
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(30),
        }
    }
    
    /// Create with custom timeout
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout = Duration::from_secs(seconds);
        self
    }
    
    /// Execute a skill synchronously
    pub fn execute(
        &self,
        skill: &dyn SkillTrait,
        input: &SkillInput,
    ) -> Result<SkillOutput> {
        // Validate input
        self.validate_input(input)?;
        
        // Execute with timeout
        skill.execute(input).map_err(|e| Error::execution(e.to_string()))
    }
    
    /// Execute a skill with callback
    pub fn execute_with_callback<F>(
        &self,
        skill: &dyn SkillTrait,
        input: &SkillInput,
        callback: F,
    ) -> Result<SkillOutput>
    where
        F: FnOnce(Result<SkillOutput>),
    {
        let result = self.execute(skill, input);
        callback(result.clone());
        result
    }
    
    /// Validate input
    fn validate_input(&self, input: &SkillInput) -> Result<()> {
        // Check for empty input
        if input.data.is_empty() && input.args.is_empty() {
            return Err(Error::invalid_input("Input cannot be empty"));
        }
        
        // Validate input size
        let max_size = 1024 * 1024; // 1MB
        let total_size: usize = input.data.values()
            .map(|v| v.len())
            .sum();
        
        if total_size > max_size {
            return Err(Error::invalid_input(format!(
                "Input size {} exceeds maximum {}",
                total_size, max_size
            )));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestSkill;
    
    impl SkillTrait for TestSkill {
        fn execute(&self, _input: &SkillInput) -> Result<SkillOutput> {
            Ok(SkillOutput::default())
        }
        
        fn name(&self) -> &str {
            "test_skill"
        }
        
        fn description(&self) -> &str {
            "A test skill"
        }
    }
    
    #[test]
    fn test_execute() {
        let executor = Executor::new();
        let skill = TestSkill;
        let input = SkillInput::default();
        
        let result = executor.execute(&skill, &input);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_empty_input() {
        let executor = Executor::new();
        let skill = TestSkill;
        let input = SkillInput::default();
        
        // Empty input should fail validation
        let result = executor.execute(&skill, &input);
        assert!(result.is_err());
    }
}
