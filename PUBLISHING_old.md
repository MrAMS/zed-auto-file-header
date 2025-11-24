# Publishing to Zed Extensions Registry

## Prerequisites

- ✅ LICENSE file (MIT) - **Required since Oct 1, 2025**
- ✅ Valid `extension.toml` with required fields
- ✅ Extension structure follows Zed conventions
- ✅ Auto-build mechanism for LSP server

## Project Structure

```
zed-auto-file-header/              ← Repository root
├── LICENSE                         ← MIT license (required)
├── README.md                       ← Documentation
├── extension/                      ← Extension code
│   ├── extension.toml              ← Extension manifest
│   ├── Cargo.toml                  ← Wasm build config
│   └── src/lib.rs                  ← Extension logic + auto-build
└── server/                         ← LSP server source
    ├── Cargo.toml
    └── src/main.rs                 ← Server implementation
```

## How Auto-Build Works

When a user installs the extension from the registry:

1. **First Launch**: Extension calls `language_server_command`
2. **Auto-Build**: If server binary not found, extension automatically runs:
   ```bash
   cargo build --release --manifest-path ../server/Cargo.toml
   ```
3. **Caching**: Built binary path is cached for subsequent uses
4. **Instant Startup**: Next time, uses cached binary (no rebuild)

**User Requirement**: Rust must be installed via `rustup` (Zed requirement for dev extensions).

## Publishing Steps

### 1. Fork and Clone Extensions Repo

```bash
# Fork https://github.com/zed-industries/extensions to your account
git clone https://github.com/YOUR_USERNAME/extensions
cd extensions
```

### 2. Add as Submodule

```bash
git submodule add https://github.com/MrAMS/zed-auto-file-header.git extensions/auto-header
```

**Important**: Use `path` field since extension is in `extension/` subdirectory.

### 3. Update extensions.toml

Add entry:
```toml
[auto-header]
submodule = "extensions/auto-header"
path = "extension"              # Point to extension subdirectory
version = "0.1.3"
```

### 4. Sort and Commit

```bash
pnpm sort-extensions
git add .gitmodules extensions.toml extensions/auto-header
git commit -m "Add auto-header extension"
git push
```

### 5. Create PR

Open PR to `zed-industries/extensions` with description:

```markdown
## Extension: Auto File Header

Automatically inserts customizable file headers when creating new empty files.

### Features
- 30+ built-in language templates
- Configurable via `.auto-header.toml`
- Auto-builds LSP server on first use
- Cross-platform (Linux/macOS/Windows)

### Technical Details
- **Auto-Build**: Extension compiles LSP server from source on first use
- **Requirement**: Users need Rust via rustup (standard for Zed dev extensions)
- **Structure**: Extension in `extension/` subdirectory (uses `path` field)

### Repository
https://github.com/MrAMS/zed-auto-file-header

### License
MIT

### Checklist
- [x] Valid LICENSE file
- [x] Extension submodule added with `path` field
- [x] Version matches extension.toml
- [x] Tested locally via "Install Dev Extension"
```

## Why This Structure Works

### Advantages
1. **No Pre-built Binaries**: Source-only distribution (smaller repo)
2. **Always Fresh**: Users get latest server code
3. **Cross-Platform**: Cargo handles platform-specific builds
4. **Version Lock**: Server and extension always match

### Trade-offs
- **First-use delay**: 1-2 minute build on first launch
- **Rust requirement**: Users must have `rustup` installed
- **Build errors**: Possible if user's Rust toolchain has issues

## Alternative: Pre-built Binaries (Future)

For faster user experience, could add:
1. GitHub Actions to build binaries for each platform
2. Download logic in `language_server_command`
3. Fallback to auto-build if download fails

This would require:
- CI pipeline producing Linux/macOS/Windows binaries
- GitHub Releases for hosting
- Download + verification logic in extension

Current auto-build approach is simpler and follows Zed's dev extension model.

## Updating the Extension

To publish updates:

1. Update version in `extension/extension.toml`
2. Commit and tag in your repo:
   ```bash
   git commit -am "v0.1.4: Description of changes"
   git tag v0.1.4
   git push --tags
   ```
3. Update submodule in `zed-industries/extensions`:
   ```bash
   cd extensions/auto-header
   git pull origin master
   cd ../..
   ```
4. Update version in `extensions.toml`
5. Create PR with changes

## Testing Before Publishing

```bash
# In this repository
./build-dev.sh

# In Zed
# 1. Install Dev Extension → select extension/ directory
# 2. Create new file in a project with .auto-header.toml
# 3. Verify header is inserted
# 4. Check Zed.log for any errors
```

## Post-Publishing

Once merged:
- Extension appears in Zed's extension registry
- Users can install via Extensions panel
- Auto-build happens on first use per-user
- Updates follow the same PR process
