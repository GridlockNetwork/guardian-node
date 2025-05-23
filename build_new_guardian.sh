#!/bin/bash
set -e

# Function to increment version number
increment_version() {
    local version=$1
    local major=$(echo $version | cut -d. -f1)
    local minor=$(echo $version | cut -d. -f2)
    local patch=$(echo $version | cut -d. -f3)
    echo "$major.$minor.$((patch + 1))"
}

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep -m 1 'version = ' backend/node/Cargo.toml | cut -d'"' -f2)
echo "Current version: $CURRENT_VERSION"

# Increment version
NEW_VERSION=$(increment_version $CURRENT_VERSION)
echo "New version: $NEW_VERSION"

# Update Cargo.toml with new version
# Handle both macOS and Linux sed syntax
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    sed -i '' "s/version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" backend/node/Cargo.toml
else
    # Linux
    sed -i "s/version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" backend/node/Cargo.toml
fi

# Determine the current platform's architecture
PLATFORM="linux/$(uname -m | sed 's/x86_64/amd64/' | sed 's/aarch64/arm64/')"
echo "Building for platform: $PLATFORM"

# Build Docker image with new version for current platform only
echo "Building gridlocknetwork/guardian-node:$NEW_VERSION"
docker build --platform $PLATFORM -t gridlocknetwork/guardian-node:$NEW_VERSION -t gridlocknetwork/guardian-node:latest .

echo "Successfully built gridlocknetwork/guardian-node:$NEW_VERSION and gridlocknetwork/guardian-node:latest"
echo "To build for all platforms, run build_prod_multiarch.sh"