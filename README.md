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
