# Publishing Guide

## Architecture

This extension uses a **zero-dependency** approach - pre-built binaries are automatically downloaded from GitHub Releases.

**Supported Platforms:**
- Linux: x86_64, ARM64 (aarch64)
- macOS: Intel (x86_64), Apple Silicon (ARM64)
- Windows: x86_64

## Release Process

### 1. Update Version

Update version in `extension.toml`:
```toml
version = "0.2.3"
```

### 2. Update CHANGELOG.md

Document all changes in `CHANGELOG.md`.

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

## Binary Download Flow

The extension (`extension/src/lib.rs`) handles binary downloads:

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
├── extension/
│   ├── extension.toml             # Extension metadata & version
│   └── src/lib.rs                 # Binary download logic
├── server/
│   └── src/main.rs                # LSP server (header insertion)
├── LICENSE                        # MIT (required)
├── README.md                      # User documentation
└── CHANGELOG.md                   # Version history
```

## Testing Checklist

Before release:

- [ ] All platforms build successfully
- [ ] Binary download works on Linux/macOS/Windows
- [ ] Headers insert correctly for all supported languages
- [ ] Config file changes reload without restart
- [ ] No Rust dependency required for end users
- [ ] Documentation is up-to-date
- Uploads to GitHub Release

### Binary Size

- Linux: ~2-3MB
- macOS: ~2-3MB
- Windows: ~2-3MB

All compressed in archives for faster download.
