use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

/// The resolved config directory, combining custom override or platform defaults.
/// This is set once and cached for subsequent calls.
/// On macOS, this is `~/.config/zed`.
/// On Linux/FreeBSD, this is `$XDG_CONFIG_HOME/zed`.
/// On Windows, this is `%APPDATA%\Zed`.
static CONFIG_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Returns the path to the configuration directory used by Zed.
fn config_dir() -> &'static PathBuf {
    CONFIG_DIR.get_or_init(|| {
        if cfg!(target_os = "windows") {
            dirs::config_dir()
                .expect("failed to determine RoamingAppData directory")
                .join("Zed")
        } else if cfg!(any(target_os = "linux", target_os = "freebsd")) {
            if let Ok(flatpak_xdg_config) = std::env::var("FLATPAK_XDG_CONFIG_HOME") {
                flatpak_xdg_config.into()
            } else {
                dirs::config_dir().expect("failed to determine XDG_CONFIG_HOME directory")
            }
            .join("zed")
        } else {
            dirs::home_dir()
                .expect("failed to determine home directory")
                .join(".config")
                .join("zed")
        }
    })
}

/// Per-language header profile: optional file prologue plus a comment wrapper.
#[derive(Debug, Clone)]
struct HeaderProfile {
    prologue: &'static str,
    comment_style: CommentStyle,
    epilogue: &'static str,
}

impl HeaderProfile {
    fn block(
        prologue: &'static str,
        start: &'static str,
        end: &'static str,
        line_prefix: &'static str,
    ) -> Self {
        Self {
            prologue,
            comment_style: CommentStyle::Block {
                start,
                end,
                line_prefix,
            },
            epilogue: "\n",
        }
    }

    fn line(prologue: &'static str, prefix: &'static str, epilogue: &'static str) -> Self {
        Self {
            prologue,
            comment_style: CommentStyle::Line { prefix },
            epilogue,
        }
    }

    fn html() -> Self {
        Self {
            prologue: "",
            comment_style: CommentStyle::HtmlComment,
            epilogue: "\n",
        }
    }

    /// Get the header profile for a file extension.
    fn for_extension(ext: &str) -> Self {
        match ext {
            // C-style languages
            "c" | "h" | "cpp" | "hpp" | "cc" | "hh" | "cxx" | "hxx" => {
                Self::block("", "/*", "*/", " *")
            }
            "cs" | "java" | "js" | "jsx" | "ts" | "tsx" | "rs" | "scala" | "kt" | "kts"
            | "swift" | "go" | "m" | "mm" | "d" | "zig" | "dart" => {
                Self::block("", "/*", "*/", " *")
            }
            "css" | "scss" | "sass" | "less" => Self::block("", "/*", "*/", " *"),

            // PHP: requires an opening tag before the comment block
            "php" | "phtml" | "php3" | "php4" | "php5" | "phps" | "phpt" => {
                Self::block("<?php\n\n", "/*", "*/", " *")
            }

            // Python
            "py" | "pyw" | "pyx" => {
                Self::block("# -*- coding: utf-8 -*-\n", "\"\"\"", "\"\"\"", "")
            }

            // Shell scripts
            "sh" | "bash" | "zsh" => Self::line("#!/usr/bin/env bash\n#\n", "#", "#\n\n"),
            "fish" => Self::line("#!/usr/bin/env fish\n#\n", "#", "#\n\n"),

            // Other scripting languages with shebang
            "rb" => Self::line("#!/usr/bin/env ruby\n#\n", "#", "#\n\n"),
            "pl" | "pm" => Self::line("#!/usr/bin/env perl\n#\n", "#", "#\n\n"),
            "r" | "R" => Self::line("#!/usr/bin/env Rscript\n#\n", "#", "#\n\n"),
            "jl" => Self::line("#!/usr/bin/env julia\n#\n", "#", "#\n\n"),

            // Simple line comments with #
            "yaml" | "yml" | "toml" | "ini" | "conf" | "cfg" | "tcl" | "nim" | "crystal" => {
                Self::line("", "#", "\n")
            }

            // HTML/XML
            "html" | "htm" | "xml" | "svg" | "xhtml" => Self::html(),

            // SQL
            "sql" => Self::line("", "--", "\n"),

            // Lua/Haskell/Ada
            "lua" | "hs" | "lhs" | "ads" | "adb" => Self::block("", "--[[", "--]]", ""),

            // Lisp family
            "lisp" | "cl" | "scm" | "clj" | "cljs" | "rkt" | "gleam" => {
                Self::line("", ";;;;", "\n")
            }

            // Erlang/Elixir
            "erl" | "hrl" | "ex" | "exs" => Self::line("", "%%", "\n"),

            // Vim
            "vim" => Self::line("", "\"", "\n"),

            // Verilog and SystemVerilog (line comments)
            "v" | "vh" | "sv" | "svh" => Self::line("", "//", "\n"),

            // Odin (C-style but single-line //)
            "odin" => Self::line("", "//", "\n"),

            // OCaml/F#
            "ml" | "mli" | "fs" | "fsi" | "fsx" => Self::block("", "(*", "*)", " *"),

            // LaTeX/TeX/Typst
            "tex" | "latex" | "sty" | "cls" | "bib" | "typ" => Self::line("", "%", "\n"),

            // Default: line comment with #
            _ => Self::line("", "#", "\n"),
        }
    }

