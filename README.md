auto-header/
# Auto File Header - Zed Extension

A Zed extension that inserts a customizable file header (Filename, Author, Date, Copyright) automatically when you create a new, empty file.

**Author:** MrAMS <2421653893@qq.com>  
**Repository:** https://github.com/MrAMS/zed-auto-file-header  
**Platforms:** Linux • macOS • Windows

> Activation requires a `.auto-header.toml` in one of: project root, `~/.config/zed/auto-header.toml`, or `~/.auto-header.toml`. See [PLATFORM_SUPPORT.md](PLATFORM_SUPPORT.md) for OS-specific notes.

## Quick Start (Usage)

### Installation from Zed Extensions (When Published)

Once published to the Zed extensions registry:

1. Open Zed → Extensions panel
2. Search for "Auto File Header"
3. Click Install
4. Create config file (required):
   ```bash
   cp .auto-header.toml ~/.auto-header.toml
   nano ~/.auto-header.toml   # edit values
   ```
5. Restart Zed

**Zero Dependencies**: The extension automatically downloads the appropriate pre-built binary for your platform (Linux x86_64/ARM64, macOS Intel/Apple Silicon, or Windows x86_64). No Rust installation required!

### Development Installation

1. Clone & enter the repository:
   ```bash
   git clone https://github.com/MrAMS/zed-auto-file-header.git
   cd zed-auto-file-header
   ```
2. Build native server binary (optional, extension auto-builds if needed):
   ```bash
   ./build-dev.sh
   ```
3. Install as a dev extension in Zed:
   - Open Zed
   - `Ctrl+Shift+P` → `zed: install dev extension`
   - Select the `extension` directory
4. Create a config (required for activation):
   ```bash
   cp .auto-header.toml ~/.auto-header.toml
   nano ~/.auto-header.toml   # edit values
   ```
5. Restart Zed and create a new empty file → header is inserted.

Configuration changes take effect immediately (no rebuild or restart needed after the initial activation).

## Header Example

```
/*
 * File: example.rs
 * Project: My Project
 * Author: Your Name <your.email@example.com>
 * Created: 2025-11-23 19:30:00
 *
 * Copyright (c) 2025 Your Name
 * All rights reserved.
 *
 * Description:
 *   Add your file description here
 */
```

## Configuration

Create `.auto-header.toml` with your details. Search order:
1. Project root: `./.auto-header.toml`
2. Zed config dir: `~/.config/zed/auto-header.toml`
3. Home alternative: `~/.auto-header.toml`

Example:
```toml
[author]
name = "Your Name"
email = "your.email@example.com"

[project]
name = "My Project"
copyright_holder = "Your Company"

[header]
template = """/*\n * File: {filename}\n * Author: {author} <{email}>\n * Date: {date}\n * Copyright (c) {year} {copyright_holder}\n */\n\n"""
```

### Template Variables
`{filename}` `{filepath}` `{date}` `{year}` `{time}` `{author}` `{email}` `{project}` `{copyright_holder}`

### Override Per Extension
```toml
[header.by_extension.py]
template = """# File: {filename}\n# Author: {author}\n# Date: {date}\n\n"""

[header.by_extension.sh]
template = """#!/bin/bash\n# File: {filename}\n# Author: {author}\n# Date: {date}\n\n"""

[header.by_extension.html]
template = """<!--\n  File: {filename}\n  Author: {author}\n  Date: {date}\n-->\n\n"""
```

## Built-in Language Support
Automatically detects and applies suitable comment styles for 30+ languages, including:
- C / C++ / C# / Java / JavaScript / TypeScript / Rust / Go / Swift / Kotlin / Objective-C / Scala
- Python (encoding/docstring aware)
- Shell: Bash, Zsh, Fish + Ruby, Perl
- HTML / XML / SVG
- CSS / SCSS / SASS / LESS
- SQL, YAML, Lua, Haskell, Lisp family, Erlang, Elixir, Vim script, R, Julia

## Architecture
This extension uses an LSP Wrapper design:
- Rust server binary (`tower-lsp`) listening to `didOpen` and injecting headers into empty files.
- Wasm Zed shim launching the server and handling path resolution.

## Project Structure
```

├── Cargo.toml          # Workspace root
├── extension/          # Wasm extension shim
│   ├── Cargo.toml
│   ├── extension.toml  # Zed extension manifest
│   └── src/lib.rs
└── server/             # LSP server logic
    ├── Cargo.toml
    └── src/main.rs
```

## Development
### Build (Manual)
```bash
cargo build --release --package auto-header-server
```
Binary: `target/release/auto-header-server`

### Build Wasm Component
```bash
rustup target add wasm32-wasip1
cargo build --release --package auto-header-extension --target wasm32-wasip1
```
Wasm: `target/wasm32-wasip1/release/auto_header_extension.wasm`

### Test Server Directly
```bash
cargo run --package auto-header-server
```
Send LSP messages via stdin to inspect behavior.

## Customization Summary
Use `.auto-header.toml` to change global or extension-specific templates; no source edits required.

## License
MIT License © 2025 MrAMS

## Contributing
Issues & PRs welcome: [GitHub repository](https://github.com/MrAMS/zed-auto-file-header)
## (Removed outdated content)
Outdated references to `install.sh`, `setup-config.sh`, and `auto-header-package/` have been removed.

