use crate::{exec, context, namespace, display};

/// Process a smart/natural language command
pub fn process_smart_command(command: &str) {
    display::print_info(&format!("🧠 Processing smart command: {}", command));
    
    // Pattern matching for common use cases
    if command.contains("run") && command.contains("on pod") {
        handle_run_command(command);
    } else if command.contains("bash") || command.contains("shell") {
        handle_shell_command(command);
    } else if command.contains("script") {
        handle_script_command(command);
    } else {
        print_smart_help();
    }
}

fn handle_run_command(command: &str) {
    // Parse pattern like: "run command 'python manage.py shell' on pod that has web-internal in its name on testing context"
    if let (Some(cmd_start), Some(cmd_end)) = (command.find('\''), command.rfind('\'')) {
        let cmd_to_run = &command[cmd_start + 1..cmd_end];
        
        // Extract pod pattern
        let pod_pattern = extract_pod_pattern(command).unwrap_or("web");
        
        // Extract and resolve context
        let context_pattern = extract_context(command);
        let resolved_context = context_pattern.and_then(|pattern| context::resolve_context_pattern(pattern));
        
        // Extract and resolve namespace
        let namespace_pattern = extract_namespace(command);
        let resolved_namespace = namespace_pattern.and_then(|pattern| 
            namespace::resolve_namespace_pattern(pattern, resolved_context.as_deref())
        );
        
        display::print_info(&format!("🎯 Parsed command:"));
        display::print_info(&format!("   Command: {}", cmd_to_run));
        display::print_info(&format!("   Pod pattern: {}", pod_pattern));
        if let Some(ref ctx) = resolved_context {
            display::print_info(&format!("   Context: {}", ctx));
        }
        if let Some(ref ns) = resolved_namespace {
            display::print_info(&format!("   Namespace: {}", ns));
        }
        
        exec::run_command_on_pod(pod_pattern, cmd_to_run, resolved_context.as_deref(), resolved_namespace.as_deref());
    } else {
        display::print_error("❓ Could not parse command. Make sure to wrap the command in single quotes.");
    }
}

fn handle_shell_command(command: &str) {
    let pod_pattern = extract_pod_pattern(command).unwrap_or("web");
    
    // Extract and resolve context
    let context_pattern = extract_context(command);
    let resolved_context = context_pattern.and_then(|pattern| context::resolve_context_pattern(pattern));
    
    // Extract and resolve namespace
    let namespace_pattern = extract_namespace(command);
    let resolved_namespace = namespace_pattern.and_then(|pattern| 
        namespace::resolve_namespace_pattern(pattern, resolved_context.as_deref())
    );
    
    display::print_info(&format!("🎯 Opening shell to pod matching: {}", pod_pattern));
    
    exec::bash_to_pod(pod_pattern, resolved_context.as_deref(), resolved_namespace.as_deref());
}

fn handle_script_command(command: &str) {
    // Parse pattern like: "run script './deploy.sh' on pod web in production context"
    if let (Some(script_start), Some(script_end)) = (command.find('\''), command.rfind('\'')) {
        let script_path = &command[script_start + 1..script_end];
        let pod_pattern = extract_pod_pattern(command).unwrap_or("web");
        
        // Extract and resolve context
        let context_pattern = extract_context(command);
        let resolved_context = context_pattern.and_then(|pattern| context::resolve_context_pattern(pattern));
        
        // Extract and resolve namespace
        let namespace_pattern = extract_namespace(command);
        let resolved_namespace = namespace_pattern.and_then(|pattern| 
            namespace::resolve_namespace_pattern(pattern, resolved_context.as_deref())
        );
        
        display::print_info(&format!("🎯 Executing script: {}", script_path));
        
        exec::exec_script_on_pod(pod_pattern, script_path, resolved_context.as_deref(), resolved_namespace.as_deref());
    } else {
        display::print_error("❓ Could not parse script path. Make sure to wrap the script path in single quotes.");
    }
}

fn extract_pod_pattern(command: &str) -> Option<&str> {
    if let Some(pos) = command.find("pod that has") {
        let after_pod = &command[pos + 12..];
        if let Some(end_pos) = after_pod.find(" in its name") {
            Some(after_pod[..end_pos].trim())
        } else {
            None
        }
    } else if let Some(pos) = command.find("pod ") {
        let after_pod = &command[pos + 4..];
        if let Some(end_pos) = after_pod.find(" ") {
            Some(after_pod[..end_pos].trim())
        } else {
            Some(after_pod.trim())
        }
    } else {
        None
    }
}

fn extract_context(command: &str) -> Option<&str> {
    if let Some(pos) = command.find("context ") {
        let after_context = &command[pos + 8..];
        if let Some(end_pos) = after_context.find(" ") {
            Some(after_context[..end_pos].trim())
        } else {
            Some(after_context.trim())
        }
    } else if let Some(pos) = command.find("on ") {
        let after_on = &command[pos + 3..];
        if let Some(end_pos) = after_on.find(" context") {
            Some(after_on[..end_pos].trim())
        } else {
            None
        }
    } else {
        None
    }
}

fn extract_namespace(command: &str) -> Option<&str> {
    if let Some(pos) = command.find("namespace ") {
        let after_namespace = &command[pos + 10..];
        if let Some(end_pos) = after_namespace.find(" ") {
            Some(after_namespace[..end_pos].trim())
        } else {
            Some(after_namespace.trim())
        }
    } else if let Some(pos) = command.find("in ") {
        let after_in = &command[pos + 3..];
        if let Some(end_pos) = after_in.find(" namespace") {
            Some(after_in[..end_pos].trim())
        } else {
            None
        }
    } else {
        None
    }
}

fn print_smart_help() {
    display::print_info("❓ Smart command not recognized. Try patterns like:");
    display::print_info("   📝 Commands:");
    display::print_info("     'run command 'python manage.py shell' on pod that has web-internal in its name on testing context'");
    display::print_info("     'run command 'ls -la' on pod web in production context'");
    display::print_info("   🐚 Shell access:");
    display::print_info("     'bash to pod web'");
    display::print_info("     'shell to pod that has api in its name'");
    display::print_info("   📜 Script execution:");
    display::print_info("     'run script './deploy.sh' on pod web in production context'");
    display::print_info("     'exec script './setup.py' on pod that has worker in its name'");
} 