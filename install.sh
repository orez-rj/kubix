#!/usr/bin/env bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
REPO_OWNER="orezra"
REPO_NAME="kubix"
VERSION=""
INSTALL_DIR=""
FORCE=false
UNINSTALL=false

# Help function
show_help() {
    cat << EOF
Kubix Installation Script

USAGE:
    $0 [OPTIONS]

OPTIONS:
    -v, --version VERSION    Install specific version (e.g., v0.1.0, latest)
    -d, --dir DIRECTORY      Installation directory (default: /usr/local/bin)
    -f, --force             Force overwrite existing installation
    -u, --uninstall         Uninstall kubix
    -h, --help              Show this help message

EXAMPLES:
    $0                                    # Install latest version to /usr/local/bin
    $0 -v v0.1.0                         # Install specific version
    $0 -d ~/.local/bin                   # Install to user directory (no sudo)
    $0 -v latest -d /opt/kubix/bin       # Install latest to custom directory
    $0 --force                           # Force reinstall
    $0 --uninstall                       # Uninstall kubix
    $0 --uninstall -d ~/.local/bin       # Uninstall from custom directory

ENVIRONMENT VARIABLES:
    KUBIX_INSTALL_DIR                    # Default installation directory
    KUBIX_VERSION                        # Default version to install

EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--version)
            VERSION="$2"
            shift 2
            ;;
        -d|--dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        -f|--force)
            FORCE=true
            shift
            ;;
        -u|--uninstall)
            UNINSTALL=true
            shift
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            echo -e "${RED}Error: Unknown option $1${NC}"
            show_help
            exit 1
            ;;
    esac
done

# Use environment variables as fallbacks
VERSION=${VERSION:-${KUBIX_VERSION:-"latest"}}
INSTALL_DIR=${INSTALL_DIR:-${KUBIX_INSTALL_DIR:-"/usr/local/bin"}}

# Uninstall function
uninstall_kubix() {
    echo -e "${BLUE}🗑️  Kubix Uninstall${NC}"
    echo "==================="
    echo -e "Uninstall directory: ${GREEN}$INSTALL_DIR${NC}"
    echo ""
    
    # Detect binary name based on platform
    local binary_name="kubix"
    if [[ "$(uname -s)" == "CYGWIN"* ]] || [[ "$(uname -s)" == "MINGW"* ]] || [[ "$(uname -s)" == "MSYS"* ]]; then
        binary_name="kubix.exe"
    fi
    
    local target_path="${INSTALL_DIR}/${binary_name}"
    
    # Check if kubix is installed
    if [[ ! -f "$target_path" ]]; then
        echo -e "${YELLOW}⚠️  Kubix is not installed at $target_path${NC}"
        
        # Try to find kubix in PATH
        if command -v kubix >/dev/null 2>&1; then
            local kubix_path
            kubix_path=$(command -v kubix)
            echo -e "${BLUE}ℹ️  Found kubix at: $kubix_path${NC}"
            read -p "Remove this installation? (y/N): " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                target_path="$kubix_path"
            else
                echo -e "${BLUE}Uninstall cancelled${NC}"
                exit 0
            fi
        else
            echo -e "${BLUE}✓ Kubix is not installed or not in PATH${NC}"
            exit 0
        fi
    fi
    
    # Confirm uninstallation
    if [[ "$FORCE" != true ]]; then
        echo -e "${YELLOW}⚠️  This will remove kubix from: $target_path${NC}"
        read -p "Are you sure you want to uninstall kubix? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo -e "${BLUE}Uninstall cancelled${NC}"
            exit 0
        fi
    fi
    
    # Remove the binary
    echo -e "${BLUE}🗑️  Removing kubix...${NC}"
    if rm "$target_path"; then
        echo -e "${GREEN}✓${NC} Successfully removed: $target_path"
    else
        echo -e "${RED}Error: Could not remove $target_path${NC}"
        echo -e "${YELLOW}Try running with sudo if installed to a system directory${NC}"
        exit 1
    fi
    
    # Verify removal
    if [[ ! -f "$target_path" ]]; then
        echo -e "${GREEN}✓${NC} Kubix has been successfully uninstalled"
        
        # Check if still in PATH
        if command -v kubix >/dev/null 2>&1; then
            local remaining_path
            remaining_path=$(command -v kubix)
            echo -e "${YELLOW}⚠️  Note: kubix is still available in PATH at: $remaining_path${NC}"
            echo -e "${YELLOW}   You may have multiple installations${NC}"
        else
            echo -e "${GREEN}✓${NC} kubix is no longer available in PATH"
        fi
    else
        echo -e "${RED}✗ Uninstall verification failed${NC}"
        exit 1
    fi
    
    # Optional: Remove config (ask user)
    local config_locations=(
        "$HOME/.config/kubix"
        "$HOME/Library/Application Support/kubix"
        "$HOME/AppData/Roaming/kubix"
    )
    
    local found_config=false
    for config_dir in "${config_locations[@]}"; do
        if [[ -d "$config_dir" ]]; then
            found_config=true
            break
        fi
    done
    
    if [[ "$found_config" == true ]]; then
        echo ""
        echo -e "${BLUE}🔍 Found kubix configuration files${NC}"
        read -p "Remove configuration files as well? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            for config_dir in "${config_locations[@]}"; do
                if [[ -d "$config_dir" ]]; then
                    echo -e "${BLUE}🗑️  Removing: $config_dir${NC}"
                    rm -rf "$config_dir"
                    echo -e "${GREEN}✓${NC} Removed configuration directory"
                fi
            done
        fi
    fi
    
    echo ""
    echo -e "${GREEN}🎉 Kubix uninstall completed successfully!${NC}"
    echo ""
    echo -e "Thank you for using Kubix! 👋"
}

