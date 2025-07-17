use crate::display;
use std::io::{self, Write};

/// Generic function to handle user selection from multiple options
/// Returns None if no matches, Some(selected_item) if single match or user selection
pub fn select_from_matches<T: Clone + std::fmt::Display>(
    matches: Vec<T>, 
    pattern: &str, 
    resource_type: &str
) -> Option<T> {
    match matches.len() {
        0 => {
            display::print_error(&format!("No {} found matching pattern: '{}'", resource_type, pattern));
            None
        }
        1 => {
            // Exactly one match - use it automatically
            let item = &matches[0];
            display::print_success(&format!("Found {}: {}", resource_type, item));
            Some(item.clone())
        }
        _ => {
            // Multiple matches - let user choose with beautiful table
            display::print_selection_table(&matches, resource_type, None);
            
            let choice = prompt_user_choice(matches.len(), resource_type);
            choice.map(|index| matches[index].clone())
        }
    }
}

/// Prompt user to choose from multiple options with retry logic
fn prompt_user_choice(max_options: usize, resource_type: &str) -> Option<usize> {
    display::print(&format!("\nSelect {} (1-{}, or 'q' to quit): ", resource_type, max_options));
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
                    display::print_error(&format!("Invalid selection. Please enter a number between 1 and {} or 'q' to quit.", max_options));
                    prompt_user_choice(max_options, resource_type) // Recursive call for retry
                }
            }
        }
        Err(_) => {
            // Handle Ctrl+C or read error as cancellation
            None
        }
    }
}

/// Prompt user for yes/no confirmation
pub fn prompt_for_confirmation(message: &str) -> bool {
    display::print(&format!("❓ {} [y/N]: ", message));
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
