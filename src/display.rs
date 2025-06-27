use tabled::{Table, Tabled, settings::{Style, Alignment, object::Columns}};
use owo_colors::OwoColorize;
use std::fmt::Display;

/// Represents a pod for table display
#[derive(Tabled)]
pub struct PodDisplay {
    #[tabled(rename = "Pod Name")]
    pub name: String,
    #[tabled(rename = "Ready")]
    pub ready: String,
    #[tabled(rename = "Status")]
    pub status: String,
    #[tabled(rename = "Restarts")]
    pub restarts: String,
    #[tabled(rename = "Age")]
    pub age: String,
}

/// Represents a command for table display
#[derive(Tabled)]
pub struct CommandDisplay {
    #[tabled(rename = "Nickname")]
    pub nickname: String,
    #[tabled(rename = "Command")]
    pub command: String,
}

/// Represents a script for table display
#[derive(Tabled)]
pub struct ScriptDisplay {
    #[tabled(rename = "Nickname")]
    pub nickname: String,
    #[tabled(rename = "Script Path")]
    pub script: String,
}

/// Represents a context for table display
#[derive(Tabled)]
pub struct ContextDisplay {
    #[tabled(rename = "Context")]
    pub context: String,
    #[tabled(rename = "Current")]
    pub current: String,
}

/// Represents a selection item for table display
#[derive(Tabled)]
pub struct SelectionDisplay {
    #[tabled(rename = "#")]
    pub number: String,
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Details")]
    pub details: String,
}

/// Apply beautiful rounded styling to a table
fn style_table(table: &mut Table) {
    table
        .with(Style::rounded())
        .modify(Columns::first(), Alignment::left())
        .modify(Columns::new(1..), Alignment::left());
}

/// Apply color to status-based content
fn colorize_status(status: &str) -> String {
    match status.to_lowercase().as_str() {
        "running" => status.green().bold().to_string(),
        "pending" => status.yellow().bold().to_string(),
        "failed" | "error" | "crashloopbackoff" => status.red().bold().to_string(),
        "succeeded" | "completed" => status.blue().bold().to_string(),
        _ => status.to_string(),
    }
}

/// Apply color to current marker
fn colorize_current_marker(marker: &str) -> String {
    if marker == "✓" {
        marker.green().bold().to_string()
    } else {
        marker.to_string()
    }
}

/// Print pods in a beautiful table format
pub fn print_pods_table(pods_output: &str, pattern: Option<&str>) {
    let lines: Vec<&str> = pods_output.lines().collect();
    if lines.is_empty() {
        println!("{}", "No pods found".yellow());
        return;
    }

    // Parse kubectl output into structured data
    let mut pod_displays = Vec::new();
    
    for line in lines.iter().skip(1) { // Skip header
        if line.trim().is_empty() {
            continue;
        }
        
        // Parse kubectl output (space-separated columns)
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            let name = parts[0].to_string();
            let ready = parts[1].to_string();
            let status = colorize_status(parts[2]);
            let restarts = parts[3].to_string();
            let age = parts[4].to_string();
            
            // Apply pattern filtering if provided
            if let Some(p) = pattern {
                if !line.contains(p) {
                    continue;
                }
            }
            
            pod_displays.push(PodDisplay {
                name: name.blue().bold().to_string(),
                ready,
                status,
                restarts,
                age,
            });
        }
    }
    
    if pod_displays.is_empty() {
        if let Some(p) = pattern {
            println!("{}", format!("No pods found matching pattern: '{}'", p).yellow());
        } else {
            println!("{}", "No pods found".yellow());
        }
        return;
    }
    
    let mut table = Table::new(&pod_displays);
    style_table(&mut table);
    
    if let Some(p) = pattern {
        println!("{}", format!("📋 Found {} pod(s) matching '{}':", pod_displays.len(), p).cyan().bold());
    } else {
        println!("{}", "📋 Pods:".cyan().bold());
    }
    
    println!("{}", table);
}

