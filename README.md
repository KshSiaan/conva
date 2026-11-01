# conva

[![Release](https://img.shields.io/github/v/release/KshSiaan/conva)](https://github.com/KshSiaan/conva/releases)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A simple, fast command-line tool for zipping and unzipping files and folders. Run it from any directory in your terminal.

Built with Rust.

```bash
conva zip my-folder
conva unzip my-folder.zip
```

---

## Features

- Zip a single file or an entire folder with one command
- Unzip archives back into files and folders
- Recursively walks directories, so nested folders are included
- Deflate and Deflate64 compression support
- Works from any folder on your machine
- Single small binary with no runtime dependencies
- Prebuilt for Windows, macOS (Intel and Apple Silicon), and Linux (x86_64 and ARM64)

---

## Installation

Pick whichever method suits you. After installing, open a **new terminal** and run `conva --version` to confirm it works.

### Option 1: Install script (no Rust needed)

**macOS / Linux**

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/KshSiaan/conva/releases/latest/download/conva-installer.sh | sh
```

**Windows (PowerShell)**

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/KshSiaan/conva/releases/latest/download/conva-installer.ps1 | iex"
```

The script installs `conva` into `~/.cargo/bin` (`%USERPROFILE%\.cargo\bin` on Windows) and adds that folder to your `PATH` if needed, so the command works from any directory.

### Option 2: Download a prebuilt binary

1. Go to the [Releases page](https://github.com/KshSiaan/conva/releases/latest).
2. Download the file for your system:

   | Platform | File |
   | --- | --- |
   | Windows (x64) | `conva-x86_64-pc-windows-msvc.zip` |
   | macOS (Apple Silicon) | `conva-aarch64-apple-darwin.tar.xz` |
   | macOS (Intel) | `conva-x86_64-apple-darwin.tar.xz` |
   | Linux (x64) | `conva-x86_64-unknown-linux-gnu.tar.xz` |
   | Linux (ARM64) | `conva-aarch64-unknown-linux-gnu.tar.xz` |

3. Extract it and move the `conva` binary to a folder on your `PATH`.

Each release also includes a `sha256.sum` file so you can verify your download.

### Option 3: With Cargo (requires Rust)

```bash
cargo install --git https://github.com/KshSiaan/conva
```

### Option 4: Build from source

```bash
git clone https://github.com/KshSiaan/conva.git
cd conva
cargo build --release
```

The binary will be at `target/release/conva`. Copy it somewhere on your `PATH`.

Requires Rust 1.85 or newer.

---

## Usage

```text
conva <COMMAND> [OPTIONS]
```

### Zip a file or folder

```bash
conva zip <FILE_OR_FOLDER>
```

Example:

```bash
conva zip photos
# creates photos.zip
```

### Unzip an archive

```bash
conva unzip <ARCHIVE.zip>
```

Example:

```bash
conva unzip photos.zip
```

### Help

```bash
conva --help
conva zip --help
conva unzip --help
```

---

## Updating

Re-run the install script from Option 1 to get the latest release. If you installed with Cargo:

```bash
cargo install --git https://github.com/KshSiaan/conva --force
```

## Uninstalling

If you used the install script or a manual download, delete the `conva` binary from where it was placed (for example `~/.cargo/bin/conva`).

If you installed with Cargo:

```bash
cargo uninstall conva
```

---

## Built with

- [clap](https://crates.io/crates/clap) for command-line parsing
- [zip](https://crates.io/crates/zip) for reading and writing zip archives
- [walkdir](https://crates.io/crates/walkdir) for recursive directory traversal

## Development

```bash
git clone https://github.com/KshSiaan/conva.git
cd conva
cargo run -- zip some-folder
```

## Releasing (maintainers)

Releases are built with [dist](https://github.com/axodotdev/cargo-dist) and GitHub Actions.

```bash
# 1. Bump the version in Cargo.toml
# 2. Commit the change
git commit -am "release v0.1.0"
# 3. Tag it (the tag must match the version) and push
git tag v0.1.0
git push && git push --tags
```

The workflow builds binaries for every platform and publishes a GitHub release with the install scripts attached.

---

## Contributing

Issues and pull requests are welcome. For larger changes, please open an issue first to discuss what you'd like to change.

## License

Released under the [MIT License](LICENSE).

## Author

Made by Khandaker Shahibul Hasan ([@KshSiaan](https://github.com/KshSiaan)).