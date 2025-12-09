# Changelog

## Version 0.2.7 - 2024-12-04

### 🐛 Critical Bug Fix

- **Fixed Extension ID Mismatch**: Resolved CI/CD packaging error in Zed extensions registry
  - Changed extension ID from `auto-header` to `auto-file-header` to match `extensions.toml`
  - Updated language server configuration key to match new ID
  - This fix ensures the extension can be properly packaged and published

---

## Version 0.2.6 - 2024-12-04

### 🐛 Critical Bug Fix

- **Fixed URI Path Decoding**: Resolved issue where paths with non-ASCII characters (e.g., Chinese, emoji) were not properly decoded
  - Previously: `/home/user/项目/` was treated as `/home/user/%E9%A1%B9%E7%9B%AE/`
  - Now uses `uri.to_file_path()` for proper URL decoding
  - Config files are now correctly found in directories with non-ASCII names

### ✨ Extended Language Support

Added support for 15+ new programming languages:

**Hardware Description Languages:**
- Verilog (`.v`, `.vh`)
- SystemVerilog (`.sv`, `.svh`)
- Tcl (`.tcl`)

**Modern Languages:**
- Zig (`.zig`)
- Dart (`.dart`)
- Nim (`.nim`)
- Crystal (`.cr`)
- D (`.d`)
- Odin (`.odin`)
- Gleam (`.gleam`)

**Functional Languages:**
- OCaml (`.ml`, `.mli`)
- F# (`.fs`, `.fsi`, `.fsx`)
- Racket (`.rkt`)

**Document/Typesetting:**
- Typst (`.typ`)
- LaTeX/TeX (`.tex`, `.latex`, `.sty`, `.cls`)
- BibTeX (`.bib`)

### 🔧 Technical Improvements

- Proper cross-platform URI to file path conversion
- Better handling of special characters in file paths
- Improved error messages when URI conversion fails

---

## Version 0.2.5 - 2024-12-04

### 🐛 Critical Bug Fixes

- **Fixed Header Insertion Logic**: Removed redundant condition check that prevented headers from being inserted
  - The bug was introduced in v0.2.4 where after checking conditions, the code had an unnecessary `if` statement
  - This caused headers not to be inserted in projects other than the extension source code
  - Now works correctly in all projects

### 🔧 Improvements

- **Simplified Extension Code**: Removed unnecessary caching mechanism
  - Extension now directly checks if the required server version exists locally
  - Only downloads from GitHub if the matching version is not found
  - No more stale cache issues when upgrading versions
  
- **Version Management**: Extension version now explicitly controls server version
  - Server binary path: `auto-header-server-v{extension_version}/auto-header-server`
  - Each extension version uses its own server binary
  - Automatic version upgrade when extension is updated

- **Removed Development Mode**: Simplified local testing
  - No more `AUTO_HEADER_DEV_MODE` environment variable needed
  - For local testing: just place the server binary in project root
  - Falls back to GitHub download if no local binary found

### 📝 Documentation

- Updated `DEV_TESTING.md` with simplified testing workflow
- Added `build-dev.sh` script for quick local builds

---

## Version 0.2.4 - 2024-12-04

### 🐛 Bug Fixes

- **Fixed Duplicate Header Insertion**: Resolved a critical issue where headers could be inserted twice when opening new files
  - Removed stateful tracking mechanism that could fail in edge cases (file deletion, editor crash, etc.)
  - Implemented stateless content-based detection for more reliable duplicate prevention
  - Headers are now only inserted when files are truly empty, regardless of how many times `did_open` is triggered
  - Properly handles scenarios where files are deleted and recreated with the same name

### 🔧 Improvements

- Simplified server architecture by removing unnecessary state management
- Improved reliability and predictability of header insertion behavior

---

## Version 0.2.3 - 2024-12-04

### ✨ New Features

- **Automatic Comment Wrapping**: Revolutionary architecture that eliminates code duplication and configuration complexity!
  - **No configuration needed** - comment wrapping is always automatic
  - Write template content **once** without any comment markers
  - Extension intelligently wraps with language-appropriate syntax:
    - C/Rust/Java/JavaScript/Go: `/* ... */`
    - Python: `""" ... """` with UTF-8 encoding header
    - Shell scripts: `#` with automatic shebang detection
    - HTML/XML: `<!-- ... -->`
    - SQL: `--`, Lua/Haskell: `--[[ ... ]]`
    - Verilog/SystemVerilog: `//`
    - Tcl: `#`
    - Lisp: `;;;;`, Erlang: `%%`, Vim: `"`
    - And 35+ more languages!

