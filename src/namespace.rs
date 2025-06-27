use crate::utils;
use std::io::{self, Write};

/// Resolve a namespace pattern to an exact namespace name
/// Returns None if user cancels, exits process if no matches found
pub fn resolve_namespace_pattern(pattern: &str, context: Option<&str>) -> Option<String> {
    // Get all namespaces
    let namespaces = match get_all_namespaces(context) {
        Ok(namespaces) => namespaces,
        Err(error) => {
            utils::print_error_and_exit(&format!("Error getting namespaces: {}", error));
            return None;
        }
    };
    
    // Find matching namespaces
    let matches: Vec<&String> = namespaces
        .iter()
        .filter(|namespace| namespace.contains(pattern))
        .collect();
    
    match matches.len() {
        0 => {
            utils::print_error_and_exit(&format!("No namespaces found matching pattern: '{}'", pattern));
            None
        }
        1 => {
            // Single match - return it
            Some(matches[0].clone())
        }
        _ => {
            // Multiple matches - let user choose
            println!("🔍 Found {} namespaces matching '{}':", matches.len(), pattern);
            for (i, namespace) in matches.iter().enumerate() {
                println!("  {}. {}", i + 1, namespace);
            }
            
            let choice = prompt_user_choice(matches.len());
            choice.map(|index| matches[index].clone())
        }
    }
}

/// Get all available namespaces as a vector
fn get_all_namespaces(context: Option<&str>) -> Result<Vec<String>, String> {
    let args = vec!["get", "namespaces", "-o", "name"];
    let mut cmd_args = Vec::new();
    
    if let Some(ctx) = context {
        cmd_args.extend(&["--context", ctx]);
    }
    cmd_args.extend(&args);
    
    match utils::execute_kubectl(&cmd_args) {
        Ok(output) => {
            let namespaces: Vec<String> = output
                .lines()
                .map(|line| line.trim_start_matches("namespace/").trim().to_string())
                .filter(|line| !line.is_empty())
                .collect();
            Ok(namespaces)
        }
        Err(error) => Err(error),
    }
}

/// Prompt user to choose from multiple options
fn prompt_user_choice(max_options: usize) -> Option<usize> {
    print!("\nSelect namespace (1-{}, or 'q' to quit): ", max_options);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim();
            
            if input.eq_ignore_ascii_case("q") || input.eq_ignore_ascii_case("quit") {
                return None;
            }
            
            match input.parse::<usize>() {
                Ok(num) if num >= 1 && num <= max_options => Some(num - 1),
                _ => {
                    println!("❌ Invalid selection. Please enter a number between 1 and {} or 'q' to quit.", max_options);
                    prompt_user_choice(max_options) // Recursive call for retry
                }
            }
        }
        Err(_) => {
            println!("❌ Failed to read input.");
            None
        }
    }
} 