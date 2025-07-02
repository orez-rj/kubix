# Kubix - Smart kubectl Wrapper

A user-friendly CLI tool that wraps kubectl to provide enhanced functionality for managing Kubernetes clusters.

## Features

- 🎯 **Smart context management** with fuzzy matching
- 🔍 **Pod discovery** with pattern matching
- 🌐 **Namespace resolution** with pattern matching
- ⚡ **Unified exec command** for bash, commands, and scripts
- 📄 **Configuration system** with command and script nicknames
- 🐚 **Easy shell access** to pods
- 🧠 **Natural language commands** for complex operations

## Installation

```bash
cargo build --release
# The binary will be available at target/release/kubix
```

## Usage

### Context Management

```bash
# List all contexts with current context marked
kubix ctx

# Switch to context by exact name
kubix ctx production

# Fuzzy match context names - if multiple matches, you'll be prompted to choose
kubix ctx prod  # might match: production, prod-us, prod-eu

# Examples of fuzzy matching:
kubix ctx us     # matches: us-prod, us-staging, etc.
kubix ctx dev    # matches: development, dev-cluster, etc.
```

### Pod Management

```bash
# List all pods in current context (both commands work the same)
kubix pods
kubix pod

# List pods matching a pattern
kubix pods web         # Shows all pods containing "web"
kubix pod api          # Shows all pods containing "api"
kubix pods nginx       # Shows all pods containing "nginx"

# Use pattern matching for context and namespace
kubix pods web --context prod          # Context pattern matching
kubix pod api -c staging -n kube       # Both context and namespace patterns

# Examples with pattern matching:
kubix pods --context prod              # Matches: production, prod-us, etc.
kubix pods --namespace kube            # Matches: kube-system, kube-public, etc.
kubix pods web -c dev -n default       # Combines all pattern matching
```

### Unified Exec Command

The `exec` command is your one-stop solution for interacting with pods:

```bash
# Open bash shell (default behavior)
kubix exec web-pod
kubix exec api --context prod

# Run specific commands
kubix exec web-pod --command "python manage.py migrate"
kubix exec api-pod -c "ls -la /app" --context production

# Execute local scripts
kubix exec web-pod --script ./deploy.sh
kubix exec api-pod -s ./setup.py --context production

# Use command nicknames from config
kubix exec web-pod -c shell          # Runs configured "shell" command
kubix exec worker -c migrate         # Runs configured "migrate" command

# Use script nicknames from config
kubix exec web-pod -s deploy         # Runs configured "deploy" script
kubix exec api-pod -s setup          # Runs configured "setup" script

# With pattern matching for context/namespace
kubix exec web -c shell --context prod --namespace frontend
```

### Configuration System

Kubix automatically creates a configuration file using platform-appropriate directories:

**Example config file:**
```toml
[commands]
shell = "python manage.py shell"
migrate = "python manage.py migrate"  
console = "rails console"
logs = "tail -f /var/log/app.log"
ps = "ps aux"
env = "printenv"

[scripts]
deploy = "./scripts/deploy.sh"
setup = "./scripts/setup.py"
backup = "~/scripts/backup.sh"
```

**Usage:**
```bash
# These are equivalent:
kubix exec web-pod -c "python manage.py shell"
kubix exec web-pod -c shell

# These are equivalent:
kubix exec web-pod -s "./scripts/deploy.sh"
kubix exec web-pod -s deploy
```

## Examples

### Typical Workflows

1. **Check and switch contexts:**
   ```bash
   kubix ctx                    # See all contexts
   kubix ctx prod              # Switch to production-like context
   ```

2. **Find and access pods with pattern matching:**
   ```bash
   kubix pods                   # List all pods
   kubix pods web              # Show only pods matching "web"
   kubix pods --context prod   # List pods in production-like context
   kubix exec web              # Open bash in web pod
   ```

