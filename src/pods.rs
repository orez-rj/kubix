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
    
    match kubectl::execute_with_context(&["get", "pods", "-o", "wide"], context, namespace) {
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
    
    // First get all pods with names only
    match kubectl::execute_with_context(&["get", "pods", "-o", "name"], context, namespace) {
        Ok(output) => {
            let matching_pods: Vec<&str> = output
                .lines()
                .filter(|line| line.contains(pattern))
                .collect();
            
            if matching_pods.is_empty() {
                println!("No pods found matching pattern: '{}'", pattern);
                return;
            }
            
            println!("📋 Found {} pod(s) matching '{}':", matching_pods.len(), pattern);
            
            // Get detailed info for matching pods
            for (index, pod_line) in matching_pods.iter().enumerate() {
                let pod_name = pod_line.trim_start_matches("pod/").trim();
                match kubectl::execute_with_context(&["get", "pod", pod_name, "-o", "wide"], context, namespace) {
                    Ok(pod_output) => {
                        // Skip the header for subsequent pods, but show it for the first one
                        let lines: Vec<&str> = pod_output.lines().collect();
                        if index == 0 {
                            // Show header for first pod
                            for line in lines {
                                println!("{}", line);
                            }
                        } else {
                            // Skip header for subsequent pods
                            if lines.len() > 1 {
                                for line in lines.iter().skip(1) {
                                    println!("{}", line);
                                }
                            }
                        }
                    }
                    Err(error) => {
                        eprintln!("❌ Error getting details for pod {}: {}", pod_name, error);
                    }
                }
            }
        }
        Err(error) => {
            utils::print_error_and_exit(&format!("Error listing pods: {}", error));
        }
    }
}

/// Find a pod by pattern (fuzzy matching)
pub fn find_pod(pattern: &str, context: Option<&str>, namespace: Option<&str>) -> Option<String> {
    match kubectl::execute_with_context(&["get", "pods", "-o", "name"], context, namespace) {
        Ok(output) => {
            let matching_pod = output
                .lines()
                .find(|line| line.contains(pattern))
                .map(|line| line.trim_start_matches("pod/").trim().to_string());
            
            matching_pod
        }
        Err(error) => {
            eprintln!("❌ Error finding pods: {}", error);
            None
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
            eprintln!("❌ Error finding pods: {}", error);
            Vec::new()
        }
    }
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