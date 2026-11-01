# conva

A simple, fast command-line tool for zipping and unzipping files and folders. Run it from any directory in your terminal.

Built with Rust.

```bash
conva zip my-folder
conva unzip my-folder.zip
```

> Replace the examples above with your exact command syntax once you confirm it (run `conva --help`).

---

## Features

- Zip a single file or an entire folder with one command
- Unzip archives back into files and folders
- Recursively walks directories, so nested folders are included
- Deflate and Deflate64 compression support
- Works from any folder on your machine
- Single small binary with no runtime dependencies

> Add or remove items to match what the tool actually does today.

---

## Installation

Pick whichever method suits you. After installing, open a **new terminal** and run `conva --version` to confirm it works.

### Option 1: Install script (no Rust needed)

**macOS / Linux**

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/<YOUR_GITHUB_USERNAME>/conva/releases/latest/download/conva-installer.sh | sh
```

**Windows (PowerShell)**

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/<YOUR_GITHUB_USERNAME>/conva/releases/latest/download/conva-installer.ps1 | iex"
```

### Option 2: Download a prebuilt binary

1. Go to the [Releases page](https://github.com/<YOUR_GITHUB_USERNAME>/conva/releases).
2. Download the file for your system:
   - Windows: `conva-x86_64-pc-windows-msvc.zip`
   - macOS (Apple Silicon): `conva-aarch64-apple-darwin.tar.xz`
   - macOS (Intel): `conva-x86_64-apple-darwin.tar.xz`
   - Linux: `conva-x86_64-unknown-linux-gnu.tar.xz`
3. Extract it and move the `conva` binary to a folder on your `PATH`.

> File names depend on your cargo-dist setup. Check your actual release assets and update this list.

### Option 3: With Cargo (requires Rust)

From GitHub:

```bash
cargo install --git https://github.com/<YOUR_GITHUB_USERNAME>/conva
```

From crates.io (once published):

```bash
cargo install conva
```

### Option 4: Build from source

```bash
git clone https://github.com/<YOUR_GITHUB_USERNAME>/conva.git
cd conva
cargo build --release
```

The binary will be at `target/release/conva`. Copy it somewhere on your `PATH`.

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

> TODO: paste the real output of `conva --help` here, and document any extra flags (output name, destination folder, compression level, etc.).

---

## Updating

Re-run the install command from the method you used originally. With Cargo:

```bash
cargo install --git https://github.com/<YOUR_GITHUB_USERNAME>/conva --force
```

## Uninstalling

With the install script or manual install, delete the `conva` binary from where you placed it.

With Cargo:

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
git clone https://github.com/<YOUR_GITHUB_USERNAME>/conva.git
cd conva
cargo run -- zip some-folder
cargo test
```

## Releasing (maintainers)

Releases are built with [cargo-dist](https://github.com/axodotdev/cargo-dist) and GitHub Actions.

```bash
# bump the version in Cargo.toml, then:
git commit -am "release v0.1.0"
git tag v0.1.0
git push --tags
```

The workflow builds binaries for every platform and publishes a GitHub release with install scripts attached.

---

## Contributing

Issues and pull requests are welcome. For larger changes, please open an issue first to discuss what you'd like to change.

## License

[MIT](LICENSE)

> Add a `LICENSE` file to the repo root to match.

## Author

Made by Khandaker Shahibul Hasan ([@KshSiaan](https://github.com/KshSiaan)).