# Publishing to Zed Extensions Registry

## Zero Dependencies Architecture

This extension uses a **zero-dependency** approach:

✅ **No Rust Required**: Users don't need to install Rust or any build tools  
✅ **Pre-built Binaries**: Cross-platform binaries automatically built via GitHub Actions  
✅ **Auto-Download**: Extension downloads appropriate binary from GitHub Releases  
✅ **Instant Startup**: No compilation delay - downloads in seconds  

### Supported Platforms

- Linux x86_64
- Linux ARM64 (aarch64)
- macOS Intel (x86_64)
- macOS Apple Silicon (ARM64)
- Windows x86_64

## Prerequisites

- ✅ LICENSE file (MIT) - **Required since Oct 1, 2025**
- ✅ Valid `extension.toml` with required fields
- ✅ GitHub repository with automated releases
- ✅ Cross-platform binaries published to GitHub Releases

## Release Workflow

### 1. Create GitHub Release

Tag a version to trigger automated builds:

```bash
git tag v0.2.0
git push origin v0.2.0
```

GitHub Actions (`.github/workflows/release.yml`) automatically:
- Builds LSP server for all platforms
- Creates compressed archives:
  - `x86_64-unknown-linux-gnu.tar.gz`
  - `aarch64-unknown-linux-gnu.tar.gz`
  - `x86_64-apple-darwin.tar.gz`
  - `aarch64-apple-darwin.tar.gz`
  - `x86_64-pc-windows-msvc.zip`
- Publishes them as release assets

### 2. Submit to zed-industries/extensions

1. Fork `https://github.com/zed-industries/extensions`

2. Add your extension to `extensions.toml`:
   ```toml
   [auto-header]
   submodule = "extensions/auto-header"
   path = "extension"  # Required - extension in subdirectory
   version = "0.2.0"
   ```

3. Add git submodule:
   ```bash
   git submodule add https://github.com/MrAMS/zed-auto-file-header.git extensions/auto-header
   cd extensions/auto-header
   git checkout v0.2.0
   cd ../..
   git add extensions.toml .gitmodules extensions/auto-header
   git commit -m "Add Auto File Header extension v0.2.0"
   ```

4. Create Pull Request with:
   ```markdown
   # Add Auto File Header Extension
   
   Automatically inserts customizable file headers when creating new files.
   
   ## Features
   - Zero dependencies (downloads pre-built binaries)
   - Supports 35+ languages
   - Customizable templates via TOML config
   - Cross-platform (Linux, macOS, Windows)
   - Multi-workspace support
   
   ## Technical Details
   - Uses LSP protocol for header insertion
   - Auto-downloads platform-specific binary from GitHub Releases
   - Config file: `.auto-header.toml` (project/global)
   
   ## Testing
   - ✅ Tested on Linux x86_64
   - ✅ Tested on macOS ARM64
   - ✅ Tested on Windows x86_64
   - ✅ Zero-dependency installation verified
   
   Repository: https://github.com/MrAMS/zed-auto-file-header
   License: MIT
   ```

## How Extension Works for End Users

1. **User installs** from Zed extensions panel
2. **First launch**: Extension calls `language_server_command()`
3. **Auto-download**:
   - Detects platform (Linux/macOS/Windows) and architecture
   - Fetches latest release from GitHub API
   - Downloads appropriate binary (e.g., `x86_64-unknown-linux-gnu.tar.gz`)
   - Extracts to extension work directory
   - Sets executable permissions (Unix)
   - Caches path for subsequent launches
4. **Subsequent launches**: Uses cached binary path (instant startup)

## Update Process

To publish a new version:

1. Update version in `extension/extension.toml`
2. Update CHANGELOG.md
3. Commit and tag:
   ```bash
   git commit -am "Release v0.3.0"
   git tag v0.3.0
   git push origin master v0.3.0
   ```
4. GitHub Actions builds and publishes binaries
5. Update zed-industries/extensions:
   ```bash
   cd extensions/auto-header
   git fetch
   git checkout v0.3.0
   cd ../..
   # Update version in extensions.toml
   sed -i 's/version = "0.2.0"/version = "0.3.0"/' extensions.toml
   git add extensions/auto-header extensions.toml
   git commit -m "Update Auto File Header to v0.3.0"
   ```

## Project Structure

```
zed-auto-file-header/
├── .github/
│   └── workflows/
│       └── release.yml          ← Auto-build workflow
├── LICENSE                       ← MIT license (required)
├── extension/
│   ├── extension.toml            ← Manifest (version, metadata)
│   ├── Cargo.toml                ← Wasm extension build
│   └── src/lib.rs                ← Auto-download logic
└── server/
    ├── Cargo.toml                ← LSP server build
    └── src/main.rs               ← Header insertion logic
```

## Verification Checklist

Before submitting PR to zed-industries/extensions:

- [ ] GitHub release published with all platform binaries
- [ ] `extension.toml` version matches git tag
- [ ] LICENSE file present (MIT)
- [ ] README.md complete with usage examples
- [ ] Extension tested on at least 2 platforms
- [ ] Config file (`.auto-header.toml`) documented
- [ ] Binary download works for all platforms
- [ ] No Rust dependency required for end users

## Technical Implementation

### Extension Code (extension/src/lib.rs)

Uses Zed Extension API:
- `latest_github_release()` - Fetches release metadata
- `download_file()` - Downloads and extracts binary
- `make_file_executable()` - Sets Unix permissions
- `current_platform()` - Detects OS and architecture

### GitHub Actions Workflow

Cross-compiles for all platforms:
- Linux: Uses `cross` or native builds
- macOS: Uses `x86_64-apple-darwin` and `aarch64-apple-darwin` targets
- Windows: Uses `x86_64-pc-windows-msvc` target
- Creates platform-specific archives
- Uploads to GitHub Release

### Binary Size

- Linux: ~2-3MB
- macOS: ~2-3MB
- Windows: ~2-3MB

All compressed in archives for faster download.
