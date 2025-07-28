#!/usr/bin/env bash
#----------------------------------------------------------------------------------
# Kubix installer / un‑installer
#----------------------------------------------------------------------------------
set -euo pipefail

# Always reset colours even on abrupt exit
trap 'echo -e "${NC:-}"' EXIT

# ────────────────────────────────────────────────────────────────────────────────
# Colours
# ────────────────────────────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# ────────────────────────────────────────────────────────────────────────────────
# Defaults / globals
# ────────────────────────────────────────────────────────────────────────────────
REPO_OWNER="orez-rj"
REPO_NAME="kubix"
VERSION=""
INSTALL_DIR=""
FORCE=false
UNINSTALL=false
BINARY_NAME="kubix"        # updated later for Windows
ARCHIVE_EXT="tar.gz"       # updated later
PLATFORM_NAME=""          # late‑bound

# ────────────────────────────────────────────────────────────────────────────────
# Helpers
# ────────────────────────────────────────────────────────────────────────────────
fetch() {
  # silent download to stdout; honour proxy env vars; fail on HTTP error
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$1"
  else
    wget -qO- "$1"
  fi
}

say()  { echo -e "$*${NC}"; }
info() { say "${BLUE}$*"; }
ok()   { say "${GREEN}✓ $*"; }
warn() { say "${YELLOW}⚠️  $*"; }
err()  { say "${RED}Error: $*"; exit 1; }

# ────────────────────────────────────────────────────────────────────────────────
# Usage
# ────────────────────────────────────────────────────────────────────────────────
show_help() {
  cat <<'EOF'
Kubix Installation Script

USAGE:
    install_kubix.sh [OPTIONS]

OPTIONS:
    -v, --version VERSION    Install specific version (e.g. v0.1.0 or 0.1.0). Default: latest
    -d, --dir DIRECTORY      Installation directory (default: /usr/local/bin or \$KUBIX_INSTALL_DIR)
    -f, --force              Force overwrite existing installation
    -u, --uninstall          Uninstall kubix
    -h, --help               Show this help message

ENVIRONMENT VARIABLES:
    KUBIX_INSTALL_DIR        Default installation directory
    KUBIX_VERSION            Default version to install
EOF
}

# ────────────────────────────────────────────────────────────────────────────────
# Parse CLI
# ────────────────────────────────────────────────────────────────────────────────
while [[ $# -gt 0 ]]; do
  case $1 in
    -v|--version) VERSION="$2"; shift 2;;
    -d|--dir)     INSTALL_DIR="$2"; shift 2;;
    -f|--force)   FORCE=true; shift;;
    -u|--uninstall) UNINSTALL=true; shift;;
    -h|--help) show_help; exit 0;;
    *) err "Unknown option $1";;
  esac
done

VERSION=${VERSION:-${KUBIX_VERSION:-"latest"}}
INSTALL_DIR=${INSTALL_DIR:-${KUBIX_INSTALL_DIR:-"/usr/local/bin"}}

# ────────────────────────────────────────────────────────────────────────────────
# Platform detection
# ────────────────────────────────────────────────────────────────────────────────
detect_platform() {
  local os arch

  case "$(uname -s)" in
    Linux*)   os="Linux"  ;;
    Darwin*)  os="Darwin" ;;
    CYGWIN*|MINGW*|MSYS*) os="Windows" ;;
    *) err "Unsupported OS $(uname -s)" ;;
  esac

  case "$(uname -m)" in
    x86_64|amd64) arch="x86_64" ;;
    arm64|aarch64) arch="arm64" ;;
    *) err "Unsupported architecture $(uname -m)" ;;
  esac

  # Rosetta: prefer native arm build when running under translation
  if [[ "$os" == "Darwin" && "$arch" == "x86_64" ]]; then
    if [[ "$(sysctl -in sysctl.proc_translated 2>/dev/null || echo 0)" == "1" ]]; then
      arch="arm64"
      warn "Detected Rosetta - selecting arm64 build"
    fi
  fi

  if [[ "$os" == "Windows" ]]; then
    BINARY_NAME="kubix.exe"
    ARCHIVE_EXT="zip"
  else
    BINARY_NAME="kubix"
    ARCHIVE_EXT="tar.gz"
  fi

  if [[ "$os" == "Linux" ]]; then
    PLATFORM_NAME="kubix-${os}-${arch}-musl.${ARCHIVE_EXT}"
  else
    PLATFORM_NAME="kubix-${os}-${arch}.${ARCHIVE_EXT}"
  fi

  ok "Detected platform: $os $arch"
}

# ────────────────────────────────────────────────────────────────────────────────
# Version helpers
# ────────────────────────────────────────────────────────────────────────────────
get_latest_version() {
  info "📡  Fetching latest version…" >&2
  local api_url="https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest"
  local latest_version
  latest_version=$(fetch "$api_url" | grep '"tag_name"' | sed -E 's/.*"tag_name": "([^"]+)".*/\1/') || true
  [[ -z "$latest_version" ]] && err "GitHub API rate-limited or network error; unable to determine latest version."
  echo "$latest_version"
}

normalise_version() {
  # Ensure the tag is prefixed with v
  local v="$1"
  [[ "$v" != v* ]] && v="v${v}"
  echo "$v"
}

