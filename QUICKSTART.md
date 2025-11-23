# Quick Reference

## Dev Installation

```bash
git clone https://github.com/MrAMS/zed-auto-file-header.git
cd zed-auto-file-header
./build-dev.sh   # Build native LSP server
```

Then in Zed:
1. Command Palette `Ctrl+Shift+P`
2. `zed: install dev extension`
3. Select this repo's `extension` directory
4. Create config file (see below) and restart Zed

## Activation
Extension only activates if `.auto-header.toml` exists in one of:
1. Project root
2. `~/.config/zed/auto-header.toml`
3. `~/.auto-header.toml`
| `{interpreter}` | Script interpreter (if applicable) | `python3`, `bash` |

## Command Reference

```bash
# Build server only
cargo build --release --package auto-header-server

# Build wasm (optional manual)
rustup target add wasm32-wasip1
cargo build --release --package auto-header-extension --target wasm32-wasip1

# Run server directly
cargo run --package auto-header-server

# Edit global config
nano ~/.config/zed/auto-header.toml
```
**Extension not working?**
1. Ensure config file exists
2. Verify file is new & empty
3. Confirm dev extension installed
4. Restart Zed completely

**Wrong comment format?**
1. Check extension supported
2. Override via `[header.by_extension.ext]`

**Headers not appearing?**
- File must be completely empty (no whitespace/newline)
- Must have supported extension at save
- [README.md](README.md) - Full documentation
```
# Quick Reference

## Installation

**For users (pre-packaged)**:
```bash
# Extract and install
tar -xzf auto-header-extension.tar.gz
cd zed-file-header
./install.sh

# Restart Zed - Done! LSP server is already bundled.
```

**For developers (from source)**:
```bash
# 1. Build the project
./build.sh

# 2. Package everything (includes LSP server)
./package.sh

# 3. Install (automatic directory detection)
./install.sh

# 4. Setup configuration (optional)
./setup-config.sh
```

## Configuration File

**Location** (checked in order):
1. `./.auto-header.toml` (project)
2. `~/.config/zed/auto-header.toml` (recommended)
3. `~/.auto-header.toml` (alternative)

## Template Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `{filename}` | File name | `main.rs` |
| `{filepath}` | Full path | `/home/user/project/main.rs` |
| `{date}` | Current date | `2025-11-23` |
| `{time}` | Current time | `19:30:00` |
| `{year}` | Current year | `2025` |
| `{author}` | Author name | `John Doe` |
| `{email}` | Author email | `john@example.com` |
| `{project}` | Project name | `My Project` |
| `{copyright_holder}` | Copyright holder | `John Doe` |
| `{interpreter}` | Script interpreter | `python3`, `bash` |

## Built-in Language Support

**30+ languages supported out of the box:**

- **C-style**: C, C++, Java, JavaScript, TypeScript, Rust, Go, Scala, Swift, Kotlin
- **Python**: Special docstring format with encoding
- **Shell**: Bash, Zsh, Fish, Ruby, Perl (with shebang)
- **Markup**: HTML, XML, SVG
- **Styles**: CSS, SCSS, SASS, LESS
- **Config**: YAML, SQL
- **Functional**: Haskell, Lisp, Scheme, Clojure, Erlang, Elixir, Lua
- **Other**: Vim script, R, Julia

See [LANGUAGES.md](LANGUAGES.md) for complete list with examples.

## Customization Examples

### Basic Author Info

```toml
[author]
name = "Your Name"
email = "you@example.com"

[project]
name = "My Project"
```

### Override Python Template

```toml
[header.by_extension.py]
template = """#!/usr/bin/env python3
# Author: {author}
# Date: {date}

"""
```

### Override C++ Template

```toml
[header.by_extension.cpp]
template = """// {filename}
// Copyright (c) {year} {author}

"""
```

### Custom Default Template

```toml
[header]
template = """/*
 * {filename} - {project}
 * (c) {year} {author}
 */

"""
```

## Command Reference

```bash
# Build everything
./build.sh

# Setup configuration interactively
./setup-config.sh

# Build server only
cargo build --release --package auto-header-server

# Build extension only
cargo build --release --package auto-header-extension --target wasm32-wasip1

# Check code
cargo check

# Edit config
nano ~/.config/zed/auto-header.toml
```

## Troubleshooting

**Extension not working?**
1. Check server binary is in PATH
2. Verify config file location
3. Check Zed extension is installed
4. Restart Zed

**Wrong comment format?**
1. Check file extension
2. See built-in templates in [LANGUAGES.md](LANGUAGES.md)
3. Override in `.auto-header.toml` if needed

**Headers not appearing?**
- Extension only triggers on **new, empty files**
- File must be completely empty (no whitespace)

## Links

- [README.md](README.md) - Full documentation
- [LANGUAGES.md](LANGUAGES.md) - Language support reference
- [CHANGELOG.md](CHANGELOG.md) - Version history
- [.auto-header.toml](.auto-header.toml) - Configuration template
