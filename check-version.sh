#!/usr/bin/env bash
#
# File: check-version.sh
# Project: My Project
# Author: Your Name <your.email@example.com>
# Created: 2025-12-09 09:03:07
#
# Copyright (c) 2025 Your Name
# All rights reserved.
#

#!/bin/bash

# Script to check version consistency across project files
# Usage: ./check-version.sh

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🔍 Checking version consistency..."

# Extract version from Cargo.toml
CARGO_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

# Extract version from extension.toml
EXTENSION_VERSION=$(grep '^version = ' extension.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

# Extract version from CHANGELOG.md (first occurrence)
CHANGELOG_VERSION=$(grep -m 1 '^## Version' CHANGELOG.md | sed 's/## Version \([0-9.]*\).*/\1/')

echo ""
echo "📋 Found versions:"
echo "  Cargo.toml:     $CARGO_VERSION"
echo "  extension.toml: $EXTENSION_VERSION"
echo "  CHANGELOG.md:   $CHANGELOG_VERSION"
echo ""

# Check if all versions match
if [ "$CARGO_VERSION" = "$EXTENSION_VERSION" ] && [ "$CARGO_VERSION" = "$CHANGELOG_VERSION" ]; then
    echo -e "${GREEN}✅ All versions match: $CARGO_VERSION${NC}"
    exit 0
else
    echo -e "${RED}❌ Version mismatch detected!${NC}"
    echo ""
    echo "Please ensure all versions are synchronized:"
    echo "  1. Update version in Cargo.toml"
    echo "  2. Update version in extension.toml"
    echo "  3. Add new version entry in CHANGELOG.md"
    echo ""
    echo "Note: lib.rs uses CARGO_PKG_VERSION (auto-synced from Cargo.toml)"
    exit 1
fi
