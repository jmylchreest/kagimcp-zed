# justfile for kagimcp-zed project

# WASM target for Zed extensions (must be wasip2 for Component Model)
WASM_TARGET := "wasm32-wasip2"

# List available commands
default:
    @just --list

# Install required tools
install-tools:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "Checking and installing required tools..."

    # Check if necessary tools are installed
    command -v cargo >/dev/null 2>&1 || { echo "cargo is required but not installed. Aborting."; exit 1; }
    command -v cargo-edit >/dev/null 2>&1 || { echo "cargo-edit is required but not installed. Installing..."; cargo install cargo-edit; }
    command -v sd >/dev/null 2>&1 || { echo "sd is required but not installed. Installing..."; cargo install sd; }

    echo "All required tools are installed."

# Get the latest git tag
tag-get-latest:
    #!/usr/bin/env bash
    LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.1")
    echo "$LATEST_TAG"

# Get the next patch version based on latest tag
tag-get-next:
    #!/usr/bin/env bash
    LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.1")
    VERSION=${LATEST_TAG#v}

    # Split version into components
    IFS='.' read -r MAJOR MINOR PATCH <<< "$VERSION"

    # Extract just the number part from PATCH if it has a dash
    if [[ "$PATCH" == *"-"* ]]; then
        PATCH=${PATCH%%-*}
    fi

    # Increment patch
    PATCH=$((PATCH + 1))
    echo "v$MAJOR.$MINOR.$PATCH"

# Get the next release version with optional bump type (patch*, minor, major)
tag-get-next-release bump_type="patch":
    #!/usr/bin/env bash
    LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.1")
    VERSION=${LATEST_TAG#v}

    # Split version into components
    IFS='.' read -r MAJOR MINOR PATCH <<< "$VERSION"

    # Extract just the number part from PATCH if it has a dash
    if [[ "$PATCH" == *"-"* ]]; then
        PATCH=${PATCH%%-*}
    fi

    case "{{bump_type}}" in
        "major")
            MAJOR=$((MAJOR + 1))
            MINOR=0
            PATCH=0
            ;;
        "minor")
            MINOR=$((MINOR + 1))
            PATCH=0
            ;;
        "patch"|*)
            PATCH=$((PATCH + 1))
            ;;
    esac

    echo "v$MAJOR.$MINOR.$PATCH"

