# Packrinth

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