# Main installation or uninstall logic
if [[ "$UNINSTALL" == true ]]; then
    uninstall_kubix
    exit 0
fi

echo -e "${BLUE}🚀 Kubix Installation Script${NC}"
echo "=============================="

# Detect platform and architecture
detect_platform() {
    local os arch
    
    case "$(uname -s)" in
        Linux*)     os="Linux" ;;
        Darwin*)    os="Darwin" ;;
        CYGWIN*|MINGW*|MSYS*) os="Windows" ;;
        *)          
            echo -e "${RED}Error: Unsupported operating system $(uname -s)${NC}"
            exit 1
            ;;
    esac
    
    case "$(uname -m)" in
        x86_64|amd64)   arch="x86_64" ;;
        arm64|aarch64)  arch="arm64" ;;
        *)              
            echo -e "${RED}Error: Unsupported architecture $(uname -m)${NC}"
            exit 1
            ;;
    esac
    
    # Determine file extension and binary name
    if [[ "$os" == "Windows" ]]; then
        BINARY_NAME="kubix.exe"
        ARCHIVE_EXT="zip"
    else
        BINARY_NAME="kubix"
        ARCHIVE_EXT="tar.gz"
    fi
    
    # Special handling for Linux - prefer musl for better compatibility
    if [[ "$os" == "Linux" ]]; then
        PLATFORM_NAME="kubix-${os}-${arch}-musl.${ARCHIVE_EXT}"
    else
        PLATFORM_NAME="kubix-${os}-${arch}.${ARCHIVE_EXT}"
    fi
    
    echo -e "${GREEN}✓${NC} Detected platform: ${os} ${arch}"
}

# Get latest version from GitHub API
get_latest_version() {
    echo -e "${BLUE}📡 Fetching latest version...${NC}"
    
    local api_url="https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest"
    local latest_version
    
    if command -v curl >/dev/null 2>&1; then
        latest_version=$(curl -s "$api_url" | grep '"tag_name":' | sed -E 's/.*"tag_name": "([^"]+)".*/\1/')
    elif command -v wget >/dev/null 2>&1; then
        latest_version=$(wget -qO- "$api_url" | grep '"tag_name":' | sed -E 's/.*"tag_name": "([^"]+)".*/\1/')
    else
        echo -e "${RED}Error: Neither curl nor wget found. Please install one of them.${NC}"
        exit 1
    fi
    
    if [[ -z "$latest_version" ]]; then
        echo -e "${RED}Error: Could not fetch latest version${NC}"
        exit 1
    fi
    
    echo "$latest_version"
}

