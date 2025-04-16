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

## Prior Art

[bc(1)](https://linux.die.net/man/1/bc) - An arbitrary precision calculator
language
