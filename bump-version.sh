#!/usr/bin/env bash
#
# File: bump-version.sh
# Project: My Project
# Author: Your Name <your.email@example.com>
# Created: 2025-12-09 09:03:20
#
# Copyright (c) 2025 Your Name
# All rights reserved.
#

#!/bin/bash

# Script to bump version across all project files
# Usage: ./bump-version.sh <new_version>
# Example: ./bump-version.sh 0.2.7

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if version argument is provided
if [ $# -eq 0 ]; then
    echo -e "${RED}❌ Error: No version specified${NC}"
    echo ""
    echo "Usage: $0 <new_version>"
    echo "Example: $0 0.2.7"
    exit 1
fi

NEW_VERSION=$1

# Validate version format (semantic versioning)
if ! [[ $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}❌ Error: Invalid version format${NC}"
    echo "Version must follow semantic versioning: MAJOR.MINOR.PATCH (e.g., 0.2.7)"
    exit 1
fi

# Get current date
CURRENT_DATE=$(date +%Y-%m-%d)

echo -e "${BLUE}🚀 Bumping version to ${NEW_VERSION}...${NC}"
echo ""

# 1. Update Cargo.toml
echo "📝 Updating Cargo.toml..."
sed -i.bak "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" Cargo.toml
rm Cargo.toml.bak

# 2. Update extension.toml
echo "📝 Updating extension.toml..."
sed -i.bak "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" extension.toml
rm extension.toml.bak

# 3. Update CHANGELOG.md (add new version header at top)
echo "📝 Updating CHANGELOG.md..."
# Create temporary file with new version header
cat > /tmp/changelog_header.txt << EOF
# Changelog

## Version ${NEW_VERSION} - ${CURRENT_DATE}

### 🔧 Changes

- TODO: Add changelog entries here

---

EOF

# Remove existing "# Changelog" header and prepend new content
tail -n +2 CHANGELOG.md > /tmp/changelog_body.txt
cat /tmp/changelog_header.txt /tmp/changelog_body.txt > CHANGELOG.md
rm /tmp/changelog_header.txt /tmp/changelog_body.txt

echo ""
echo -e "${GREEN}✅ Version updated successfully!${NC}"
echo ""
echo "📋 Summary:"
echo "  New version: ${NEW_VERSION}"
echo "  Date: ${CURRENT_DATE}"
echo ""
echo "📝 Next steps:"
echo "  1. Edit CHANGELOG.md to add actual changelog entries"
echo "  2. Review changes: git diff"
echo "  3. Build and test: cargo build --release"
echo "  4. Commit: git add -A && git commit -m 'Bump version to ${NEW_VERSION}'"
echo "  5. Push: git push origin master"
echo ""
echo -e "${YELLOW}⚠️  Don't forget to update the CHANGELOG.md with actual changes!${NC}"
