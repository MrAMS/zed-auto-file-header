#!/bin/bash
# Simplified build script - Only builds the LSP server
# The Wasm extension is built automatically by Zed

set -e

echo "=== Building Auto Header LSP Server ==="
echo ""

# Build the LSP server (native binary)
echo "Building LSP server..."
cargo build --release --package auto-header-server

# Copy to extension directory so Zed can find it
echo "Copying binary to extension directory..."
cp target/release/auto-header-server extension/

echo ""
echo "✅ Build complete!"
echo ""
echo "LSP server: extension/auto-header-server"
echo ""
echo "=== Installation Instructions ==="
echo ""
echo "1. Open Zed editor"
echo "2. Press Cmd/Ctrl + Shift + P (Command Palette)"
echo "3. Type: 'zed: install dev extension'"
echo "4. Select the 'extension' directory from this project:"
echo "   $(pwd)/extension"
echo ""
echo "Zed will automatically:"
echo "  - Build the Wasm component correctly"
echo "  - Install the extension"
echo "  - Make the LSP server binary available"
echo ""
echo "After installation, restart Zed and test by creating a new empty file."
echo ""
