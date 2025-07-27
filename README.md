# Kubix - Smart kubectl Wrapper

A user-friendly CLI tool that wraps kubectl to provide enhanced functionality for managing Kubernetes clusters.

## Features

- 🎯 **Smart context management** with fuzzy matching
- 🔍 **Pod discovery** with pattern matching
- 🌐 **Namespace resolution** with pattern matching
- ⚡ **Unified exec command** for bash, commands, and scripts
- 📄 **Configuration system** with command and script nicknames
- 🐚 **Easy shell access** to pods
- 📊 **Pod logs viewing** with advanced options

## Table of Contents

- [Installation](#installation)
  - [Quick Install (Recommended)](#quick-install-recommended)
  - [Installation Options](#installation-options)
  - [Environment Variables](#environment-variables)
  - [Manual Installation](#manual-installation)
  - [Building from Source](#building-from-source)
  - [Installation Directories](#installation-directories)
  - [Updating Kubix](#updating-kubix)
- [Uninstalling Kubix](#uninstalling-kubix)
  - [Quick Uninstall](#quick-uninstall)
  - [What Gets Removed](#what-gets-removed)
  - [Manual Uninstall](#manual-uninstall)
  - [Multiple Installations](#multiple-installations)
- [Usage](#usage)
  - [Context Management](#context-management)
  - [Pod Management](#pod-management)
  - [Unified Exec Command](#unified-exec-command)
  - [Pod Logs](#pod-logs)
  - [Configuration System](#configuration-system)
- [Examples](#examples)
  - [Typical Workflows](#typical-workflows)
- [Command Reference](#command-reference)
- [Pattern Matching](#pattern-matching)
  - [Context Patterns](#context-patterns)
  - [Namespace Patterns](#namespace-patterns)
  - [Interactive Selection](#interactive-selection)
- [Configuration File](#configuration-file)
  - [Location](#location)
  - [Structure](#structure)
  - [Auto-Creation](#auto-creation)
  - [Advanced Features](#advanced-features)
- [Options](#options)
- [Tips](#tips)
- [Dependencies](#dependencies)

## Installation

### Quick Install (Recommended)

The easiest way to install Kubix is using our installation script:

```bash
# Install latest version to /usr/local/bin (requires sudo)
curl -sSfL https://raw.githubusercontent.com/orezra/kubix/main/install.sh | bash

# Or using wget
wget -qO- https://raw.githubusercontent.com/orezra/kubix/main/install.sh | bash
```

### Installation Options

The installation script supports various options for different use cases:

```bash
# Install specific version
curl -sSfL https://raw.githubusercontent.com/orezra/kubix/main/install.sh | bash -s -- -v v0.1.0

# Install to user directory (no sudo required)
curl -sSfL https://raw.githubusercontent.com/orezra/kubix/main/install.sh | bash -s -- -d ~/.local/bin

# Install latest version to custom directory
curl -sSfL https://raw.githubusercontent.com/orezra/kubix/main/install.sh | bash -s -- -d /opt/kubix/bin

# Force reinstall/upgrade
curl -sSfL https://raw.githubusercontent.com/orezra/kubix/main/install.sh | bash -s -- --force

# View all options
curl -sSfL https://raw.githubusercontent.com/orezra/kubix/main/install.sh | bash -s -- --help
```

### Environment Variables

You can configure default installation behavior using environment variables:

```bash
# Set default installation directory
export KUBIX_INSTALL_DIR="$HOME/.local/bin"

# Set default version
export KUBIX_VERSION="v0.1.0"

# Then install with defaults
curl -sSfL https://raw.githubusercontent.com/orezra/kubix/main/install.sh | bash
```

### Manual Installation

If you prefer to install manually or need more control:

1. **Download the binary for your platform** from the [releases page](https://github.com/orezra/kubix/releases/latest):
   - **Linux (x86_64-gnu)**: `kubix-Linux-x86_64-gnu.tar.gz`
   - **Linux (x86_64-musl)**: `kubix-Linux-x86_64-musl.tar.gz` (statically linked, more portable)
   - **Windows (x86_64)**: `kubix-Windows-x86_64.zip`
   - **macOS (Intel)**: `kubix-Darwin-x86_64.tar.gz`
   - **macOS (Apple Silicon)**: `kubix-Darwin-arm64.tar.gz`

2. **Extract and install**:
   ```bash
   # Linux/macOS
   tar -xzf kubix-*.tar.gz
   chmod +x kubix
   sudo mv kubix /usr/local/bin/
   
   # Or install to user directory (no sudo)
   mkdir -p ~/.local/bin
   mv kubix ~/.local/bin/
   export PATH="$HOME/.local/bin:$PATH"  # Add to your shell profile
   ```

   ```powershell
   # Windows (PowerShell)
   Expand-Archive kubix-Windows-x86_64.zip
   # Move kubix.exe to a directory in your PATH
   ```

3. **Verify installation**:
   ```bash
   kubix --help
   ```

### Building from Source

For development or if you prefer to build from source:

```bash
# Clone the repository
git clone https://github.com/orezra/kubix.git
cd kubix

# Build release binary
cargo build --release

# Install to PATH
sudo cp target/release/kubix /usr/local/bin/
# Or to user directory
cp target/release/kubix ~/.local/bin/
```

### Installation Directories

Choose the installation directory based on your needs:

| Directory | Scope | Sudo Required | Notes |
|-----------|-------|---------------|--------|
| `/usr/local/bin` | System-wide | ✅ Yes | Default, available to all users |
| `~/.local/bin` | User-only | ❌ No | Add to PATH: `export PATH="$HOME/.local/bin:$PATH"` |
| `/opt/kubix/bin` | System-wide | ✅ Yes | Custom system location |
| `~/bin` | User-only | ❌ No | Traditional user binary directory |

### Updating Kubix

To update to the latest version:

```bash
# Using the install script with force flag
curl -sSfL https://raw.githubusercontent.com/orezra/kubix/main/install.sh | bash -s -- --force

# Or specify a version
curl -sSfL https://raw.githubusercontent.com/orezra/kubix/main/install.sh | bash -s -- -v v0.2.0 --force
```

## Uninstalling Kubix

### Quick Uninstall

The easiest way to uninstall Kubix is using the same installation script:

```bash
# Uninstall from default location (/usr/local/bin)
curl -sSfL https://raw.githubusercontent.com/orezra/kubix/main/install.sh | bash -s -- --uninstall

# Uninstall from custom directory
curl -sSfL https://raw.githubusercontent.com/orezra/kubix/main/install.sh | bash -s -- --uninstall -d ~/.local/bin

# Force uninstall (no confirmation prompts)
curl -sSfL https://raw.githubusercontent.com/orezra/kubix/main/install.sh | bash -s -- --uninstall --force
```

### What Gets Removed

The uninstall process will:

1. **Remove the binary** from the specified directory
2. **Verify removal** and check for remaining installations
3. **Optionally remove configuration files** (asks for confirmation):
   - Linux: `~/.config/kubix/`
   - macOS: `~/Library/Application Support/kubix/`
   - Windows: `%APPDATA%\kubix\`

### Manual Uninstall

If you prefer to uninstall manually:

```bash
# Remove the binary
sudo rm /usr/local/bin/kubix
# Or from user directory
rm ~/.local/bin/kubix

# Remove configuration (optional)
rm -rf ~/.config/kubix  # Linux
rm -rf ~/Library/Application\ Support/kubix  # macOS
```

### Multiple Installations

If you have multiple installations of Kubix in different directories, the uninstall script will:
- Remove the binary from the specified directory
- Warn you if other installations are found in your PATH
- Allow you to remove each installation separately

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

### Pod Logs

View and follow pod logs with advanced options and built-in filtering:

```bash
# View logs from a pod
kubix logs web-pod
kubix log api-pod                    # 'log' is an alias for 'logs'

# Follow logs in real time
kubix logs web-pod --follow
kubix logs api-pod -f                # Short form

# Show last N lines
kubix logs web-pod --tail 50
kubix logs api-pod -t 100

# View logs from previous container instance
kubix logs web-pod --previous
kubix logs api-pod -p

# Multi-container pods - specify container
kubix logs web-pod --container nginx
kubix logs api-pod -c app

# Combine options with context/namespace patterns
kubix logs web -f -t 100 --context prod --namespace frontend
```

#### Built-in Log Filtering 🔍

Kubix includes powerful built-in filtering capabilities using regex patterns, eliminating the need for external piping:

```bash
# Filter logs to show only ERROR messages
kubix logs web-pod --grep "ERROR"

# Filter for INFO logs but exclude debug messages
kubix logs web-pod --grep "INFO" --exclude "debug"

# Exclude all WARNING messages
kubix logs web-pod --exclude "WARNING"

# Complex regex filtering
kubix logs web-pod --grep "log_level.*(ERROR|CRITICAL)"

# Case-sensitive filtering
kubix logs web-pod --grep "Error" --exclude "ErrorHandler"

# Filter with other options
kubix logs web-pod -f --tail 100 --grep "ERROR" -c nginx

# Multiple exclusion patterns
kubix logs web-pod --exclude "debug|trace|verbose"
```

**Why use built-in filtering instead of pipes?**
- ✅ **No piping issues** - Works seamlessly with kubix's enhanced output
- ✅ **Line numbering preserved** - Only filtered lines get numbered
- ✅ **Visual feedback** - Shows active filters in the header
- ✅ **Real-time filtering** - Works with follow mode (`-f`)
- ✅ **Regex validation** - Catches invalid patterns with helpful errors

**Filtering Logic:**
1. **Exclude first**: If `--exclude` matches, line is hidden
2. **Then grep**: If `--grep` is provided, line must match to be shown
3. **Visual indicators**: Active filters are shown in the enhanced header

Example with visual output:
```
════════════════════════════════════════════════════════════════════════════════
📋 Logs for pod: web-server-abc123
🏷️  Container: nginx
🌐 Context: production  
📦 Namespace: default
🔍 Grep: ERROR|CRITICAL
❌ Exclude: debug
🔄 Mode: Following (live)
💡 Tip: Press Ctrl+C to stop following
════════════════════════════════════════════════════════════════════════════════

   1 │ {"log_level": "ERROR", "message": "Database connection failed"}
   2 │ {"log_level": "CRITICAL", "message": "Service unavailable"}
   3 │ {"log_level": "ERROR", "message": "Invalid user credentials"}
```

### Configuration System

Kubix uses a sophisticated configuration system with support for command nicknames, script nicknames, and custom interpreters.

**Configuration file location:**
- Linux: `~/.config/kubix/kubix.toml`
- macOS: `~/Library/Application Support/kubix/kubix.toml`
- Windows: `%APPDATA%\kubix\kubix.toml`

**Configuration structure:**
```toml
[commands]
shell = "$BIN_PATH/python manage.py shell"
ps = "ps aux"

[scripts]
deploy = "/Users/myuser/scripts/deploy.sh"
setup = "~/scripts/setup.py"

[interpreters]
py = "/opt/app/venv/bin/python"

[settings]
script_delay_seconds = 10
```

**Managing configuration:**
```bash
# View current configuration
kubix config

# Add command nicknames
kubix config add-command shell "python manage.py shell"
kubix config add-command migrate "python manage.py migrate"

# Add script nicknames
kubix config add-script deploy "./scripts/deploy.sh"
kubix config add-script setup "~/scripts/setup.py"

# Add custom interpreters for file extensions
kubix config add-interpreter py "/opt/app/venv/bin/python"
kubix config add-interpreter js "/usr/local/bin/node"

# Remove configurations
kubix config remove-command shell
kubix config remove-script deploy
kubix config remove-interpreter py
```

**Using configuration:**
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
   kubix exec api -c ps --context prod # Check processes in production
   ```

4. **View and filter logs:**
   ```bash
   kubix logs web-pod -f               # Follow logs in real time
   kubix logs api-pod -t 50            # Show last 50 lines
   kubix logs web -f --context prod    # Follow logs in production
   kubix logs api -c nginx -f          # Follow specific container logs
   
   # Built-in filtering - no more piping issues!
   kubix logs web --grep "ERROR"       # Show only error messages
   kubix logs api --exclude "debug"    # Hide debug messages
   kubix logs web -f --grep "user.*login" --exclude "test"  # Complex filtering
   kubix logs api -t 100 --grep "log_level.*(ERROR|WARN)"   # Regex patterns
   ```

5. **Executing scripts from local machine on pod:**
   ```bash
   kubix exec web -s deploy --context staging    # Deploy to staging
   kubix exec web -s backup --context prod       # Run backup script
   kubix exec api -s setup --context dev         # Setup development environment
   ```

6. **Cross-environment operations:**
   ```bash
   kubix exec web -c migrate --context prod --namespace backend
   kubix logs worker -f --context staging --namespace queue
   ```

7. **Pattern matching workflows:**
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
| `kubix logs <pod>` | View pod logs with filtering | `kubix logs web -f -t 100 --grep "ERROR"` |
| `kubix log <pod>` | Same as logs (alias) | `kubix log api -f --exclude "debug"` |
| `kubix config` | Manage configuration | `kubix config add-command shell "python manage.py shell"` |

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

[interpreters]
extension = "interpreter_path"

[settings]
script_delay_seconds = 10
```

### Auto-Creation
The config file is automatically created with examples on first run. You can edit it manually or use the `kubix config` subcommands to manage it.

### Advanced Features

**Script Interpreters:**
Kubix supports automatic interpreter detection for scripts based on file extensions:
- `.py` → `python3` (or custom interpreter from config)
- `.js` → `node`
- `.rb` → `ruby`
- `.sh`, `.bash` → `bash`
- `.pl` → `perl`
- `.php` → `php`
- `.r` → `Rscript`
- `.lua` → `lua`
- `.scala` → `scala`
- `.groovy` → `groovy`

You can override default interpreters using the configuration system:
```bash
kubix config add-interpreter py "/opt/app/venv/bin/python"
```

## Options

All commands support these optional flags with pattern matching:
- `--context, -x`: Specify kubectl context (supports patterns)
- `--namespace, -n`: Specify kubernetes namespace (supports patterns)

## Tips

1. **Pattern Matching**: Context, namespace, and pod names all support partial matching
2. **Interactive Selection**: When multiple matches are found, you'll be prompted to choose
3. **Current Context**: The `kubix ctx` command shows your current context with a ✓ marker
4. **Pod Filtering**: Use `kubix pods <pattern>` to quickly find specific pods
5. **Command Aliases**: Both `pod` and `pods` work the same way, as do `log` and `logs`
6. **Configuration Management**: Use `kubix config` subcommands to manage your configuration
7. **Environment Switching**: Use context patterns to quickly switch between environments
8. **Namespace Discovery**: Use namespace patterns to find resources across namespaces
9. **Unified Exec**: One command for bash, commands, and scripts - no need to remember multiple commands
10. **Script Execution**: Local scripts are automatically executed with appropriate interpreters

## Dependencies

- `kubectl` must be installed and configured
- Rust 1.70+ for building from source
