# Cross-Platform Code Review Checklist

This document tracks all cross-platform considerations in the codebase.

## ✅ Completed Cross-Platform Improvements

### Extension (`extension/src/lib.rs`)

- [x] **Binary name detection**
  ```rust
  let binary_name = if cfg!(target_os = "windows") {
      "auto-header-server.exe"
  } else {
      "auto-header-server"
  };
  ```

- [x] **Home directory detection**
  ```rust
  let home = std::env::var("HOME")
      .or_else(|_| std::env::var("USERPROFILE"))  // Windows fallback
      .unwrap_or_default();
  ```

- [x] **Path separator handling**
  ```rust
  std::path::MAIN_SEPARATOR  // Uses \ on Windows, / on Unix
  ```

- [x] **Platform-specific Zed paths**
  ```rust
  // Windows: %APPDATA%\Zed\extensions\installed
  // macOS: ~/Library/Application Support/Zed/extensions/installed
  // Linux: ~/.local/share/zed/extensions/installed
  ```

- [x] **Executable permission check**
  ```rust
  #[cfg(unix)]
  use std::os::unix::fs::MetadataExt;  // Unix-only
  
  #[cfg(not(unix))]
  path.exists()  // Windows fallback
  ```

- [x] **Embedded binary**
  ```rust
  #[cfg(not(target_os = "windows"))]
  static EMBEDDED_SERVER: &[u8] = include_bytes!("../auto-header-server");
  
  #[cfg(target_os = "windows")]
  static EMBEDDED_SERVER: &[u8] = include_bytes!("../auto-header-server.exe");
  ```

### Server (`server/src/main.rs`)

- [x] **Config directory paths** (using `dirs` crate)
  ```rust
  dirs::config_dir()  // Cross-platform
  // Linux: ~/.config
  // macOS: ~/Library/Application Support
  // Windows: %APPDATA%
  ```

- [x] **Home directory** (using `dirs` crate)
  ```rust
  dirs::home_dir()  // Cross-platform
  ```

- [x] **Path joining** (using `PathBuf::join()`)
  ```rust
  config_dir.join("zed").join("auto-header.toml")  // Platform-agnostic
  ```

- [x] **URI path handling**
  ```rust
  // Handle Windows drive letters: file:///C:/path
  if cfg!(target_os = "windows") && uri_path.starts_with('/') {
      if uri_path.chars().nth(2) == Some(':') {
          &uri_path[1..]  // Remove leading '/'
      }
  }
  ```

## Platform-Specific Dependencies

### `dirs` Crate (v5.0)
- ✅ Cross-platform directory detection
- Handles: `home_dir()`, `config_dir()`
- Tested on: Windows, macOS, Linux

### `std::path` Module
- ✅ Uses `PathBuf` for all path operations
- ✅ `join()` uses platform-native separators
- ✅ `MAIN_SEPARATOR` constant for explicit separators

## Testing Matrix

| Platform | Build | Runtime | Config Loading | Header Insertion |
|----------|-------|---------|----------------|------------------|
| Linux x64 | ✅ | ✅ | ✅ | ✅ |
| macOS Intel | ⏳ | ⏳ | ⏳ | ⏳ |
| macOS ARM64 | ⏳ | ⏳ | ⏳ | ⏳ |
| Windows x64 | ⏳ | ⏳ | ⏳ | ⏳ |

Legend:
- ✅ Tested and working
- ⏳ Not yet tested
- ❌ Known issue

## Potential Platform Issues (Future)

### File System
- [ ] Case sensitivity differences (Linux vs Windows/macOS)
- [ ] Path length limits (Windows MAX_PATH = 260)
- [ ] Special characters in filenames

### Line Endings
- [x] Uses platform default (Zed handles this)
- No explicit CRLF/LF conversion needed

### Symbolic Links
- [ ] Test behavior with symlinked config files
- [ ] Test workspace_root with symlinked directories

## Build Targets

### Supported
- `x86_64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-pc-windows-msvc`

### Wasm Target
- `wasm32-wasip1` (extension only)

## Code Patterns to Avoid

❌ **Don't use:**
```rust
format!("{}/.config/app", home)  // Hardcoded Unix path
"/home/user/file"                // Hardcoded Unix separator
std::env::var("HOME").unwrap()   // Panic on Windows
```

✅ **Do use:**
```rust
dirs::config_dir().join("app")                    // Cross-platform
PathBuf::from(home).join("file")                 // Platform separators
std::env::var("HOME").or_else(|_| ...)          // Fallback
```

## Documentation

- [x] PLATFORM_SUPPORT.md - Comprehensive platform guide
- [x] README.md - Updated with platform support info
- [x] CROSS_PLATFORM_CHECKLIST.md - This document

## Future Improvements

- [ ] Automated CI testing on all platforms
- [ ] Windows-specific installer
- [ ] macOS code signing
- [ ] Linux package (.deb, .rpm, AppImage)

Last Updated: 2025-11-23
