use std::process::{Command, exit};

/// Execute a kubectl command and return the output
pub fn execute_kubectl(args: &[&str]) -> Result<String, String> {
    let output = Command::new("kubectl")
        .args(args)
        .output()
        .expect("Failed to execute kubectl command");

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Execute a kubectl command with interactive mode (for bash, etc.)
pub fn execute_kubectl_interactive(args: &[&str]) -> bool {
    let status = Command::new("kubectl")
        .args(args)
        .status()
        .expect("Failed to execute kubectl command");

    status.success()
}

/// Print success message with green checkmark
pub fn print_success(message: &str) {
    println!("✅ {}", message);
}

/// Print error message with red X and exit
pub fn print_error_and_exit(message: &str) {
    eprintln!("❌ {}", message);
    exit(1);
}

/// Print info message with blue icon
pub fn print_info(message: &str) {
    println!("ℹ️ {}", message);
}

/// Print working message with appropriate icon
pub fn print_working(message: &str) {
    println!("⚡ {}", message);
} 