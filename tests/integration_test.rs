//! Integration tests for rsklls framework

use rsklls::{
    config::Config,
    error::Error,
    skill::{Executor, Registry, SkillInput, SkillOutput, SkillTrait},
};

/// Test skill implementation
struct TestSkill;

impl SkillTrait for TestSkill {
    fn execute(&self, input: &SkillInput) -> Result<SkillOutput, Error> {
        let result = input
            .arg(0)
            .map(|s| format!("Hello, {}!", s))
            .unwrap_or_else(|| "Hello, World!".to_string());
        
        Ok(SkillOutput::from_string(result))
    }
    
    fn name(&self) -> &str {
        "test_skill"
    }
    
    fn description(&self) -> &str {
        "A test skill for integration tests"
    }
}

#[test]
fn test_registry_register() {
    let mut registry = Registry::new();
    let skill = TestSkill;
    
    let result = registry.register(skill);
    assert!(result.is_ok());
}

#[test]
fn test_registry_duplicate() {
    let mut registry = Registry::new();
    let skill = TestSkill;
    
    registry.register(skill).unwrap();
    
    // Duplicate registration should fail
    let skill2 = TestSkill;
    let result = registry.register(skill2);
    assert!(result.is_err());
}

#[test]
fn test_registry_get() {
    let mut registry = Registry::new();
    let skill = TestSkill;
    
    registry.register(skill).unwrap();
    
    let retrieved = registry.get("test_skill");
    assert!(retrieved.is_some());
}

#[test]
fn test_registry_list() {
    let mut registry = Registry::new();
    let skill = TestSkill;
    
    registry.register(skill).unwrap();
    
    let skills = registry.list();
    assert_eq!(skills.len(), 1);
    assert_eq!(skills[0], "test_skill");
}

#[test]
fn test_executor_valid_input() {
    let executor = Executor::new();
    let skill = TestSkill;
    
    let mut input = SkillInput::default();
    input.args.push("Test".to_string());
    
    let result = executor.execute(&skill, &input);
    assert!(result.is_ok());
    
    let output = result.unwrap();
    assert!(output.success);
}

#[test]
fn test_config_default() {
    let config = Config::default();
    
    assert_eq!(config.skill_timeout, 30);
    assert_eq!(config.max_concurrent, 4);
    assert!(config.enable_registry);
    assert!(config.enable_executor);
}

#[test]
fn test_config_with_log_level() {
    use rsklls::config::LogLevel;
    
    let config = Config::default().with_log_level(LogLevel::Debug);
    
    assert_eq!(config.log_level, LogLevel::Debug);
}

#[test]
fn test_framework_info() {
    let info = rsklls::info();
    
    assert_eq!(info.name, "rsklls");
}
