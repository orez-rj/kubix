use crate::{kubectl, utils, display};

/// Handle the pods command - list all pods or filter by pattern
pub fn handle_pods_command(pattern: Option<&str>, context: Option<&str>, namespace: Option<&str>) {
    match pattern {
        None => display::print_working("Listing pods..."),
        Some(p) => display::print_working(&format!("Listing pods matching pattern '{}'...", p)),
    }
    list_pods(pattern, context, namespace);
}

/// List pods in the specified context and namespace, optionally filtered by pattern
pub fn list_pods(pattern: Option<&str>, context: Option<&str>, namespace: Option<&str>) {
    match kubectl::execute_with_context(&["get", "pods"], context, namespace) {
        Ok(output) => {
            display::print_pods_table(&output, pattern);
        }
        Err(error) => {
            display::print_error_and_exit(&format!("Error listing pods: {}", error));
        }
    }
}

/// Find all pods matching a pattern
pub fn find_pods(pattern: &str, context: Option<&str>, namespace: Option<&str>) -> Vec<String> {
    match kubectl::execute_with_context(&["get", "pods", "-o", "name"], context, namespace) {
        Ok(output) => {
            output
                .lines()
                .filter(|line| line.contains(pattern))
                .map(|line| line.trim_start_matches("pod/").trim().to_string())
                .collect()
        }
        Err(error) => {
            display::print_error(&format!("Error finding pods: {}", error));
            Vec::new()
        }
    }
}

/// Select a pod by pattern with user interaction if multiple matches
pub fn select_pod(pattern: &str, context: Option<&str>, namespace: Option<&str>) -> Option<String> {
    let matching_pods = find_pods(pattern, context, namespace);
    utils::select_from_matches(matching_pods, pattern, "pod")
}
 