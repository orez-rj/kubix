use crate::utils;

/// Build kubectl command args with optional context and namespace
pub fn build_args(
    base_args: &[&str], 
    context: Option<&str>, 
    namespace: Option<&str>
) -> Vec<String> {
    let mut args = Vec::new();
    
    if let Some(ctx) = context {
        args.extend_from_slice(&["--context".to_string(), ctx.to_string()]);
    }
    
    if let Some(ns) = namespace {
        args.extend_from_slice(&["-n".to_string(), ns.to_string()]);
    }
    
    args.extend(base_args.iter().map(|s| s.to_string()));
    
    args
}

/// Execute a kubectl command with context and namespace support
pub fn execute_with_context(
    base_args: &[&str],
    context: Option<&str>,
    namespace: Option<&str>
) -> Result<String, String> {
    let args = build_args(base_args, context, namespace);
    let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    utils::execute_kubectl(&args_refs)
}

/// Execute an interactive kubectl command with context and namespace support
pub fn execute_interactive_with_context(
    base_args: &[&str],
    context: Option<&str>,
    namespace: Option<&str>
) -> bool {
    let args = build_args(base_args, context, namespace);
    let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    utils::execute_kubectl_interactive(&args_refs)
} 