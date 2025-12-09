# Publishing Guide

## Architecture

This extension uses a **zero-dependency** approach - pre-built binaries are automatically downloaded from GitHub Releases.

**Supported Platforms:**
- Linux: x86_64, ARM64 (aarch64)
- macOS: Intel (x86_64), Apple Silicon (ARM64)
- Windows: x86_64

## Release Process

### 1. Update Version

**Automated Method (Recommended):**

```bash
# Bump version to 0.2.8 and update all files
./bump-version.sh 0.2.8

# Edit CHANGELOG.md to add actual changes (replace TODO section)
# Then verify all versions match
./check-version.sh
```

The `bump-version.sh` script automatically updates:
- `Cargo.toml` - Main version source
- `extension.toml` - Extension metadata
- `CHANGELOG.md` - Adds new version header with current date
- `src/lib.rs` - Uses `env!("CARGO_PKG_VERSION")` to auto-sync from Cargo.toml

**Manual Method:**

If you prefer manual updates, ensure these files are synchronized:
1. Update `Cargo.toml`: `version = "0.2.8"`
2. Update `extension.toml`: `version = "0.2.8"`
3. Add entry to `CHANGELOG.md` with date and changes
4. Run `./check-version.sh` to verify consistency

**Note:** `src/lib.rs` automatically reads version from `Cargo.toml` at compile time via `env!("CARGO_PKG_VERSION")`, so you never need to manually update it.

### 2. Build and Test

```bash
# Build the extension
cargo build --release

# Test locally if needed
# (See DEV_TESTING.md for details)
```

### 3. Create Git Tag

```bash
git tag v0.2.3
git push origin v0.2.3
```

### 4. Automated Build

GitHub Actions (`.github/workflows/release.yml`) automatically:
- Builds LSP server for all 5 platforms
- Creates release with binary assets
- Publishes to GitHub Releases

### 5. Submit to Zed Extensions

1. Fork: https://github.com/zed-industries/extensions
2. Add/update your extension in the fork
3. Create pull request
4. Wait for review and merge

## Version Management

### Version Sync Scripts

**`check-version.sh`** - Validates version consistency:
- Checks `Cargo.toml`, `extension.toml`, and `CHANGELOG.md`
- Exits with error if versions don't match
- Use before committing to catch mistakes

**`bump-version.sh <version>`** - Automates version updates:
- Updates all version files in one command
- Adds CHANGELOG.md template with current date
- Shows next steps for commit and push
- Example: `./bump-version.sh 0.2.8`

### Version Sources

| File | Purpose | Auto-Updated |
|------|---------|--------------|
| `Cargo.toml` | **Primary source** - Rust package version | Manual/Script |
| `extension.toml` | Zed extension metadata | Manual/Script |
| `CHANGELOG.md` | User-facing version history | Manual/Script |
| `src/lib.rs` | Runtime version (via `CARGO_PKG_VERSION`) | ✅ Automatic |

The `lib.rs` uses `env!("CARGO_PKG_VERSION")` to read from `Cargo.toml` at compile time, eliminating manual sync errors.

## Binary Download Flow

The extension (`src/lib.rs`) handles binary downloads:

```rust
fn language_server_binary_path(...) -> Result<String> {
    // 1. Check if binary already exists
    // 2. If not, fetch latest release from GitHub
    // 3. Download appropriate binary for platform
    // 4. Extract and make executable
    // 5. Return path to binary
}
```

Users see download progress in Zed's status bar.

## Files Overview

```
zed-auto-file-header/
├── .github/workflows/release.yml   # Automated cross-platform builds
├── Cargo.toml                     # Extension Rust code config
├── extension.toml                 # Extension metadata & version
├── src/lib.rs                     # Binary download logic
├── server/
│   └── src/main.rs                # LSP server (header insertion)
├── LICENSE                        # MIT (required)
├── README.md                      # User documentation
└── CHANGELOG.md                   # Version history
```

## Testing Checklist

Before release:

- [ ] Run `./check-version.sh` - all versions match
- [ ] CHANGELOG.md updated with actual changes (not TODO)
- [ ] Build succeeds: `cargo build --release`
- [ ] All platforms build successfully (via GitHub Actions)
- [ ] Binary download works on Linux/macOS/Windows
- [ ] Headers insert correctly for all supported languages
- [ ] Config file changes reload without restart
- [ ] No Rust dependency required for end users
- [ ] Documentation is up-to-date
- [ ] Git tag created and pushed
- [ ] GitHub Release created with binaries

### Binary Size

- Linux: ~2-3MB
- macOS: ~2-3MB
- Windows: ~2-3MB

All compressed in archives for faster download.
