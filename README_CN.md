# Auto File Header - Zed 扩展

零依赖的 Zed 编辑器扩展，在创建新的空文件时自动插入可自定义的文件头（文件名、作者、日期、版权信息）。

**作者:** MrAMS <2421653893@qq.com>  
**仓库:** https://github.com/MrAMS/zed-auto-file-header  
**平台:** Linux (x86_64/ARM64) • macOS (Intel/Apple Silicon) • Windows (x86_64)

## ✨ 主要特性

- **🚀 零依赖**: 无需安装 Rust 或构建工具 - 自动下载预编译二进制文件
- **🌍 跨平台**: 支持所有主流平台和架构
- **⚡ 自动识别**: 识别 30+ 种编程语言并使用适当的注释风格
- **🎨 完全可定制**: 定义全局或针对特定语言的模板
- **🔄 动态配置**: 配置更改立即生效，无需重启

## 快速开始

### 从 Zed 扩展市场安装

1. 打开 Zed → 扩展面板 (`Ctrl+Shift+P` → "zed: extensions")
2. 搜索 "Auto File Header"
3. 点击安装
4. 创建配置文件（必需）:
   ```bash
   cp .auto-header.toml ~/.auto-header.toml
   nano ~/.auto-header.toml   # 编辑您的信息
   ```
5. 重启 Zed

扩展会在首次使用时自动为您的平台下载合适的预编译二进制文件。**无需安装 Rust！**

### 开发者安装

1. 克隆仓库:
   ```bash
   git clone https://github.com/MrAMS/zed-auto-file-header.git
   cd zed-auto-file-header
   ```

2. 在 Zed 中安装为开发扩展:
   - 打开 Zed
   - `Ctrl+Shift+P` → `zed: install dev extension`
   - 选择 `extension` 目录

3. 创建配置文件（必需）:
   ```bash
   cp .auto-header.toml ~/.auto-header.toml
   nano ~/.auto-header.toml   # 编辑您的信息
   ```

4. 重启 Zed 并创建新的空文件 → 文件头会自动插入

配置更改会立即生效（无需重新构建或重启）。

## 文件头示例

```rust
/*
 * File: example.rs
 * Project: My Project
 * Author: Your Name <your.email@example.com>
 * Created: 2025-11-24 19:30:00
 *
 * Copyright (c) 2025 Your Name
 * All rights reserved.
 */
```

## 配置

扩展会按以下顺序搜索 `.auto-header.toml` 配置文件：

1. **项目根目录**: `./.auto-header.toml`（项目特定配置）
2. **Zed 配置目录**: `~/.config/zed/auto-header.toml` (Linux/macOS) 或 `%APPDATA%\Zed\auto-header.toml` (Windows)
3. **用户目录**: `~/.auto-header.toml`（用户全局配置）

### 配置示例

```toml
[author]
name = "您的名字"
email = "your.email@example.com"

[project]
name = "我的项目"
copyright_holder = "您的公司"  # 可选，默认使用作者名

[header]
template = """
/*
 * 文件: {filename}
 * 项目: {project}
 * 作者: {author} <{email}>
 * 创建时间: {date} {time}
 *
 * Copyright (c) {year} {copyright_holder}
 * 保留所有权利
 */

"""
```

### 模板变量

| 变量 | 描述 | 示例 |
|------|------|------|
| `{filename}` | 仅文件名 | `example.rs` |
| `{filepath}` | 完整文件路径 | `/home/user/project/example.rs` |
| `{date}` | 当前日期 | `2025-11-24` |
| `{time}` | 当前时间 | `19:30:00` |
| `{year}` | 当前年份 | `2025` |
| `{author}` | 作者名 | `您的名字` |
| `{email}` | 作者邮箱 | `your.email@example.com` |
| `{project}` | 项目名 | `我的项目` |
| `{copyright_holder}` | 版权持有人 | `您的公司` |
| `{interpreter}` | 脚本解释器 | `python3`, `bash` 等 |

### 针对特定扩展名的模板

可以为特定文件扩展名覆盖默认模板：

```toml
[header.by_extension.py]
template = """
# -*- coding: utf-8 -*-
\"\"\"
文件: {filename}
项目: {project}
作者: {author} <{email}>
创建时间: {date} {time}

Copyright (c) {year} {copyright_holder}
保留所有权利
\"\"\"

"""

[header.by_extension.sh]
template = """
#!/usr/bin/env bash
#
# 文件: {filename}
# 作者: {author}
# 日期: {date}
#

"""

[header.by_extension.html]
template = """
<!--
  文件: {filename}
  作者: {author}
  日期: {date}
-->

"""
```

