# Auto File Header - Zed Extension

A zero-dependency Zed extension that automatically inserts customizable file headers (Filename, Author, Date, Copyright) when you create a new, empty file.

**Author:** MrAMS <2421653893@qq.com>  
**Repository:** https://github.com/MrAMS/zed-auto-file-header  
**Platforms:** Linux (x86_64/ARM64) • macOS (Intel/Apple Silicon) • Windows (x86_64)

## ✨ Key Features

- **🚀 Zero Dependencies**: No Rust or build tools required - downloads pre-built binaries automatically
- **🌍 Cross-Platform**: Supports all major platforms and architectures
- **⚡ Auto-Detection**: Recognizes 30+ programming languages with appropriate comment styles
- **🎨 Fully Customizable**: Define global or per-language templates
- **🔄 Dynamic Configuration**: Changes take effect immediately without restarting

## Quick Start

### Installation from Zed Extensions

1. Open Zed → Extensions panel (`Ctrl+Shift+P` → "zed: extensions")
2. Search for "Auto File Header"
3. Click Install
4. Create config file (required):
   ```bash
   cp .auto-header.toml ~/.auto-header.toml
   nano ~/.auto-header.toml   # edit with your details
   ```
5. Restart Zed

The extension automatically downloads the appropriate pre-built binary for your platform on first use. **No Rust installation required!**

### Development Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/MrAMS/zed-auto-file-header.git
   cd zed-auto-file-header
   ```

2. Install as a dev extension in Zed:
   - Open Zed
   - `Ctrl+Shift+P` → `zed: install dev extension`
   - Select the `extension` directory

3. Create a config file (required):
   ```bash
   cp .auto-header.toml ~/.auto-header.toml
   nano ~/.auto-header.toml   # edit with your details
   ```

4. Restart Zed and create a new empty file → header is automatically inserted

Configuration changes take effect immediately (no rebuild or restart needed).

## Header Example

```rust
/*
 * File: example.rs
 * Project: My Project
 * Author: Your Name <your.email@example.com>
 * Created: 2025-11-24 19:30:00
 *
 * Copyright (c) 2025 Your Name
 * All rights reserved.
 */
```

## Configuration

The extension searches for `.auto-header.toml` in the following locations (in order):

1. **Project root**: `./.auto-header.toml` (project-specific settings)
2. **Zed config**: `~/.config/zed/auto-header.toml` (Linux/macOS) or `%APPDATA%\Zed\auto-header.toml` (Windows)
3. **Home directory**: `~/.auto-header.toml` (user-wide settings)

### Example Configuration

```toml
[author]
name = "Your Name"
email = "your.email@example.com"

[project]
name = "My Project"
copyright_holder = "Your Company"  # Optional, defaults to author name

[header]
template = """
/*
 * File: {filename}
 * Project: {project}
 * Author: {author} <{email}>
 * Created: {date} {time}
 *
 * Copyright (c) {year} {copyright_holder}
 * All rights reserved.
 */

"""
```

### Template Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `{filename}` | File name only | `example.rs` |
| `{filepath}` | Full file path | `/home/user/project/example.rs` |
| `{date}` | Current date | `2025-11-24` |
| `{time}` | Current time | `19:30:00` |
| `{year}` | Current year | `2025` |
| `{author}` | Author name | `Your Name` |
| `{email}` | Author email | `your.email@example.com` |
| `{project}` | Project name | `My Project` |
| `{copyright_holder}` | Copyright holder | `Your Company` |
| `{interpreter}` | Script interpreter | `python3`, `bash`, etc. |

### Per-Extension Templates

Override the default template for specific file extensions:

```toml
[header.by_extension.py]
template = """
# -*- coding: utf-8 -*-
\"\"\"
File: {filename}
Project: {project}
Author: {author} <{email}>
Created: {date} {time}

Copyright (c) {year} {copyright_holder}
All rights reserved.
\"\"\"

"""

[header.by_extension.sh]
template = """
#!/usr/bin/env bash
#
# File: {filename}
# Author: {author}
# Date: {date}
#

"""

