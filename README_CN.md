# Auto File Header - Zed 扩展

自动为新建的空白文件插入可定制的文件头（文件名、作者、日期、版权等）。

**作者:** MrAMS <2421653893@qq.com>  
**仓库:** https://github.com/MrAMS/zed-auto-file-header  
**平台:** Linux • macOS • Windows

> 激活条件：在以下任一位置存在 `.auto-header.toml`：项目根目录、`~/.config/zed/auto-header.toml`、`~/.auto-header.toml`。

**配置文件搜索顺序**：
1. `./.auto-header.toml`
2. `~/.config/zed/auto-header.toml`
3. `~/.auto-header.toml`

## 🚀 安装（发布后）

### 从 Zed 扩展商店安装

1. 打开 Zed → 扩展面板
2. 搜索 "Auto File Header"
3. 点击安装
4. 创建配置文件（必需）：
   ```bash
   cp .auto-header.toml ~/.auto-header.toml
   nano ~/.auto-header.toml   # 编辑配置
   ```
5. 重启 Zed

**零依赖**: 扩展会自动为您的平台下载预编译的二进制文件（Linux x86_64/ARM64、macOS Intel/Apple Silicon、Windows x86_64）。无需安装 Rust!

## 🔧 开发安装

### 步骤 1: 克隆仓库

```bash
git clone https://github.com/MrAMS/zed-auto-file-header.git
cd zed-auto-file-header
```

### 步骤 2: 构建本地服务器

```bash
./build-dev.sh
```

**注意**: 开发版需要 Rust 工具链，但发布版不需要。

### 步骤 3: 通过 Zed 安装扩展

1. 打开 Zed 编辑器
2. 按 `Ctrl + Shift + P` 打开命令面板
3. 输入 `zed: install dev extension`
4. 选择本项目的 `extension` 目录

### 步骤 4: 创建配置文件（必需）

```bash
cp .auto-header.toml ~/.auto-header.toml
# 编辑配置文件填入你的信息
nano ~/.auto-header.toml
```

### 步骤 5: 重启 Zed

完全关闭并重新打开 Zed 后，新建空文件即可自动插入。

## 📝 使用方法

1. **创建新文件**: `Cmd/Ctrl + N`
2. **不要输入任何内容**（文件必须完全空白）
3. **保存文件**: `Cmd/Ctrl + S`，输入文件名如 `test.rs`
4. **自动插入**: 文件头会自动出现！

### 示例输出

创建 `example.rs` 后会自动插入：

```
/*
 * File: example.rs
 * Project: My Project
 * Author: Your Name <your@email.com>
 * Created: 2025-11-23 20:25:00
 *
 * Copyright (c) 2025 Your Name
 * All rights reserved.
 *
 * Description:
 *   在此填写描述
 */
```

## ⚙️ 配置

创建并编辑配置文件：

推荐：
```bash
cp .auto-header.toml ~/.auto-header.toml
nano ~/.auto-header.toml
```

或放入 Zed 配置目录：
```bash
mkdir -p ~/.config/zed
cp .auto-header.toml ~/.config/zed/auto-header.toml
```

示例：
```toml
[author]
name = "Your Name"
email = "your@email.com"

[project]
name = "My Project"
copyright_holder = "Your Company"

[header]
template = """/*\n * File: {filename}\n * Author: {author} <{email}>\n * Date: {date}\n * Copyright (c) {year} {copyright_holder}\n */\n\n"""
```

### 模板变量
`{filename}` `{filepath}` `{date}` `{year}` `{time}` `{author}` `{email}` `{project}` `{copyright_holder}`

### 按扩展名覆盖示例
```toml
[header.by_extension.py]
template = """# File: {filename}\n# Author: {author}\n# Date: {date}\n\n"""

[header.by_extension.sh]
template = """#!/bin/bash\n# File: {filename}\n# Author: {author}\n# Date: {date}\n\n"""

[header.by_extension.html]
template = """<!--\n  File: {filename}\n  Author: {author}\n  Date: {date}\n-->\n\n"""
```

## 🌍 支持的语言

内置支持 30+ 编程语言，自动识别文件类型并应用正确的注释格式：

- **C/C++, Rust, Go, Java, JavaScript, TypeScript, Scala, Swift, Kotlin**
- **Python** (特殊格式，包含编码声明和 docstring)
- **Shell 脚本** (Bash, Zsh, Fish, Ruby, Perl) - 自动添加 shebang
- **HTML, XML, CSS, SCSS, YAML, JSON, SQL**
- **Lua, Haskell, Lisp, Clojure, Erlang, Elixir**
- 更多...

查看完整列表：[LANGUAGES.md](LANGUAGES.md)

## 🔧 故障排查

### 扩展未加载？

1. 检查是否已通过 "Install Dev Extension" 安装
2. 完全重启 Zed（不只是重新加载窗口）
3. 查看 Zed 日志：
   ```bash
   tail -f ~/.local/share/zed/logs/Zed.log
   ```

### 文件头未插入？

**检查清单**:
- [ ] 文件是否**完全空白**？（没有任何字符、空格、换行）
- [ ] 是否先保存了文件？（需要有文件扩展名）
- [ ] 文件扩展名是否支持？（见 LANGUAGES.md）
- [ ] 扩展是否已在 Zed 中加载？

**测试步骤**:
1. `Cmd/Ctrl + N` 创建新文件
2. **立即保存**（不要输入任何内容）
3. 保存为 `test.cpp`
4. 头部应立即出现

### LSP 服务器问题？

测试服务器：
```bash
cargo run --package auto-header-server
```

## 📚 文档

- [LANGUAGES.md](LANGUAGES.md) - 支持的语言和示例
- [QUICKSTART.md](QUICKSTART.md) - 快速参考
- [TESTING.md](TESTING.md) - 详细测试指南
- [ARCHITECTURE.md](ARCHITECTURE.md) - 架构说明

## 🛠️ 开发

### 项目结构

```
zed-file-header/
├── extension/              # Zed 扩展 (Wasm)
│   ├── src/lib.rs          # 扩展代码
│   ├── extension.toml      # 扩展声明
│   └── Cargo.toml
├── server/                 # LSP 服务器 (Rust)
│   ├── src/main.rs        # LSP 实现
│   └── Cargo.toml
├── .auto-header.toml      # 配置模板
└── build-dev.sh           # 构建脚本
```

### 修改代码后

1. 修改服务器代码：编辑 `server/src/main.rs`
2. 重新构建：`./build-dev.sh`
3. 在 Zed 中重新加载扩展：
   - Command Palette → `zed: reload extensions`

### 修改扩展代码后

1. 修改扩展代码：编辑 `extension/src/lib.rs`
2. 在 Zed 中重新安装：
   - Command Palette → `zed: install dev extension`
   - 选择 `extension` 目录

## ❓ 常见问题

**Q: 为什么不能直接复制到 extensions 目录？**  
A: Zed 需要自行编译 Wasm component。使用 "Install Dev Extension" 进行安装。

**Q: 文件头格式可以自定义吗？**  
A: 是的！编辑 `.auto-header.toml` 配置文件即可。配置修改立即生效，无需重启。

**Q: 扩展不工作？**  
A: 是否存在配置文件且文件是新建且空白？是否使用支持的扩展名？

## 🤝 贡献
欢迎提交 Issue / PR: [GitHub](https://github.com/MrAMS/zed-auto-file-header)

## 📜 许可证
MIT License © 2025 MrAMS
