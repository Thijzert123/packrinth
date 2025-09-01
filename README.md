# Packrinth
Packrinth is a CLI tool for creating and managing Minecraft modpacks with Modrinth projects. The main features are being able to automatically update mods in a modpack and separate a modpack in branches.
If you want to see Packrinth in action, please take a look at [Client+](https://github.com/Thijzert123/client-plus), a modpack managed by Packrinth.

For more detail on how to use Packrinth, go [the website](https://thijzert123.github.io/packrinth).

## Installation
### Cargo
To install Packrinth with Cargo, run: 
```bash
cargo install --git https://github.com/Thijzert123/packrinth
```

### Pre-compiled binaries
Pre-compiled binaries will be available soon.

## Compiling
To compile Packrinth, clone the Git repository with:
```bash
git clone https://github.com/Thijzert123/packrinth.git
```
Then, compile the release build if you want to use the binary for production with:
```bash
cargo build --release
```
If you just want a quick build for debugging purposes, you can run:
```bash
cargo build
```

### Debian packages
To compile Debian packages, you need `cargo-deb`. You can install it with:
```bash
cargo install cargo-deb
```
After you have done that, run this to assemble a Debian package file:
```bash
cargo deb
```

## Crate as library
Packrinth provides a library alongside the binary.
This library is currently not optimised for third-party usage, so it isn't well documented.
If you don't mind reading through the source code, you can use it to easily modify
Packrinth's configuration.