# Changelog

## Version 0.1.3

### Changes
- Cross-platform path handling (Linux/macOS/Windows) unified.
- Dynamic config reload per header insertion (removed stale cache).
- Removed obsolete packaging & install scripts (`package.sh`, `install.sh`, `setup-config.sh`).
- Updated documentation (README, README_CN, QUICKSTART, ARCHITECTURE, TESTING).
- Embedded server fallback for non-Windows builds.

### Notes
Activation now strictly requires a `.auto-header.toml` in project or global locations.

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
