#!/usr/bin/env bash
set -euo pipefail

REPO="yinguobing/qweather-skill"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Detect OS and architecture
detect_target() {
    local os arch
    os=$(uname -s)
    arch=$(uname -m)

    case "$os" in
        Linux)
            case "$arch" in
                x86_64)  echo "x86_64-unknown-linux-musl" ;;
                aarch64) echo "aarch64-unknown-linux-musl" ;;
                *)       echo "Unsupported architecture: $arch" >&2; exit 1 ;;
            esac
            ;;
        Darwin)
            case "$arch" in
                x86_64)  echo "x86_64-apple-darwin" ;;
                arm64)   echo "aarch64-apple-darwin" ;;
                *)       echo "Unsupported architecture: $arch" >&2; exit 1 ;;
            esac
            ;;
        *)
            echo "Unsupported OS: $os" >&2
            exit 1
            ;;
    esac
}

# Fetch the latest release tag from GitHub API
get_latest_version() {
    curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
}

main() {
    echo "==> Detecting target..."
    target=$(detect_target)
    echo "    Target: $target"

    echo "==> Fetching latest release..."
    version=$(get_latest_version)
    if [ -z "$version" ]; then
        echo "ERROR: Could not determine latest release version." >&2
        exit 1
    fi
    echo "    Version: $version"

    asset="qweather-${version}-${target}.tar.gz"
    url="https://github.com/$REPO/releases/download/$version/$asset"

    tmpdir=$(mktemp -d)
    trap 'rm -rf "$tmpdir"' EXIT

    echo "==> Downloading $asset..."
    curl -sSL -o "$tmpdir/$asset" "$url"

    echo "==> Extracting..."
    tar -xzf "$tmpdir/$asset" -C "$tmpdir"

    echo "==> Installing to $INSTALL_DIR..."
    mkdir -p "$INSTALL_DIR"
    mv "$tmpdir/qweather" "$INSTALL_DIR/qweather"

    # Warn if install dir is not in PATH
    case ":$PATH:" in
        *":$INSTALL_DIR:"*) ;;
        *)
            echo ""
            echo "⚠️  Warning: $INSTALL_DIR is not in your PATH."
            echo "    Add the following line to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
            echo ""
            echo "    export PATH=\"$INSTALL_DIR:\$PATH\""
            echo ""
            ;;
    esac

    echo "==> Done. Installed:"
    "$INSTALL_DIR/qweather" --version || true
}

main "$@"
