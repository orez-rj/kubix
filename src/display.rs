use tabled::{Table, Tabled, settings::{
    Style, object::{Rows, Columns, Cell}, Color
}};
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

/// Apply styling to a table
fn style_table(table: &mut Table) {
    table
        .with(Style::rounded())
        .modify(Columns::first(), Color::FG_BRIGHT_BLUE)
        .modify(Rows::first(), Color::FG_BRIGHT_MAGENTA);
}

/// Apply color to status-based content
fn colorize_status(status: &str) -> Color {
    match status.to_lowercase().as_str() {
        "running" => Color::FG_GREEN,
        "pending" => Color::FG_YELLOW,
        "failed" | "error" | "crashloopbackoff" => Color::FG_RED,
        "succeeded" | "completed" => Color::FG_BLUE,
        _ => Color::FG_WHITE,
    }
}

/// Print pods in a beautiful table format
pub fn print_pods_table(pods_output: &str, pattern: Option<&str>) {
    let lines: Vec<&str> = pods_output.lines().collect();
    if lines.is_empty() {
        print_line(&"No pods found".yellow().to_string());
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
            let status = parts[2].to_string();
            let restarts = parts[3].to_string();
            let age = parts[4].to_string();
            
            // Apply pattern filtering if provided
            if let Some(p) = pattern {
                if !line.contains(p) {
                    continue;
                }
            }
            
            pod_displays.push(PodDisplay {
                name,
                ready,
                status,
                restarts,
                age,
            });
        }
    }
    
    if pod_displays.is_empty() {
        if let Some(p) = pattern {
            print_line(&format!("No pods found matching pattern: '{}'", p).yellow().to_string());
        } else {
            print_line(&"No pods found".yellow().to_string());
        }
        return;
    }
    
    let mut table = Table::new(&pod_displays);
    style_table(&mut table);
    for (i, d) in pod_displays.iter().enumerate() {
        table.modify(Cell::new(i + 1, 2), colorize_status(&d.status));
    }
    
    let header = if let Some(p) = pattern {
        format!("📋 Found {} pod(s) matching '{}':", pod_displays.len(), p).cyan().bold().to_string()
    } else {
        "📋 Pods:".cyan().bold().to_string()
    };
    
    print_lines(&[&header, &table.to_string()]);
}

/// Print commands in a beautiful table format
pub fn print_commands_table(commands: &std::collections::HashMap<String, String>) {
    if commands.is_empty() {
        return;
    }

    let mut command_displays: Vec<CommandDisplay> = commands
        .iter()
        .map(|(nickname, command)| CommandDisplay {
            nickname: nickname.to_string(),
            command: command.to_string(),
        })
        .collect();
    
    command_displays.sort_by(|a, b| a.nickname.cmp(&b.nickname));
    
    let mut table = Table::new(&command_displays);
    style_table(&mut table);
    
    let header = "⚡ Commands:".yellow().bold().to_string();
    print_lines(&[&header, &table.to_string()]);
}

/// Print scripts in a beautiful table format
pub fn print_scripts_table(scripts: &std::collections::HashMap<String, String>) {
    if scripts.is_empty() {
        return;
    }

    let mut script_displays: Vec<ScriptDisplay> = scripts
        .iter()
        .map(|(nickname, script)| ScriptDisplay {
            nickname: nickname.to_string(),
            script: script.to_string(),
        })
        .collect();
    
    script_displays.sort_by(|a, b| a.nickname.cmp(&b.nickname));
    
    let mut table = Table::new(&script_displays);
    style_table(&mut table);
    
    let header = "📜 Scripts:".yellow().bold().to_string();
    print_lines(&[&header, &table.to_string()]);
}

/// Print contexts in a beautiful table format
pub fn print_contexts_table(contexts_output: &str, current_context: Option<&str>) {
    let lines: Vec<&str> = contexts_output.lines().collect();
    if lines.is_empty() {
        print_line(&"No contexts found".yellow().to_string());
        return;
    }

    let mut context_displays = Vec::new();
    
    for line in lines {
        let context = line.trim();
        if !context.is_empty() {
            let is_current = current_context.map_or(false, |current| current == context);
            let current_marker = if is_current { "✓" } else { "" };
            
            context_displays.push(ContextDisplay {
                context: context.to_string(),
                current: current_marker.to_string(),
            });
        }
    }
    
    if context_displays.is_empty() {
        print_line(&"No contexts found".yellow().to_string());
        return;
    }
    
    let mut table = Table::new(&context_displays);
    style_table(&mut table);
    
    // Apply green color to current context rows
    for (i, display) in context_displays.iter().enumerate() {
        if display.current == "✓" {
            table
                .modify(Rows::single(i + 1), Color::FG_GREEN);  // Context name
        }
    }
    
    let header = "📋 Available kubectl contexts:".cyan().bold().to_string();
    print_lines(&[&header, &table.to_string()]);
}

/// Print selection items in a beautiful table format
pub fn print_selection_table<T: Display>(items: &[T], resource_type: &str, details_fn: Option<fn(&T) -> String>) {
    if items.is_empty() {
        print_line(&format!("No {} found", resource_type).yellow().to_string());
        return;
    }

    let selection_displays: Vec<SelectionDisplay> = items
        .iter()
        .enumerate()
        .map(|(i, item)| SelectionDisplay {
            number: format!("{}", i + 1).to_string(),
            name: item.to_string().to_string(),
            details: details_fn.map_or_else(|| "".to_string(), |f| f(item).to_string()),
        })
        .collect();
    
    let mut table = Table::new(&selection_displays);
    style_table(&mut table);
    
    let header = format!("🔍 Found {} {}(s):", items.len(), resource_type).cyan().bold().to_string();
    print_lines(&[&header, &table.to_string()]);
}

/// Print a simple info message with styling
pub fn print_info(message: &str) {
    print_line(&format!("{} {}", "ℹ️".cyan(), message.bright_blue()));
}

/// Print a success message with styling
pub fn print_success(message: &str) {
    print_line(&format!("{} {}", "✅".green(), message.green().bold()));
}

/// Print an error message with styling
pub fn print_error(message: &str) {
    eprint_line(&format!("{} {}", "❌".red(), message.red().bold()));
}

/// Print an error message and exit
pub fn print_error_and_exit(message: &str) -> ! {
    print_error(message);
    std::process::exit(1);
}

/// Print a warning message with styling
pub fn print_warning(message: &str) {
    eprint_line(&format!("{} {}", "⚠️".yellow(), message.yellow().bold()));
}

/// Print a working message with styling
pub fn print_working(message: &str) {
    print_line(&format!("{} {}", "⚡".yellow(), message.cyan()));
}

/// Centralized error print line function
pub fn eprint_line(message: &str) {
    eprintln!("{}", message);
}

/// Centralized print line function
pub fn print_line(message: &str) {
    println!("{}", message);
}

/// Print multiple lines efficiently in a single call
pub fn print_lines(lines: &[&str]) {
    if lines.is_empty() {
        return;
    }
    
    let output = lines.join("\n");
    println!("{}", output);
}