[header.by_extension.html]
template = """
<!--
  File: {filename}
  Author: {author}
  Date: {date}
-->

"""
```

## Built-in Language Support

The extension automatically recognizes and applies appropriate comment styles for 30+ languages:

### Block Comment Languages
C, C++, C#, Java, JavaScript, TypeScript, Rust, Go, Swift, Kotlin, Scala, Objective-C

### Line Comment Languages  
Python (with encoding), Shell (Bash, Zsh, Fish), Ruby, Perl, R, Julia

### Markup Languages
HTML, XML, SVG

### Style Languages
CSS, SCSS, SASS, LESS

### Database Languages
SQL

### Configuration Languages
YAML

### Other Languages
Lua, Haskell, Lisp, Scheme, Clojure, Erlang, Elixir, Vim script

For details, see [LANGUAGES.md](LANGUAGES.md).

## Troubleshooting

### Extension not working

1. **Check config file exists**: The extension only activates when a `.auto-header.toml` file is found in one of the search locations.
   ```bash
   # Verify config file exists
   ls -la ~/.auto-header.toml
   # or
   ls -la ~/.config/zed/auto-header.toml
   ```

2. **Restart Zed**: After creating or modifying the config file for the first time, restart Zed.

3. **Check Zed logs**: Open Zed's log panel to see detailed error messages:
   - `Ctrl+Shift+P` → "zed: open log"

### Download failures

If you see errors like "Failed to fetch release from GitHub":

1. **Check internet connection**: Ensure you can access github.com
2. **Manual download**: Download the binary manually from [Releases](https://github.com/MrAMS/zed-auto-file-header/releases) and place it in your project directory with the name:
   - Linux: `auto-header-server`
   - macOS: `auto-header-server`
   - Windows: `auto-header-server.exe`

### Platform not supported

If you see "Unsupported platform" errors:

- **Supported platforms**:
  - Linux: x86_64, ARM64
  - macOS: x86_64 (Intel), ARM64 (Apple Silicon)
  - Windows: x86_64

- Report unsupported platforms at: https://github.com/MrAMS/zed-auto-file-header/issues

### Headers not inserting

1. **File must be completely empty**: The extension only inserts headers into newly created, empty files
2. **Config must exist**: Ensure `.auto-header.toml` is in a search location
3. **Check language support**: Verify your file extension is recognized

## Development

### Project Structure

```
├── Cargo.toml              # Workspace root
├── .github/workflows/
│   └── release.yml         # Automated cross-platform builds
├── extension/              # Zed extension (Wasm)
│   ├── Cargo.toml
│   ├── extension.toml      # Extension manifest
│   └── src/lib.rs          # Binary download & LSP launcher
└── server/                 # Language server (native)
    ├── Cargo.toml
    └── src/main.rs         # LSP server logic
```

### Building Locally

**Server binary:**
```bash
cargo build --release --package auto-header-server
# Output: target/release/auto-header-server
```

**Extension Wasm:**
```bash
rustup target add wasm32-wasip1
cargo build --release --package auto-header-extension --target wasm32-wasip1
# Output: target/wasm32-wasip1/release/auto_header_extension.wasm
```

### Testing

**Test server directly:**
```bash
cargo run --package auto-header-server
# Sends LSP messages via stdin
```

**Test extension in Zed:**
1. Build the extension Wasm (see above)
2. `Ctrl+Shift+P` → "zed: install dev extension" → select `extension/` directory
3. Create a new file to test

### Release Process

Tags matching `v*` automatically trigger GitHub Actions to build binaries for all platforms and publish a release. See [PUBLISHING.md](PUBLISHING.md) for details.

## Architecture

This extension uses an **LSP Wrapper** design:

1. **Zed Extension (Wasm)**: 
   - Checks for cached/local `auto-header-server` binary
   - Downloads from GitHub Releases if not found
   - Launches the LSP server

2. **LSP Server (Native)**:
   - Listens for `didOpen` events
   - Checks if file is empty and config exists
   - Injects appropriate header template

This architecture enables zero-dependency installation while maintaining full LSP capabilities.

## License

MIT License © 2025 MrAMS

## Contributing

Issues and pull requests are welcome!

**Repository:** https://github.com/MrAMS/zed-auto-file-header

---

**Note:** This extension requires a `.auto-header.toml` configuration file to activate. Without it, the extension will not insert headers.

