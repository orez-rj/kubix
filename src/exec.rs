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
            display::print_error_and_exit("Cannot specify both command and script. Use either --command or --script, not both.");
        }
    }
}

/// Open a bash shell session to a pod
pub fn bash_to_pod(pod_pattern: &str, context: Option<&str>, namespace: Option<&str>) {
    if let Some(pod_name) = pods::select_pod(pod_pattern, context, namespace) {
        display::print_working(&format!("Opening bash session to pod: {}", pod_name));
        
        let base_args = vec!["exec", "-it", &pod_name, "--", "bash"];
        
        if !kubectl::execute_interactive_with_context(&base_args, context, namespace) {
            display::print_error_and_exit("Failed to open bash session");
        }
    } else {
        display::print_error_and_exit(&format!("No pod found matching pattern: {}", pod_pattern));
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
        display::print_working(&format!("Running command '{}' on pod: {}", command, pod_name));
        
        let base_args = vec!["exec", "-it", &pod_name, "--", "sh", "-c", command];
        
        if !kubectl::execute_interactive_with_context(&base_args, context, namespace) {
            display::print_error_and_exit("Failed to run command");
        }
    } else {
        display::print_error_and_exit(&format!("No pod found matching pattern: {}", pod_pattern));
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
        display::print_working(&format!("Executing script '{}' on pod: {}", script_path, pod_name));
        
        // Read the script content
        let script_content = fs::read_to_string(script_path)
            .unwrap_or_else(|_| {
                display::print_error_and_exit(&format!("Failed to read script file: {}", script_path));
            });
        
        // Build kubectl command
        let mut cmd = Command::new("kubectl");
        
        if let Some(ctx) = context {
            cmd.args(&["--context", ctx]);
        }
        
        if let Some(ns) = namespace {
            cmd.args(&["-n", ns]);
        }
        
        cmd.args(&["exec", "-it", &pod_name, "--", "sh"])
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
            display::print_error_and_exit("Failed to execute script");
        } else {
            display::print_success("Script executed successfully");
        }
    } else {
        display::print_error_and_exit(&format!("No pod found matching pattern: {}", pod_pattern));
    }
}
