# justfile for kagimcp-zed project

# List available commands
default:
    @just --list

# Build for current platform with updated versions
build:
    #!/usr/bin/env bash
    set -euo pipefail
    
    # === VERSION MANAGEMENT ===
    echo "Setting versions based on git tags..."
    
    # Check if necessary tools are installed
    command -v cargo >/dev/null 2>&1 || { echo "cargo is required but not installed. Aborting."; exit 1; }
    command -v cargo-edit >/dev/null 2>&1 || { echo "cargo-edit is required but not installed. Installing..."; cargo install cargo-edit; }
    command -v sd >/dev/null 2>&1 || { echo "sd is required but not installed. Installing..."; cargo install sd; }
    
    # Get latest tag
    LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.1")
    
    # Try to get exact tag match at HEAD
    if TAG_AT_HEAD=$(git describe --exact-match --tags HEAD 2>/dev/null); then
        # HEAD is at a tag, use that version
        VERSION=${TAG_AT_HEAD#v}
        echo "HEAD is at tag: $TAG_AT_HEAD, setting version to $VERSION"
    else
        # HEAD is not at a tag, increment patch and add SNAPSHOT
        VERSION=${LATEST_TAG#v}
        
        # Split version into components
        IFS='.' read -r MAJOR MINOR PATCH <<< "$VERSION"
        
        # Extract just the number part from PATCH if it has a dash
        if [[ "$PATCH" == *"-"* ]]; then
            PATCH=${PATCH%%-*}
        fi
        
        # Increment patch
        PATCH=$((PATCH + 1))
        
        # Get commit timestamp
        COMMIT_TIME=$(git show -s --format=%ct HEAD)
        
        # Set version with SNAPSHOT
        VERSION="$MAJOR.$MINOR.$PATCH-SNAPSHOT-$COMMIT_TIME"
        echo "HEAD is not at a tag. Setting version to $VERSION"
    fi
    
    # Update versions in toml files
    echo "Updating version to $VERSION in all workspace crates..."
    cargo set-version --workspace "$VERSION"
    
    # Update extension.toml
    echo "Updating version in extension.toml..."
    sd '^version = "[^"]*"' "version = \"$VERSION\"" extension.toml
    
    echo "Version updated to $VERSION in all files."
    
    # === BUILD PROCESS ===
    echo "Building binary for current platform..."
    
    # Detect platform and set variables accordingly
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        TARGET="x86_64-unknown-linux-gnu"
        EXT=""
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # Detect macOS architecture
        if [[ "$(uname -m)" == "arm64" ]]; then
            TARGET="aarch64-apple-darwin"
        else
            TARGET="x86_64-apple-darwin"
        fi
        EXT=""
    elif [[ "$OSTYPE" == "msys"* ]] || [[ "$OSTYPE" == "cygwin"* ]] || [[ "$OSTYPE" == "win32"* ]]; then
        TARGET="x86_64-pc-windows-msvc"
        EXT=".exe"
    else
        echo "Unsupported platform: $OSTYPE"
        exit 1
    fi
    
    echo "Building for target: $TARGET"
    
    # Build the workspace
    cargo build --release --workspace --target "$TARGET"
    
    # Create dist directory if it doesn't exist
    mkdir -p dist
    
    # Copy binary to dist directory
    cp "target/$TARGET/release/kagi-mcp-server$EXT" "dist/kagi-mcp-server$EXT"
    
    echo "Binary built and copied to dist/kagi-mcp-server$EXT"
    echo "Build complete with version $VERSION"