- **Expanded Language Support**:
  - Added Verilog (`.v`, `.vh`)
  - Added SystemVerilog (`.sv`, `.svh`)
  - Added Tcl (`.tcl`)

- **Optional Configuration**: The `[header]` section is now optional
  - If omitted, a sensible default template will be used
  - Users can start with minimal configuration (just `[author]` and `[project]`)

- **Open Source License Examples**: Added documentation and examples for:
  - MIT License
  - Mozilla Public License (MPL-2.0)
  - Apache License 2.0
  - Easy to customize for other licenses

### 🏗️ Code Architecture

- **Major Refactoring**: Eliminated hundreds of lines of duplicated template code
  - Created `CommentStyle` enum to abstract comment format rules
  - Single source of truth for each language's comment syntax
  - Removed redundant `get_builtin_template()` method (300+ lines)
  - Removed redundant `wrap_with_comment_style()` method (150+ lines)
  - Simplified `get_template_for_file()` to just 15 lines
  - **Result**: Cleaner, more maintainable codebase

### 🗑️ Breaking Changes

- **Removed `use_builtin_comment_style` option**: Comment wrapping is now always enabled
  - This simplifies the configuration and eliminates user confusion
  - The feature is so useful that there's no reason to disable it

### 🏗️ Project Structure Changes

- **Extension code moved to root**: For better Zed compatibility
  - Extension Rust code now in root `src/` instead of `extension/src/`
  - `Cargo.toml` in root instead of workspace configuration
  - `server/` remains independent for LSP server
  - **Note**: This only affects development - users are not affected
  - Users who need exact comment control can still include syntax in their template

### 📚 Documentation

- Updated all configuration examples to reflect simpler design
- Removed references to `use_builtin_comment_style` option
- Clarified that comment wrapping is automatic and always-on
- Improved README with clearer explanation of the wrapping feature

## Version 0.2.2 - 2025-11-24

### 🎨 User Experience

- **Download Status Indicator**: Added native Zed status bar notification during first-time binary download
  - Shows "auto-header: Downloading..." in status bar
  - Uses official Zed `set_language_server_installation_status()` API
  - Status automatically clears when download completes
  - Much cleaner UX compared to console logs

### 🔧 Technical Improvements

- Integrated with Zed's `LanguageServerInstallationStatus` API for proper status reporting
- Removed verbose eprintln logs in favor of native UI integration
- Better alignment with Zed extension best practices

## Version 0.2.1 - 2025-11-24

### 🔧 Improvements

- **Enhanced Error Messages**: Added detailed, user-friendly error messages for all failure scenarios:
  - Unsupported platforms: Lists all supported platforms with issue reporting link
  - Download failures: Shows URL, target directory, and network troubleshooting tips
  - Asset not found: Displays expected filename and available assets for debugging
  - Permission failures: Shows binary path and detailed error information

### 📚 Documentation

- **Comprehensive README Updates**: 
  - Complete rewrite emphasizing zero-dependency architecture
  - Added detailed troubleshooting section covering common issues
  - Improved quick start guide with clearer steps
  - Platform support matrix with all supported architectures
  - Better configuration examples and template variables documentation
- **Chinese Documentation**: Fully synchronized README_CN.md with all improvements
- **Project Cleanup**: Removed outdated documentation files (ARCHITECTURE.md, QUICKSTART.md, etc.)

### 🐛 Bug Fixes

- **Critical**: Fixed `pre_release` flag set to `false` to correctly fetch stable releases (was causing "finding a prerelease" errors)

### 🧹 Code Quality

- Cleaned up GitHub Actions workflow (removed debug output)
- Improved code organization and error handling
- Better separation of concerns in extension code

## Version 0.2.0 - 2024-11-24

### 🎉 Major Changes - Zero Dependencies!

**BREAKING CHANGE**: Complete architectural overhaul - users no longer need Rust installed!

- **Zero Dependencies**: Extension now downloads pre-built binaries from GitHub Releases automatically
- **Instant Startup**: No compilation delay - binaries download in seconds (vs 1-2 minutes compilation)
- **Cross-Platform Binaries**: Pre-built for Linux (x86_64, ARM64), macOS (Intel, Apple Silicon), Windows (x86_64)
- **GitHub Actions CI/CD**: Automated cross-platform builds on every release

### Technical Implementation

- Uses Zed Extension API's `latest_github_release()` and `download_file()` functions
- Automatically detects platform and architecture via `current_platform()`
- Downloads appropriate binary archive (.tar.gz for Unix, .zip for Windows)
- Caches binary for instant subsequent launches
- Binary size: ~2-3MB per platform

### Migration Notes for Users

