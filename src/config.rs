use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

    /// Resolve a command nickname or return the input as-is
    pub fn resolve_command(&self, input: &str) -> String {
        self.commands.get(input).cloned().unwrap_or_else(|| input.to_string())
    }

    /// Resolve a script nickname or return the input as-is
    pub fn resolve_script(&self, input: &str) -> String {
        self.scripts.get(input).cloned().unwrap_or_else(|| input.to_string())
    }

    /// List all available command nicknames
    pub fn list_commands(&self) -> Vec<String> {
        self.commands.keys().cloned().collect()
    }

    /// List all available script nicknames
    pub fn list_scripts(&self) -> Vec<String> {
        self.scripts.keys().cloned().collect()
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