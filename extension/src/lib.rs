use zed_extension_api::{self as zed, Result};
use std::io::Write;

// Embed the native LSP server binary as bytes (Linux/macOS). This allows dev usage
// even when Zed does not copy the binary into its work/installed directories.
// The file must exist in the extension directory before running 'Install Dev Extension'.
// For Windows, a separate binary would be needed.
#[cfg(not(target_os = "windows"))]
static EMBEDDED_SERVER: &[u8] = include_bytes!("../auto-header-server");
#[cfg(target_os = "windows")]
static EMBEDDED_SERVER: &[u8] = include_bytes!("../auto-header-server.exe");

struct AutoHeaderExtension {
    cached_binary_path: Option<String>,
}

impl zed::Extension for AutoHeaderExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Helper to check if a path looks executable (best-effort; sandbox may limit metadata)
        fn is_executable(path: &std::path::Path) -> bool {
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                if let Ok(meta) = std::fs::metadata(path) {
                    let mode = meta.mode();
                    // Any execute bit
                    return mode & 0o111 != 0;
                }
                false
            }
            #[cfg(not(unix))]
            {
                // On non-unix we assume created native binaries are executable if present
                path.exists()
            }
        }
        // If we've already found the binary, use the cached path
        if let Some(ref path) = self.cached_binary_path {
            return Ok(zed::Command {
                command: path.clone(),
                args: vec![],
                env: Default::default(),
            });
        }

        // Try to find the server binary in multiple locations:
        // 1. Extension directory (bundled with extension)
        // 2. System PATH (user installed)
        // 3. Common installation locations
        
        let binary_name = if cfg!(target_os = "windows") {
            "auto-header-server.exe"
        } else {
            "auto-header-server"
        };

        let work_dir = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;
        
        // Get home directory (cross-platform)
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))  // Windows fallback
            .unwrap_or_default();

        // Candidate locations for the server binary
        let mut candidates: Vec<std::path::PathBuf> = Vec::new();
        let work_dir_binary = work_dir.join(binary_name);
        candidates.push(work_dir_binary.clone());
        
        // Installed extension directory (platform-aware path transformation)
        // Try to transform /work/ to /installed/ in the current path
        let work_str = work_dir.to_string_lossy();
        let installed_dir_transformed = if work_str.contains("extensions") {
            // Universal approach: replace work segment with installed
            let replaced = work_str
                .replace(&format!("{}extensions{}work{}", std::path::MAIN_SEPARATOR, std::path::MAIN_SEPARATOR, std::path::MAIN_SEPARATOR),
                         &format!("{}extensions{}installed{}", std::path::MAIN_SEPARATOR, std::path::MAIN_SEPARATOR, std::path::MAIN_SEPARATOR));
            std::path::PathBuf::from(replaced)
        } else {
            work_dir.clone()
        };
        candidates.push(installed_dir_transformed.join(binary_name));
        
        // Direct constructed installed path using HOME (platform-specific)
        if !home.is_empty() {
            let zed_extensions_path = if cfg!(target_os = "windows") {
                // Windows: %APPDATA%\Zed\extensions\installed
                std::path::PathBuf::from(&home)
                    .join("AppData")
                    .join("Roaming")
                    .join("Zed")
                    .join("extensions")
                    .join("installed")
                    .join("auto-header")
                    .join(binary_name)
            } else if cfg!(target_os = "macos") {
                // macOS: ~/Library/Application Support/Zed/extensions/installed
                std::path::PathBuf::from(&home)
                    .join("Library")
                    .join("Application Support")
                    .join("Zed")
                    .join("extensions")
                    .join("installed")
                    .join("auto-header")
                    .join(binary_name)
            } else {
                // Linux: ~/.local/share/zed/extensions/installed
                std::path::PathBuf::from(&home)
                    .join(".local")
                    .join("share")
                    .join("zed")
                    .join("extensions")
                    .join("installed")
                    .join("auto-header")
                    .join(binary_name)
            };
            candidates.push(zed_extensions_path);
        }

        // Try system PATH last (handled separately)

        // Iterate candidates
        for candidate in &candidates {
            if candidate.exists() {
                // If not in work dir, replicate there for consistency
                if candidate != &work_dir_binary && !work_dir_binary.exists() {
                    let _ = std::fs::copy(candidate, &work_dir_binary);
                }
                // Ensure executable bit using Zed API (portable)
                let _ = zed::make_file_executable(
                    candidate.to_string_lossy().as_ref()
                );
                if !is_executable(candidate) {
                    // Try work dir copy exec bit
                    let _ = zed::make_file_executable(work_dir_binary.to_string_lossy().as_ref());
                }
                let chosen = if is_executable(candidate) { candidate } else { &work_dir_binary };
                let path_str = chosen.to_string_lossy().to_string();
                self.cached_binary_path = Some(path_str.clone());
                println!("auto-header: using binary {:?}", chosen);
                return Ok(zed::Command { command: path_str, args: vec![], env: Default::default() });
            }
        }

        // 4. Fallback: materialize embedded binary into work directory if present
        if !work_dir_binary.exists() {
            // Try writing embedded bytes
            if !EMBEDDED_SERVER.is_empty() {
                if let Ok(mut f) = std::fs::File::create(&work_dir_binary) {
                    let _ = f.write_all(EMBEDDED_SERVER);
                    let _ = zed::make_file_executable(work_dir_binary.to_string_lossy().as_ref());
                    if work_dir_binary.exists() {
                        let path_str = work_dir_binary.to_string_lossy().to_string();
                        self.cached_binary_path = Some(path_str.clone());
                        println!("auto-header: using embedded binary at {:?}", work_dir_binary);
                        return Ok(zed::Command { command: path_str, args: vec![], env: Default::default() });
                    }
                }
            }
        }

        if let Some(path) = worktree.which(binary_name) {
            self.cached_binary_path = Some(path.clone());
            println!("auto-header: using PATH binary {}", path);
            return Ok(zed::Command { command: path, args: vec![], env: Default::default() });
        }

        // Build diagnostic string of checked candidates
        let checked: String = candidates
            .into_iter()
            .map(|p| format!("{} -> exists={}", p.display(), p.exists()))
            .collect::<Vec<_>>()
            .join("\n");

        Err(format!(
            "Auto Header LSP server not found.\nChecked candidates:\n{checked}\n\nWork dir: {work}\nBinary name: {bin}\nHOME: {home}\n\nPlace binary in any candidate path or add to PATH.",
            checked = checked,
            work = work_dir.display(),
            bin = binary_name,
            home = home
        ))
    }
}

zed::register_extension!(AutoHeaderExtension);
