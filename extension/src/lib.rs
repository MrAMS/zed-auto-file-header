use std::fs;
use zed_extension_api::{
    self as zed, current_platform, download_file, latest_github_release, make_file_executable,
    Architecture, DownloadedFileType, GithubReleaseOptions, Os, Result,
};

struct AutoHeaderExtension {
    cached_binary_path: Option<String>,
}

impl AutoHeaderExtension {
    fn language_server_binary_path(&mut self, worktree: &zed::Worktree) -> Result<String> {
        // Return cached path if available
        if let Some(ref path) = self.cached_binary_path {
            if std::path::Path::new(path).exists() {
                return Ok(path.clone());
            }
        }

        let binary_name = if cfg!(target_os = "windows") {
            "auto-header-server.exe"
        } else {
            "auto-header-server"
        };

        // Check if binary exists in extension's work directory
        let work_dir = worktree.root_path();
        let work_dir_binary = std::path::Path::new(&work_dir).join(binary_name);
        if work_dir_binary.exists() {
            let path_str = work_dir_binary.to_string_lossy().to_string();
            self.cached_binary_path = Some(path_str.clone());
            return Ok(path_str);
        }

        // Check system PATH
        if let Some(path) = worktree.which(binary_name) {
            self.cached_binary_path = Some(path.clone());
            return Ok(path);
        }

        // Download from GitHub Releases
        let release = latest_github_release(
            "MrAMS/zed-auto-file-header",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: true,  // Allow prereleases for now
            },
        )?;

        let (platform, arch) = current_platform();
        let asset_name = match (platform, arch) {
            (Os::Linux, Architecture::X8664) => "x86_64-unknown-linux-gnu",
            (Os::Linux, Architecture::Aarch64) => "aarch64-unknown-linux-gnu",
            (Os::Mac, Architecture::X8664) => "x86_64-apple-darwin",
            (Os::Mac, Architecture::Aarch64) => "aarch64-apple-darwin",
            (Os::Windows, Architecture::X8664) => "x86_64-pc-windows-msvc",
            _ => {
                return Err(format!(
                    "Unsupported platform: {:?} {:?}",
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
                    "No asset found for platform {}-{} in release {}",
                    asset_name, asset_extension, release.version
                )
            })?;

        let version_dir = format!("auto-header-server-{}", release.version);
        let binary_path = format!("{version_dir}/{binary_name}");

        // Check if this version is already downloaded
        if fs::metadata(&binary_path).is_ok() {
            self.cached_binary_path = Some(binary_path.clone());
            return Ok(binary_path);
        }

        // Download and extract
        let file_type = if platform == Os::Windows {
            DownloadedFileType::Zip
        } else {
            DownloadedFileType::GzipTar
        };

        download_file(&asset.download_url, &version_dir, file_type)
            .map_err(|e| format!("Failed to download server binary: {}", e))?;

        // Set executable permissions on Unix-like systems
        if platform != Os::Windows {
            make_file_executable(&binary_path)?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
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
        let binary_path = self.language_server_binary_path(worktree)?;
        
        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(AutoHeaderExtension);
