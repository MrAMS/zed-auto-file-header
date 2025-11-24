# Auto File Header - Zed Extension

A zero-dependency Zed extension that automatically inserts customizable file headers when you create new, empty files.

**Author:** MrAMS <2421653893@qq.com>  
**Repository:** https://github.com/MrAMS/zed-auto-file-header  
**Platforms:** Linux (x86_64/ARM64) • macOS (Intel/Apple Silicon) • Windows (x86_64)

## ✨ Features

- **🚀 Zero Dependencies**: No Rust or build tools required - downloads pre-built binaries automatically
- **🌍 Cross-Platform**: Supports all major platforms and architectures
- **⚡ Auto-Detection**: Recognizes 30+ programming languages with appropriate comment styles
- **🎨 Fully Customizable**: Define global or per-language templates
- **🔄 Dynamic Configuration**: Changes take effect immediately without restarting
- **📁 Flexible Config Location**: Project-specific or user-global configuration

## 📦 Installation

1. Open Zed → Extensions panel (`Ctrl+Shift+P` → "zed: extensions")
2. Search for "Auto File Header"
3. Click Install
4. **Create configuration file** (required - see [Configuration](#-configuration) section below)
5. Restart Zed

On first use, the extension automatically downloads the appropriate pre-built binary for your platform. You'll see "auto-header: Downloading..." in the status bar - this only happens once.

## 📋 Supported Languages

The extension automatically recognizes and applies appropriate comment styles for **30+ languages**:

### Block Comment Languages
C, C++, C#, Java, JavaScript, TypeScript, Rust, Go, Swift, Kotlin, Scala, Objective-C

### Script Languages (with shebang support)
Python, Shell (Bash, Zsh, Fish), Ruby, Perl, R, Julia

### Markup Languages
HTML, XML, SVG

### Style Languages
CSS, SCSS, SASS, LESS

### Database & Config Languages
SQL, YAML

### Functional & Other Languages
Lua, Haskell, Lisp, Scheme, Clojure, Erlang, Elixir, Vim script

For detailed templates of each language, see [LANGUAGES.md](LANGUAGES.md).

## ⚙️ Configuration

### Configuration File: `.auto-header.toml`

**The extension only activates when a `.auto-header.toml` file exists.** This file defines your author information, project details, and header templates.

### Configuration Priority (High to Low)

The extension searches for `.auto-header.toml` in the following order and uses the **first one found**:

1. **Project Root** (highest priority)
   - Path: `./.auto-header.toml` (in your project's root directory)
   - Use case: Project-specific headers with custom copyright, team info, etc.

2. **Zed Config Directory**
   - Linux/macOS: `~/.config/zed/auto-header.toml`
   - Windows: `%APPDATA%\Zed\auto-header.toml`
   - Use case: User-wide default settings for all projects

3. **Home Directory** (lowest priority)
   - Path: `~/.auto-header.toml`
   - Use case: Fallback location, traditional dotfile approach

**Recommendation**: 
- Use **project root** for team projects with specific copyright/license requirements
- Use **Zed config directory** for personal default settings across all projects

### Basic Configuration Example

Create one of the above files with your information:

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
| `{author}` | Author name from config | `Your Name` |
| `{email}` | Author email from config | `your.email@example.com` |
| `{project}` | Project name from config | `My Project` |
| `{copyright_holder}` | Copyright holder (defaults to author) | `Your Company` |
| `{interpreter}` | Script interpreter (for shebang) | `python3`, `bash`, etc. |

### Per-Language Template Override

You can override the default template for specific file extensions:

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

### Complete Configuration Example

See the included [`.auto-header.toml`](.auto-header.toml) file in this repository for a complete example with multiple language overrides.

## 📝 Usage

1. **Create a `.auto-header.toml` file** in one of the locations mentioned above
2. **Restart Zed** (only needed after creating config for the first time)
3. **Create a new file** in Zed
4. **Header is automatically inserted** when you open an empty file

**Note**: Headers are only inserted into completely empty files. If a file already has content, no header will be added.

## 💡 Header Example

With the basic configuration above, creating a new `example.rs` file will automatically insert:

```rust
/*
 * File: example.rs
 * Project: My Project
 * Author: Your Name <your.email@example.com>
 * Created: 2025-11-24 19:30:00
 *
 * Copyright (c) 2025 Your Company
 * All rights reserved.
 */
```

## 🔧 Troubleshooting

### Extension not working

**Problem**: Creating new files doesn't insert headers

**Solutions**:
1. **Check config file exists**:
   ```bash
   # Check project root
   ls -la .auto-header.toml
   
   # Check Zed config directory (Linux/macOS)
   ls -la ~/.config/zed/auto-header.toml
   
   # Check home directory
   ls -la ~/.auto-header.toml
   ```

2. **Restart Zed** after creating the config file for the first time

3. **Ensure file is empty**: Headers are only inserted into new, completely empty files

4. **Check Zed logs**:
   - `Ctrl+Shift+P` → "zed: open log"
   - Look for "Auto File Header" messages

### Download failures

**Problem**: Extension shows errors like "Failed to fetch release from GitHub"

**Solutions**:
1. **Check internet connection**: Ensure you can access github.com

2. **Wait and retry**: GitHub API may have rate limits

3. **Manual download** (as fallback):
   - Download binary from [Releases](https://github.com/MrAMS/zed-auto-file-header/releases)
   - Place in your project directory with name:
     - Linux/macOS: `auto-header-server`
     - Windows: `auto-header-server.exe`

### First-time download status

**Expected behavior**: On first use, you'll see "auto-header: Downloading..." in Zed's status bar for a few seconds. This only happens once as the binary (~2-3 MB) is downloaded and cached.

### Platform not supported

**Problem**: Error message "Unsupported platform"

**Supported platforms**:
- Linux: x86_64, ARM64 (aarch64)
- macOS: x86_64 (Intel), ARM64 (Apple Silicon)
- Windows: x86_64

If your platform isn't supported, please [open an issue](https://github.com/MrAMS/zed-auto-file-header/issues).

### Headers not inserting for specific languages

**Problem**: Headers work for some files but not others

**Solutions**:
1. **Check file extension**: Ensure your file extension is in the [supported languages](#-supported-languages) list

2. **Check built-in templates**: Not all extensions have built-in templates, but you can add custom ones in `.auto-header.toml`:
   ```toml
   [header.by_extension.xyz]
   template = """
   # Your custom template for .xyz files
   """
   ```

## 👨‍💻 Development

### Development Installation

For extension developers who want to modify or test the extension:

1. Clone the repository:
   ```bash
   git clone https://github.com/MrAMS/zed-auto-file-header.git
   cd zed-auto-file-header
   ```

2. Install as a dev extension in Zed:
   - Open Zed
   - `Ctrl+Shift+P` → `zed: install dev extension`
   - Select the `extension` directory

3. Create a config file:
   ```bash
   cp .auto-header.toml ~/.auto-header.toml
   nano ~/.auto-header.toml   # Edit with your details
   ```

4. Restart Zed and test

Configuration changes take effect immediately (no rebuild needed).

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

### Release Process

Tags matching `v*` automatically trigger GitHub Actions to build binaries for all platforms and publish a release. See [PUBLISHING.md](PUBLISHING.md) for details.

## 🏗️ Architecture

This extension uses an **LSP Wrapper** design:

1. **Zed Extension (Wasm)**: 
   - Checks for cached/local `auto-header-server` binary
   - Downloads from GitHub Releases if not found (with status indicator)
   - Launches the LSP server

2. **LSP Server (Native)**:
   - Listens for `didOpen` events
   - Checks if file is empty and config exists
   - Injects appropriate header template based on language

This architecture enables zero-dependency installation while maintaining full LSP capabilities.

## 📄 License

MIT License © 2025 MrAMS

## 🤝 Contributing

Issues and pull requests are welcome!

**Repository:** https://github.com/MrAMS/zed-auto-file-header

---

**Quick Setup Checklist:**
- ✅ Install extension from Zed marketplace
- ✅ Create `.auto-header.toml` (project root or `~/.config/zed/`)
- ✅ Add your author info and customize template
- ✅ Restart Zed
- ✅ Create a new file and see the magic! ✨

