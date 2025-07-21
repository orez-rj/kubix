use crate::{kubectl, display};
use crate::commands::{pods, resolve_context_pattern, resolve_namespace_pattern};
use owo_colors::OwoColorize;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

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
        
        // Show enhanced header with pod information
        show_logs_header(&pod_name, container, resolved_context.as_deref(), resolved_namespace.as_deref(), follow);
        
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
        
        // Execute with visual enhancement wrapper
        if follow {
            execute_logs_with_line_numbers_streaming(&all_args, resolved_context.as_deref(), resolved_namespace.as_deref());
        } else {
            execute_logs_with_line_numbers(&all_args, resolved_context.as_deref(), resolved_namespace.as_deref());
        }
    } else {
        display::print_error_and_exit(&format!("No pod found matching pattern: {}", pod_pattern));
    }
}

/// Show enhanced header with pod and context information
fn show_logs_header(pod_name: &str, container: Option<&str>, context: Option<&str>, namespace: Option<&str>, follow: bool) {
    let header_line = "═".repeat(80);
    display::print_lines(&[
        "",
        &header_line.bright_blue().to_string(), 
        &format!("{} {}", "Logs for pod:".cyan().bold(), pod_name.bright_white().bold()),
    ]);
    
    // Additional info on same line or separate lines based on length
    let mut info_parts = Vec::new();
    
    if let Some(container_name) = container {
        info_parts.push(format!("🏷️  Container: {}", container_name.bright_white()));
    }
    
    if let Some(ctx) = context {
        info_parts.push(format!("🌐 Context: {}", ctx.bright_white()));
    }
    
    if let Some(ns) = namespace {
        info_parts.push(format!("📦 Namespace: {}", ns.bright_white()));
    }
    
    // Display additional info
    for info in info_parts {
        display::print_line(&info);
    }
    
    // Show mode and helpful tips
    if follow {
        display::print_lines(&[
            &format!("🔄 {} {}", "Mode:".cyan(), "Following (live)".bright_green().bold()),
            &format!("💡 {} {}", "Tip:".yellow(), "Press Ctrl+C to stop following".bright_black()),
        ]);
    } else {
        display::print_line(&format!("📄 {} {}", "Mode:".cyan(), "Static view".bright_blue()));
    }
    
    display::print_lines(&[
        &header_line.bright_blue().to_string(),
        "",
    ]);
}

/// Execute logs command with line numbers for static output
fn execute_logs_with_line_numbers(args: &[&str], context: Option<&str>, namespace: Option<&str>) {
    let kubectl_args = kubectl::build_args(args, context, namespace);
    let args_refs: Vec<&str> = kubectl_args.iter().map(|s| s.as_str()).collect();
    
    match kubectl::execute_kubectl(&args_refs) {
        Ok(output) => {
            for (line_num, line) in output.lines().enumerate() {
                print_line_with_number(line, line_num + 1);
            }
        }
        Err(error) => {
            display::print_error_and_exit(&format!("Failed to get logs: {}", error));
        }
    }
}

/// Execute logs command with streaming line numbers for follow mode
fn execute_logs_with_line_numbers_streaming(args: &[&str], context: Option<&str>, namespace: Option<&str>) {
    let kubectl_args = kubectl::build_args(args, context, namespace);
    
    let mut cmd = Command::new("kubectl");
    cmd.args(&kubectl_args);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    
    let mut child = cmd.spawn()
        .unwrap_or_else(|_| display::print_error_and_exit("Failed to start kubectl process"));
    
    let stdout = child.stdout.take()
        .unwrap_or_else(|| display::print_error_and_exit("Failed to capture kubectl stdout"));
    
    let reader = BufReader::new(stdout);
    let mut line_num = 1;
    
    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                print_line_with_number(&line_content, line_num);
                line_num += 1;
            }
            Err(_) => break,
        }
    }
    
    let _ = child.wait();
}

/// Print a line with simple visual differentiation
fn print_line_with_number(line: &str, line_num: usize) {
    // Simple alternating background for readability
    let line_prefix = format!("{:4} │ ", line_num);

    display::print_line(&format!("{}{}\n", line_prefix.cyan().bold(), line));
    
} 