---
title: Home
layout: home
nav_order: 1
---

<img src="logo.png" alt="logo" width=100 height=100 />
{: .text-center}

# Packrinth
{: .text-center}
Packrinth is a CLI tool for creating and managing Minecraft modpacks with Modrinth projects.
{: .text-center}

[![Crates.io Version](https://img.shields.io/crates/v/packrinth?style=for-the-badge)](https://crates.io/crates/packrinth)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/packrinth?style=for-the-badge)](https://crates.io/crates/packrinth)
{: .text-center}

---

## Features
- Automatic mod updates from Modrinth
- Automatic dependency fetching
- Separation into branches
- Exporting modpacks to Modrinth modpacks
- Automatically generating documentation

If you want to see an example of how Packrinth is used, see [Client+](https://github.com/Thijzert123/client-plus):
a Minecraft modpack managed by Packrinth.

## Installation
If you have Cargo installed, you can use that to install Packrinth:
```bash
cargo install packrinth
```

You can also manually install Packrinth with one of the pre-compiled binaries from the [latest GitHub release](https://github.com/Thijzert123/packrinth/releases/latest).

## Getting started
Please read the [full guide](full-guide.html). It explains all the basics for Packrinth.
If you just want a reference for the CLI and configuration, take a look at [CLI help](cli-help.html) and
[configuration reference](configuration-reference.html).

## Packrinth as library
Packrinth provides a library alongside the binary. Using this, you can modify Packrinth's configuration
in an idiomatic way. For example, you can write your own CLI interface for Packrinth!