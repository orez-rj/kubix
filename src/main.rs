mod cli;
mod utils;
mod kubectl;
mod commands;
mod display;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{
    handle_ctx_command, 
    handle_pods_command, 
    handle_exec_command, 
    handle_config_command,
    process_smart_command,
    resolve_context_pattern,
    resolve_namespace_pattern
};

fn main() {
    // Setup signal handling for graceful cancellation
    ctrlc::set_handler(move || {
        display::print_error("\nOperation cancelled by user");
        std::process::exit(130); // Standard exit code for SIGINT
    }).expect("Error setting Ctrl+C handler");

    let cli: Cli = Cli::parse();
    handle_command(&cli.command);
}

fn handle_command(command: &Commands) {
    match command {
        Commands::Ctx { name } => {
            handle_ctx_command(name.as_deref());
        }
        Commands::PodsList { pattern, context, namespace } => {
            let resolved_context = resolve_context_if_provided(context.as_deref());
            let resolved_namespace = resolve_namespace_if_provided(namespace.as_deref(), resolved_context.as_deref());
            handle_pods_command(pattern.as_deref(), resolved_context.as_deref(), resolved_namespace.as_deref());
        }
        Commands::Pod { pattern, context, namespace } => {
            let resolved_context = resolve_context_if_provided(context.as_deref());
            let resolved_namespace = resolve_namespace_if_provided(namespace.as_deref(), resolved_context.as_deref());
            handle_pods_command(pattern.as_deref(), resolved_context.as_deref(), resolved_namespace.as_deref());
        }
        Commands::Exec { pod, command, script, context, namespace } => {
            let resolved_context = resolve_context_if_provided(context.as_deref());
            let resolved_namespace = resolve_namespace_if_provided(namespace.as_deref(), resolved_context.as_deref());
            handle_exec_command(
                pod, 
                command.as_deref(), 
                script.as_deref(), 
                resolved_context.as_deref(), 
                resolved_namespace.as_deref()
            );
        }
        Commands::Smart { command } => {
            process_smart_command(command);
        }
        Commands::Config { command } => {
            handle_config_command(command.as_ref());
        }
    }
}

/// Resolve context pattern if provided, otherwise return None
fn resolve_context_if_provided(context_pattern: Option<&str>) -> Option<String> {
    context_pattern.and_then(|pattern| resolve_context_pattern(pattern))
}

/// Resolve namespace pattern if provided, otherwise return None
fn resolve_namespace_if_provided(namespace_pattern: Option<&str>, context: Option<&str>) -> Option<String> {
    namespace_pattern.and_then(|pattern| resolve_namespace_pattern(pattern, context))
}
