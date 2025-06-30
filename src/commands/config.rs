use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{utils, display};
use crate::cli::ConfigCommands;

#[derive(Debug, Deserialize, Serialize)]
pub struct KubixConfig {
    #[serde(default = "default_commands")]
    pub commands: HashMap<String, String>,
    #[serde(default = "default_scripts")]
    pub scripts: HashMap<String, String>,
    #[serde(default = "default_interpreters")]
    pub interpreters: HashMap<String, String>,
}

impl Default for KubixConfig {
    fn default() -> Self {
        Self {
            commands: default_commands(),
            scripts: default_scripts(),
            interpreters: default_interpreters(),
        }
    }
}

impl KubixConfig {
    /// Load configuration using confy
    pub fn load() -> Self {
        match confy::load("kubix", Some("kubix")) {
            Ok(config) => config,
            Err(err) => {
                display::print_warning(&format!("Failed to load config: {}", err));
                display::print_warning("Using default configuration");
                Self::default()
            }
        }
    }

    /// Save configuration using confy
    pub fn save(&self) -> Result<(), String> {
        match confy::store("kubix", Some("kubix"), self) {
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

    /// Resolve an interpreter for a file extension
    pub fn resolve_interpreter(&self, extension: &str) -> Option<String> {
        self.interpreters.get(extension).cloned()
    }

    /// Get the config file path
    pub fn get_config_path() -> String {
        match confy::get_configuration_file_path("kubix", Some("kubix")) {
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
        Some(ConfigCommands::AddInterpreter { extension, interpreter_path }) => {
            add_interpreter(extension, interpreter_path);
        }
        Some(ConfigCommands::RemoveCommand { nickname }) => {
            remove_command(nickname);
        }
        Some(ConfigCommands::RemoveScript { nickname }) => {
            remove_script(nickname);
        }
        Some(ConfigCommands::RemoveInterpreter { extension }) => {
            remove_interpreter(extension);
        }
    }
}

/// Show current configuration
pub fn show_config() {
    let config = KubixConfig::load();
    let config_path = KubixConfig::get_config_path();
    
    display::print_info(&format!("Config file: {}", config_path));
    display::print_commands_table(&config.commands);
    display::print_scripts_table(&config.scripts);
    display::print_interpreters_table(&config.interpreters);
    
    if config.commands.is_empty() && config.scripts.is_empty() && config.interpreters.is_empty() {
        display::print_info("No custom commands, scripts, or interpreters configured.\n");
    } else {
        display::print_line("");
    }
    display::print_info("Add or remove configurations using the subcommands or edit the config file directly:");
    display::print_lines(&[
        "  • kubix config add-command <nickname> <command>",
        "  • kubix config add-script <nickname> <script>", 
        "  • kubix config add-interpreter <extension> <interpreter_path>",
        "  • kubix config remove-command <nickname>",
        "  • kubix config remove-script <nickname>",
        "  • kubix config remove-interpreter <extension>\n",
    ]);
    
    display::print_info("💡 Usage:");
    display::print_lines(&[
        "  • kubix exec <pod> -c <command>   # Use command nickname",
        "  • kubix exec <pod> -s <script>    # Use script nickname or file with custom interpreter"
    ]);
}

/// Add a command with confirmation if it already exists
pub fn add_command(nickname: &str, command: &str) {
    let mut config = KubixConfig::load();
    
    // Check if command already exists
    if let Some(existing_command) = config.commands.get(nickname) {
        display::print_warning(&format!("Command '{}' already exists: '{}'", nickname, existing_command));
        if !utils::prompt_for_confirmation("Do you want to overwrite it?") {
            display::print_error("Operation cancelled.");
            return;
        }
    }
    
    // Add or update the command
    config.commands.insert(nickname.to_string(), command.to_string());
    
    match config.save() {
        Ok(_) => {
            display::print_success(&format!("Command '{}' added successfully", nickname));
        }
        Err(err) => {
            display::print_error_and_exit(&format!("Failed to save config: {}", err));
        }
    }
}

/// Add a script with confirmation if it already exists
pub fn add_script(nickname: &str, script: &str) {
    let mut config = KubixConfig::load();
    
    // Check if script already exists
    if let Some(existing_script) = config.scripts.get(nickname) {
        display::print_warning(&format!("Script '{}' already exists: '{}'", nickname, existing_script));
        if !utils::prompt_for_confirmation("Do you want to overwrite it?") {
            display::print_error("Operation cancelled.");
            return;
        }
    }
    
    // Add or update the script
    config.scripts.insert(nickname.to_string(), script.to_string());
    
    match config.save() {
        Ok(_) => {
            display::print_success(&format!("Script '{}' added successfully", nickname));
        }
        Err(err) => {
            display::print_error_and_exit(&format!("Failed to save config: {}", err));
        }
    }
}

/// Add an interpreter with confirmation if it already exists
pub fn add_interpreter(extension: &str, interpreter_path: &str) {
    let mut config = KubixConfig::load();
    
    // Check if interpreter already exists
    if let Some(existing_interpreter) = config.interpreters.get(extension) {
        display::print_warning(&format!("Interpreter for '{}' already exists: '{}'", extension, existing_interpreter));
        if !utils::prompt_for_confirmation("Do you want to overwrite it?") {
            display::print_error("Operation cancelled.");
            return;
        }
    }
    
    // Add or update the interpreter
    config.interpreters.insert(extension.to_string(), interpreter_path.to_string());
    
    match config.save() {
        Ok(_) => {
            display::print_success(&format!("Interpreter for '{}' added successfully", extension));
        }
        Err(err) => {
            display::print_error_and_exit(&format!("Failed to save config: {}", err));
        }
    }
}

/// Remove a command
pub fn remove_command(nickname: &str) {
    let mut config = KubixConfig::load();
    
    if config.commands.remove(nickname).is_some() {
        match config.save() {
            Ok(_) => {
                display::print_success(&format!("Command '{}' removed successfully", nickname));
            }
            Err(err) => {
                display::print_error_and_exit(&format!("Failed to save config: {}", err));
            }
        }
    } else {
        display::print_error(&format!("Command '{}' not found", nickname));
    }
}

/// Remove a script
pub fn remove_script(nickname: &str) {
    let mut config = KubixConfig::load();
    
    if config.scripts.remove(nickname).is_some() {
        match config.save() {
            Ok(_) => {
                display::print_success(&format!("Script '{}' removed successfully", nickname));
            }
            Err(err) => {
                display::print_error_and_exit(&format!("Failed to save config: {}", err));
            }
        }
    } else {
        display::print_error(&format!("Script '{}' not found", nickname));
    }
}

/// Remove an interpreter
pub fn remove_interpreter(extension: &str) {
    let mut config = KubixConfig::load();
    
    if config.interpreters.remove(extension).is_some() {
        match config.save() {
            Ok(_) => {
                display::print_success(&format!("Interpreter for '{}' removed successfully", extension));
            }
            Err(err) => {
                display::print_error_and_exit(&format!("Failed to save config: {}", err));
            }
        }
    } else {
        display::print_error(&format!("Interpreter for '{}' not found", extension));
    }
}

/// Default commands for the configuration
fn default_commands() -> HashMap<String, String> {
    let mut commands = HashMap::new();
    commands.insert("shell".to_string(), "$BIN_PATH/python manage.py shell".to_string());
    commands.insert("ps".to_string(), "ps aux".to_string());
    commands
}

/// Default scripts for the configuration
fn default_scripts() -> HashMap<String, String> {
    let mut scripts = HashMap::new();
    scripts.insert("deploy".to_string(), "/Users/myuser/scripts/deploy.sh".to_string());
    scripts.insert("setup".to_string(), "~/scripts/setup.py".to_string());
    scripts
}

/// Default interpreters for the configuration
fn default_interpreters() -> HashMap<String, String> {
    let mut interpreters = HashMap::new();
    interpreters.insert("py".to_string(), "/opt/app/venv/bin/python".to_string());
    interpreters
} 