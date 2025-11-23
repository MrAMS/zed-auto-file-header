# Platform Support

This extension is designed to work seamlessly across Windows, macOS, and Linux.

## Supported Platforms

- ✅ **Linux** (x86_64, aarch64)
- ✅ **macOS** (Intel, Apple Silicon)
- ✅ **Windows** (x86_64)

## Platform-Specific Paths

### Configuration File Locations

The extension searches for `.auto-header.toml` in the following locations (in priority order):

#### Linux
1. `{project_root}/.auto-header.toml` (project-specific)
2. `~/.config/zed/auto-header.toml` (Zed-specific)
3. `~/.auto-header.toml` (user-global)

#### macOS
1. `{project_root}/.auto-header.toml` (project-specific)
2. `~/Library/Application Support/Zed/auto-header.toml` (Zed-specific)
3. `~/.auto-header.toml` (user-global)

#### Windows
1. `{project_root}\.auto-header.toml` (project-specific)
2. `%APPDATA%\Zed\auto-header.toml` (Zed-specific)
3. `%USERPROFILE%\.auto-header.toml` (user-global)

### Extension Installation Paths

#### Linux
- Work directory: `~/.local/share/zed/extensions/work/auto-header/`
- Installed: `~/.local/share/zed/extensions/installed/auto-header/`

#### macOS
- Work directory: `~/Library/Application Support/Zed/extensions/work/auto-header/`
- Installed: `~/Library/Application Support/Zed/extensions/installed/auto-header/`

#### Windows
- Work directory: `%APPDATA%\Zed\extensions\work\auto-header\`
- Installed: `%APPDATA%\Zed\extensions\installed\auto-header\`

## Binary Names

- Linux/macOS: `auto-header-server`
- Windows: `auto-header-server.exe`

## Cross-Platform Features

### Path Handling
- Uses platform-native path separators automatically
- Handles Windows drive letters in file URIs (`file:///C:/...`)
- Uses `dirs` crate for platform-specific directories

### File Permissions
- Unix/Linux: Sets executable bit (0o755) using `zed::make_file_executable`
- Windows: No explicit permission setting needed

### Environment Variables
- Uses `HOME` on Unix/Linux
- Falls back to `USERPROFILE` on Windows
- Uses `dirs::home_dir()` as platform-agnostic fallback

## Building for Different Platforms

### Current Platform
```bash
./build-dev.sh
```

### Cross-Compilation (Advanced)

For Windows from Linux:
```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --package auto-header-server --target x86_64-pc-windows-gnu
```

For macOS from Linux:
```bash
# Requires osxcross toolchain
rustup target add x86_64-apple-darwin
cargo build --release --package auto-header-server --target x86_64-apple-darwin
```

## Testing on Different Platforms

1. Install via "Install Dev Extension"
2. Ensure `.auto-header.toml` exists (activation required)
3. Create a brand new, completely empty file (no whitespace) and save with supported extension
4. Verify header inserted; check line endings appropriate for platform

## Known Platform Differences

### Line Endings
- Windows: CRLF (`\r\n`)
- Unix/Linux/macOS: LF (`\n`)

The extension uses the platform's default line ending style automatically.

### Path Case Sensitivity
- Windows: Case-insensitive
- Unix/Linux: Case-sensitive
- macOS: Case-insensitive by default (configurable)

Configuration file names use lowercase (`.auto-header.toml`) consistently.

## Troubleshooting Platform Issues

### Windows
- If binary doesn't run, confirm `.exe` name and Defender not blocking
- Check paths: `echo %APPDATA%`

### macOS
- If blocked by Gatekeeper, allow in System Settings → Privacy & Security

### Linux
- Verify execute permission: `ls -l .../auto-header-server` → expect `-rwxr-xr-x`

## Reporting Platform-Specific Bugs

When reporting platform-specific issues, please include:
1. Operating System and version
2. Zed version
3. Output of checking extension directory paths
4. Any error messages from Zed logs

See [GitHub Issues](https://github.com/MrAMS/zed-auto-file-header/issues) to report platform-specific bugs (include OS, Zed version, config path used).
