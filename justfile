# justfile for kagimcp-zed project

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

    # Get the next release version first
    NEW_RELEASE_TAG=$(just tag-get-next-release "{{bump_type}}")
    echo "🏷️  Next release tag: $NEW_RELEASE_TAG"

    # Check if tag already exists
    if git rev-parse "$NEW_RELEASE_TAG" >/dev/null 2>&1; then
        echo "❌ Tag $NEW_RELEASE_TAG already exists!"
        exit 1
    fi

    # Check current version in Cargo.toml
    CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
    TARGET_VERSION=${NEW_RELEASE_TAG#v}

    echo "📝 Setting version to $NEW_RELEASE_TAG..."
    just set-version "$NEW_RELEASE_TAG"

    # Check if there are any changes after setting version
    if git diff --quiet; then
        echo "ℹ️  Version files already at $TARGET_VERSION, no version changes to commit"
        SKIP_VERSION_COMMIT=true
    else
        echo "✅ Version files updated from $CURRENT_VERSION to $TARGET_VERSION"
        SKIP_VERSION_COMMIT=false
    fi

    echo "🧪 Running tests..."
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

    if [ "$SKIP_VERSION_COMMIT" = "false" ]; then
        echo "📦 Committing version changes..."
        git add Cargo.toml crates/*/Cargo.toml extension.toml Cargo.lock
        git commit -m "chore: bump version to $NEW_RELEASE_TAG"
        echo "✅ Version bump committed"
    else
        echo "ℹ️  No version changes to commit"
    fi

    echo "🏷️  Creating tag $NEW_RELEASE_TAG..."
    git tag "$NEW_RELEASE_TAG"

    echo "✅ Release $NEW_RELEASE_TAG created successfully!"
    echo ""
    echo "To push the release:"
    echo "  git push origin main && git push origin $NEW_RELEASE_TAG"
    echo ""
    echo "Or to push everything:"
    echo "  git push --follow-tags"

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
