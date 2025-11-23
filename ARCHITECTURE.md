# Architecture & Design

## Zed Extension Architecture

This extension follows Zed's recommended LSP-based extension pattern with a focus on: **embedded / bundled LSP server for zero external dependency**.

### Standard Zed Extension Approaches

1. **System-installed LSP** (e.g., `rust-analyzer`)
   - User installs LSP server separately
   - Extension finds it in PATH
   - ❌ Requires manual installation
   
2. **Auto-download LSP** (e.g., TypeScript extension)
   - Extension downloads binary on first use
   - ❌ Complex, requires network
   
3. **Bundled / Embedded LSP** (this extension) ✅
   - Server binary built locally and discovered automatically
   - Embedded fallback bytes (non-Windows) to ensure availability
   - ✅ Zero external download
   - ✅ Version compatibility guaranteed
   - ✅ Works offline

## Our Architecture

```
User installs extension
         │
         ▼
┌─────────────────────────────────────────┐
│  Zed Extensions Directory               │
│  (Installed dev extension directory)    │
│                                          │
│  ├── extension.wasm      (Wasm shim)    │
│  ├── auto-header-server  (LSP binary) ◄─┼─── Bundled!
│  ├── extension.toml      (Manifest)     │
│  └── docs/                               │
└─────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────┐
│  Extension Loading (lib.rs)             │
│                                          │
│  1. Try candidate directories (work/installed) │
│  2. Transform between work⇄installed paths    │
│  3. Construct platform-specific path          │
│  4. Use PATH fallback                         │
│  5. Use embedded binary if still missing      │
│  6. Cache resolved path                       │
└─────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────┐
│  LSP Server (main.rs)                   │
│                                          │
│  1. Load config from standard locations │
│  2. Listen for didOpen events           │
│  3. Detect empty files                  │
│  4. Apply language-specific templates   │
└─────────────────────────────────────────┘
```

## Component Breakdown

### 1. Extension (Wasm) - `extension/src/lib.rs`

**Responsibility**: Find and launch the LSP server

**Key Features**:
- **Smart Binary Discovery**:
  1. Extension directory (bundled) - highest priority
  2. System PATH (fallback)
  3. Clear error messages if not found
  
- **Path Caching**: Avoids repeated filesystem lookups

- **Cross-platform**: Handles Windows `.exe` extension

```rust
fn language_server_command(&mut self, ...) -> Result<Command> {
    // 1. Check cache
    // 2. Try extension directory
    // 3. Try PATH
    // 4. Return error with helpful message
}
```

### 2. LSP Server (Native Binary) - `server/src/main.rs`

**Responsibility**: Implement the Language Server Protocol

**Key Features**:
- **Config Loading**: Multi-location search
- **Language Detection**: 30+ built-in templates
- **Header Generation**: Variable substitution
- **Event Handling**: `didOpen` triggers header insertion

```rust
struct AutoHeaderServer {
   client: Client,
   // Config reloaded per header generation (no long-lived cache)
}

async fn did_open(&self, params: DidOpenTextDocumentParams) {
   // 1. Check if file is empty
   // 2. Reload config (project/global)
   // 3. Detect file type
   // 4. Generate header (template + variables)
   // 5. Apply edit via LSP
}
```

### 3. Configuration - `.auto-header.toml`

**Locations** (priority order):
1. Project: `./.auto-header.toml`
2. User: `~/.config/zed/auto-header.toml`
3. Alternative: `~/.auto-header.toml`
4. Built-in defaults

**Design Philosophy**:
- ✅ Zero config by default (built-in templates)
- ✅ Per-project override capability
- ✅ Global user preferences
- ✅ No rebuild required for config changes

## Build & Distribution Strategy

### Development Build
```bash
./build-dev.sh
```
Produces native server in `target/release/auto-header-server`. Zed dev extension install will build the Wasm component automatically.

### Manual Builds
```bash
cargo build --release --package auto-header-server
rustup target add wasm32-wasip1
cargo build --release --package auto-header-extension --target wasm32-wasip1
```

### Embedded Fallback
Non-Windows targets embed server bytes enabling operation even if path lookup fails.

## Why This Design?

### User Experience
- ✅ **One-step installation**: Just copy directory
- ✅ **No dependencies**: Everything bundled
- ✅ **Works offline**: No downloads required
- ✅ **Version locked**: Extension and server always compatible

### Developer Experience
- ✅ **Simple build**: `./build-dev.sh`
- ✅ **Direct dev install**: Use Zed "Install Dev Extension"
- ✅ **Testable**: Run server standalone

### Maintenance
- ✅ **Single version**: No version mismatch issues
- ✅ **Static binary**: No runtime dependencies
- ✅ **Clear errors**: Helpful messages if something's wrong
- ✅ **Debuggable**: Can test server independently

## Comparison with Other Extensions

| Approach | Pros | Cons | Our Choice |
|----------|------|------|------------|
| System LSP | Flexible, reuses existing tools | Requires manual install | ❌ Not user-friendly |
| Auto-download | Automatic | Network required, complexity | ❌ Added failure points |
| **Bundled/Embedded** | **Zero setup, offline, version-locked** | **Slightly larger binary** | **✅ Best UX** |

## Future Enhancements

Potential improvements:

1. **Auto-update mechanism**:
   - Check for updates on launch
   - Optional auto-update via GitHub releases

2. **Multiple server versions**:
   - Support different server versions per workspace
   - Useful for compatibility

3. **Extension marketplace**:
   - Publish to official Zed extensions registry
   - One-click installation from Zed
4. **CI matrix**: Automated builds/test on Linux/macOS/Windows

## Security Considerations

- ✅ **Binary verification**: Users can build from source
- ✅ **No network access**: Fully offline operation
- ✅ **Sandboxed**: LSP runs in separate process
- ✅ **Open source**: Auditable code

## Performance

- **Startup**: < 50ms (binary already on disk)
- **Config load**: < 15ms (reloaded per header insertion)
- **Header insertion**: < 5ms (simple text replacement)
- **Memory**: ~10 MB per server instance
- **Binary size**: 4.7 MB (statically linked, no dependencies)

## File Locations Summary

```
User System:
├── ~/.config/zed/
│   ├── extensions/
│   │   └── auto-header/          ← Extension installation
│   │       ├── extension.wasm
│   │       └── auto-header-server  ← Bundled LSP
│   └── auto-header.toml          ← Global config
│
└── ~/projects/my-project/
    └── .auto-header.toml         ← Project-specific config

Development:
└── zed-file-header/
   ├── target/
   │   ├── release/auto-header-server         ← Native server
   │   └── wasm32-wasip1/release/auto_header_extension.wasm ← Wasm (if manually built)
   └── extension/extension.toml               ← Manifest
```

## Conclusion

This architecture provides:
- ✅ **Best user experience**: Zero installation hassle
- ✅ **Reliable operation**: No external dependencies
- ✅ **Easy distribution**: Single package
- ✅ **Maintainability**: Clear structure, good separation of concerns

The bundled/embedded LSP approach is ideal where the server is small, custom, and tightly coupled to the extension version without external install steps.
