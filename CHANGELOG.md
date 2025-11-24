# Changelog

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