# Set version in all workspace files
set-version version:
    #!/usr/bin/env bash
    set -euo pipefail

    VERSION="{{version}}"
    # Remove 'v' prefix if present
    VERSION=${VERSION#v}

    echo "Updating version to $VERSION in all workspace crates..."
    cargo set-version --workspace "$VERSION"

    echo "Updating version in extension.toml..."
    sd '^version = "[^"]*"' "version = \"$VERSION\"" extension.toml

    echo "Version updated to $VERSION in all files."

# Detect current platform target
detect-target:
    #!/usr/bin/env bash
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "x86_64-unknown-linux-gnu"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # Detect macOS architecture
        if [[ "$(uname -m)" == "arm64" ]]; then
            echo "aarch64-apple-darwin"
        else
            echo "x86_64-apple-darwin"
        fi
    elif [[ "$OSTYPE" == "msys"* ]] || [[ "$OSTYPE" == "cygwin"* ]] || [[ "$OSTYPE" == "win32"* ]]; then
        echo "x86_64-pc-windows-msvc"
    else
        echo "Unsupported platform: $OSTYPE" >&2
        exit 1
    fi

# Run tests for current platform
test:
    #!/usr/bin/env bash
    set -euo pipefail

    TARGET=$(just detect-target)
    echo "Running tests for target: $TARGET"
    cargo test --release --workspace --target "$TARGET"

# Build for current platform with updated versions
build: install-tools
    #!/usr/bin/env bash
    set -euo pipefail

    echo "Setting versions based on git tags..."

    # Try to get exact tag match at HEAD
    if TAG_AT_HEAD=$(git describe --exact-match --tags HEAD 2>/dev/null); then
        # HEAD is at a tag, use that version
        VERSION=${TAG_AT_HEAD#v}
        echo "HEAD is at tag: $TAG_AT_HEAD, setting version to $VERSION"
    else
        # HEAD is not at a tag, increment patch and add SNAPSHOT
        LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.1")
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

    just set-version "$VERSION"

    # Build for current platform
    TARGET=$(just detect-target)

    # Determine file extension
    if [[ "$TARGET" == *"windows"* ]]; then
        EXT=".exe"
    else
        EXT=""
    fi

    echo "Building for target: $TARGET"
    cargo build --release --workspace --target "$TARGET"

    # Create dist directory if it doesn't exist
    mkdir -p dist

    # Copy binary to dist directory
    cp "target/$TARGET/release/kagi-mcp-server$EXT" "dist/kagi-mcp-server$EXT"

    echo "Binary built and copied to dist/kagi-mcp-server$EXT"
    echo "Build complete with version $VERSION"

# Create a new release with version bumping and tagging
release bump_type="patch": install-tools
    #!/usr/bin/env bash
    set -euo pipefail

    echo "=== CREATING RELEASE ==="

    # Check if working directory is clean
    if ! git diff-index --quiet HEAD --; then
        echo "❌ Working directory is not clean. Please commit or stash your changes first."
        git status --porcelain
        exit 1
    fi

    # Get the next release version
    NEW_RELEASE_TAG=$(just tag-get-next-release "{{bump_type}}")
    echo "🏷️  Next release tag: $NEW_RELEASE_TAG"

    # Check if tag already exists
    if git rev-parse "$NEW_RELEASE_TAG" >/dev/null 2>&1; then
        echo "❌ Tag $NEW_RELEASE_TAG already exists!"
        exit 1
    fi

    echo "🧪 Running tests before making any changes..."
    just test

    echo "🔨 Building for local platform..."

    # Build for current platform
    TARGET=$(just detect-target)

    # Determine file extension
    if [[ "$TARGET" == *"windows"* ]]; then
        EXT=".exe"
    else
        EXT=""
    fi

    echo "Building for target: $TARGET"
    cargo build --release --workspace --target "$TARGET"

    # Create dist directory if it doesn't exist
    mkdir -p dist

    # Copy binary to dist directory
    cp "target/$TARGET/release/kagi-mcp-server$EXT" "dist/kagi-mcp-server$EXT"

    echo "Binary built and copied to dist/kagi-mcp-server$EXT"

    # Get current version for logging
    CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
    TARGET_VERSION=${NEW_RELEASE_TAG#v}

    echo "📝 Updating version files and committing atomically..."
    echo "   Updating from $CURRENT_VERSION to $TARGET_VERSION"

    # Update versions atomically with commit
    just set-version "$NEW_RELEASE_TAG"
    git add Cargo.toml crates/*/Cargo.toml extension.toml Cargo.lock
    git commit -m "chore: bump version to $NEW_RELEASE_TAG"

    echo "✅ Version bump committed"

    echo "🏷️  Creating tag $NEW_RELEASE_TAG..."
    git tag "$NEW_RELEASE_TAG"

    echo "✅ Release $NEW_RELEASE_TAG created successfully!"
    echo ""
    echo "To push the release:"
    echo "  git push origin main && git push origin $NEW_RELEASE_TAG"
    echo ""
    echo "Or to push everything:"
    echo "  git push --follow-tags"

# Build WASM extension for local testing
build-wasm:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "🔧 Building WASM extension..."

    # Check if wasm target is installed
    if ! rustup target list --installed | grep -q "{{WASM_TARGET}}"; then
        echo "Installing {{WASM_TARGET}} target..."
        rustup target add {{WASM_TARGET}}
    fi

    # Build the WASM extension
    cargo build --release --target {{WASM_TARGET}} -p kagimcp-zed

    # Copy to extension.wasm
    cp "target/{{WASM_TARGET}}/release/kagimcp_zed.wasm" extension.wasm

    echo "✅ WASM extension built: extension.wasm"

# Install extension locally for testing in Zed
install-local: build-wasm
    #!/usr/bin/env bash
    set -euo pipefail

    # Determine Zed extensions directory based on OS
    if [[ "$OSTYPE" == "darwin"* ]]; then
        ZED_EXTENSIONS_DIR="$HOME/Library/Application Support/Zed/extensions/installed"
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        ZED_EXTENSIONS_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/zed/extensions/installed"
    else
        echo "❌ Unsupported OS for local install: $OSTYPE"
        exit 1
    fi

    EXTENSION_DIR="$ZED_EXTENSIONS_DIR/kagimcp"

    echo "📦 Installing extension to: $EXTENSION_DIR"

    # Create extension directory
    mkdir -p "$EXTENSION_DIR"

    # Copy extension files
    cp extension.toml "$EXTENSION_DIR/"
    cp extension.wasm "$EXTENSION_DIR/"
    cp -r configuration "$EXTENSION_DIR/" 2>/dev/null || true

    echo "✅ Extension installed locally!"
    echo ""
    echo "To use the extension:"
    echo "  1. Restart Zed"
    echo "  2. The extension should appear in your extensions list"
    echo ""
    echo "Note: You may need to configure your Kagi API key in Zed settings."

# Build everything (native binary + WASM extension)
build-all: build build-wasm
    @echo "✅ All builds complete!"

# Push the latest release tag and trigger CI
push-release:
    #!/usr/bin/env bash
    set -euo pipefail

    # Get the latest tag
    LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || { echo "No tags found!"; exit 1; })

    echo "Pushing main branch and tag $LATEST_TAG..."
    git push origin main
    git push origin "$LATEST_TAG"

    echo "✅ Release pushed! CI will build and publish $LATEST_TAG"