    fn wrap(&self, content: &str) -> String {
        let mut result = String::from(self.prologue);
        result.push_str(&self.comment_style.wrap(content));
        result.push_str(self.epilogue);
        result
    }
}

/// Comment body style used inside a header profile.
#[derive(Debug, Clone)]
enum CommentStyle {
    /// Block comment style with start and end markers (e.g., /* ... */)
    Block {
        start: &'static str,
        end: &'static str,
        line_prefix: &'static str,
    },
    /// Line comment style with a prefix (e.g., # or //)
    Line { prefix: &'static str },
    /// HTML/XML style
    HtmlComment,
}

impl CommentStyle {
    /// Wrap content with this comment style.
    fn wrap(&self, content: &str) -> String {
        match self {
            Self::Block {
                start,
                end,
                line_prefix,
            } => {
                let mut result = format!("{}\n", start);
                for line in content.lines() {
                    if line.trim().is_empty() {
                        if !line_prefix.is_empty() {
                            result.push_str(line_prefix);
                        }
                        result.push('\n');
                    } else {
                        if !line_prefix.is_empty() {
                            result.push_str(line_prefix);
                            result.push(' ');
                        }
                        result.push_str(line);
                        result.push('\n');
                    }
                }
                if line_prefix.is_empty() {
                    result.push_str(end);
                } else {
                    result.push(' ');
                    result.push_str(end);
                }
                result.push('\n');
                result
            }
            Self::Line { prefix } => {
                let mut result = String::new();
                for line in content.lines() {
                    result.push_str(prefix);
                    if !line.trim().is_empty() {
                        result.push(' ');
                        result.push_str(line);
                    }
                    result.push('\n');
                }
                result
            }
            Self::HtmlComment => {
                let mut result = String::from("<!--\n");
                for line in content.lines() {
                    if !line.trim().is_empty() {
                        result.push_str("  ");
                        result.push_str(line);
                    }
                    result.push('\n');
                }
                result.push_str("-->\n");
                result
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AuthorConfig {
    name: String,
    email: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ProjectConfig {
    name: String,
    #[serde(default)]
    copyright_holder: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct HeaderConfig {
    template: String,
    #[serde(default)]
    by_extension: HashMap<String, ExtensionHeaderConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ExtensionHeaderConfig {
    template: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Config {
    author: AuthorConfig,
    project: ProjectConfig,
    header: HeaderConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            author: AuthorConfig {
                name: "Auto Header".to_string(),
                email: "auto@header.dev".to_string(),
            },
            project: ProjectConfig {
                name: "My Project".to_string(),
                copyright_holder: String::new(),
            },
            header: HeaderConfig {
                template: r#"File: {filename}
Author: {author}
Date: {date}
Copyright (c) {year} {copyright_holder}"#
                    .to_string(),
                by_extension: HashMap::new(),
            },
        }
    }
}

/// Partial versions of config structs for merging from multiple sources.
/// All fields are Option so each config file only needs to specify what it overrides.
#[derive(Debug, Clone, Deserialize, Default)]
struct PartialAuthorConfig {
    name: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct PartialProjectConfig {
    name: Option<String>,
    copyright_holder: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct PartialHeaderConfig {
    template: Option<String>,
    by_extension: Option<HashMap<String, ExtensionHeaderConfig>>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct PartialConfig {
    author: Option<PartialAuthorConfig>,
    project: Option<PartialProjectConfig>,
    header: Option<PartialHeaderConfig>,
}

impl PartialConfig {
    /// Merge two partial configs. `self` has higher priority than `lower`.
    /// Fields present in `self` win; missing fields fall back to `lower`.
    fn merge(self, lower: PartialConfig) -> PartialConfig {
        PartialConfig {
            author: match (self.author, lower.author) {
                (Some(high), Some(low)) => Some(PartialAuthorConfig {
                    name: high.name.or(low.name),
                    email: high.email.or(low.email),
                }),
                (a, b) => a.or(b),
            },
            project: match (self.project, lower.project) {
                (Some(high), Some(low)) => Some(PartialProjectConfig {
                    name: high.name.or(low.name),
                    copyright_holder: high.copyright_holder.or(low.copyright_holder),
                }),
                (a, b) => a.or(b),
            },
            header: match (self.header, lower.header) {
                (Some(high), Some(low)) => Some(PartialHeaderConfig {
                    template: high.template.or(low.template),
                    by_extension: Some({
                        // Start from lower priority map, then override with higher priority keys
                        let mut merged = low.by_extension.unwrap_or_default();
                        if let Some(ext) = high.by_extension {
                            merged.extend(ext);
                        }
                        merged
                    }),
                }),
                (a, b) => a.or(b),
            },
        }
    }

    /// Convert to a full `Config`, filling any missing fields with defaults.
    fn into_config(self) -> Config {
        let default = Config::default();
        let author = self.author.unwrap_or_default();
        let project = self.project.unwrap_or_default();
        let header = self.header.unwrap_or_default();
        Config {
            author: AuthorConfig {
                name: author.name.unwrap_or(default.author.name),
                email: author.email.unwrap_or(default.author.email),
            },
            project: ProjectConfig {
                name: project.name.unwrap_or(default.project.name),
                copyright_holder: project
                    .copyright_holder
                    .unwrap_or(default.project.copyright_holder),
            },
            header: HeaderConfig {
                template: header.template.unwrap_or(default.header.template),
                by_extension: {
                    // Default extension map first, then overlay whatever was configured
                    let mut merged = default.header.by_extension;
                    if let Some(ext) = header.by_extension {
                        merged.extend(ext);
                    }
                    merged
                },
            },
        }
    }
}

impl Config {
    /// Check if any config file exists in the search paths
    /// Takes an optional workspace root directory to check for project-local config
    fn config_exists(workspace_root: Option<&Path>) -> bool {
        let mut config_paths = Vec::new();

        // 1. Project root (if provided)
        if let Some(root) = workspace_root {
            config_paths.push(root.join(".auto-header.toml"));
        }

        // 2. Home directory: ~/.auto-header.toml
        if let Some(home_dir) = dirs::home_dir() {
            config_paths.push(home_dir.join(".auto-header.toml"));
        }

        // 3. Platform-specific config directory fallback
        config_paths.push(config_dir().join("auto-header.toml"));

        config_paths
            .iter()
            .any(|path| !path.as_os_str().is_empty() && path.exists())
    }

    /// Load and merge configs from all locations with explicit priority:
    /// Project root > Home directory > Platform config > built-in default
    ///
    /// Each file is optional and only needs to specify the keys it wants to override.
    fn load_from_workspace(workspace_root: Option<&Path>) -> Self {
        let try_load = |path: std::path::PathBuf| -> Option<PartialConfig> {
            if path.as_os_str().is_empty() || !path.exists() {
                return None;
            }
            let content = std::fs::read_to_string(&path).ok()?;
            toml::from_str::<PartialConfig>(&content).ok()
        };

        // Platform config directory (lowest priority among file sources)
        let platform_config =
            try_load(config_dir().join("zed").join("auto-header.toml")).unwrap_or_default();

        // Home directory (medium priority)
        let home_config = dirs::home_dir()
            .and_then(|d| try_load(d.join(".auto-header.toml")))
            .unwrap_or_default();

        // Project root (highest priority)
        let project_config = workspace_root
            .and_then(|r| try_load(r.join(".auto-header.toml")))
            .unwrap_or_default();

        // Merge: project > home > platform, then fill gaps with built-in default
        project_config
            .merge(home_config)
            .merge(platform_config)
            .into_config()
    }

    fn load() -> Self {
        Self::load_from_workspace(None)
    }

    fn get_template_for_file(&self, file_path: &str) -> String {
        // Extract file extension
        let ext = Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        // Get the header profile for this file type
        let header_profile = HeaderProfile::for_extension(ext);

        // Priority 1: Check if user has custom template for this specific extension
        if let Some(ext_config) = self.header.by_extension.get(ext) {
            // Wrap user's extension-specific template with appropriate comment syntax
            return header_profile.wrap(&ext_config.template);
        }

        // Priority 2: Use user's default template from [header] section
        // Always wrap with appropriate comment syntax
        header_profile.wrap(&self.header.template)
    }
}

#[derive(Debug)]
struct AutoHeaderServer {
    client: Client,
    // retained for future feature (e.g., config watching); currently dynamic reload used
    _initial_config: Config,
    // Store workspace root(s) from initialization
    workspace_folders: std::sync::Arc<tokio::sync::RwLock<Vec<std::path::PathBuf>>>,
}

impl AutoHeaderServer {
    fn new(client: Client) -> Self {
        let initial_config = Config::load();
        Self {
            client,
            _initial_config: initial_config,
            workspace_folders: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }

    fn generate_header(&self, file_path: &str, workspace_root: Option<&Path>) -> String {
        // Reload config each time to pick up changes to .auto-header.toml without restarting Zed
        let config = Config::load_from_workspace(workspace_root);
        let now = Local::now();
        let date = now.format("%Y-%m-%d").to_string();
        let time = now.format("%H:%M:%S").to_string();
        let year = now.format("%Y").to_string();

        // Extract filename and extension from path
        let path = Path::new(file_path);
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        // Get the appropriate template for this file type
        let mut template = config.get_template_for_file(file_path);

        // Determine copyright holder
        let copyright_holder = if config.project.copyright_holder.is_empty() {
            &config.author.name
        } else {
            &config.project.copyright_holder
        };

        // Determine interpreter for shebang
        let interpreter = match ext {
            "py" | "pyw" | "pyx" => "python3",
            "rb" => "ruby",
            "pl" | "pm" => "perl",
            "sh" => "sh",
            "bash" => "bash",
            "zsh" => "zsh",
            "fish" => "fish",
            "r" | "R" => "Rscript",
            "jl" => "julia",
            _ => "",
        };

        // Replace all variables in the template
        template = template
            .replace("{filename}", filename)
            .replace("{filepath}", file_path)
            .replace("{date}", &date)
            .replace("{time}", &time)
            .replace("{year}", &year)
            .replace("{author}", &config.author.name)
            .replace("{email}", &config.author.email)
            .replace("{project}", &config.project.name)
            .replace("{copyright_holder}", copyright_holder)
            .replace("{interpreter}", interpreter);

        template
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for AutoHeaderServer {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        // Store workspace folders from initialization
        let mut folders = self.workspace_folders.write().await;

        if let Some(workspace_folders) = params.workspace_folders {
            for folder in workspace_folders {
                if let Ok(path) = folder.uri.to_file_path() {
                    folders.push(path);
                }
            }
        } else if let Some(root_uri) = params.root_uri {
            if let Ok(path) = root_uri.to_file_path() {
                folders.push(path);
            }
        }

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(
                MessageType::INFO,
                "[Auto Header] Server initialized successfully",
            )
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.text_document.text;

        // Log file opening
        self.client
            .log_message(
                MessageType::INFO,
                format!(
                    "[Auto Header] File opened: {}, length: {}",
                    uri.path(),
                    content.len()
                ),
            )
            .await;

        // Convert URI to file path (this handles URL decoding automatically)
        let file_path = match uri.to_file_path() {
            Ok(path) => path,
            Err(_) => {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!(
                            "[Auto Header] Failed to convert URI to file path: {}",
                            uri.path()
                        ),
                    )
                    .await;
                return;
            }
        };

        let file_path_str = file_path.to_str().unwrap_or("");

        // Find the workspace root by checking which workspace folder contains this file
        let folders = self.workspace_folders.read().await;
        let workspace_root = folders
            .iter()
            .find(|folder| file_path.starts_with(folder))
            .map(|p| p.as_path());

        // Only insert header if:
        // 1. File is completely empty
        // 2. A .auto-header.toml config file exists in search paths
        let config_exists = Config::config_exists(workspace_root);

        if !content.trim().is_empty() {
            self.client
                .log_message(
                    MessageType::INFO,
                    format!("[Auto Header] Skipping non-empty file: {}", uri.path()),
                )
                .await;
            return;
        }

        if !config_exists {
            self.client
                .log_message(
                    MessageType::INFO,
                    format!(
                        "[Auto Header] No .auto-header.toml found, skipping: {}",
                        uri.path()
                    ),
                )
                .await;
            return;
        }

        // Both conditions met, insert header
        self.client
            .log_message(
                MessageType::INFO,
                format!("[Auto Header] Inserting header for: {}", uri.path()),
            )
            .await;

        let header = self.generate_header(file_path_str, workspace_root);

        // Create a text edit to insert the header at the beginning
        let edit = TextEdit {
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 0,
                },
            },
            new_text: header,
        };

        // Apply the workspace edit
        let mut changes = std::collections::HashMap::new();
        changes.insert(uri.clone(), vec![edit]);

        let workspace_edit = WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        };

        // Send the edit to the client
        if let Err(e) = self.client.apply_edit(workspace_edit).await {
            self.client
                .log_message(
                    MessageType::ERROR,
                    format!("Failed to apply header edit: {:?}", e),
                )
                .await;
        } else {
            self.client
                .log_message(
                    MessageType::INFO,
                    format!("Header inserted for {}", uri.path()),
                )
                .await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Helpers ───────────────────────────────────────────────────────────────

    /// Build an `ExtensionHeaderConfig` map from a slice of (ext, template) pairs.
    fn ext_map(entries: &[(&str, &str)]) -> HashMap<String, ExtensionHeaderConfig> {
        entries
            .iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    ExtensionHeaderConfig {
                        template: v.to_string(),
                    },
                )
            })
            .collect()
    }

    /// Convenience builder: only populate the fields that are `Some`.
    fn make_partial(
        author_name: Option<&str>,
        author_email: Option<&str>,
        project_name: Option<&str>,
        copyright_holder: Option<&str>,
        header_template: Option<&str>,
        by_extension: Option<HashMap<String, ExtensionHeaderConfig>>,
    ) -> PartialConfig {
        let has_author = author_name.is_some() || author_email.is_some();
        let has_project = project_name.is_some() || copyright_holder.is_some();
        let has_header = header_template.is_some() || by_extension.is_some();
        PartialConfig {
            author: has_author.then(|| PartialAuthorConfig {
                name: author_name.map(str::to_string),
                email: author_email.map(str::to_string),
            }),
            project: has_project.then(|| PartialProjectConfig {
                name: project_name.map(str::to_string),
                copyright_holder: copyright_holder.map(str::to_string),
            }),
            header: has_header.then(|| PartialHeaderConfig {
                template: header_template.map(str::to_string),
                by_extension,
            }),
        }
    }

    // ── header profiles ───────────────────────────────────────────────────────

    #[test]
    fn php_header_profile_adds_opening_tag_before_comment_block() {
        let header = HeaderProfile::for_extension("php").wrap("File: example.php\nAuthor: Alice");

        assert!(header.starts_with("<?php\n\n/*\n"));
        assert!(header.contains(" * File: example.php\n"));
        assert!(header.contains(" * Author: Alice\n"));
        assert!(header.ends_with(" */\n\n"));
        assert!(!header.contains("?>"));
    }

    #[test]
    fn php_variants_use_the_php_profile() {
        for ext in ["php", "phtml", "php3", "php4", "php5", "phps", "phpt"] {
            let header = HeaderProfile::for_extension(ext).wrap("File: index.php");
            assert!(
                header.starts_with("<?php\n\n/*\n"),
                "expected PHP prologue for .{ext}, got {header:?}"
            );
        }
    }

    #[test]
    fn shell_profile_still_emits_shebang_and_comment_guards() {
        let header = HeaderProfile::for_extension("sh").wrap("File: deploy.sh\nDate: 2026-04-10");

        assert_eq!(
            header,
            "#!/usr/bin/env bash\n#\n# File: deploy.sh\n# Date: 2026-04-10\n#\n\n"
        );
    }

    #[test]
    fn python_profile_uses_encoding_line_and_docstring_body() {
        let header = HeaderProfile::for_extension("py").wrap("File: script.py");

        assert_eq!(
            header,
            "# -*- coding: utf-8 -*-\n\"\"\"\nFile: script.py\n\"\"\"\n\n"
        );
    }

    // ── into_config ───────────────────────────────────────────────────────────

    /// An all-empty PartialConfig must produce the exact built-in defaults.
    #[test]
    fn empty_partial_yields_defaults() {
        let config = PartialConfig::default().into_config();
        let default = Config::default();
        assert_eq!(config.author.name, default.author.name);
        assert_eq!(config.author.email, default.author.email);
        assert_eq!(config.project.name, default.project.name);
        assert_eq!(
            config.project.copyright_holder,
            default.project.copyright_holder
        );
        assert_eq!(config.header.template, default.header.template);
        assert_eq!(
            config.header.by_extension.len(),
            default.header.by_extension.len()
        );
    }

    /// A single overridden field should use the given value; all other fields default.
    #[test]
    fn single_field_override_rest_are_defaults() {
        let partial = PartialConfig {
            author: Some(PartialAuthorConfig {
                name: Some("Alice".to_string()),
                email: None,
            }),
            ..Default::default()
        };
        let config = partial.into_config();
        let default = Config::default();
        assert_eq!(config.author.name, "Alice");
        assert_eq!(config.author.email, default.author.email);
        assert_eq!(config.project.name, default.project.name);
        assert_eq!(config.header.template, default.header.template);
    }

    // ── merge: two-source priority ────────────────────────────────────────────

    /// When both configs set the same key, `self` (higher priority) wins.
    #[test]
    fn project_wins_over_home_same_key() {
        let home = make_partial(
            Some("HomeUser"),
            Some("home@example.com"),
            None,
            None,
            None,
            None,
        );
        let project = make_partial(Some("ProjectUser"), None, None, None, None, None);
        let config = project.merge(home).into_config();
        assert_eq!(config.author.name, "ProjectUser"); // project wins
        assert_eq!(config.author.email, "home@example.com"); // home fills the gap
    }

    /// When only the lower-priority config sets a key, that value is used.
    #[test]
    fn missing_in_high_falls_through_to_low() {
        let platform = make_partial(
            Some("PlatformUser"),
            Some("platform@example.com"),
            None,
            None,
            None,
            None,
        );
        let home = make_partial(Some("HomeUser"), None, None, None, None, None);
        let config = home.merge(platform).into_config();
        assert_eq!(config.author.name, "HomeUser"); // home wins
        assert_eq!(config.author.email, "platform@example.com"); // falls through to platform
    }

    /// When neither config sets a field, the built-in default is used.
    #[test]
    fn missing_everywhere_uses_default() {
        // Only author.name is set across all sources; everything else should be default.
        let project = make_partial(Some("OnlyName"), None, None, None, None, None);
        let config = project
            .merge(PartialConfig::default())
            .merge(PartialConfig::default())
            .into_config();
        let default = Config::default();
        assert_eq!(config.author.name, "OnlyName");
        assert_eq!(config.author.email, default.author.email);
        assert_eq!(config.project.name, default.project.name);
        assert_eq!(config.header.template, default.header.template);
    }

    // ── merge: three-source priority chain ────────────────────────────────────

    /// Full three-way merge: each field comes from a different source to verify
    /// the complete project > home > platform priority chain.
    #[test]
    fn three_way_merge_each_field_from_correct_source() {
        let platform = make_partial(
            Some("PlatformUser"),
            Some("platform@corp.com"),
            Some("PlatformApp"),
            Some("Platform Corp"),
            Some("platform template"),
            None,
        );
        let home = make_partial(
            Some("HomeUser"), // overrides platform author.name
            None,
            Some("HomeApp"), // overrides platform project.name
            None,
            None, // header.template: not set → falls to platform
            None,
        );
        let project = make_partial(
            None, // author.name: not set → falls to home
            None,
            None,                     // project.name: not set → falls to home
            Some("My Startup"),       // overrides both home and platform
            Some("project template"), // overrides both home and platform
            None,
        );

        let config = project.merge(home).merge(platform).into_config();

        assert_eq!(config.author.name, "HomeUser"); // home > platform
        assert_eq!(config.author.email, "platform@corp.com"); // only in platform
        assert_eq!(config.project.name, "HomeApp"); // home > platform
        assert_eq!(config.project.copyright_holder, "My Startup"); // project wins
        assert_eq!(config.header.template, "project template"); // project wins
    }

    // ── merge: by_extension HashMap ───────────────────────────────────────────

    /// Higher-priority keys override lower-priority ones; unique keys from every
    /// source are all preserved in the final map.
    #[test]
    fn by_extension_merge_preserves_unique_keys_from_all_sources() {
        let platform = make_partial(
            None,
            None,
            None,
            None,
            None,
            Some(ext_map(&[("rs", "platform rs"), ("py", "platform py")])),
        );
        let home = make_partial(
            None,
            None,
            None,
            None,
            None,
            Some(ext_map(&[("rs", "home rs"), ("go", "home go")])),
        );
        let project = make_partial(
            None,
            None,
            None,
            None,
            None,
            Some(ext_map(&[("rs", "project rs")])),
        );

        let config = project.merge(home).merge(platform).into_config();
        let by_ext = &config.header.by_extension;

        assert_eq!(by_ext["rs"].template, "project rs"); // project wins
        assert_eq!(by_ext["go"].template, "home go"); // home unique key preserved
        assert_eq!(by_ext["py"].template, "platform py"); // platform unique key preserved
    }

    /// When only home and platform set the same extension key, home wins.
    #[test]
    fn by_extension_home_wins_over_platform() {
        let platform = make_partial(
            None,
            None,
            None,
            None,
            None,
            Some(ext_map(&[("ts", "platform ts")])),
        );
        let home = make_partial(
            None,
            None,
            None,
            None,
            None,
            Some(ext_map(&[("ts", "home ts")])),
        );

        let config = PartialConfig::default()
            .merge(home)
            .merge(platform)
            .into_config();

        assert_eq!(config.header.by_extension["ts"].template, "home ts");
    }

    // ── TOML deserialization ──────────────────────────────────────────────────

    /// A TOML file with only a subset of fields must parse into a PartialConfig
    /// where only those fields are Some and the rest are None.
    #[test]
    fn toml_partial_parse_only_author_name() {
        let toml_str = r#"
[author]
name = "TOML User"
"#;
        let partial: PartialConfig = toml::from_str(toml_str).unwrap();
        let author = partial.author.as_ref().unwrap();
        assert_eq!(author.name, Some("TOML User".to_string()));
        assert!(author.email.is_none());
        assert!(partial.project.is_none());
        assert!(partial.header.is_none());
    }

    /// A TOML file with no content at all must deserialize as an all-None PartialConfig.
    #[test]
    fn toml_empty_string_yields_default_partial() {
        let partial: PartialConfig = toml::from_str("").unwrap();
        assert!(partial.author.is_none());
        assert!(partial.project.is_none());
        assert!(partial.header.is_none());
    }

    // ── End-to-end TOML three-way merge ───────────────────────────────────────

    /// Simulate reading three real TOML config files and verify the final
    /// merged Config reflects the correct priority for every field.
    #[test]
    fn toml_three_way_merge_end_to_end() {
        let platform_toml = r#"
[author]
name = "Platform"
email = "platform@corp.com"

[project]
name = "PlatformApp"
copyright_holder = "Platform Corp"

[header]
template = "platform template"
"#;
        let home_toml = r#"
[author]
name = "HomeUser"

[header]
template = "home template"
"#;
        let project_toml = r#"
[project]
copyright_holder = "My Startup"
"#;

        let platform: PartialConfig = toml::from_str(platform_toml).unwrap();
        let home: PartialConfig = toml::from_str(home_toml).unwrap();
        let project: PartialConfig = toml::from_str(project_toml).unwrap();

        let config = project.merge(home).merge(platform).into_config();

        // author.name: project=None, home=Some → "HomeUser"
        assert_eq!(config.author.name, "HomeUser");
        // author.email: project=None, home=None, platform=Some → "platform@corp.com"
        assert_eq!(config.author.email, "platform@corp.com");
        // project.name: project=None, home=None, platform=Some → "PlatformApp"
        assert_eq!(config.project.name, "PlatformApp");
        // project.copyright_holder: project=Some → "My Startup"
        assert_eq!(config.project.copyright_holder, "My Startup");
        // header.template: project=None, home=Some → "home template"
        assert_eq!(config.header.template, "home template");
    }

    /// Simulate TOML configs that include by_extension, verifying key-level
    /// priority inside the merged map.
    #[test]
    fn toml_by_extension_three_way_merge() {
        let platform_toml = r#"
[header]
template = "default"

[header.by_extension]
rs = { template = "Rust (platform)" }
py = { template = "Python (platform)" }
"#;
        let home_toml = r#"
[header.by_extension]
rs = { template = "Rust (home)" }
go = { template = "Go (home)" }
"#;
        let project_toml = r#"
[header.by_extension]
rs = { template = "Rust (project)" }
"#;

        let platform: PartialConfig = toml::from_str(platform_toml).unwrap();
        let home: PartialConfig = toml::from_str(home_toml).unwrap();
        let project: PartialConfig = toml::from_str(project_toml).unwrap();

        let config = project.merge(home).merge(platform).into_config();
        let by_ext = &config.header.by_extension;

        assert_eq!(by_ext["rs"].template, "Rust (project)"); // project wins
        assert_eq!(by_ext["go"].template, "Go (home)"); // home unique key
        assert_eq!(by_ext["py"].template, "Python (platform)"); // platform unique key
                                                                // header.template only set in platform, should survive the merge
        assert_eq!(config.header.template, "default");
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(AutoHeaderServer::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
