use crate::{kubectl, display};
use crate::commands::{pods, resolve_context_pattern, resolve_namespace_pattern};

/// Handle the logs command - view logs from a pod
pub fn handle_logs_command(
    pod_pattern: &str,
    context_pattern: Option<&str>,
    namespace_pattern: Option<&str>,
    follow: bool,
    tail: Option<u32>,
    previous: bool,
    container: Option<&str>,
) {
    // Resolve context and namespace patterns
    let resolved_context = context_pattern.and_then(|pattern| resolve_context_pattern(pattern));
    let resolved_namespace = namespace_pattern.and_then(|pattern| resolve_namespace_pattern(pattern, resolved_context.as_deref()));
    
    // Find the pod using pattern matching
    if let Some(pod_name) = pods::select_pod(pod_pattern, resolved_context.as_deref(), resolved_namespace.as_deref()) {
        display::print_working(&format!("Getting logs for pod: {}", pod_name));
        
        // Build kubectl logs command
        let mut base_args = vec!["logs"];
        base_args.push(&pod_name);
        
        // Convert options to owned strings to manage lifetimes
        let mut additional_args = Vec::new();
        
        if follow {
            additional_args.push("-f".to_string());
        }
        
        if let Some(tail_lines) = tail {
            additional_args.push("--tail".to_string());
            additional_args.push(tail_lines.to_string());
        }
        
        if previous {
            additional_args.push("-p".to_string());
        }
        
        if let Some(container_name) = container {
            additional_args.push("-c".to_string());
            additional_args.push(container_name.to_string());
        }
        
        // Combine base args with additional args
        let mut all_args = base_args;
        let additional_refs: Vec<&str> = additional_args.iter().map(|s| s.as_str()).collect();
        all_args.extend(additional_refs);
        
        // Execute kubectl logs with context and namespace
        if !kubectl::execute_interactive_with_context(&all_args, resolved_context.as_deref(), resolved_namespace.as_deref()) {
            display::print_error_and_exit("Failed to get logs");
        }
    } else {
        display::print_error_and_exit(&format!("No pod found matching pattern: {}", pod_pattern));
    }
} 