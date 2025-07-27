use crate::{display, utils, kubectl};

/// Resolve a namespace pattern to an exact namespace name
/// Returns None if user cancels, exits process if no matches found
pub fn resolve_namespace_pattern(pattern: &str, context: Option<&str>) -> Option<String> {
    display::print_working(&format!("Resolving namespace with pattern {}...", pattern));
    // Get all namespaces
    let namespaces = match get_all_namespaces(context) {
        Ok(namespaces) => namespaces,
        Err(error) => {
            display::print_error_and_exit(&format!("Error getting namespaces: {}", error));
        }
    };
    
    // Find matching namespaces
    let matches: Vec<String> = namespaces
        .into_iter()
        .filter(|namespace| namespace.contains(pattern))
        .collect();
    
    let resolved_namespace = utils::select_from_matches(matches, pattern, "namespace");
    if let Some(namespace) = &resolved_namespace {
        display::print_working(&format!("Using namespace: {}", namespace));
    }
    resolved_namespace
}

/// Get all available namespaces as a vector
fn get_all_namespaces(context: Option<&str>) -> Result<Vec<String>, String> {
    let args = vec!["get", "namespaces", "-o", "name"];
    let mut cmd_args = Vec::new();
    
    if let Some(ctx) = context {
        cmd_args.extend(&["--context", ctx]);
    }
    cmd_args.extend(&args);
    
    match kubectl::execute_kubectl(&cmd_args) {
        Ok(output) => {
            let namespaces: Vec<String> = output
                .lines()
                .map(|line| line.trim_start_matches("namespace/").trim().to_string())
                .filter(|line| !line.is_empty())
                .collect();
            Ok(namespaces)
        }
        Err(error) => Err(error),
    }
} 