#!/bin/bash
set -e

# This script is specifically for building multi-architecture Docker images
# It does NOT increment the version number as that is handled by build_new_guardian.sh
# This script is meant to be run after build_new_guardian.sh to create the full release
# with support for both AMD64 and ARM64 architectures.

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep -m 1 'version = ' backend/node/Cargo.toml | cut -d'"' -f2)
echo "Building multi-architecture images for version: $CURRENT_VERSION"

# Create and use a new builder instance
docker buildx create --name guardian-builder --use || true

# Build and push multi-architecture images
echo "Building multi-architecture images for gridlocknetwork/guardian-node:$CURRENT_VERSION"
docker buildx build --platform linux/amd64,linux/arm64 \
    -t gridlocknetwork/guardian-node:$CURRENT_VERSION \
    -t gridlocknetwork/guardian-node:latest \
    --push .

echo "Successfully built and pushed multi-architecture images for gridlocknetwork/guardian-node:$CURRENT_VERSION and gridlocknetwork/guardian-node:latest" 