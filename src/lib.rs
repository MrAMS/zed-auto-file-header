use std::fs;
use zed_extension_api::{
    self as zed, current_platform, download_file, latest_github_release, make_file_executable,
    set_language_server_installation_status, Architecture, DownloadedFileType,
    GithubReleaseOptions, LanguageServerInstallationStatus, Os, Result,
};

// Extension version - MUST match version in extension.toml
const EXTENSION_VERSION: &str = "0.2.6";

struct AutoHeaderExtension;

impl AutoHeaderExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<String> {
        let binary_name = if cfg!(target_os = "windows") {
            "auto-header-server.exe"
        } else {
            "auto-header-server"
        };

        // Use extension version as server version
        // This ensures we always download the matching server version
        let version_dir = format!("auto-header-server-v{}", EXTENSION_VERSION);
        let binary_path = format!("{version_dir}/{binary_name}");

        // Check if this version is already downloaded
        if fs::metadata(&binary_path).is_ok() {
            return Ok(binary_path);
        }

        // Need to download this version from GitHub
        let release = latest_github_release(
            "MrAMS/zed-auto-file-header",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )
        .map_err(|e| {
            format!(
                "Auto File Header: Failed to fetch release from GitHub. \n\
                Please check your internet connection and try again. \n\
                If the problem persists, visit: https://github.com/MrAMS/zed-auto-file-header/releases \n\
                Error: {}",
                e
            )
        })?;

        let (platform, arch) = current_platform();
        let asset_name = match (platform, arch) {
            (Os::Linux, Architecture::X8664) => "x86_64-unknown-linux-gnu",
            (Os::Linux, Architecture::Aarch64) => "aarch64-unknown-linux-gnu",
            (Os::Mac, Architecture::X8664) => "x86_64-apple-darwin",
            (Os::Mac, Architecture::Aarch64) => "aarch64-apple-darwin",
            (Os::Windows, Architecture::X8664) => "x86_64-pc-windows-msvc",
            _ => {
                return Err(format!(
                    "Auto File Header: Unsupported platform {:?}-{:?}. \
                    Supported platforms: Linux (x86_64/aarch64), macOS (x86_64/aarch64), Windows (x86_64). \
                    Please report this issue at https://github.com/MrAMS/zed-auto-file-header/issues",
                    platform, arch
                ))
            }
        };

        let asset_extension = if platform == Os::Windows {
            "zip"
        } else {
            "tar.gz"
        };
        let asset_full_name = format!("{}.{}", asset_name, asset_extension);

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_full_name)
            .ok_or_else(|| {
                format!(
                    "Auto File Header: Pre-built binary not found for your platform. \n\
                    Looking for: {} \n\
                    Release version: {} \n\
                    Available assets: {} \n\
                    Please report this at: https://github.com/MrAMS/zed-auto-file-header/issues",
                    asset_full_name,
                    release.version,
                    release.assets.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(", ")
                )
            })?;

        // Notify user: downloading binary
        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::Downloading,
        );

        // Download and extract
        let file_type = if platform == Os::Windows {
            DownloadedFileType::Zip
        } else {
            DownloadedFileType::GzipTar
        };

        download_file(&asset.download_url, &version_dir, file_type)
            .map_err(|e| {
                format!(
                    "Auto File Header: Failed to download and extract the language server binary. \n\
                    Download URL: {} \n\
                    Target directory: {} \n\
                    Please check your internet connection and disk space. \n\
                    Error: {}",
                    asset.download_url, version_dir, e
                )
            })?;

        // Set executable permissions on Unix-like systems
        if platform != Os::Windows {
            make_file_executable(&binary_path)
                .map_err(|e| {
                    format!(
                        "Auto File Header: Failed to set executable permissions on binary. \n\
                        Binary path: {} \n\
                        Error: {}",
                        binary_path, e
                    )
                })?;
        }

        // Download complete
        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::None,
        );

        Ok(binary_path)
    }
}

impl zed::Extension for AutoHeaderExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary_path = self.language_server_binary_path(language_server_id, worktree)?;
        
        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(AutoHeaderExtension);
