use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Write};
use crate::utils;
use crate::cli::ConfigCommands;

#[derive(Debug, Deserialize, Serialize)]
pub struct KubixConfig {
    #[serde(default = "default_commands")]
    pub commands: HashMap<String, String>,
    #[serde(default = "default_scripts")]
    pub scripts: HashMap<String, String>,
}

impl Default for KubixConfig {
    fn default() -> Self {
        Self {
            commands: default_commands(),
            scripts: default_scripts(),
        }
    }
}

impl KubixConfig {
    /// Load configuration using confy
    pub fn load() -> Self {
        match confy::load("kubix", None) {
            Ok(config) => config,
            Err(err) => {
                eprintln!("⚠️ Warning: Failed to load config: {}", err);
                eprintln!("Using default configuration");
                Self::default()
            }
        }
    }

    /// Save configuration using confy
    pub fn save(&self) -> Result<(), String> {
        match confy::store("kubix", None, self) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to save config: {}", err)),
        }
    }

    /// Resolve a command nickname or return the input as-is
    pub fn resolve_command(&self, input: &str) -> String {
        self.commands.get(input).cloned().unwrap_or_else(|| input.to_string())
    }

    /// Resolve a script nickname or return the input as-is
    pub fn resolve_script(&self, input: &str) -> String {
        self.scripts.get(input).cloned().unwrap_or_else(|| input.to_string())
    }

    /// Get the config file path
    pub fn get_config_path() -> String {
        match confy::get_configuration_file_path("kubix", None) {
            Ok(path) => path.to_string_lossy().to_string(),
            Err(_) => "Unknown".to_string(),
        }
    }
}

/// Handle the config command - display current configuration
pub fn handle_config_command(config_cmd: Option<&ConfigCommands>) {
    match config_cmd {
        None => {
            // No subcommand provided, show config (default behavior)
            show_config();
        }
        Some(ConfigCommands::List) => {
            show_config();
        }
        Some(ConfigCommands::AddCommand { nickname, command }) => {
            add_command(nickname, command);
        }
        Some(ConfigCommands::AddScript { nickname, script }) => {
            add_script(nickname, script);
        }
        Some(ConfigCommands::RemoveCommand { nickname }) => {
            remove_command(nickname);
        }
        Some(ConfigCommands::RemoveScript { nickname }) => {
            remove_script(nickname);
        }
    }
}

/// Show current configuration
pub fn show_config() {
    let config = KubixConfig::load();
    let config_path = KubixConfig::get_config_path();
    
    println!("📋 Kubix Configuration");
    println!("📁 Config file: {}", config_path);
    println!();
    
    // Display commands
    if !config.commands.is_empty() {
        println!("⚡ Commands:");
        let mut commands: Vec<_> = config.commands.iter().collect();
        commands.sort_by_key(|&(k, _)| k);
        for (nickname, command) in commands {
            println!("  {} → {}", nickname, command);
        }
        println!();
    }
    
    // Display scripts
    if !config.scripts.is_empty() {
        println!("📜 Scripts:");
        let mut scripts: Vec<_> = config.scripts.iter().collect();
        scripts.sort_by_key(|&(k, _)| k);
        for (nickname, script) in scripts {
            println!("  {} → {}", nickname, script);
        }
        println!();
    }
    
    if config.commands.is_empty() && config.scripts.is_empty() {
        utils::print_info("No custom commands or scripts configured");
    }
    println!("Add configurations using the subcommands:");
    println!("  kubix config add-command <nickname> <command>");
    println!("  kubix config add-script <nickname> <script>");
    println!();
    
    println!("💡 Usage:");
    println!("  kubix exec <pod> -c <command>   # Use command nickname");
    println!("  kubix exec <pod> -s <script>    # Use script nickname");
}

/// Add a command with confirmation if it already exists
pub fn add_command(nickname: &str, command: &str) {
    let mut config = KubixConfig::load();
    
    // Check if command already exists
    if let Some(existing_command) = config.commands.get(nickname) {
        println!("⚠️  Command '{}' already exists: '{}'", nickname, existing_command);
        if !prompt_for_confirmation("Do you want to overwrite it?") {
            println!("Operation cancelled.");
            return;
        }
    }
    
    // Add or update the command
    config.commands.insert(nickname.to_string(), command.to_string());
    
    match config.save() {
        Ok(_) => {
            utils::print_success(&format!("Command '{}' added successfully", nickname));
        }
        Err(err) => {
            utils::print_error_and_exit(&format!("Failed to save config: {}", err));
        }
    }
}

/// Add a script with confirmation if it already exists
pub fn add_script(nickname: &str, script: &str) {
    let mut config = KubixConfig::load();
    
    // Check if script already exists
    if let Some(existing_script) = config.scripts.get(nickname) {
        println!("⚠️  Script '{}' already exists: '{}'", nickname, existing_script);
        if !prompt_for_confirmation("Do you want to overwrite it?") {
            println!("Operation cancelled.");
            return;
        }
    }
    
    // Add or update the script
    config.scripts.insert(nickname.to_string(), script.to_string());
    
    match config.save() {
        Ok(_) => {
            utils::print_success(&format!("Script '{}' added successfully", nickname));
        }
        Err(err) => {
            utils::print_error_and_exit(&format!("Failed to save config: {}", err));
        }
    }
}

/// Remove a command
pub fn remove_command(nickname: &str) {
    let mut config = KubixConfig::load();
    
    if config.commands.remove(nickname).is_some() {
        match config.save() {
            Ok(_) => {
                utils::print_success(&format!("Command '{}' removed successfully", nickname));
            }
            Err(err) => {
                utils::print_error_and_exit(&format!("Failed to save config: {}", err));
            }
        }
    } else {
        utils::print_error(&format!("Command '{}' not found", nickname));
    }
}

/// Remove a script
pub fn remove_script(nickname: &str) {
    let mut config = KubixConfig::load();
    
    if config.scripts.remove(nickname).is_some() {
        match config.save() {
            Ok(_) => {
                utils::print_success(&format!("Script '{}' removed successfully", nickname));
            }
            Err(err) => {
                utils::print_error_and_exit(&format!("Failed to save config: {}", err));
            }
        }
    } else {
        utils::print_error(&format!("Script '{}' not found", nickname));
    }
}

/// Prompt user for yes/no confirmation
fn prompt_for_confirmation(message: &str) -> bool {
    print!("❓ {} [y/N]: ", message);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim().to_lowercase();
            input == "y" || input == "yes"
        }
        Err(_) => {
            // Handle Ctrl+C or read error as cancellation
            false
        }
    }
}

/// Default commands for the configuration
fn default_commands() -> HashMap<String, String> {
    let mut commands = HashMap::new();
    commands.insert("shell".to_string(), "python manage.py shell".to_string());
    commands.insert("migrate".to_string(), "python manage.py migrate".to_string());
    commands.insert("console".to_string(), "rails console".to_string());
    commands.insert("logs".to_string(), "tail -f /var/log/app.log".to_string());
    commands.insert("ps".to_string(), "ps aux".to_string());
    commands.insert("env".to_string(), "printenv".to_string());
    commands
}

/// Default scripts for the configuration
fn default_scripts() -> HashMap<String, String> {
    let mut scripts = HashMap::new();
    scripts.insert("deploy".to_string(), "./scripts/deploy.sh".to_string());
    scripts.insert("setup".to_string(), "./scripts/setup.py".to_string());
    scripts.insert("backup".to_string(), "~/scripts/backup.sh".to_string());
    scripts
} 