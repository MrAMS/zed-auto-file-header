#!/bin/bash
# Build script for dev extension in Zed

set -e

echo "🔨 Building LSP server..."
cd server
cargo build --release
cd ..

echo "📦 Building extension (WASM)..."
cargo build --target wasm32-wasip2 --release

echo "📋 Copying binaries for dev extension..."
cp server/target/release/auto-header-server .
chmod +x auto-header-server

echo "✅ Build complete!"
echo ""
echo "To install dev extension in Zed:"
echo "  1. Press Cmd/Ctrl+Shift+P"
echo "  2. Type 'zed: install dev extension'"
echo "  3. Select this directory: $(pwd)"
echo ""
echo "Dev extension will use the local server binary."
