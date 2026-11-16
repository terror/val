## val

[![release](https://img.shields.io/github/release/terror/val.svg?label=release&style=flat&labelColor=1d1d1d&color=424242&logo=github)](https://github.com/terror/val/releases/latest)
[![crates.io](https://img.shields.io/crates/v/val.svg?style=flat&labelColor=1d1d1d&color=424242&logo=rust)](https://crates.io/crates/val)
[![build](https://img.shields.io/github/actions/workflow/status/terror/val/ci.yaml?branch=master&style=flat&labelColor=1d1d1d&color=424242&logo=GitHub%20Actions&logoColor=white&label=build)](https://github.com/terror/val/actions/workflows/ci.yaml)
[![codecov](https://img.shields.io/codecov/c/gh/terror/val?style=flat&labelColor=1d1d1d&color=424242&logo=Codecov&logoColor=white)](https://codecov.io/gh/terror/val)
[![docs.rs](https://img.shields.io/docsrs/val?style=flat&labelColor=1d1d1d&color=424242&logo=docs.rs&logoColor=white&label=docs.rs)](https://docs.rs/val)
[![dependency status](https://img.shields.io/endpoint?url=https%3A%2F%2Fdeps.rs%2Frepo%2Fgithub%2Fterror%2Fval%2Fshield.json&style=flat&labelColor=1d1d1d&color=424242&logo=rust&logoColor=white)](https://deps.rs/repo/github/terror/val)

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