# ────────────────────────────────────────────────────────────────────────────────
# Uninstall
# ────────────────────────────────────────────────────────────────────────────────
uninstall_kubix() {
  info "🗑️  Kubix Uninstall"
  local bin_name="kubix"; [[ "$(uname -s)" =~ CYGWIN|MINGW|MSYS ]] && bin_name="kubix.exe"
  local target_path="${INSTALL_DIR}/${bin_name}"

  # Check if we're running interactively
  local interactive=false
  if [[ -t 0 && -t 1 ]]; then
    interactive=true
  fi

  if [[ ! -f "$target_path" ]]; then
    warn "Kubix not found at $target_path"
    if command -v kubix >/dev/null 2>&1; then
      target_path="$(command -v kubix)"
      if [[ "$interactive" == true ]]; then
        read -r -p "Remove $target_path? (y/N): "
        [[ ! $REPLY =~ ^[Yy]$ ]] && { info "Cancelled"; exit 0; }
      elif [[ "$FORCE" != true ]]; then
        err "Non-interactive mode: use --force to confirm removal of $target_path"
      else
        info "Force removing $target_path (non-interactive mode)"
      fi
    else
      info "Nothing to do."
      exit 0
    fi
  fi

  if [[ "$FORCE" != true ]]; then
    if [[ "$interactive" == true ]]; then
      read -r -p "Confirm removal of $target_path? (y/N): "
      [[ ! $REPLY =~ ^[Yy]$ ]] && { info "Cancelled"; exit 0; }
    else
      err "Non-interactive mode: use --force to confirm removal of $target_path"
    fi
  fi

  if ! rm "$target_path" 2>/dev/null; then
    sudo rm "$target_path" || err "Could not remove $target_path"
  fi
  ok "Removed $target_path"
  exit 0
}

# ────────────────────────────────────────────────────────────────────────────────
# Download, verify, install
# ────────────────────────────────────────────────────────────────────────────────
install_kubix() {
  local tag="$1"; tag=$(normalise_version "$tag")
  local url_base="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/${tag}"
  local archive_url="${url_base}/${PLATFORM_NAME}"
  local checksum_url="${archive_url}.sha256"
  local tmp
  tmp=$(mktemp -d)

  info "📥  Downloading $PLATFORM_NAME"
  info "     $archive_url"

  if ! (curl -fL -o "$tmp/$PLATFORM_NAME" "$archive_url" 2>/dev/null || wget -q -O "$tmp/$PLATFORM_NAME" "$archive_url"); then
    err "Download failed - binary not found for $PLATFORM_NAME (tag ${tag})."
  fi
  ok "Download complete"

  # checksum
  if command -v shasum >/dev/null 2>&1; then
    if ! (curl -fL -o "$tmp/$PLATFORM_NAME.sha256" "$checksum_url" 2>/dev/null || wget -q -O "$tmp/$PLATFORM_NAME.sha256" "$checksum_url"); then
      warn "Checksum file missing - skipping verification"
    else
      (cd "$tmp" && shasum -a 256 -c "$PLATFORM_NAME.sha256") || err "Checksum mismatch"
      ok "Checksum verified"
    fi
  fi

  info "📦  Extracting"
  (cd "$tmp" && {
    if [[ "$ARCHIVE_EXT" == "tar.gz" ]]; then
      tar -xzf "$PLATFORM_NAME"
    else
      unzip -q "$PLATFORM_NAME"
    fi
  })
  [[ ! -f "$tmp/$BINARY_NAME" ]] && err "Binary $BINARY_NAME not found in archive"
  chmod +x "$tmp/$BINARY_NAME"

  # ensure destination directory exists
  if [[ ! -d "$INSTALL_DIR" ]]; then
    info "Creating $INSTALL_DIR"
    mkdir -p "$INSTALL_DIR" 2>/dev/null || sudo mkdir -p "$INSTALL_DIR" || err "Cannot create $INSTALL_DIR"
  fi

  local destination="${INSTALL_DIR}/${BINARY_NAME}"
  if [[ -f "$destination" && "$FORCE" != true ]]; then
    warn "Kubix already exists at $destination"
    read -r -p "Overwrite? (y/N): "
    [[ ! $REPLY =~ ^[Yy]$ ]] && { info "Cancelled"; exit 0; }
  fi

  info "Installing to $destination"
  if ! cp "$tmp/$BINARY_NAME" "$destination" 2>/dev/null; then
    sudo cp "$tmp/$BINARY_NAME" "$destination" || err "Copy failed"
  fi
  ok "Installed"

  rm -rf "$tmp"
}

verify_install() {
  local cmd="${INSTALL_DIR}/${BINARY_NAME}"
  info "🔍  Verifying install"
  "$cmd" --version || warn "Version check failed"
  if ! command -v kubix >/dev/null 2>&1; then
    warn "kubix not in PATH - append 'export PATH=\"$INSTALL_DIR:$PATH\"' to your shell rc file."
  fi
  ok "Installation verified"
}

# ────────────────────────────────────────────────────────────────────────────────
# Main
# ────────────────────────────────────────────────────────────────────────────────
[[ "$UNINSTALL" == true ]] && { uninstall_kubix; }

echo -e "${BLUE}🚀 Kubix Installer${NC}"

detect_platform

if [[ "$VERSION" == "latest" ]]; then
  VERSION=$(get_latest_version)
fi
ok "Version to install: $VERSION"

install_kubix "$VERSION"
verify_install

echo -e "\n${GREEN}🎉  Kubix installation complete${NC}"
info "Run: kubix --help"