## 内置语言支持

扩展自动识别 30+ 种语言并应用适当的注释风格：

### 块注释语言
C, C++, C#, Java, JavaScript, TypeScript, Rust, Go, Swift, Kotlin, Scala, Objective-C

### 行注释语言  
Python（带编码声明）, Shell (Bash, Zsh, Fish), Ruby, Perl, R, Julia

### 标记语言
HTML, XML, SVG

### 样式语言
CSS, SCSS, SASS, LESS

### 数据库语言
SQL

### 配置语言
YAML

### 其他语言
Lua, Haskell, Lisp, Scheme, Clojure, Erlang, Elixir, Vim script

详见 [LANGUAGES.md](LANGUAGES.md)。

## 故障排除

### 扩展不工作

1. **检查配置文件是否存在**: 扩展只在找到 `.auto-header.toml` 文件时才激活。
   ```bash
   # 验证配置文件存在
   ls -la ~/.auto-header.toml
   # 或
   ls -la ~/.config/zed/auto-header.toml
   ```

2. **重启 Zed**: 首次创建或修改配置文件后，需要重启 Zed。

3. **查看 Zed 日志**: 打开 Zed 的日志面板查看详细错误信息:
   - `Ctrl+Shift+P` → "zed: open log"

### 下载失败

如果看到 "Failed to fetch release from GitHub" 错误:

1. **检查网络连接**: 确保可以访问 github.com
2. **手动下载**: 从 [Releases](https://github.com/MrAMS/zed-auto-file-header/releases) 手动下载二进制文件，并放置在项目目录中，命名为:
   - Linux: `auto-header-server`
   - macOS: `auto-header-server`
   - Windows: `auto-header-server.exe`

### 平台不支持

如果看到 "Unsupported platform" 错误:

- **支持的平台**:
  - Linux: x86_64, ARM64
  - macOS: x86_64 (Intel), ARM64 (Apple Silicon)
  - Windows: x86_64

- 报告不支持的平台: https://github.com/MrAMS/zed-auto-file-header/issues

### 文件头未插入

1. **文件必须完全为空**: 扩展只在新创建的空文件中插入文件头
2. **配置必须存在**: 确保 `.auto-header.toml` 在搜索位置之一
3. **检查语言支持**: 验证您的文件扩展名是否被识别

## 开发

### 项目结构

```
├── Cargo.toml              # 工作空间根
├── .github/workflows/
│   └── release.yml         # 自动化跨平台构建
├── extension/              # Zed 扩展 (Wasm)
│   ├── Cargo.toml
│   ├── extension.toml      # 扩展清单
│   └── src/lib.rs          # 二进制下载 & LSP 启动器
└── server/                 # 语言服务器 (原生)
    ├── Cargo.toml
    └── src/main.rs         # LSP 服务器逻辑
```

### 本地构建

**服务器二进制文件:**
```bash
cargo build --release --package auto-header-server
# 输出: target/release/auto-header-server
```

**扩展 Wasm:**
```bash
rustup target add wasm32-wasip1
cargo build --release --package auto-header-extension --target wasm32-wasip1
# 输出: target/wasm32-wasip1/release/auto_header_extension.wasm
```

### 测试

**直接测试服务器:**
```bash
cargo run --package auto-header-server
# 通过 stdin 发送 LSP 消息
```

**在 Zed 中测试扩展:**
1. 构建扩展 Wasm（见上文）
2. `Ctrl+Shift+P` → "zed: install dev extension" → 选择 `extension/` 目录
3. 创建新文件进行测试

### 发布流程

匹配 `v*` 的标签会自动触发 GitHub Actions 为所有平台构建二进制文件并发布 release。详见 [PUBLISHING.md](PUBLISHING.md)。

## 架构

本扩展使用 **LSP 包装器** 设计：

1. **Zed 扩展 (Wasm)**: 
   - 检查缓存的/本地的 `auto-header-server` 二进制文件
   - 如果未找到则从 GitHub Releases 下载
   - 启动 LSP 服务器

2. **LSP 服务器 (原生)**:
   - 监听 `didOpen` 事件
   - 检查文件是否为空且配置存在
   - 注入适当的文件头模板

这种架构实现了零依赖安装，同时保持完整的 LSP 功能。

## 许可证

MIT License © 2025 MrAMS

## 贡献

欢迎提交 Issues 和 Pull Requests！

**仓库:** https://github.com/MrAMS/zed-auto-file-header

---

**注意:** 此扩展需要 `.auto-header.toml` 配置文件才能激活。没有配置文件，扩展将不会插入文件头。
