# Installation and Testing Guide

## Problem Analysis

### Issue 1: Wrong Installation Directory ❌

**Problem**: Documentation指示复制到 `~/.config/zed/extensions/`  
# Testing & Troubleshooting Guide

This guide covers validating the extension after installing via Zed's **Install Dev Extension** and diagnosing common issues.

## 1. Prerequisites
```bash
./build-dev.sh     # Build native server
```
Install dev extension through Zed (Command Palette → `zed: install dev extension`).

Ensure a config exists (activation required):
```bash
cp .auto-header.toml ~/.auto-header.toml
```

## 2. Basic Validation
1. Restart Zed completely.
2. Create new empty file → save as `test.rs`.
3. Header should appear instantly.

## 3. Manual Server Test
```bash
cargo run --package auto-header-server
```
Server starts and waits for LSP input (Ctrl+C to exit).

## 4. Log Inspection
```bash
tail -f ~/.local/share/zed/logs/Zed.log
```
Look for extension load and header insertion messages.

## 5. Verification Checklist
- [ ] Config file present (project or global)
- [ ] File is brand new & empty
- [ ] Supported extension (see LANGUAGES.md)
- [ ] Dev extension installed & visible
- [ ] No relevant errors in Zed log

## 6. Common Issues
**No header inserted**:
- File not empty (contains whitespace/newline)
- Unsupported extension
- Missing config file

**Wrong comment style**:
- Verify extension mapping in LANGUAGES.md
- Override via `[header.by_extension.ext]` in config

**Extension not active**:
- Confirm config exists
- Restart Zed fully (close all processes)

## 7. Advanced Checks
Ensure server binary exists after build:
```bash
ls -l target/release/auto-header-server
```
Make it executable if needed (Unix):
```bash
chmod +x target/release/auto-header-server
```

## 8. Quick Test Script
```bash
#!/usr/bin/env bash
set -e
echo "=== Auto Header Quick Test ==="

BIN=target/release/auto-header-server
if [ -x "$BIN" ]; then echo "✅ Server binary present"; else echo "❌ Missing server binary"; exit 1; fi

CONF="$(ls .auto-header.toml 2>/dev/null || echo ~/.auto-header.toml)"
if [ -f "$CONF" ]; then echo "✅ Config detected ($CONF)"; else echo "❌ No config file found"; fi

echo "Run in Zed: create new empty file and save. Expect header." 
```

## 9. Example Header
```
/*
 * File: test.rs
 * Project: My Project
 * Author: Your Name <you@example.com>
 * Created: 2025-11-23 20:15:00
 *
 * Copyright (c) 2025 Your Name
 * All rights reserved.
 */
```

## 10. When Reporting Issues
Include:
1. OS + version
2. Zed version
3. Whether config file exists
4. Snippet from Zed log around header attempt
5. File extension used

## 11. Success Indicators
- ✅ Headers on new empty files
- ✅ Immediate template changes after config edit
- ✅ Correct language-specific comment style
- ✅ No unresolved errors in logs
