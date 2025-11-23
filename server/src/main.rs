use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

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
                template: r#"/*
 * File: {filename}
 * Author: {author}
 * Date: {date}
 * Copyright (c) {year} {copyright_holder}
 */

"#.to_string(),
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

        // Check if user has custom template for this extension
        if let Some(ext_config) = self.header.by_extension.get(ext) {
            return ext_config.template.clone();
        }

        // Use built-in language-specific templates
        let builtin_template = Self::get_builtin_template(ext);
        if !builtin_template.is_empty() {
            return builtin_template;
        }
        
        // Fall back to default template
        self.header.template.clone()
    }

    fn get_builtin_template(ext: &str) -> String {
        match ext {
            // C/C++/C#/Java/JavaScript/TypeScript/Rust/Scala/Kotlin/Swift/Go
            "c" | "h" | "cpp" | "hpp" | "cc" | "hh" | "cxx" | "hxx" | 
            "cs" | "java" | "js" | "jsx" | "ts" | "tsx" | 
            "rs" | "scala" | "kt" | "kts" | "swift" | "go" | "m" | "mm" => {
                r#"/*
 * File: {filename}
 * Project: {project}
 * Author: {author} <{email}>
 * Created: {date} {time}
 * 
 * Copyright (c) {year} {copyright_holder}
 * All rights reserved.
 */

"#.to_string()
            },
            
            // Python
            "py" | "pyw" | "pyx" => {
                r#"# -*- coding: utf-8 -*-
"""
File: {filename}
Project: {project}
Author: {author} <{email}>
Created: {date} {time}

Copyright (c) {year} {copyright_holder}
All rights reserved.
"""

"#.to_string()
            },
            
            // Ruby/Perl/Shell/Bash/Zsh/Fish/R/Julia
            "rb" | "pl" | "pm" | "sh" | "bash" | "zsh" | "fish" | "r" | "R" | "jl" => {
                r#"#!/usr/bin/env {interpreter}
#
# File: {filename}
# Project: {project}
# Author: {author} <{email}>
# Created: {date} {time}
#
# Copyright (c) {year} {copyright_holder}
# All rights reserved.
#

"#.to_string()
            },
            
            // HTML/XML/SVG
            "html" | "htm" | "xml" | "svg" | "xhtml" => {
                r#"<!--
  File: {filename}
  Project: {project}
  Author: {author} <{email}>
  Created: {date} {time}
  
  Copyright (c) {year} {copyright_holder}
  All rights reserved.
-->

"#.to_string()
            },
            
            // CSS/SCSS/SASS/LESS
            "css" | "scss" | "sass" | "less" => {
                r#"/**
 * File: {filename}
 * Project: {project}
 * Author: {author} <{email}>
 * Created: {date} {time}
 * 
 * Copyright (c) {year} {copyright_holder}
 * All rights reserved.
 */

"#.to_string()
            },
            
            // SQL
            "sql" => {
                r#"-- File: {filename}
-- Project: {project}
-- Author: {author} <{email}>
-- Created: {date} {time}
--
-- Copyright (c) {year} {copyright_holder}
-- All rights reserved.

"#.to_string()
            },
            
            // YAML
            "yaml" | "yml" => {
                r#"# File: {filename}
# Project: {project}
# Author: {author} <{email}>
# Created: {date} {time}
#
# Copyright (c) {year} {copyright_holder}
# All rights reserved.

"#.to_string()
            },
            
            // Lua/Haskell
            "lua" | "hs" | "lhs" => {
                r#"--[[
  File: {filename}
  Project: {project}
  Author: {author} <{email}>
  Created: {date} {time}
  
  Copyright (c) {year} {copyright_holder}
  All rights reserved.
--]]

"#.to_string()
            },
            
            // Lisp/Scheme/Clojure
            "lisp" | "cl" | "scm" | "clj" | "cljs" => {
                r#";;;; File: {filename}
;;;; Project: {project}
;;;; Author: {author} <{email}>
;;;; Created: {date} {time}
;;;;
;;;; Copyright (c) {year} {copyright_holder}
;;;; All rights reserved.

"#.to_string()
            },
            
            // Erlang/Elixir
            "erl" | "hrl" | "ex" | "exs" => {
                r#"%% File: {filename}
%% Project: {project}
%% Author: {author} <{email}>
%% Created: {date} {time}
%%
%% Copyright (c) {year} {copyright_holder}
%% All rights reserved.

"#.to_string()
            },
            
            // Vim script
            "vim" => {
                r#"" File: {filename}
" Project: {project}
" Author: {author} <{email}>
" Created: {date} {time}
"
" Copyright (c) {year} {copyright_holder}
" All rights reserved.

"#.to_string()
            },
            
            // No built-in template
            _ => String::new(),
        }
    }
}

#[derive(Debug)]
struct AutoHeaderServer {
    client: Client,
    // retained for future feature (e.g., config watching); currently dynamic reload used
    _initial_config: Config,
}

impl AutoHeaderServer {
    fn new(client: Client) -> Self {
        let initial_config = Config::load();
        Self { client, _initial_config: initial_config }
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
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
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

        // Extract workspace root from file path (parent directory of the file)
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
        let workspace_root = file_path.parent();

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

    let (service, socket) = LspService::new(|client| AutoHeaderServer::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