# Download and extract binary
download_and_install() {
    local version="$1"
    local download_url="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/${version}/${PLATFORM_NAME}"
    local temp_dir
    temp_dir=$(mktemp -d)
    
    echo -e "${BLUE}📥 Downloading ${PLATFORM_NAME}...${NC}"
    echo "   URL: $download_url"
    
    # Download the archive
    if command -v curl >/dev/null 2>&1; then
        if ! curl -L -o "${temp_dir}/${PLATFORM_NAME}" "$download_url"; then
            echo -e "${RED}Error: Download failed${NC}"
            exit 1
        fi
    elif command -v wget >/dev/null 2>&1; then
        if ! wget -O "${temp_dir}/${PLATFORM_NAME}" "$download_url"; then
            echo -e "${RED}Error: Download failed${NC}"
            exit 1
        fi
    fi
    
    echo -e "${GREEN}✓${NC} Downloaded successfully"
    
    # Extract the binary
    echo -e "${BLUE}📦 Extracting binary...${NC}"
    cd "$temp_dir"
    
    if [[ "$ARCHIVE_EXT" == "tar.gz" ]]; then
        tar -xzf "$PLATFORM_NAME"
    elif [[ "$ARCHIVE_EXT" == "zip" ]]; then
        if command -v unzip >/dev/null 2>&1; then
            unzip -q "$PLATFORM_NAME"
        else
            echo -e "${RED}Error: unzip not found. Please install unzip.${NC}"
            exit 1
        fi
    fi
    
    # Verify binary exists
    if [[ ! -f "$BINARY_NAME" ]]; then
        echo -e "${RED}Error: Binary $BINARY_NAME not found in archive${NC}"
        exit 1
    fi
    
    # Make binary executable
    chmod +x "$BINARY_NAME"
    
    echo -e "${GREEN}✓${NC} Extracted successfully"
    
    # Create installation directory if it doesn't exist
    if [[ ! -d "$INSTALL_DIR" ]]; then
        echo -e "${BLUE}📁 Creating installation directory: $INSTALL_DIR${NC}"
        if ! mkdir -p "$INSTALL_DIR"; then
            echo -e "${RED}Error: Could not create directory $INSTALL_DIR${NC}"
            echo -e "${YELLOW}Try running with sudo or choose a different directory with -d flag${NC}"
            exit 1
        fi
    fi
    
    # Check if kubix already exists
    local target_path="${INSTALL_DIR}/${BINARY_NAME}"
    if [[ -f "$target_path" ]] && [[ "$FORCE" != true ]]; then
        echo -e "${YELLOW}⚠️  Kubix is already installed at $target_path${NC}"
        echo -e "${YELLOW}   Use --force to overwrite, or choose a different directory with -d${NC}"
        read -p "Overwrite existing installation? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo -e "${BLUE}Installation cancelled${NC}"
            exit 0
        fi
    fi
    
    # Install the binary
    echo -e "${BLUE}📥 Installing to $target_path...${NC}"
    if ! cp "$BINARY_NAME" "$target_path"; then
        echo -e "${RED}Error: Could not copy binary to $target_path${NC}"
        echo -e "${YELLOW}Try running with sudo or choose a different directory with -d flag${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✓${NC} Installed successfully"
    
    # Cleanup
    cd - > /dev/null
    rm -rf "$temp_dir"
}

# Verify installation
verify_installation() {
    local target_path="${INSTALL_DIR}/${BINARY_NAME}"
    
    echo -e "${BLUE}🔍 Verifying installation...${NC}"
    
    if [[ -f "$target_path" ]]; then
        echo -e "${GREEN}✓${NC} Binary installed at: $target_path"
        
        # Check if it's in PATH
        if command -v kubix >/dev/null 2>&1; then
            echo -e "${GREEN}✓${NC} kubix is available in PATH"
            local installed_version
            installed_version=$(kubix --version 2>/dev/null || echo "version check failed")
            echo -e "${GREEN}✓${NC} Version: $installed_version"
        else
            echo -e "${YELLOW}⚠️  kubix is not in your PATH${NC}"
            echo -e "${YELLOW}   Add $INSTALL_DIR to your PATH, or run: export PATH=\"$INSTALL_DIR:\$PATH\"${NC}"
        fi
    else
        echo -e "${RED}✗ Installation verification failed${NC}"
        exit 1
    fi
}

# Main installation flow
main() {
    echo -e "Version: ${GREEN}$VERSION${NC}"
    echo -e "Install directory: ${GREEN}$INSTALL_DIR${NC}"
    echo ""
    
    # Detect platform
    detect_platform
    
    # Get version to install
    if [[ "$VERSION" == "latest" ]]; then
        VERSION=$(get_latest_version)
        echo -e "${GREEN}✓${NC} Latest version: $VERSION"
    else
        echo -e "${GREEN}✓${NC} Target version: $VERSION"
    fi
    
    # Download and install
    download_and_install "$VERSION"
    
    # Verify installation
    verify_installation
    
    echo ""
    echo -e "${GREEN}🎉 Kubix installation completed successfully!${NC}"
    echo ""
    echo -e "Try running: ${BLUE}kubix --help${NC}"
    echo -e "Get started: ${BLUE}kubix ctx${NC}"
    echo ""
    echo -e "To uninstall: ${BLUE}$0 --uninstall${NC}"
}

# Run main function
main 