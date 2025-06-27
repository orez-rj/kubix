use crate::{kubectl, pods, display, config};
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

/// Handle the unified exec command
pub fn handle_exec_command(
    pod_pattern: &str, 
    command: Option<&str>,
    script: Option<&str>, 
    context: Option<&str>, 
    namespace: Option<&str>
) {
    // Load configuration for command/script resolution
    let config = config::KubixConfig::load();
    
    match (command, script) {
        (Some(cmd), None) => {
            // Execute a command
            let resolved_command = config.resolve_command(cmd);
            run_command_on_pod(pod_pattern, &resolved_command, context, namespace);
        }
        (None, Some(script_input)) => {
            // Execute a script
            let resolved_script = config.resolve_script(script_input);
            exec_script_on_pod(pod_pattern, &resolved_script, context, namespace);
        }
        (None, None) => {
            // Default to bash
            bash_to_pod(pod_pattern, context, namespace);
        }
        (Some(_), Some(_)) => {
            // This should be prevented by clap's argument group, but handle it gracefully
            display::print_error_styled("Cannot specify both command and script. Use either --command or --script, not both.");
            std::process::exit(1);
        }
    }
}

/// Open a bash shell session to a pod
pub fn bash_to_pod(pod_pattern: &str, context: Option<&str>, namespace: Option<&str>) {
    if let Some(pod_name) = pods::select_pod(pod_pattern, context, namespace) {
        display::print_working_styled(&format!("Opening bash session to pod: {}", pod_name));
        
        let base_args = vec!["exec", "-it", &pod_name, "--", "bash"];
        
        if !kubectl::execute_interactive_with_context(&base_args, context, namespace) {
            display::print_error_styled("Failed to open bash session");
            std::process::exit(1);
        }
    } else {
        display::print_error_styled(&format!("No pod found matching pattern: {}", pod_pattern));
        std::process::exit(1);
    }
}

/// Run a command on a pod
pub fn run_command_on_pod(
    pod_pattern: &str, 
    command: &str, 
    context: Option<&str>, 
    namespace: Option<&str>
) {
    if let Some(pod_name) = pods::select_pod(pod_pattern, context, namespace) {
        display::print_working_styled(&format!("Running command '{}' on pod: {}", command, pod_name));
        
        let base_args = vec!["exec", "-it", &pod_name, "--", "sh", "-c", command];
        
        if !kubectl::execute_interactive_with_context(&base_args, context, namespace) {
            display::print_error_styled("Failed to run command");
            std::process::exit(1);
        }
    } else {
        display::print_error_styled(&format!("No pod found matching pattern: {}", pod_pattern));
        std::process::exit(1);
    }
}

/// Execute a local script on a pod
pub fn exec_script_on_pod(
    pod_pattern: &str, 
    script_path: &str, 
    context: Option<&str>, 
    namespace: Option<&str>
) {
    if let Some(pod_name) = pods::select_pod(pod_pattern, context, namespace) {
        display::print_working_styled(&format!("Executing script '{}' on pod: {}", script_path, pod_name));
        
        // Read the script content
        let script_content = fs::read_to_string(script_path)
            .unwrap_or_else(|_| {
                display::print_error_styled(&format!("Failed to read script file: {}", script_path));
                std::process::exit(1);
            });
        
        // Build kubectl command
        let mut cmd = Command::new("kubectl");
        
        if let Some(ctx) = context {
            cmd.args(&["--context", ctx]);
        }
        
        if let Some(ns) = namespace {
            cmd.args(&["-n", ns]);
        }
        
        cmd.args(&["exec", "-i", &pod_name, "--", "sh"])
           .stdin(Stdio::piped());
        
        let mut child = cmd.spawn()
            .expect("Failed to spawn kubectl process");
        
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(script_content.as_bytes())
                .expect("Failed to write to stdin");
        }
        
        let status = child.wait()
            .expect("Failed to wait for kubectl process");
        
        if !status.success() {
            display::print_error_styled("Failed to execute script");
            std::process::exit(1);
        } else {
            display::print_success_styled("Script executed successfully");
        }
    } else {
        display::print_error_styled(&format!("No pod found matching pattern: {}", pod_pattern));
        std::process::exit(1);
    }
}

/// Open a shell session to a pod (try bash first, then sh)
pub fn shell_to_pod(pod_pattern: &str, context: Option<&str>, namespace: Option<&str>) {
    if let Some(pod_name) = pods::select_pod(pod_pattern, context, namespace) {
        display::print_working_styled(&format!("Opening shell session to pod: {}", pod_name));
        
        // Try bash first
        let bash_args = vec!["exec", "-it", &pod_name, "--", "/bin/bash"];
        if kubectl::execute_interactive_with_context(&bash_args, context, namespace) {
            return;
        }
        
        // Fallback to sh
        display::print_info_styled("Bash not available, trying sh...");
        let sh_args = vec!["exec", "-it", &pod_name, "--", "/bin/sh"];
        if !kubectl::execute_interactive_with_context(&sh_args, context, namespace) {
            display::print_error_styled("Failed to open shell session");
            std::process::exit(1);
        }
    } else {
        display::print_error_styled(&format!("No pod found matching pattern: {}", pod_pattern));
        std::process::exit(1);
    }
} 