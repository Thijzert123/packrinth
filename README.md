# Packrinth
Packrinth is a CLI tool for creating and managing Minecraft modpacks with Modrinth projects. The main features are being able to automatically update mods in a modpack and separate a modpack in branches.
If you want to see Packrinth in action, please take a look at [Client+](https://github.com/Thijzert123/client-plus), a modpack managed by Packrinth.

For more detail on how to use Packrinth, go [the website](https://thijzert123.github.io/packrinth).

## Installation
### Cargo
To install the latest version of Packrinth with Cargo, run:
```bash
cargo install packrinth
```

### Pre-compiled binaries
Pre-compiled binaries will be available soon.

## Compiling
To compile Packrinth, clone the Git repository with:
```bash
git clone https://github.com/Thijzert123/packrinth.git
```
Then, compile the debug build with:
```bash
cargo build
```

You can also manually install Packrinth with one of the pre-compiled binaries from the [latest GitHub release](https://github.com/Thijzert123/packrinth/releases/latest).

## Crate as library
Packrinth provides a library alongside the binary. Using this, you can modify Packrinth's configuration
in an idiomatic way. For example, you can write your own CLI interface for Packrinth!