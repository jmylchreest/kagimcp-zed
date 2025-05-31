#!/bin/bash

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <new_version>"
    echo "Example: $0 0.2.0"
    exit 1
fi

NEW_VERSION="$1"

echo "Updating extension version to $NEW_VERSION..."

# Update version in extension.toml
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" extension.toml

# Update version in Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Remove backup files
rm -f extension.toml.bak Cargo.toml.bak

echo "Updated files:"
echo "  extension.toml: $(grep '^version = ' extension.toml)"
echo "  Cargo.toml: $(grep '^version = ' Cargo.toml)"

# Build to ensure everything still works
echo ""
echo "Building extension to verify..."
cargo build --target wasm32-unknown-unknown --release

echo ""
echo "Version update complete!"
echo ""
echo "Next steps:"
echo "1. git add ."
echo "2. git commit -m \"Bump version to $NEW_VERSION\""
echo "3. git tag v$NEW_VERSION"
echo "4. git push origin main --tags"