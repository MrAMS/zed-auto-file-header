# Development Testing Guide

This guide explains how to test the extension locally during development.

## Two Testing Modes

### Mode 1: Local Server (Development)

Use locally built server binary without publishing to GitHub. Perfect for rapid iteration.

**Setup:**
```bash
# 1. Build the server
cd server
cargo build --release
cd ..

# 2. Build the extension WASM
cargo build --target wasm32-wasip2 --release

# 3. Set environment variable before starting Zed
export AUTO_HEADER_DEV_MODE=local

# 4. Start Zed
zed

# 5. Install dev extension
# In Zed: Cmd/Ctrl+Shift+P -> "zed: install dev extension"
# Select this project directory
```

**How it works:**
- Extension will search for the server binary in these locations (in order):
  1. Project root: `./auto-header-server`
  2. Build directory: `./server/target/release/auto-header-server`
  3. System PATH
- If not found, shows helpful error message with searched locations

**Advantages:**
- ✅ Test changes immediately without GitHub release
- ✅ Fast iteration cycle
- ✅ No internet required

**Disadvantages:**
- ❌ Doesn't test the GitHub download flow
- ❌ Requires manual build steps

---

### Mode 2: GitHub Release (Production Simulation)

Downloads the server from GitHub Releases, simulating real user experience.

**Setup:**
```bash
# 1. Make sure AUTO_HEADER_DEV_MODE is NOT set
unset AUTO_HEADER_DEV_MODE

# 2. Clear cached server (if testing a new version)
rm -rf ~/.local/share/zed/extensions/work/auto-header

# 3. Start Zed normally
zed

# 4. Install dev extension (or use published extension)
# Extension will download server from latest GitHub Release
```

**How it works:**
- Extension will download the server binary from:
  `https://github.com/MrAMS/zed-auto-file-header/releases`
- Caches the downloaded binary in:
  `~/.local/share/zed/extensions/work/auto-header/auto-header-server-v{VERSION}/`

**Advantages:**
- ✅ Tests the real user experience
- ✅ Validates GitHub Release artifacts
- ✅ Tests cross-platform download logic

**Disadvantages:**
- ❌ Requires publishing a GitHub Release
- ❌ Slower iteration (need to push, wait for CI, download)
- ❌ Requires internet connection

---

## Quick Build Script

Save as `build-dev.sh`:

```bash
#!/bin/bash
set -e

echo "🔨 Building server..."
cd server
cargo build --release
cd ..

echo "📦 Building extension..."
cargo build --target wasm32-wasip2 --release

echo "✅ Build complete!"
echo ""
echo "To test with local server:"
echo "  export AUTO_HEADER_DEV_MODE=local"
echo "  zed"
echo ""
echo "To test with GitHub downloads:"
echo "  unset AUTO_HEADER_DEV_MODE"
echo "  zed"
```

Make executable:
```bash
chmod +x build-dev.sh
```

---

## Troubleshooting

### Server not starting in local mode?

Check the error message in Zed logs (Cmd/Ctrl+Shift+P -> "zed: open log"):
- If it says "binary not found", build the server first
- Check the searched paths in the error message

### Server not downloading in GitHub mode?

1. Check internet connection
2. Verify the release exists: https://github.com/MrAMS/zed-auto-file-header/releases
3. Check Zed logs for download errors
4. Clear cache and retry:
   ```bash
   rm -rf ~/.local/share/zed/extensions/work/auto-header
   ```

### How to switch modes?

```bash
# Switch to local mode
export AUTO_HEADER_DEV_MODE=local

# Switch to GitHub mode  
unset AUTO_HEADER_DEV_MODE

# Then reload extension in Zed:
# Cmd/Ctrl+Shift+P -> "zed: reload extensions"
# Or restart Zed
```

---

## Recommended Workflow

1. **During active development:**
   - Use `AUTO_HEADER_DEV_MODE=local`
   - Quick iteration without releases

2. **Before releasing:**
   - Test with `AUTO_HEADER_DEV_MODE=local` first
   - Then unset and test GitHub download flow
   - Publish release only after both modes work

3. **After release:**
   - Install published extension (not dev)
   - Verify it works for end users