Upgrading from v0.1.x:
- **No longer need** Rust toolchain installed
- **First launch** downloads binary instead of compiling (much faster)
- **No code changes** needed in your projects or configs
- Extension will automatically clean up old compiled binaries

### Migration Notes for Developers

- Replaced `std::process::Command::new("cargo")` with `download_file()`
- Added `.github/workflows/release.yml` for automated builds
- Updated all documentation to reflect zero-dependency approach

## Version 0.1.3

### Major Changes
- **Auto-build LSP server**: Extension now automatically compiles the LSP server binary on first use when installed from Zed extensions registry
- **Workspace folder tracking**: Fixed bug where files in subdirectories didn't trigger header insertion
- **Cross-platform path handling**: Unified for Linux/macOS/Windows

### Features
- Dynamic config reload per header insertion (no stale cache)
- Proper workspace root detection from LSP initialization params
- Auto-compilation fallback when binary not found

### Bug Fixes
- **Critical**: Files in project subdirectories now correctly insert headers (was using file parent instead of workspace root)
- Removed embedded binary approach (incompatible with published extensions)

### Documentation
- Added PUBLISHING.md with detailed publication guide
- Updated README with installation methods (registry vs dev)
- Removed obsolete scripts (`package.sh`, `install.sh`, `setup-config.sh`)
- Updated QUICKSTART, ARCHITECTURE, TESTING guides

### Breaking Changes
- Users installing from registry will need Rust via rustup (first-time build ~1-2 min)
- Dev installation no longer requires pre-building (optional via `build-dev.sh`)

### Technical Details
- Extension structure: `extension/` subdirectory (requires `path` field in extensions.toml)
- Build process: `cargo build --release` invoked automatically
- Binary caching: Built once, reused for subsequent launches

### Notes
Activation strictly requires `.auto-header.toml` in project/global locations.

## Version 0.1.0

### Features

#### Built-in Language Support
- **30+ Programming Languages**: Automatic comment format detection for:
  - C/C++, C#, Java, JavaScript, TypeScript, Rust, Scala, Kotlin, Swift, Go
  - Python (with encoding declaration and docstring format)
  - Shell scripts (Bash, Zsh, Fish, Perl, Ruby, R, Julia) with shebang
  - HTML, XML, SVG, CSS, SCSS, SASS, LESS
  - SQL, YAML, Lua, Haskell
  - Lisp, Scheme, Clojure
  - Erlang, Elixir
  - Vim script
  - And more...

#### Smart Configuration
- **Configuration File**: `.auto-header.toml` (follows hidden config file convention)
- **Multiple Locations**: Searches in project directory, `~/.config/zed/`, and home directory
- **No Rebuild Required**: Configuration changes take effect immediately
- **Template Variables**: Rich set of variables including filename, date, author, project, etc.
- **Per-Language Override**: Custom templates for specific file extensions

#### Developer-Friendly (Initial)
- Comprehensive `LANGUAGES.md` with examples
- Default templates usable out of the box
- Works without user config (falls back to built-ins)

### Architecture

- **LSP-Based**: Lightweight Language Server Protocol implementation
- **Wasm Extension**: Minimal Zed extension shim
- **Rust Implementation**: Fast and reliable using `tower-lsp`, `tokio`, `chrono`

### Files

- `.auto-header.toml` - Configuration file with examples and documentation
 (Removed in later versions) interactive setup script now deprecated in favor of manual editing.
- `README.md` - Complete user documentation
- `LANGUAGES.md` - Language support reference
- `server/` - LSP server implementation
- `extension/` - Zed extension (Wasm)

## Design Decisions

### Why `.auto-header.toml`?
- More specific than generic `config.toml`
- Follows Unix convention for hidden config files (dotfile)
- Compatible with common editor extension patterns
- Easy to identify in project directories

### Why Built-in Language Templates?
- **Better UX**: Works out of the box without configuration
- **Consistency**: Standard comment formats across projects
- **Flexibility**: Can still override any template
- **Maintainability**: Users don't need to define 30+ templates

### Configuration Priority
1. Project-specific: `./.auto-header.toml` (allows per-project customization)
2. User global: `~/.config/zed/auto-header.toml` (standard XDG location)
3. Alternative: `~/.auto-header.toml` (traditional dotfile location)
4. Built-in: Default templates (fallback)

## Future Enhancements

Potential improvements for future versions:
- [ ] Support for more languages (Zig, Nim, V, etc.)
- [ ] Template conditions (e.g., different headers for test files)
- [ ] Project-specific variables from `Cargo.toml`, `package.json`, etc.
- [ ] Header update on file save (update date/author)
- [ ] Integration with Git (author from git config)
- [ ] Custom template functions (e.g., word wrap, case conversion)
