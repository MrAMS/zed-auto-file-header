# Language Support

The Auto Header extension includes built-in templates for 30+ programming languages and file types. The appropriate comment format is automatically selected based on the file extension.

## Supported Languages

### C-Style Comments (`/* ... */`)

**Languages**: C, C++, C#, Java, JavaScript, TypeScript, Rust, Scala, Kotlin, Swift, Go, Objective-C

**Extensions**: `.c`, `.h`, `.cpp`, `.hpp`, `.cc`, `.hh`, `.cxx`, `.hxx`, `.cs`, `.java`, `.js`, `.jsx`, `.ts`, `.tsx`, `.rs`, `.scala`, `.kt`, `.kts`, `.swift`, `.go`, `.m`, `.mm`

**Example**:
```c
/*
 * File: main.cpp
 * Project: My Project
 * Author: John Doe <john@example.com>
 * Created: 2025-11-23 19:30:00
 * 
 * Copyright (c) 2025 John Doe
 * All rights reserved.
 */
```

### Python Docstring

**Languages**: Python

**Extensions**: `.py`, `.pyw`, `.pyx`

**Example**:
```python
# -*- coding: utf-8 -*-
"""
File: script.py
Project: My Project
Author: John Doe <john@example.com>
Created: 2025-11-23 19:30:00

Copyright (c) 2025 John Doe
All rights reserved.
"""
```

### Shell Script (with Shebang)

**Languages**: Bash, Zsh, Fish, Perl, Ruby, R, Julia

**Extensions**: `.sh`, `.bash`, `.zsh`, `.fish`, `.rb`, `.pl`, `.pm`, `.r`, `.R`, `.jl`

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

**Example**:
```html
<!--
  File: index.html
  Project: My Project
  Author: John Doe <john@example.com>
  Created: 2025-11-23 19:30:00
  
  Copyright (c) 2025 John Doe
  All rights reserved.
-->
```

### CSS Block Comments

**Languages**: CSS, SCSS, SASS, LESS

**Extensions**: `.css`, `.scss`, `.sass`, `.less`

**Example**:
```css
/**
 * File: styles.css
 * Project: My Project
 * Author: John Doe <john@example.com>
 * Created: 2025-11-23 19:30:00
 * 
 * Copyright (c) 2025 John Doe
 * All rights reserved.
 */
```

### SQL Line Comments

**Languages**: SQL

**Extensions**: `.sql`

**Example**:
```sql
-- File: schema.sql
-- Project: My Project
-- Author: John Doe <john@example.com>
-- Created: 2025-11-23 19:30:00
--
-- Copyright (c) 2025 John Doe
-- All rights reserved.
```

### YAML Hash Comments

**Languages**: YAML

**Extensions**: `.yaml`, `.yml`

**Example**:
```yaml
# File: config.yaml
# Project: My Project
# Author: John Doe <john@example.com>
# Created: 2025-11-23 19:30:00
#
# Copyright (c) 2025 John Doe
# All rights reserved.
```

### Lua/Haskell Block Comments

**Languages**: Lua, Haskell

**Extensions**: `.lua`, `.hs`, `.lhs`

**Example**:
```lua
--[[
  File: script.lua
  Project: My Project
  Author: John Doe <john@example.com>
  Created: 2025-11-23 19:30:00
  
  Copyright (c) 2025 John Doe
  All rights reserved.
--]]
```

### Lisp Semicolon Comments

**Languages**: Lisp, Scheme, Clojure

**Extensions**: `.lisp`, `.cl`, `.scm`, `.clj`, `.cljs`

**Example**:
```lisp
;;;; File: app.clj
;;;; Project: My Project
;;;; Author: John Doe <john@example.com>
;;;; Created: 2025-11-23 19:30:00
;;;;
;;;; Copyright (c) 2025 John Doe
;;;; All rights reserved.
```

### Erlang/Elixir Percent Comments

**Languages**: Erlang, Elixir

**Extensions**: `.erl`, `.hrl`, `.ex`, `.exs`

**Example**:
```erlang
%% File: server.erl
%% Project: My Project
%% Author: John Doe <john@example.com>
%% Created: 2025-11-23 19:30:00
%%
%% Copyright (c) 2025 John Doe
%% All rights reserved.
```

### Vim Script

**Languages**: Vim script

**Extensions**: `.vim`

**Example**:
```vim
" File: config.vim
" Project: My Project
" Author: John Doe <john@example.com>
" Created: 2025-11-23 19:30:00
"
" Copyright (c) 2025 John Doe
" All rights reserved.
```

## Customization

All built-in templates can be overridden by defining custom templates in your `.auto-header.toml` file. See the [README](README.md#overriding-built-in-templates) for details.

## Adding Support for New Languages

If you need support for a language not listed here, you can easily add it by defining a custom template in `.auto-header.toml`:

```toml
[header.by_extension.YOUR_EXT]
template = """Your custom header template here
"""
```

For example, to add support for `.zig` files:

```toml
[header.by_extension.zig]
template = """//! File: {filename}
//! Author: {author}
//! Date: {date}

"""
```
