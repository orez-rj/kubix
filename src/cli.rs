use clap::{Parser, Subcommand, ArgGroup};

#[derive(Parser)]
#[command(name = "kubix")]
#[command(version)]
#[command(about = "Smart CLI wrapper for kubectl", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Add a command nickname
    #[command(name = "add-command")]
    AddCommand {
        /// Nickname for the command
        nickname: String,
        /// The actual command to execute
        command: String,
    },
    
    /// Add a script nickname
    #[command(name = "add-script")]
    AddScript {
        /// Nickname for the script
        nickname: String,
        /// Path to the script file
        script: String,
    },
    
    /// Add or update an interpreter path for a file extension
    #[command(name = "add-interpreter")]
    AddInterpreter {
        /// File extension (e.g., "py", "js", "rb")
        extension: String,
        /// Full path to the interpreter (e.g., "/opt/app/venv/bin/python")
        interpreter_path: String,
    },
    
    /// Remove a command nickname
    #[command(name = "remove-command")]
    RemoveCommand {
        /// Nickname of the command to remove
        nickname: String,
    },
    
    /// Remove a script nickname
    #[command(name = "remove-script")]
    RemoveScript {
        /// Nickname of the script to remove
        nickname: String,
    },
    
    /// Remove a custom interpreter
    #[command(name = "remove-interpreter")]
    RemoveInterpreter {
        /// File extension to remove custom interpreter for
        extension: String,
    },
    
    /// List current configuration (default action)
    List,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage kubectl contexts - list all contexts or switch to one by pattern
    Ctx {
        /// Context name or pattern to switch to (optional - if not provided, lists all contexts)
        name: Option<String>,
    },
    
    /// List pods, optionally filtered by pattern
    #[command(name = "pods")]
    PodsList {
        /// Pod name pattern to filter by (optional - if not provided, lists all pods)
        pattern: Option<String>,
        /// Context to use (optional, uses current context if not specified)
        #[arg(long, short)]
        context: Option<String>,
        /// Namespace to list pods from (optional, uses default if not specified)
        #[arg(long, short)]
        namespace: Option<String>,
    },
    
    /// List pods, optionally filtered by pattern (alias for pods)
    #[command(name = "pod")]
    Pod {
        /// Pod name pattern to filter by (optional - if not provided, lists all pods)
        pattern: Option<String>,
        /// Context to use (optional, uses current context if not specified)
        #[arg(long, short)]
        context: Option<String>,
        /// Namespace to list pods from (optional, uses default if not specified)
        #[arg(long, short)]
        namespace: Option<String>,
    },

    /// View logs from a pod
    #[command(name = "logs")]
    Logs {
        /// Pod name or pattern to match
        pod: String,
        /// Context to use (optional)
        #[arg(long, short = 'x')]
        context: Option<String>,
        /// Namespace (optional)
        #[arg(long, short)]
        namespace: Option<String>,
        /// Follow log output
        #[arg(long, short)]
        follow: bool,
        /// Number of lines to show from the end of the logs
        #[arg(long, short)]
        tail: Option<u32>,
        /// Show logs from previous terminated container
        #[arg(long, short)]
        previous: bool,
        /// Container name (for multi-container pods)
        #[arg(long, short)]
        container: Option<String>,
        /// Filter logs using regex pattern (include lines matching this pattern)
        #[arg(long, short)]
        grep: Option<String>,
        /// Exclude lines matching this regex pattern (used with or without --grep)
        #[arg(long, short)]
        exclude: Option<String>,
    },

    /// View logs from a pod (alias for logs)
    #[command(name = "log")]
    Log {
        /// Pod name or pattern to match
        pod: String,
        /// Context to use (optional)
        #[arg(long, short = 'x')]
        context: Option<String>,
        /// Namespace (optional)
        #[arg(long, short)]
        namespace: Option<String>,
        /// Follow log output
        #[arg(long, short)]
        follow: bool,
        /// Number of lines to show from the end of the logs
        #[arg(long, short)]
        tail: Option<u32>,
        /// Show logs from previous terminated container
        #[arg(long, short)]
        previous: bool,
        /// Container name (for multi-container pods)
        #[arg(long, short)]
        container: Option<String>,
        /// Filter logs using regex pattern (include lines matching this pattern)
        #[arg(long, short)]
        grep: Option<String>,
        /// Exclude lines matching this regex pattern (used with or without --grep)
        #[arg(long, short)]
        exclude: Option<String>,
    },

    /// Execute command or script on a pod (defaults to bash if no command/script specified)
    #[command(group(
        ArgGroup::new("exec_type")
            .args(["command", "script"])
            .multiple(false)
    ))]
    Exec {
        /// Pod name or pattern to match
        pod: String,
        /// Command to execute (can be a full command or a nickname from config)
        #[arg(long, short)]
        command: Option<String>,
        /// Script to execute (can be a file path or a nickname from config)
        #[arg(long, short)]
        script: Option<String>,
        /// Context to use (optional)
        #[arg(long, short = 'x')]
        context: Option<String>,
        /// Namespace (optional)
        #[arg(long, short)]
        namespace: Option<String>,
    },

    /// Manage kubix configuration
    Config {
        #[command(subcommand)]
        command: Option<ConfigCommands>,
    },
}
