---
title: Shell completion
layout: default
nav_order: 5
---

# Shell completion
If you want to create shell completions for Packrinth, run this command:
```bash
$ packrinth completions <SHELL>
```
Possible shell values are:

| Shell      | Command value |
|------------|---------------|
| Bash       | `bash`        |
| Elvis      | `elvis`       |
| Fish       | `fish`        |
| PowerShell | `powershell`  |
| Zsh        | `zsh`         |

This will generate a shell completion script. Make sure the script loads every time you start your shell.