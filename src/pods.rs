use crate::{kubectl, utils};

/// Handle the pods command - list all pods or filter by pattern
pub fn handle_pods_command(pattern: Option<&str>, context: Option<&str>, namespace: Option<&str>) {
    match pattern {
        None => list_pods(context, namespace),
        Some(pattern_str) => list_pods_with_pattern(pattern_str, context, namespace),
    }
}

/// List pods in the specified context and namespace
pub fn list_pods(context: Option<&str>, namespace: Option<&str>) {
    utils::print_working("Listing pods...");
    
    match kubectl::execute_with_context(&["get", "pods"], context, namespace) {
        Ok(output) => {
            println!("{}", output);
        }
        Err(error) => {
            utils::print_error_and_exit(&format!("Error listing pods: {}", error));
        }
    }
}

/// List pods filtered by pattern
pub fn list_pods_with_pattern(pattern: &str, context: Option<&str>, namespace: Option<&str>) {
    utils::print_working(&format!("Listing pods matching pattern '{}'...", pattern));
    
    match kubectl::execute_with_context(&["get", "pods"], context, namespace) {
        Ok(output) => {
            let lines: Vec<&str> = output.lines().collect();
            if lines.is_empty() {
                println!("No pods found");
                return;
            }
            
            // Find header line and matching pods
            let header = lines.first().unwrap();
            let matching_lines: Vec<&str> = lines
                .iter()
                .skip(1) // Skip header
                .filter(|line| line.contains(pattern))
                .copied()
                .collect();
            
            if matching_lines.is_empty() {
                println!("No pods found matching pattern: '{}'", pattern);
                return;
            }
            
            println!("📋 Found {} pod(s) matching '{}':", matching_lines.len(), pattern);
            
            // Print header and matching pods
            println!("{}", header);
            for line in matching_lines {
                println!("{}", line);
            }
        }
        Err(error) => {
            utils::print_error_and_exit(&format!("Error listing pods: {}", error));
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
            utils::print_error(&format!("Error finding pods: {}", error));
            Vec::new()
        }
    }
}

/// Select a pod by pattern with user interaction if multiple matches
pub fn select_pod(pattern: &str, context: Option<&str>, namespace: Option<&str>) -> Option<String> {
    let matching_pods = find_pods(pattern, context, namespace);
    utils::select_from_matches(matching_pods, pattern, "pod")
}

/// Get detailed information about a pod
pub fn get_pod_info(pod_name: &str, context: Option<&str>, namespace: Option<&str>) {
    match kubectl::execute_with_context(&["describe", "pod", pod_name], context, namespace) {
        Ok(output) => {
            println!("{}", output);
        }
        Err(error) => {
            utils::print_error_and_exit(&format!("Error getting pod info: {}", error));
        }
    }
} 