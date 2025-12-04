# Auto File Header - Zed 扩展

零依赖的 Zed 编辑器扩展，在创建新的空文件时自动插入可自定义的文件头。

**作者:** MrAMS <2421653893@qq.com>  
**仓库:** https://github.com/MrAMS/zed-auto-file-header  
**平台:** Linux (x86_64/ARM64) • macOS (Intel/Apple Silicon) • Windows (x86_64)

## ✨ 功能特性

- **🚀 零依赖**: 无需安装 Rust 或构建工具 - 自动下载预编译二进制文件
- **🌍 跨平台**: 支持所有主流平台和架构
- **⚡ 自动识别**: 识别 35+ 种编程语言并使用适当的注释风格
- **🎨 完全可定制**: 定义全局或针对特定语言的模板
- **🔄 动态配置**: 配置更改立即生效，无需重启
- **📁 灵活配置位置**: 支持项目特定配置或用户全局配置

## 📦 安装

1. 打开 Zed → 扩展面板 (`Ctrl+Shift+P` → "zed: extensions")
2. 搜索 "Auto File Header"
3. 点击安装
4. **创建配置文件**（必需 - 请参阅下方[配置](#-配置)章节）
5. 重启 Zed

首次使用时，扩展会自动为您的平台下载合适的预编译二进制文件。您会在状态栏看到 "auto-header: Downloading..." - 这只会发生一次。

## 📋 支持的语言

扩展自动识别并为 **35+ 种语言**应用适当的注释风格：

- **C 风格**: C, C++, C#, Java, JavaScript, TypeScript, Rust, Go, Swift, Kotlin, Scala
- **脚本**: Python, Bash, Zsh, Fish, Ruby, Perl, R, Julia, Tcl
- **硬件**: Verilog, SystemVerilog
- **标记**: HTML, XML, SVG
- **样式**: CSS, SCSS, SASS, LESS
- **数据库**: SQL
- **配置**: YAML, TOML, INI
- **函数式**: Lua, Haskell, Lisp, Scheme, Clojure, Erlang, Elixir
- **编辑器**: Vim script

📖 **完整详情和示例请参阅 [LANGUAGES.md](LANGUAGES.md)。**

## ⚙️ 配置

### 配置文件：`.auto-header.toml`

**扩展只在存在 `.auto-header.toml` 文件时才会激活。** 该文件定义您的作者信息、项目详情和文件头模板。

### 配置文件优先级（从高到低）

扩展按以下顺序搜索 `.auto-header.toml` 并使用**第一个找到的文件**：

1. **项目根目录**（最高优先级）
   - 路径：`./.auto-header.toml`（项目根目录下）
   - 使用场景：项目特定的文件头，包含自定义版权、团队信息等

2. **Zed 配置目录**
   - Linux/macOS: `~/.config/zed/auto-header.toml`
   - Windows: `%APPDATA%\Zed\auto-header.toml`
   - 使用场景：用户级默认设置，应用于所有项目

3. **用户主目录**（最低优先级）
   - 路径：`~/.auto-header.toml`
   - 使用场景：备用位置，传统的点文件方式

**推荐用法**：
- 团队项目使用**项目根目录**配置，设置特定的版权/许可信息
- 个人项目使用 **Zed 配置目录**作为默认设置

### 基础配置示例

在上述任一位置创建文件并填入您的信息：

```toml
[author]
name = "您的名字"
email = "your.email@example.com"

[project]
name = "我的项目"
copyright_holder = "您的公司"  # 可选，默认使用作者名

[header]
# [header] 部分是可选的 - 如果省略，将使用默认模板
# 只需编写内容，无需添加注释符号！
# 扩展会自动为每种语言添加正确的注释格式
template = """
文件: {filename}
项目: {project}
作者: {author} <{email}>
创建时间: {date} {time}

Copyright (c) {year} {copyright_holder}
保留所有权利
"""
```

**✨ 自动注释包装**：只需编写模板内容**一次**，无需任何注释语法。扩展会智能地为每种语言添加正确的注释格式：
- **C/Rust/Java/JavaScript**：`/* ... */`
- **Python**：`""" ... """`（包含 UTF-8 编码声明）
- **Shell 脚本**：`#`（包含自动添加的 shebang，如 `#!/usr/bin/env bash`）
- **HTML**：`<!-- ... -->`
- **SQL**：`--`
- **Lua**：`--[[ ... ]]`
- **Verilog/SystemVerilog**：`//`
- **Tcl**：`#`
- 以及 35+ 种其他语言！

**💡 这让您的配置更简洁、更具可移植性** - 编写一次，处处适用！

### 模板变量

| 变量 | 描述 | 示例 |
|------|------|------|
| `{filename}` | 仅文件名 | `example.rs` |
| `{filepath}` | 完整文件路径 | `/home/user/project/example.rs` |
| `{date}` | 当前日期 | `2025-11-24` |
| `{time}` | 当前时间 | `19:30:00` |
| `{year}` | 当前年份 | `2025` |
| `{author}` | 配置中的作者名 | `您的名字` |
| `{email}` | 配置中的作者邮箱 | `your.email@example.com` |
| `{project}` | 配置中的项目名 | `我的项目` |
| `{copyright_holder}` | 版权持有人（默认为作者） | `您的公司` |
| `{interpreter}` | 脚本解释器（用于 shebang） | `python3`, `bash` 等 |

### 开源许可证支持

您可以轻松自定义模板以包含开源许可证：

**MIT 许可证示例**:
```toml
[header]
template = """
文件: {filename}
作者: {author} <{email}>
日期: {date}

Copyright (c) {year} {copyright_holder}

特此免费授予任何获得本软件副本和相关文档文件（"软件"）的人不受限制地处置该软件的权利，
包括不受限制地使用、复制、修改、合并、发布、分发、再许可和/或出售该软件副本，
以及再授权被配发了本软件的人如上的权利，须在下列条件下：

上述版权声明和本许可声明应包含在该软件的所有副本或实质成分中。

本软件是"如此"提供的，没有任何形式的明示或暗示的保证，包括但不限于
对适销性、特定用途的适用性和不侵权的保证。
"""
```

**Mozilla 公共许可证 (MPL-2.0) 示例**:
```toml
[header]
template = """
文件: {filename}
作者: {author}

此源代码的使用受 Mozilla Public License v. 2.0 许可协议的约束
如果未随此文件分发 MPL 许可证副本，您可以在以下网址获得：
https://mozilla.org/MPL/2.0/
"""
```

**Apache 许可证 2.0 示例**:
```toml
[header]
template = """
文件: {filename}

Copyright {year} {copyright_holder}

根据 Apache 许可证 2.0 版本（"许可证"）授权；
除非遵守许可证，否则您不得使用此文件。
您可以在以下网址获得许可证副本：

    http://www.apache.org/licenses/LICENSE-2.0

除非适用法律要求或书面同意，否则根据许可证分发的软件
是按"原样"分发的，不附带任何明示或暗示的保证或条件。
"""
```

### 针对特定语言的模板覆盖

可以为特定文件扩展名覆盖默认模板：

```toml
[header.by_extension.py]
template = """
文件: {filename}
项目: {project}
作者: {author} <{email}>
创建时间: {date} {time}

Copyright (c) {year} {copyright_holder}
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

### 完整配置示例

本仓库中包含的 [`.auto-header.toml`](.auto-header.toml) 文件提供了包含多种语言覆盖的完整示例。

## 📝 使用方法

1. **创建 `.auto-header.toml` 文件**，放在上述任一位置
2. **重启 Zed**（仅首次创建配置文件时需要）
3. **在 Zed 中创建新文件**
4. **打开空文件时文件头会自动插入**

**注意**：文件头只会插入到完全为空的文件中。如果文件已有内容，则不会添加文件头。

## 💡 文件头示例

使用上述基础配置，创建新的 `example.rs` 文件时会自动插入：

```rust
/*
 * 文件: example.rs
 * 项目: 我的项目
 * 作者: 您的名字 <your.email@example.com>
 * 创建时间: 2025-11-24 19:30:00
 *
 * Copyright (c) 2025 您的公司
 * 保留所有权利
 */
```

## 🔧 故障排除

### 扩展不工作

**问题**：创建新文件时不插入文件头

**解决方案**：
1. **检查配置文件是否存在**：
   ```bash
   # 检查项目根目录
   ls -la .auto-header.toml
   
   # 检查 Zed 配置目录（Linux/macOS）
   ls -la ~/.config/zed/auto-header.toml
   
   # 检查用户主目录
   ls -la ~/.auto-header.toml
   ```

2. **重启 Zed**（首次创建配置文件后需要）

3. **确保文件为空**：文件头只会插入到新建的完全空白的文件中

4. **查看 Zed 日志**：
   - `Ctrl+Shift+P` → "zed: open log"
   - 查找 "Auto File Header" 相关消息

### 下载失败

**问题**：扩展显示错误 "Failed to fetch release from GitHub"

**解决方案**：
1. **检查网络连接**：确保可以访问 github.com

2. **等待并重试**：GitHub API 可能有速率限制

3. **手动下载**（备用方案）：
   - 从 [Releases](https://github.com/MrAMS/zed-auto-file-header/releases) 下载二进制文件
   - 放置在项目目录中，命名为：
     - Linux/macOS: `auto-header-server`
     - Windows: `auto-header-server.exe`

### 首次下载状态

**预期行为**：首次使用时，您会在 Zed 状态栏看到 "auto-header: Downloading..." 几秒钟。这只会发生一次，因为二进制文件（约 2-3 MB）会被下载并缓存。

### 平台不支持

**问题**：错误信息 "Unsupported platform"

**支持的平台**：
- Linux: x86_64, ARM64 (aarch64)
- macOS: x86_64 (Intel), ARM64 (Apple Silicon)
- Windows: x86_64

如果您的平台不在支持列表中，请[提交 issue](https://github.com/MrAMS/zed-auto-file-header/issues)。

### 特定语言的文件头未插入

**问题**：某些文件类型可以插入文件头，但其他类型不行

**解决方案**：
1. **检查文件扩展名**：确保文件扩展名在[支持的语言](#-支持的语言)列表中

2. **检查内置模板**：并非所有扩展名都有内置模板，但您可以在 `.auto-header.toml` 中添加自定义模板：
   ```toml
   [header.by_extension.xyz]
   template = """
   # 您的 .xyz 文件自定义模板
   """
   ```

## 👨‍💻 开发

### 开发者安装

如果您想修改或测试扩展：

1. 克隆仓库：
   ```bash
   git clone https://github.com/MrAMS/zed-auto-file-header.git
   cd zed-auto-file-header
   ```

2. 在 Zed 中安装为开发扩展：
   - 打开 Zed
   - `Ctrl+Shift+P` → `zed: install dev extension`
   - 选择 `extension` 目录

3. 创建配置文件：
   ```bash
   cp .auto-header.toml ~/.auto-header.toml
   nano ~/.auto-header.toml   # 编辑您的信息
   ```

4. 重启 Zed 并测试

配置更改会立即生效（无需重新构建）。

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

### 发布流程

匹配 `v*` 的标签会自动触发 GitHub Actions 为所有平台构建二进制文件并发布 release。详见 [PUBLISHING.md](PUBLISHING.md)。

## 🏗️ 架构

本扩展使用 **LSP 包装器**设计：

1. **Zed 扩展 (Wasm)**: 
   - 检查缓存的/本地的 `auto-header-server` 二进制文件
   - 如果未找到则从 GitHub Releases 下载（带状态指示）
   - 启动 LSP 服务器

2. **LSP 服务器 (原生)**:
   - 监听 `didOpen` 事件
   - 检查文件是否为空且配置存在
   - 根据语言注入适当的文件头模板

这种架构实现了零依赖安装，同时保持完整的 LSP 功能。

## 📄 许可证

MIT License © 2025 MrAMS

## 🤝 贡献

欢迎提交 Issues 和 Pull Requests！

**仓库:** https://github.com/MrAMS/zed-auto-file-header

---

**快速设置清单：**
- ✅ 从 Zed 市场安装扩展
- ✅ 创建 `.auto-header.toml`（项目根目录或 `~/.config/zed/`）
- ✅ 添加作者信息并自定义模板
- ✅ 重启 Zed
- ✅ 创建新文件，见证魔法！✨
