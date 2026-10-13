# Installation

Install val with a package manager, or download a pre-built binary.

## Cargo

With [Rust and Cargo](https://www.rust-lang.org/tools/install) installed, run:

```sh
cargo install val
```

Make sure Cargo's binary directory is on your `PATH`.

## Homebrew

With [Homebrew](https://brew.sh) installed, run:

```sh
brew install terror/tap/val
```

## Pre-built binaries

Download the archive for your operating system and architecture from the
[releases page](https://github.com/terror/val/releases). Binaries are available
for Linux, macOS, and Windows. Extract the archive and place the executable in a
directory on your `PATH`.

## First calculation

Evaluate an expression directly from your shell:

```console
$ val -e '0.1 + 0.2'
0.3
```

Run `val` without arguments to start the interactive REPL. Continue with the
[command-line guide](usage.md) or the [language guide](language.md).
