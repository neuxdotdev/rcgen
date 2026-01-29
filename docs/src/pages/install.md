# RCGen Installation

This document describes various RCGen installation methods for various platforms.

## System Requirements

- **Git**: RCGen requires Git to run
- **Rust Toolchain** (only for source installations): rustc 1.70.0 or later
- **Storage**: Minimum 10MB of free space

## Installation Methods

### 1. Installing from Source (Rust/Cargo)

This method is recommended for developers who want the latest version or want to contribute.

#### Step 1: Install the Rust Toolchain

```bash
# Install Rust using rustup
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart terminal or run:
$ source $HOME/.cargo/env

# Verify installation
$ rustc --version
$ cargo --version
```

#### Step 2: Install RCGen

**Option A: Install directly from the repository**

```bash
$ cargo install --git https://github.com/neuxdotdev/rcgen
```

**Option B: Clone and build manually**

```bash
# Clone repository
$ git clone https://github.com/neuxdotdev/rcgen
$ cd rcgen

# Build and install
$ cargo install --path .

# Or build for release
$ cargo build --release
# Binaries will be in ./target/release/rcgen
```

## Verify Installation

After installation, verify with:

```bash
$ rcgen --version
$ rcgen --help
```

The output should show the RCGen version and a list of available commands.

## Update RCGen

### For Cargo installation:

```bash
$ cargo install --git https://github.com/neuxdotdev/rcgen --force
```

### For binary releases:

Redownload the binary from the latest release.

### For Homebrew:

```bash
$ brew upgrade rcgen
```

## Troubleshooting

### 1. "Command not found" after installation

Make sure the installation directory is in the PATH:

```bash
$ echo $PATH
# Make sure ~/.cargo/bin (for Cargo) or /usr/local/bin is in the PATH
```

### 2. Error building from source

Make sure the Rust toolchain is installed correctly:

```bash
$ rustup update
$ cargo clean
$ cargo build --release
```

### 3. Permission denied on Linux/macOS

```bash
# Grant execute permission
$ chmod +x rcgen

# For system installations
$ sudo chmod +x /usr/local/bin/rcgen
```

### 4. Missing dependencies on Windows

Install [Microsoft Visual Studio 2019] C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe).

## Installation for Development

If you want to contribute to RCGen:

```bash
# Clone repository
$ git clone https://github.com/neuxdotdev/rcgen
$ cd rcgen

# Build in development mode
$ cargo build

# Run tests
$ cargo test

# Build documentation
$ cargo doc --open
```

## Uninstall

### Cargo

```bash
$ cargo uninstall rcgen
```

### Binary release

```bash
# Remove binary
$ sudo rm /usr/local/bin/rcgen # or your installation location
```

### Homebrew

```bash
$ brew uninstall rcgen
$ brew untap neuxdotdev/tap
```

## Next Steps

Once RCGen is installed, see:

- [Configuration](./configuration.md) to configure RCGen
- [Home](../home.md) for usage examples
