use crate::{kubectl, display};
use crate::commands::{pods, config, resolve_context_pattern, resolve_namespace_pattern};
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use std::path::Path;

/// Handle the unified exec command
pub fn handle_exec_command(
    pod_pattern: &str, 
    command: Option<&str>,
    script: Option<&str>, 
    context_pattern: Option<&str>, 
    namespace_pattern: Option<&str>
) {
    // Resolve context and namespace patterns
    let resolved_context = context_pattern.and_then(|pattern| resolve_context_pattern(pattern));
    let resolved_namespace = namespace_pattern.and_then(|pattern| resolve_namespace_pattern(pattern, resolved_context.as_deref()));
    
    // Load configuration for command/script resolution
    let config = config::KubixConfig::load();
    
    match (command, script) {
        (Some(cmd), None) => {
            // Execute a command
            let resolved_command = config.resolve_command(cmd);
            run_command_on_pod(pod_pattern, &resolved_command, resolved_context.as_deref(), resolved_namespace.as_deref());
        }
        (None, Some(script_input)) => {
            // Execute a script
            let resolved_script = config.resolve_script(script_input);
            exec_script_on_pod(pod_pattern, &resolved_script, &config, resolved_context.as_deref(), resolved_namespace.as_deref());
        }
        (None, None) => {
            // Default to bash
            bash_to_pod(pod_pattern, resolved_context.as_deref(), resolved_namespace.as_deref());
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

/// Determine the appropriate interpreter based on file extension and config
fn get_interpreter_for_script(script_path: &str, config: &config::KubixConfig) -> Option<String> {
    let path = Path::new(script_path);
    if let Some(extension) = path.extension()?.to_str() {
        // First check for custom interpreter in config
        if let Some(custom_interpreter) = config.resolve_interpreter(extension) {
            return Some(custom_interpreter);
        }
        
        // Fall back to default interpreters
        match extension {
            "py" => Some("python3".to_string()),
            "js" => Some("node".to_string()),
            "rb" => Some("ruby".to_string()),
            "pl" => Some("perl".to_string()),
            "php" => Some("php".to_string()),
            "sh" | "bash" => Some("bash".to_string()),
            "r" => Some("Rscript".to_string()),
            "lua" => Some("lua".to_string()),
            "scala" => Some("scala".to_string()),
            "groovy" => Some("groovy".to_string()),
            _ => None, // Unknown extension, fall back to shebang detection
        }
    } else {
        None
    }
}

/// Execute a local script on a pod
pub fn exec_script_on_pod(
    pod_pattern: &str, 
    script_path: &str,
    config: &config::KubixConfig,
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
        
        // Determine the interpreter to use
        let interpreter = get_interpreter_for_script(script_path, &config);
        
        // Build kubectl command
        let mut cmd = Command::new("kubectl");
        
        if let Some(ctx) = context {
            cmd.args(&["--context", ctx]);
        }
        
        if let Some(ns) = namespace {
            cmd.args(&["-n", ns]);
        }
        
        // Choose execution strategy based on interpreter detection
        match interpreter {
            Some(interp) => {
                // Use detected interpreter directly
                display::print_info(&format!("🔍 Detected interpreter: {}", interp));
                
                // Standard interpreter execution - all interpreters can read from stdin
                cmd.args(&["exec", "-i", &pod_name, "--", &interp]);
            }
            None => {
                // Fall back to shell execution
                display::print_info("🔍 No file extension detected, using shell with shebang detection");
                cmd.args(&["exec", "-i", &pod_name, "--", "sh"]);
            }
        }
        
        cmd.stdin(Stdio::piped());
        let mut child = cmd.spawn()
            .expect("Failed to spawn kubectl process");
        
        // Send script content to the process
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
