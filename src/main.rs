//! rsklls CLI - Command-line interface

use std::process;

use rsklls::{init, Config, Result};

/// CLI argument parser
#[derive(Debug)]
struct Args {
    command: String,
    args: Vec<String>,
}

impl Args {
    /// Parse command line arguments
    fn parse() -> Self {
        let mut args = std::env::args().skip(1).collect::<Vec<_>>();
        
        let command = args.first().cloned().unwrap_or_else(|| "help".to_string());
        args.remove(0);
        
        Self {
            command,
            args,
        }
    }
}

/// Print usage information
fn print_usage() {
    println!("rsklls - Universal Rust Skills Framework");
    println!();
    println!("Usage:");
    println!("  rsklls <command> [args...]");
    println!();
    println!("Commands:");
    println!("  info          Show framework info");
    println!("  list          List available skills");
    println!("  exec <skill>  Execute a skill");
    println!("  config        Show current configuration");
    println!("  help          Show this help message");
    println!();
    println!("Examples:");
    println!("  rsklls info");
    println!("  rsklls list");
    println!("  rsklls exec greet --name World");
}

/// Show framework info
fn cmd_info() -> Result<()> {
    let info = rsklls::info();
    println!("{}", info);
    Ok(())
}

/// List skills
fn cmd_list() -> Result<()> {
    let config = init()?;
    println!("Configuration: {:?}", config.log_level);
    println!("No skills registered yet.");
    Ok(())
}

/// Execute a skill
fn cmd_exec(name: &str, args: &[String]) -> Result<()> {
    println!("Executing skill: {}", name);
    println!("Arguments: {:?}", args);
    Err(rsklls::Error::skill_not_found(name))
}

/// Show configuration
fn cmd_config() -> Result<()> {
    let config = Config::default();
    println!("Current configuration:");
    println!("  Log level: {}", config.log_level);
    println!("  Enable registry: {}", config.enable_registry);
    println!("  Enable executor: {}", config.enable_executor);
    println!("  Skill timeout: {}s", config.skill_timeout);
    println!("  Max concurrent: {}", config.max_concurrent);
    Ok(())
}

fn main() {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();
    
    // Parse arguments
    let args = Args::parse();
    
    // Execute command
    let result = match args.command.as_str() {
        "info" => cmd_info(),
        "list" => cmd_list(),
        "exec" => {
            if args.args.is_empty() {
                eprintln!("Error: exec requires a skill name");
                print_usage();
                process::exit(1);
            }
            let skill_name = &args.args[0];
            let skill_args = &args.args[1..];
            cmd_exec(skill_name, skill_args)
        }
        "config" => cmd_config(),
        "help" | "-h" | "--help" => {
            print_usage();
            Ok(())
        }
        _ => {
            eprintln!("Unknown command: {}", args.command);
            print_usage();
            process::exit(1);
        }
    };
    
    // Handle result
    match result {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
