## val

[![release](https://img.shields.io/github/release/terror/val.svg?label=release&style=flat&labelColor=282c34&logo=github)](https://github.com/terror/val/releases/latest)
[![crates.io](https://shields.io/crates/v/val.svg)](https://crates.io/crates/val)
[![CI](https://github.com/terror/val/actions/workflows/ci.yaml/badge.svg)](https://github.com/terror/val/actions/workflows/ci.yaml)
[![codecov](https://codecov.io/gh/terror/val/graph/badge.svg)](https://codecov.io/gh/terror/val)
[![docs.rs](https://img.shields.io/docsrs/val)](https://docs.rs/val)
[![dependency status](https://deps.rs/repo/github/terror/val/status.svg)](https://deps.rs/repo/github/terror/val)

**val** (e**val**) is a simple arbitrary precision calculator language built
on top of [**chumsky**](https://github.com/zesterer/chumsky) and
[**ariadne**](https://github.com/zesterer/ariadne).

[Website](https://val-one-phi.vercel.app/) ·
[Documentation](https://val-one-phi.vercel.app/documentation) ·
[Playground](https://val-one-phi.vercel.app/playground)

<img width="1667" alt="val" src="screenshot.png" />

`val` supports exact arithmetic, configurable precision, functions, loops,
lists, and an interactive REPL.

## Installation

The easiest way to install it is by using
[cargo](https://doc.rust-lang.org/cargo/index.html), the Rust package manager:

```bash
cargo install val
```

See the [installation guide](docs/installation.md) for other package managers
and pre-built binaries.

## Usage

See the documentation for [command-line usage](docs/usage.md),
[the language](docs/language.md), [numbers and precision](docs/numbers.md),
[built-in functions and constants](docs/builtins.md), and
[the playground](docs/playground.md).

See [examples](examples) for complete programs and the [grammar](GRAMMAR.txt)
for the language syntax.

## Prior Art

[bc(1)](https://linux.die.net/man/1/bc) - An arbitrary precision calculator
language
