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
    handle_config_command
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
            handle_pods_command(pattern.as_deref(), context.as_deref(), namespace.as_deref());
        }
        Commands::Pod { pattern, context, namespace } => {
            handle_pods_command(pattern.as_deref(), context.as_deref(), namespace.as_deref());
        }
        Commands::Exec { pod, command, script, context, namespace } => {
            handle_exec_command(
                pod, 
                command.as_deref(), 
                script.as_deref(), 
                context.as_deref(), 
                namespace.as_deref()
            );
        }
        Commands::Config { command } => {
            handle_config_command(command.as_ref());
        }
    }
}