/// Print commands in a beautiful table format
pub fn print_commands_table(commands: &std::collections::HashMap<String, String>) {
    if commands.is_empty() {
        return;
    }

    let mut command_displays: Vec<CommandDisplay> = commands
        .iter()
        .map(|(nickname, command)| CommandDisplay {
            nickname: nickname.blue().bold().to_string(),
            command: command.bright_black().to_string(),
        })
        .collect();
    
    command_displays.sort_by(|a, b| a.nickname.cmp(&b.nickname));
    
    let mut table = Table::new(&command_displays);
    style_table(&mut table);
    
    println!("{}", "⚡ Commands:".yellow().bold());
    println!("{}", table);
}

/// Print scripts in a beautiful table format
pub fn print_scripts_table(scripts: &std::collections::HashMap<String, String>) {
    if scripts.is_empty() {
        return;
    }

    let mut script_displays: Vec<ScriptDisplay> = scripts
        .iter()
        .map(|(nickname, script)| ScriptDisplay {
            nickname: nickname.blue().bold().to_string(),
            script: script.bright_black().to_string(),
        })
        .collect();
    
    script_displays.sort_by(|a, b| a.nickname.cmp(&b.nickname));
    
    let mut table = Table::new(&script_displays);
    style_table(&mut table);
    
    println!("{}", "📜 Scripts:".yellow().bold());
    println!("{}", table);
}

/// Print contexts in a beautiful table format
pub fn print_contexts_table(contexts_output: &str, current_context: Option<&str>) {
    let lines: Vec<&str> = contexts_output.lines().collect();
    if lines.is_empty() {
        println!("{}", "No contexts found".yellow());
        return;
    }

    let mut context_displays = Vec::new();
    
    for line in lines {
        let context = line.trim();
        if !context.is_empty() {
            let is_current = current_context.map_or(false, |current| current == context);
            let current_marker = if is_current { colorize_current_marker("✓") } else { " ".to_string() };
            
            context_displays.push(ContextDisplay {
                context: if is_current {
                    context.green().bold().to_string()
                } else {
                    context.to_string()
                },
                current: current_marker,
            });
        }
    }
    
    if context_displays.is_empty() {
        println!("{}", "No contexts found".yellow());
        return;
    }
    
    let mut table = Table::new(&context_displays);
    style_table(&mut table);
    
    println!("{}", "📋 Available kubectl contexts:".cyan().bold());
    println!("{}", table);
}

/// Print selection items in a beautiful table format
pub fn print_selection_table<T: Display>(items: &[T], resource_type: &str, details_fn: Option<fn(&T) -> String>) {
    if items.is_empty() {
        println!("{}", format!("No {} found", resource_type).yellow());
        return;
    }

    let selection_displays: Vec<SelectionDisplay> = items
        .iter()
        .enumerate()
        .map(|(i, item)| SelectionDisplay {
            number: format!("{}", i + 1).cyan().bold().to_string(),
            name: item.to_string().blue().to_string(),
            details: details_fn.map_or_else(|| "".to_string(), |f| f(item).bright_black().to_string()),
        })
        .collect();
    
    let mut table = Table::new(&selection_displays);
    style_table(&mut table);
    
    println!("{}", format!("🔍 Found {} {}(s):", items.len(), resource_type).cyan().bold());
    println!("{}", table);
}

/// Print a simple info message with styling
pub fn print_info_styled(message: &str) {
    println!("{} {}", "ℹ️".cyan(), message.bright_blue());
}

/// Print a success message with styling
pub fn print_success_styled(message: &str) {
    println!("{} {}", "✅".green(), message.green().bold());
}

/// Print an error message with styling
pub fn print_error_styled(message: &str) {
    eprintln!("{} {}", "❌".red(), message.red().bold());
}

/// Print a working message with styling
pub fn print_working_styled(message: &str) {
    println!("{} {}", "⚡".yellow(), message.cyan());
} 