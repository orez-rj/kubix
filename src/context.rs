use crate::utils;
use std::io::{self, Write};

/// Handle the ctx command - list contexts or switch to one by pattern
pub fn handle_ctx_command(name_pattern: Option<&str>) {
    match name_pattern {
        None => list_contexts_with_current(),
        Some(pattern) => switch_to_context_by_pattern(pattern),
    }
}

/// Resolve a context pattern to an exact context name
/// Returns None if user cancels, exits process if no matches found
pub fn resolve_context_pattern(pattern: &str) -> Option<String> {
    // Get all contexts
    let contexts = match get_all_contexts() {
        Ok(contexts) => contexts,
        Err(error) => {
            utils::print_error_and_exit(&format!("Error getting contexts: {}", error));
            return None;
        }
    };
    
    // Find matching contexts
    let matches: Vec<&String> = contexts
        .iter()
        .filter(|context| context.contains(pattern))
        .collect();
    
    match matches.len() {
        0 => {
            utils::print_error_and_exit(&format!("No contexts found matching pattern: '{}'", pattern));
            None
        }
        1 => {
            // Single match - return it
            Some(matches[0].clone())
        }
        _ => {
            // Multiple matches - let user choose
            println!("🔍 Found {} contexts matching '{}':", matches.len(), pattern);
            for (i, context) in matches.iter().enumerate() {
                println!("  {}. {}", i + 1, context);
            }
            
            let choice = prompt_user_choice(matches.len());
            choice.map(|index| matches[index].clone())
        }
    }
}

/// List all available kubectl contexts and mark the current one
pub fn list_contexts_with_current() {
    println!("📋 Available kubectl contexts:");
    
    let current_context = get_current_context();
    
    match utils::execute_kubectl(&["config", "get-contexts", "-o", "name"]) {
        Ok(output) => {
            for context in output.lines() {
                let context = context.trim();
                if !context.is_empty() {
                    if let Some(ref current) = current_context {
                        if context == current {
                            println!("  🔹 {} (current)", context);
                        } else {
                            println!("  • {}", context);
                        }
                    } else {
                        println!("  • {}", context);
                    }
                }
            }
        }
        Err(error) => {
            utils::print_error_and_exit(&format!("Error listing contexts: {}", error));
        }
    }
}

/// Switch to a context by pattern (with fuzzy matching and interactive selection)
pub fn switch_to_context_by_pattern(pattern: &str) {
    if let Some(resolved_context) = resolve_context_pattern(pattern) {
        use_context(&resolved_context);
    } else {
        println!("Operation cancelled.");
    }
}

/// List all available kubectl contexts
pub fn list_contexts() {
    println!("📋 Available kubectl contexts:");
    
    match utils::execute_kubectl(&["config", "get-contexts", "-o", "name"]) {
        Ok(output) => {
            for context in output.lines() {
                if !context.trim().is_empty() {
                    println!("  • {}", context.trim());
                }
            }
        }
        Err(error) => {
            utils::print_error_and_exit(&format!("Error listing contexts: {}", error));
        }
    }
}

/// Switch to a specific kubectl context
pub fn use_context(name: &str) {
    println!("🔄 Switching to context: {}", name);
    
    match utils::execute_kubectl(&["config", "use-context", name]) {
        Ok(_) => {
            utils::print_success(&format!("Successfully switched to context: {}", name));
        }
        Err(error) => {
            utils::print_error_and_exit(&format!("Error switching context: {}", error));
        }
    }
}

/// Get the current kubectl context
pub fn get_current_context() -> Option<String> {
    match utils::execute_kubectl(&["config", "current-context"]) {
        Ok(output) => Some(output.trim().to_string()),
        Err(_) => None,
    }
}

/// Get all available contexts as a vector
fn get_all_contexts() -> Result<Vec<String>, String> {
    match utils::execute_kubectl(&["config", "get-contexts", "-o", "name"]) {
        Ok(output) => {
            let contexts: Vec<String> = output
                .lines()
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())
                .collect();
            Ok(contexts)
        }
        Err(error) => Err(error),
    }
}

/// Prompt user to choose from multiple options
fn prompt_user_choice(max_options: usize) -> Option<usize> {
    print!("\nSelect context (1-{}, or 'q' to quit): ", max_options);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim();
            
            if input.eq_ignore_ascii_case("q") || input.eq_ignore_ascii_case("quit") {
                return None;
            }
            
            match input.parse::<usize>() {
                Ok(num) if num >= 1 && num <= max_options => Some(num - 1),
                _ => {
                    println!("❌ Invalid selection. Please enter a number between 1 and {} or 'q' to quit.", max_options);
                    prompt_user_choice(max_options) // Recursive call for retry
                }
            }
        }
        Err(_) => {
            println!("❌ Failed to read input.");
            None
        }
    }
} 