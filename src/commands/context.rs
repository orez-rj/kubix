use crate::{utils, kubectl, display};

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
    display::print_working(&format!("Resolving context with pattern {}...", pattern));
    // Get all contexts
    let contexts = match get_all_contexts() {
        Ok(contexts) => contexts,
        Err(error) => {
            display::print_error_and_exit(&format!("Error getting contexts: {}", error));
        }
    };
    
    // Find matching contexts
    let matches: Vec<String> = contexts
        .into_iter()
        .filter(|context| context.contains(pattern))
        .collect();
    
    let resolved_context = utils::select_from_matches(matches, pattern, "context");
    if let Some(context) = &resolved_context {
        display::print_working(&format!("Using context: {}", context));
    }
    resolved_context
}

/// List all available kubectl contexts and mark the current one
pub fn list_contexts_with_current() {
    let current_context = get_current_context();
    
    display::print_working("Listing contexts...");
    match kubectl::execute_kubectl(&["config", "get-contexts", "-o", "name"]) {
        Ok(output) => {
            display::print_contexts_table(&output, current_context.as_deref());
        }
        Err(error) => {
            display::print_error_and_exit(&format!("Error listing contexts: {}", error));
        }
    }
}

/// Switch to a context by pattern (with fuzzy matching and interactive selection)
pub fn switch_to_context_by_pattern(pattern: &str) {
    if let Some(resolved_context) = resolve_context_pattern(pattern) {
        use_context(&resolved_context);
    } else {
        display::print_error("Operation cancelled.");
    }
}

/// Switch to a specific kubectl context
pub fn use_context(name: &str) {    
    match kubectl::execute_kubectl(&["config", "use-context", name]) {
        Ok(_) => {
            display::print_success(&format!("Successfully switched to context: {}", name));
        }
        Err(error) => {
            display::print_error_and_exit(&format!("Error switching context: {}", error));
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