<div align="center">
  <a href="https://packrinth.thijzert.nl"><img src="https://github.com/Thijzert123/packrinth/blob/ff8455254b966d7879ca2c378a4350c1a56cbfc6/logo.png?raw=true" alt="logo" width=100 height=100 /></a>
  <h1>Packrinth</h1>
  CLI tool for creating and managing Minecraft modpacks with Modrinth projects

  <p></p>

  [![AUR Version](https://img.shields.io/aur/version/packrinth?style=for-the-badge)](https://aur.archlinux.org/packages/packrinth)
  [![Crates.io Version](https://img.shields.io/crates/v/packrinth?style=for-the-badge)](https://crates.io/crates/packrinth)
  [![Crates.io Total Downloads](https://img.shields.io/crates/d/packrinth?style=for-the-badge)](https://crates.io/crates/packrinth)
</div>

Packrinth is a CLI tool for creating and managing Minecraft modpacks with Modrinth projects. The main features are being able to automatically update mods in a modpack and separate a modpack in branches.
If you want to see Packrinth in action, please take a look at [Client+](https://github.com/Thijzert123/client-plus), a modpack managed by Packrinth.

### For more details on how to use Packrinth, go [the website](https://packrinth.thijzert.nl).

## Installation
### Cargo
To install the latest version of Packrinth with Cargo, run:
```bash
cargo install packrinth
```

### AUR
Packrinth is available on the [AUR](https://aur.archlinux.org/packages/packrinth). You can install it with:
```bash
yay -S packrinth
```

### Pre-compiled binaries
You can also manually install Packrinth with one of the pre-compiled binaries from the [latest GitHub release](https://github.com/Thijzert123/packrinth/releases/latest).

## Compiling
To compile Packrinth, clone the Git repository with:
```bash
git clone https://github.com/Thijzert123/packrinth.git
```
Then, compile the debug build with:
```bash
cargo build
```

## Crate as library
Packrinth provides a library alongside the binary. Using this, you can modify Packrinth's configuration
in an idiomatic way. For example, you can write your own CLI interface for Packrinth!