3. **Execute common commands:**
   ```bash
   kubix exec web -c shell             # Django/Python shell
   kubix exec web -c migrate           # Run migrations
   kubix exec worker -c logs           # View logs
   kubix exec api -c ps --context prod # Check processes in production
   ```

4. **Deploy and maintenance:**
   ```bash
   kubix exec web -s deploy --context staging    # Deploy to staging
   kubix exec web -s backup --context prod       # Run backup script
   kubix exec api -s setup --context dev         # Setup development environment
   ```

5. **Cross-environment operations:**
   ```bash
   kubix exec web -c migrate --context prod --namespace backend
   kubix exec worker -c logs --context staging --namespace queue
   ```

6. **Pattern matching workflows:**
   ```bash
   kubix pods --namespace kube        # Lists pods in kube-system, kube-public, etc.
   kubix exec api -c shell -n monitor # Shell in monitoring namespace
   ```

## Command Reference

| Command | Description | Example |
|---------|-------------|---------|
| `kubix ctx [pattern]` | List contexts or switch by pattern | `kubix ctx prod` |
| `kubix pods [pattern]` | List all pods or filter by pattern | `kubix pods web -c prod` |
| `kubix pod [pattern]` | Same as pods (alias) | `kubix pod api -n kube` |
| `kubix exec <pod>` | Open bash shell in pod | `kubix exec web` |
| `kubix exec <pod> -c <cmd>` | Run command on pod | `kubix exec api -c shell` |
| `kubix exec <pod> -s <script>` | Execute script on pod | `kubix exec web -s deploy` |
| `kubix smart "<command>"` | Natural language command | `kubix smart "bash to pod web"` |

## Pattern Matching

All commands now support intelligent pattern matching for contexts and namespaces:

### Context Patterns
- `--context prod` matches: `production`, `prod-us`, `prod-eu`, etc.
- `--context dev` matches: `development`, `dev-cluster`, `dev-staging`, etc.
- `--context us` matches: `us-prod`, `us-staging`, `us-west`, etc.

### Namespace Patterns
- `--namespace kube` matches: `kube-system`, `kube-public`, `kube-dns`, etc.
- `--namespace monitor` matches: `monitoring`, `monitor-system`, etc.
- `--namespace data` matches: `database`, `data-warehouse`, etc.

### Interactive Selection
When multiple matches are found, kubix will:
1. Display all matching options with numbers
2. Prompt you to select one
3. Allow you to quit with 'q'

## Configuration File

### Location
- Linux: `~/.config/kubix/kubix.toml`
- macOS: `~/Library/Application Support/kubix/kubix.toml`
- Windows: `%APPDATA%\kubix\kubix.toml`

### Structure
```toml
[commands]
nickname = "actual command"

[scripts]  
nickname = "path/to/script"
```

### Auto-Creation
The config file is automatically created with examples on first run. You can edit it to add your own commands and scripts.

## Options

All commands support these optional flags with pattern matching:
- `--context, -x`: Specify kubectl context (supports patterns)
- `--namespace, -n`: Specify kubernetes namespace (supports patterns)

## Tips

1. **Pattern Matching**: Context, namespace, and pod names all support partial matching
2. **Interactive Selection**: When multiple matches are found, you'll be prompted to choose
3. **Current Context**: The `kubix ctx` command shows your current context with a 🔹 marker
4. **Pod Filtering**: Use `kubix pods <pattern>` to quickly find specific pods
5. **Command Aliases**: Both `pod` and `pods` work the same way
6. **Configuration**: Edit `~/.config/kubix/config.toml` to add your custom commands and scripts
7. **Environment Switching**: Use context patterns to quickly switch between environments
8. **Namespace Discovery**: Use namespace patterns to find resources across namespaces
9. **Unified Exec**: One command for bash, commands, and scripts - no need to remember multiple commands
10. **Smart Commands**: Use single quotes around commands and script paths in smart commands

## Dependencies

- `kubectl` must be installed and configured
- Rust 1.70+ for building from source
