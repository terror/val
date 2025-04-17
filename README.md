## val

[![release](https://img.shields.io/github/release/terror/val.svg?label=release&style=flat&labelColor=282c34&logo=github)](https://github.com/terror/val/releases/latest)
[![crates.io](https://shields.io/crates/v/val.svg)](https://crates.io/crates/val)
[![CI](https://github.com/terror/val/actions/workflows/ci.yaml/badge.svg)](https://github.com/terror/val/actions/workflows/ci.yaml)
[![docs.rs](https://img.shields.io/docsrs/val)](https://docs.rs/val)
[![dependency status](https://deps.rs/repo/github/terror/val/status.svg)](https://deps.rs/repo/github/terror/val)

**val** (e**val**) is a simple arbitrary precision calculator language built
on top of [**chumsky**](https://github.com/zesterer/chumsky) and
[**ariadne**](https://github.com/zesterer/ariadne).

<img width="1667" alt="Screenshot 2025-04-16 at 1 57 23 AM" src="https://github.com/user-attachments/assets/c295e572-90fa-4b33-ace0-baad1ead64fd" />

## Installation

`val` should run on any system, including Linux, MacOS, and the BSDs.

The easiest way to install it is by using [cargo](https://doc.rust-lang.org/cargo/index.html),
the Rust package manager:

```bash
cargo install val
```

### Pre-built binaries

Pre-built binaries for Linux, MacOS, and Windows can be found on [the releases
page](https://github.com/terror/val/releases).

## Usage

The primary way to use **val** is via the provided command-line interface. There
is currently ongoing work on a Rust library and web playground, which will
provide a few extra ways to interact with the runtime.

Below is the output of `val --help`, which describes some of the
arguments/options we support:

```present cargo run -- --help
Usage: val [OPTIONS] [FILENAME]

Arguments:
  [FILENAME]

Options:
  -e, --expression <EXPRESSION>
  -h, --help                     Print help
  -V, --version                  Print version
```

Running **val** on its own will spawn a repl (read–eval–print loop) environment,
where you can evaluate arbitrary **val** code and see its output immediately. We
use [rustyline](https://github.com/kkawakam/rustyline) for its implementation,
and we support a few quality of life features:

- Syntax highlighting (see image above)
- Persistent command history
- Emacs-style editing support by default
- Filename completions
- Hints (virtual text pulled from history)

The **val** language supports not only expressions, but quite a few
[statements](https://github.com/terror/val/blob/ea0c163934ee3f4afe118384b1281d296f116539/src/ast.rs#L35) as well.
You may want to save **val** programs and execute them later, so
the command-line interface provides a way to evaluate entire files.

For instance, lets say you have the following **val** program at
`factorial.val`:

```rust
fn factorial(n) {
  if (n <= 1) {
    return 1
  } else {
    return n * factorial(n - 1)
  }
}

println(factorial(5));
```

You can execute this program by running `val factorial.val`, which will write to
standard output `120`.

Lastly, you may want to evaluate a val expression and use it within another
program. The tool supports executing arbitrary expressions inline using the
`--expression` or `-e` option:

```bash
val -e 'sin(2) * e ^ pi * cos(sum([1, 2, 3]))'
16.481455793912883
```

**n.b.** The `--expression` option and `filename` argument are mutually
exclusive.

## Features

This section describes some of the language features **val** implements in
detail, and should serve as a guide to anyone wanting to write a **val**
program.

### Statements

**val** supports a few statement constructs

## Prior Art

[bc(1)](https://linux.die.net/man/1/bc) - An arbitrary precision calculator
language
