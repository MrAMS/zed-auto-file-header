use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Comment style for different programming languages
#[derive(Debug, Clone)]
enum CommentStyle {
    /// Block comment style with start and end markers (e.g., /* ... */)
    Block { start: &'static str, end: &'static str, line_prefix: &'static str },
    /// Line comment style with a prefix (e.g., # or //)
    Line { prefix: &'static str },
    /// Line comment with optional shebang
    LineWithShebang { prefix: &'static str, shebang: &'static str },
    /// Python style with encoding declaration and docstring
    PythonDoc,
    /// HTML/XML style
    HtmlComment,
}

impl CommentStyle {
    /// Get the comment style for a file extension
    fn for_extension(ext: &str) -> Self {
        match ext {
            // C-style languages
            "c" | "h" | "cpp" | "hpp" | "cc" | "hh" | "cxx" | "hxx" => 
                Self::Block { start: "/*", end: "*/", line_prefix: " *" },
            "cs" | "java" | "js" | "jsx" | "ts" | "tsx" | "rs" | "scala" | 
            "kt" | "kts" | "swift" | "go" | "m" | "mm" => 
                Self::Block { start: "/*", end: "*/", line_prefix: " *" },
            "css" | "scss" | "sass" | "less" => 
                Self::Block { start: "/*", end: "*/", line_prefix: " *" },
            
            // Python
            "py" | "pyw" | "pyx" => Self::PythonDoc,
            
            // Shell scripts
            "sh" | "bash" | "zsh" => Self::LineWithShebang { prefix: "#", shebang: "#!/usr/bin/env bash" },
            "fish" => Self::LineWithShebang { prefix: "#", shebang: "#!/usr/bin/env fish" },
            
            // Other scripting languages with shebang
            "rb" => Self::LineWithShebang { prefix: "#", shebang: "#!/usr/bin/env ruby" },
            "pl" | "pm" => Self::LineWithShebang { prefix: "#", shebang: "#!/usr/bin/env perl" },
            "r" | "R" => Self::LineWithShebang { prefix: "#", shebang: "#!/usr/bin/env Rscript" },
            "jl" => Self::LineWithShebang { prefix: "#", shebang: "#!/usr/bin/env julia" },
            
            // Simple line comments
            "yaml" | "yml" | "toml" | "ini" | "conf" | "cfg" => Self::Line { prefix: "#" },
            
            // HTML/XML
            "html" | "htm" | "xml" | "svg" | "xhtml" => Self::HtmlComment,
            
            // SQL
            "sql" => Self::Line { prefix: "--" },
            
            // Lua/Haskell
            "lua" | "hs" | "lhs" => Self::Block { start: "--[[", end: "--]]", line_prefix: "" },
            
            // Lisp family
            "lisp" | "cl" | "scm" | "clj" | "cljs" => Self::Line { prefix: ";;;;" },
            
            // Erlang/Elixir
            "erl" | "hrl" | "ex" | "exs" => Self::Line { prefix: "%%" },
            
            // Vim
            "vim" => Self::Line { prefix: "\"" },
            
            // Verilog and SystemVerilog
            "v" | "vh" | "sv" | "svh" => Self::Line { prefix: "//" },
            
            // Tcl
            "tcl" => Self::Line { prefix: "#" },
            
            // Default: line comment with #
            _ => Self::Line { prefix: "#" },
        }
    }
    
    /// Wrap content with this comment style
    fn wrap(&self, content: &str) -> String {
        match self {
            Self::Block { start, end, line_prefix } => {
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
                result.push_str(&format!(" {}\n\n", end));
                result
            },
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
                result.push('\n');
                result
            },
            Self::LineWithShebang { prefix, shebang } => {
                let mut result = format!("{}\n{}\n", shebang, prefix);
                for line in content.lines() {
                    result.push_str(prefix);
                    if !line.trim().is_empty() {
                        result.push(' ');
                        result.push_str(line);
                    }
                    result.push('\n');
                }
                result.push_str(&format!("{}\n\n", prefix));
                result
            },
            Self::PythonDoc => {
                format!("# -*- coding: utf-8 -*-\n\"\"\"\n{}\n\"\"\"\n\n", content)
            },
            Self::HtmlComment => {
                let mut result = String::from("<!--\n");
                for line in content.lines() {
                    if !line.trim().is_empty() {
                        result.push_str("  ");
                        result.push_str(line);
                    }
                    result.push('\n');
                }
                result.push_str("-->\n\n");
                result
            },
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
Copyright (c) {year} {copyright_holder}"#.to_string(),
                by_extension: HashMap::new(),
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
        
        // 2. Platform-specific config directory
        // Windows: %APPDATA%\Zed\auto-header.toml
        // macOS: ~/Library/Application Support/Zed/auto-header.toml
        // Linux: ~/.config/zed/auto-header.toml
        if let Some(config_dir) = dirs::config_dir() {
            config_paths.push(config_dir.join("zed").join("auto-header.toml"));
        }
        
        // 3. Home directory fallback: ~/.auto-header.toml
        if let Some(home_dir) = dirs::home_dir() {
            config_paths.push(home_dir.join(".auto-header.toml"));
        }

        config_paths.iter().any(|path| !path.as_os_str().is_empty() && path.exists())
    }

    /// Load config from multiple locations, preferring project-local config
    fn load_from_workspace(workspace_root: Option<&Path>) -> Self {
        // Try to load config from multiple locations:
        // 1. Project root: ./.auto-header.toml (if workspace_root provided)
        // 2. Platform config: (see config_exists for platform-specific paths)
        // 3. Home directory: ~/.auto-header.toml
        
        let mut config_paths = Vec::new();
        
        if let Some(root) = workspace_root {
            config_paths.push(root.join(".auto-header.toml"));
        }
        
        if let Some(config_dir) = dirs::config_dir() {
            config_paths.push(config_dir.join("zed").join("auto-header.toml"));
        }
        if let Some(home_dir) = dirs::home_dir() {
            config_paths.push(home_dir.join(".auto-header.toml"));
        }

        for path in config_paths {
            if !path.as_os_str().is_empty() && path.exists() {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(config) = toml::from_str::<Config>(&content) {
                        return config;
                    }
                }
            }
        }

        // Return default config if no config file found
        Config::default()
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

        // Get the comment style for this file type
        let comment_style = CommentStyle::for_extension(ext);

        // Priority 1: Check if user has custom template for this specific extension
        if let Some(ext_config) = self.header.by_extension.get(ext) {
            // Wrap user's extension-specific template with appropriate comment syntax
            return comment_style.wrap(&ext_config.template);
        }

        // Priority 2: Use user's default template from [header] section
        // Always wrap with appropriate comment syntax
        comment_style.wrap(&self.header.template)
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
        
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

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
            .log_message(MessageType::INFO, "Auto Header Server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.text_document.text;

        // Convert URI to file path
        // Handle cross-platform URI paths:
        // - Unix: file:///home/user/project/file.rs
        // - Windows: file:///C:/Users/user/project/file.rs
        let uri_path = uri.path();
        
        // Convert URI path to system path (handle Windows drive letters)
        let file_path_str = if cfg!(target_os = "windows") && uri_path.starts_with('/') && uri_path.len() > 2 {
            // Remove leading '/' from Windows paths like '/C:/...'
            if uri_path.chars().nth(2) == Some(':') {
                &uri_path[1..]
            } else {
                uri_path
            }
        } else {
            uri_path
        };
        
        let file_path = Path::new(file_path_str);
        
        // Find the workspace root by checking which workspace folder contains this file
        let folders = self.workspace_folders.read().await;
        let workspace_root = folders.iter()
            .find(|folder| file_path.starts_with(folder))
            .map(|p| p.as_path());

        // Only insert header if:
        // 1. File is completely empty
        // 2. A .auto-header.toml config file exists in search paths
        if content.trim().is_empty() && Config::config_exists(workspace_root) {
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
                    .log_message(MessageType::INFO, format!("Header inserted for {}", uri.path()))
                    .await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(AutoHeaderServer::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
