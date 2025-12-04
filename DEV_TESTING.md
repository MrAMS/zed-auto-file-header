# Development Testing Guide

This guide explains how to test the Auto File Header extension during development.

## Local Development Testing

For local testing, simply place the compiled server binary in your project root.

### Quick Setup

```bash
# 1. Build the server
cd server
cargo build --release

# 2. Copy binary to project root (for testing)
cp target/release/auto-header-server ../

# 3. Reload Zed extension
# In Zed: Cmd/Ctrl+Shift+P → "zed: reload extensions"
```

The extension will automatically find and use the local binary if it exists in the project root.

### How It Works

The extension searches for the server binary in this order:
1. **Project root** - Checks if `auto-header-server` exists in the workspace root
2. **GitHub Release** - Downloads from latest release if not found locally

This makes local testing simple:
- **Development**: Place binary in project root → instant testing
- **Production**: No local binary → automatic download from GitHub

### Debugging

Check Zed logs for detailed information:
```bash
tail -f ~/.local/share/zed/logs/Zed.log | grep "Auto Header"
```

Key log messages:
- `[Auto Header] Server initialized successfully` - Server started
- `[Auto Header] File opened: <path>, length: <n>` - File detected
- `[Auto Header] Inserting header for: <path>` - Header being inserted
- `[Auto Header] Skipping non-empty file` - File already has content
- `[Auto Header] No .auto-header.toml found` - Config file missing

### Testing Workflow

1. **Make changes to server code**
2. **Rebuild**: `cd server && cargo build --release`
3. **Copy to root**: `cp target/release/auto-header-server ../`
4. **Reload Zed**: Cmd/Ctrl+Shift+P → "zed: reload extensions"
5. **Test**: Create a new empty file

### Clean Up

Before committing, remove the binary from project root:
```bash
rm auto-header-server
```

The binary is in `.gitignore` so it won't be accidentally committed.

## Testing in Other Projects

To test the extension in other projects (not the extension source code):
1. Make sure no local binary exists in that project
2. Extension will download from GitHub automatically
3. Check logs to verify download and initialization

## Quick Build Script

The `build-dev.sh` script automates the build and copy process:

```bash
#!/bin/bash
cd server
cargo build --release
cp target/release/auto-header-server ../
echo "✓ Server built and copied to project root"
echo "  Reload Zed extension to use updated binary"
```

Usage:
```bash
./build-dev.sh
```
