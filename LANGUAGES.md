# Language Support

The Auto Header extension automatically wraps your template content with the appropriate comment format for **35+ programming languages**. Simply write your template once, and the extension handles the rest!

## How It Works

You write **plain content** in your `.auto-header.toml`:

```toml
[header]
template = """
File: {filename}
Author: {author}
Date: {date}
"""
```

The extension automatically wraps it with the correct comment syntax for each language:

## Supported Languages

### C-Style Block Comments (`/* ... */`)

**Languages**: C, C++, C#, Java, JavaScript, TypeScript, Rust, Scala, Kotlin, Swift, Go, Objective-C, CSS, SCSS, SASS, LESS

**Extensions**: `.c`, `.h`, `.cpp`, `.hpp`, `.cc`, `.hh`, `.cxx`, `.hxx`, `.cs`, `.java`, `.js`, `.jsx`, `.ts`, `.tsx`, `.rs`, `.scala`, `.kt`, `.kts`, `.swift`, `.go`, `.m`, `.mm`, `.css`, `.scss`, `.sass`, `.less`

**Auto-generated Format**:
```c
/*
 * File: {filename}
 * Author: {author}
 * Date: {date}
 */
```

### C++ Style Line Comments (`//`)

**Languages**: Verilog, SystemVerilog

**Extensions**: `.v`, `.vh`, `.sv`, `.svh`

**Auto-generated Format**:
```verilog
// File: {filename}
// Author: {author}
// Date: {date}
```

### Python Docstring

**Languages**: Python

**Extensions**: `.py`, `.pyw`, `.pyx`

**Auto-generated Format**:
```python
# -*- coding: utf-8 -*-
"""
File: {filename}
Author: {author}
Date: {date}
"""
```

### Shell Script (Hash Comments with Shebang)

**Languages**: Bash, Zsh, Fish

**Extensions**: `.sh`, `.bash`, `.zsh`, `.fish`

**Auto-generated Format**:
```bash
#!/usr/bin/env bash
#
# File: {filename}
# Author: {author}
# Date: {date}
#
```

### Scripting Languages with Shebang

**Languages**: Ruby, Perl, R, Julia

**Extensions**: `.rb`, `.pl`, `.pm`, `.r`, `.R`, `.jl`

**Auto-generated Format**:
```ruby
#!/usr/bin/env ruby
#
# File: {filename}
# Author: {author}
# Date: {date}
#
```

### Hash Comments (No Shebang)

**Languages**: YAML, TOML, Tcl, INI, Config files

**Extensions**: `.yaml`, `.yml`, `.toml`, `.ini`, `.conf`, `.cfg`, `.tcl`

**Auto-generated Format**:
```yaml
# File: {filename}
# Author: {author}
# Date: {date}
```

**Example**:
```bash
#!/usr/bin/env bash
#
# File: deploy.sh
# Project: My Project
# Author: John Doe <john@example.com>
# Created: 2025-11-23 19:30:00
#
# Copyright (c) 2025 John Doe
# All rights reserved.
#
```

### HTML/XML Comments

**Languages**: HTML, XML, SVG

**Extensions**: `.html`, `.htm`, `.xml`, `.svg`, `.xhtml`

**Auto-generated Format**:
```html
<!--
  File: {filename}
  Author: {author}
  Date: {date}
-->
```

### SQL Line Comments

**Languages**: SQL

**Extensions**: `.sql`

**Auto-generated Format**:
```sql
-- File: {filename}
-- Author: {author}
-- Date: {date}
```

### Lua/Haskell Block Comments

**Languages**: Lua, Haskell

**Extensions**: `.lua`, `.hs`, `.lhs`

**Auto-generated Format**:
```lua
--[[
  File: {filename}
  Author: {author}
  Date: {date}
--]]
```

### Lisp Semicolon Comments

**Languages**: Lisp, Scheme, Clojure

**Extensions**: `.lisp`, `.cl`, `.scm`, `.clj`, `.cljs`

**Auto-generated Format**:
```lisp
;;;; File: {filename}
;;;; Author: {author}
;;;; Date: {date}
```

### Erlang/Elixir Percent Comments

**Languages**: Erlang, Elixir

**Extensions**: `.erl`, `.hrl`, `.ex`, `.exs`

**Auto-generated Format**:
```erlang
%% File: {filename}
%% Author: {author}
%% Date: {date}
```

### Vim Script

**Languages**: Vim script

**Extensions**: `.vim`

**Auto-generated Format**:
```vim
" File: {filename}
" Author: {author}
" Date: {date}
```

## Key Features

✨ **Write Once, Run Everywhere**: Your template content works for all languages  
🎯 **Automatic Format Detection**: Comment style chosen based on file extension  
🔧 **Fully Customizable**: Override any language with custom templates  
📦 **35+ Languages Supported**: From C to Verilog, Python to Vim script  

## Customization

You can override the automatic wrapping for specific languages:

```toml
[header.by_extension.py]
template = """
Custom Python header
Add your own content here
"""
```

The extension will still wrap it with Python's `""" """` format automatically.

## Complete Language List

| Category | Languages | Extensions |
|----------|-----------|------------|
| **C-Style** | C, C++, C#, Java, JavaScript, TypeScript, Rust, Go, Swift, Kotlin, Scala | `.c`, `.cpp`, `.cs`, `.java`, `.js`, `.ts`, `.rs`, `.go`, `.swift`, `.kt`, `.scala` |
| **Line Comments** | Verilog, SystemVerilog | `.v`, `.vh`, `.sv`, `.svh` |
| **Python** | Python | `.py`, `.pyw`, `.pyx` |
| **Shell** | Bash, Zsh, Fish, Ruby, Perl, R, Julia, Tcl | `.sh`, `.bash`, `.zsh`, `.fish`, `.rb`, `.pl`, `.r`, `.jl`, `.tcl` |
| **Markup** | HTML, XML, SVG | `.html`, `.xml`, `.svg` |
| **Style** | CSS, SCSS, SASS, LESS | `.css`, `.scss`, `.sass`, `.less` |
| **Config** | YAML, TOML, INI | `.yaml`, `.yml`, `.toml`, `.ini` |
| **Database** | SQL | `.sql` |
| **Functional** | Lua, Haskell, Lisp, Scheme, Clojure, Erlang, Elixir | `.lua`, `.hs`, `.lisp`, `.scm`, `.clj`, `.erl`, `.ex` |
| **Editor** | Vim script | `.vim` |
