pub mod config;
pub mod context;
pub mod pods;
pub mod namespace;
pub mod exec;
pub mod smart;

// Re-export main functions for clean imports
pub use config::handle_config_command;
pub use context::{handle_ctx_command, resolve_context_pattern};
pub use pods::handle_pods_command;
pub use namespace::resolve_namespace_pattern;
pub use exec::handle_exec_command;
pub use smart::process_smart_command; 