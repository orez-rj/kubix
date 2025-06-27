use crate::{utils, kubectl};

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
        }
    };
    
    // Find matching contexts
    let matches: Vec<String> = contexts
        .into_iter()
        .filter(|context| context.contains(pattern))
        .collect();
    
    utils::select_from_matches(matches, pattern, "context")
}

/// List all available kubectl contexts and mark the current one
pub fn list_contexts_with_current() {
    println!("📋 Available kubectl contexts:");
    
    let current_context = get_current_context();
    
    match kubectl::execute_kubectl(&["config", "get-contexts", "-o", "name"]) {
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
    
    match kubectl::execute_kubectl(&["config", "get-contexts", "-o", "name"]) {
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
    
    match kubectl::execute_kubectl(&["config", "use-context", name]) {
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
    match kubectl::execute_kubectl(&["config", "current-context"]) {
        Ok(output) => Some(output.trim().to_string()),
        Err(_) => None,
    }
}

/// Get all available contexts as a vector
fn get_all_contexts() -> Result<Vec<String>, String> {
    match kubectl::execute_kubectl(&["config", "get-contexts", "-o", "name"]) {
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