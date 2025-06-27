use clap::{Parser, Subcommand, ArgGroup};

#[derive(Parser)]
#[command(name = "kubix")]
#[command(about = "Smart CLI wrapper for kubectl", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
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
    
    /// Smart command - combine multiple operations
    Smart {
        /// Natural language command description
        command: String,
    },
}