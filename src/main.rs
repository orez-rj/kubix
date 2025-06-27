mod cli;
mod utils;
mod kubectl;
mod context;
mod namespace;
mod pods;
mod exec;
mod smart;
mod config;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli: Cli = Cli::parse();
    handle_command(&cli.command);
}

fn handle_command(command: &Commands) {
    match command {
        Commands::Ctx { name } => {
            context::handle_ctx_command(name.as_deref());
        }
        Commands::PodsList { pattern, context, namespace } => {
            let resolved_context = resolve_context_if_provided(context.as_deref());
            let resolved_namespace = resolve_namespace_if_provided(namespace.as_deref(), resolved_context.as_deref());
            pods::handle_pods_command(pattern.as_deref(), resolved_context.as_deref(), resolved_namespace.as_deref());
        }
        Commands::Pod { pattern, context, namespace } => {
            let resolved_context = resolve_context_if_provided(context.as_deref());
            let resolved_namespace = resolve_namespace_if_provided(namespace.as_deref(), resolved_context.as_deref());
            pods::handle_pods_command(pattern.as_deref(), resolved_context.as_deref(), resolved_namespace.as_deref());
        }
        Commands::Exec { pod, command, script, context, namespace } => {
            let resolved_context = resolve_context_if_provided(context.as_deref());
            let resolved_namespace = resolve_namespace_if_provided(namespace.as_deref(), resolved_context.as_deref());
            exec::handle_exec_command(
                pod, 
                command.as_deref(), 
                script.as_deref(), 
                resolved_context.as_deref(), 
                resolved_namespace.as_deref()
            );
        }
        Commands::Smart { command } => {
            smart::process_smart_command(command);
        }
    }
}

/// Resolve context pattern if provided, otherwise return None
fn resolve_context_if_provided(context_pattern: Option<&str>) -> Option<String> {
    context_pattern.and_then(|pattern| context::resolve_context_pattern(pattern))
}

/// Resolve namespace pattern if provided, otherwise return None
fn resolve_namespace_if_provided(namespace_pattern: Option<&str>, context: Option<&str>) -> Option<String> {
    namespace_pattern.and_then(|pattern| namespace::resolve_namespace_pattern(pattern, context))
}
