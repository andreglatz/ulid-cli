# ULID CLI

A simple command-line tool for generating ULIDs (Universally Unique Lexicographically Sortable Identifiers) written in Rust.

## What is a ULID?

ULID stands for Universally Unique Lexicographically Sortable Identifier. ULIDs are a great alternative to UUIDs with several advantages:

- **128-bit compatibility** with UUID
- **1.21e+24 unique ULIDs per millisecond**
- **Lexicographically sortable**
- **Canonically encoded as a 26 character string**
- **Uses Crockford's base32** for better efficiency and readability
- **Case insensitive**
- **No special characters** (URL safe)
- **Monotonic sort order** (correctly detects and handles the same millisecond)

## Installation

### From Source

Make sure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

```bash
git clone https://github.com/yourusername/ulid-cli.git
cd ulid-cli
cargo build --release
```

The compiled binary will be available at `target/release/ulid-cli`.

### Using Cargo

```bash
cargo install --path .
```

## Usage

### Basic Usage

Generate a new ULID:

```bash
ulid-cli
```

Output:

```
01ARZ3NDEKTSV4RRFFQ69G5FAV
```

### Copy to Clipboard

Generate a ULID and automatically copy it to your clipboard:

```bash
ulid-cli --clipboard
```

or

```bash
ulid-cli -c
```

Output:

```
01ARZ3NDEKTSV4RRFFQ69G5FAV
Copied to clipboard!
```

### Help

View all available options:

```bash
ulid-cli --help
```

## Features

- ✅ Generate cryptographically strong ULIDs
- ✅ Copy generated ULIDs to clipboard
- ✅ Simple and fast CLI interface
- ✅ Cross-platform support (Windows, macOS, Linux)

## Dependencies

This project uses the following Rust crates:

- [`ulid`](https://crates.io/crates/ulid) - ULID generation
- [`clap`](https://crates.io/crates/clap) - Command-line argument parsing
- [`arboard`](https://crates.io/crates/arboard) - Cross-platform clipboard access
- [`colored`](https://crates.io/crates/colored) - Terminal text coloring

## Examples

### Shell Scripting

You can use this tool in shell scripts:

```bash
#!/bin/bash
ID=$(ulid-cli)
echo "Generated ID: $ID"
```

### Generate Multiple ULIDs

```bash
for i in {1..5}; do ulid-cli; done
```

## ULID Format

ULIDs are composed of:

```
 01AN4Z07BY      79KA1307SR9X4MV3
|----------|    |----------------|
 Timestamp          Randomness
   48bits             80bits
```

- **Timestamp**: 48-bit integer, UNIX-time in milliseconds
- **Randomness**: 80-bits of randomness

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [ULID Specification](https://github.com/ulid/spec) for the ULID standard
- The Rust ULID crate maintainers for the excellent implementation
