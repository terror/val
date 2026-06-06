// Copyright © 2017–2026 Trevor Spiteri

// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License and a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.

/*!
# Rust low-level bindings for GMP, MPFR and MPC

The gmp-mpfr-sys crate provides Rust FFI bindings to the following
[GNU] arbitrary-precision libraries:

  * [GMP] for integers and rational numbers,
  * [MPFR] for floating-point numbers, and
  * [MPC] for complex numbers.

The source of the three libraries is included in the package.

The gmp-mpfr-sys crate is free software: you can redistribute it
and/or modify it under the terms of the GNU Lesser General Public
License as published by the Free Software Foundation, either version 3
of the License, or (at your option) any later version. See the full
text of the [GNU LGPL] and [GNU GPL] for details.

## Basic features

This crate contains three modules:

  * [`gmp`] provides external FFI bindings to [GMP].
  * [`mpfr`] provides external FFI bindings to [MPFR].
  * [`mpc`] provides external FFI bindings to [MPC].

The versions provided by this crate release are [GMP] version 6.3.0, [MPFR]
version 4.2.2, and [MPC] version 1.4.1.

If you want a high-level API, consider using [Rug][rug crate], a crate
which provides integers and floating-point numbers with arbitrary
precision and correct rounding:

  * [`Integer`] is a bignum integer with arbitrary precision.
  * [`Rational`] is a bignum rational number with arbitrary precision.
  * [`Float`] is a multi-precision floating-point number with correct
    rounding.
  * [`Complex`] is a multi-precision complex number with correct
    rounding.

### Name prefixes

Since modules and enumerated types provide namespacing, most prefixes
in the C names are removed. However, when the prefix is not a whole
word it is not removed. For example [`mp_set_memory_functions`]
becomes [`gmp::set_memory_functions`], but [`mpz_init`] becomes
[`gmp::mpz_init`] not `gmp::z_init`, and [`MPFR_RNDN`] in
[`enum MPFR_RND_T`] becomes [`mpfr::rnd_t::RNDN`] not
`mpfr::rnd_t::N`. Also, the types [`mpfr::mpfr_t`] and [`mpc::mpc_t`]
are *not* shortened to `mpfr::t` or `mpc::t`.

### Types

Unlike in the C libraries, the types [`gmp::mpz_t`], [`gmp::mpq_t`],
[`gmp::mpf_t`], [`gmp::randstate_t`], [`mpfr::mpfr_t`] and
[`mpc::mpc_t`] are defined directly as structs, not as single-element
arrays.

### Undocumented or obsolete functions

The bindings do not cover undocumented or obsolete functions and
macros.

## Using gmp-mpfr-sys

The gmp-mpfr-sys crate is available on [crates.io][sys crate]. To use
gmp-mpfr-sys in your crate, add it as a dependency inside
[*Cargo.toml*]:

```toml
[dependencies]
gmp-mpfr-sys = "1.7"
```

This crate requires rustc version 1.71.0 or later.

If the C libraries have a major version bump with some deprecated
functions removed, but no features are removed in the Rust bindings,
then gmp-mpfr-sys will have a minor version bump rather than a major
version bump. This allows more compatiblity across crates that use the
Rust bindings but do not use the C libraries directly.

If on the other hand a dependent crate makes use of internal
implementation details, or includes a C library that directly uses the
header (*.h*) and library (*.a*) files built using C, it can be a good
idea to depend on version `"~1.7"` instead of version `"1.7"` in order
to ensure backwards compatibility at the C level as well.

## Optional features

The gmp-mpfr-sys crate has two optional features:

 1. `mpfr`, enabled by default. Required to include the [MPFR]
    library.
 2. `mpc`, enabled by default. Required to include the [MPC] library.
    This feature requires the `mpfr` feature.

The [GMP] library is always included.

The two optional features are enabled by default; to use features
selectively, you can add the dependency like this to [*Cargo.toml*]:

```toml
[dependencies.gmp-mpfr-sys]
version = "1.7"
default-features = false
features = ["mpfr"]
```

Here only the `mpfr` feature is selected.

## Experimental optional features

It is not considered a breaking change if experimental features are
removed. The removal of experimental features would however require a
minor version bump.

Experimental features may also not work on all platforms.

There are three experimental features:

 1. `use-system-libs`, disabled by default. Using this feature, the
    system libraries for [GMP], and [MPFR] and [MPC] if enabled, will
    be used instead of building them from source. The major versions
    of the system libraries must be equal to those provided by the
    crate, and the minor versions of the system libraries must be
    greater or equal to those provided by the crate. There are no
    restriction on the patch version.
 2. `force-cross`, disabled by default. Without this feature, the
    build will fail if cross compilation is detected, because cross
    compilation is not tested or supported and can lead to silent
    failures that are hard to debug, especially if this crate is an
    indirect dependency. As an exception, cross compiling from x86_64
    to i686 does not need this feature. (Compiling on MinGW does not
    have this exception because MinGW does not support cross
    compilation from 64-bit to 32-bit.)
 3. `c-no-tests`, disabled by default. Using this feature will skip
    testing the C libraries. This is not advised; the risk that the
    GMP sources are miscompiled is unfortunately quite high. And if
    they indeed are miscompiled, the tests are very likely to trigger
    the compiler-introduced bug.

## Metadata

The gmp-mpfr-sys crate passes some metadata to its dependents:

 1. `DEP_GMP_LIMB_BITS` contains the number of bits per limb, which is
    32 or 64.
 2. `DEP_GMP_OUT_DIR` contains the path of a directory that contains
    two subdirectories: the first subdirectory is named *lib* and
    contains the generated library (*.a*) files, and the second
    subdirectory is named *include* and contains the corresponding
    header (*.h*) files.
 3. `DEP_GMP_LIB_DIR` contains the path of the *lib* subdirectory of
    the `DEP_GMP_OUT_DIR` directory.
 4. `DEP_GMP_INCLUDE_DIR` contains the path of the *include*
    subdirectory of the `DEP_GMP_OUT_DIR` directory.

A dependent crate can use these environment variables in its build
script.

## Building on GNU/Linux

**Warning:** The build system does not support building in paths that contain
spaces.

To build on GNU/Linux, simply make sure you have `diffutils`, `gcc`,
`m4` and `make` installed on your system. For example on Fedora:

```sh
sudo dnf install diffutils gcc m4 make
```

Note that you can use Clang instead of GCC by installing `clang` and setting the
environment variable `CC=clang` before building the crate.

## Building on macOS

**Warning:** The build system does not support building in paths that contain
spaces.

To build on macOS, you need the command-line developer tools. To
install them, run the following command in a terminal:

```sh
xcode-select --install
```

## Building on Windows

**Warning:** The build system does not support building in paths that contain
spaces.

You can build on Windows with the Rust GNU toolchain and an up-to-date
MSYS2 installation.

To install MSYS2:

 1. Install MSYS2 using the [installer][msys].

 2. Launch the MSYS2 MinGW 64-bit terminal from the start menu.

 3. Install the required tools.

    ```sh
    pacman -S pacman-mirrors
    pacman -S diffutils m4 make mingw-w64-x86_64-gcc
    ```

Then, to build a crate with a dependency on this crate:

 1. Launch the MSYS2 MinGW 64-bit terminal from the start menu.

 2. Change to the crate directory.

 3. Build the crate using `cargo`.

Note that you can use Clang instead of GCC by installing
`mingw-w64-x86_64-clang` and setting the environment variable `CC=clang` before
building the crate.

## Cross compilation

While some cross compilation is possible, it is not tested
automatically, and may not work. Merge requests that improve cross
compilation are accepted.

The experimental feature `force-cross` must be enabled for cross
compilation. There is one case which is allowed even without the
feature: when the only difference between host and target is that the
host is x86_64 and the target is i686.

## Caching the built C libraries

Building the C libraries can take some time. In order to save
compilation time, the built libraries are cached in the user’s cache
directory as follows:

  * on GNU/Linux: inside `$XDG_CACHE_HOME/gmp-mpfr-sys` or
    `$HOME/.cache/gmp-mpfr-sys`
  * on macOS: inside `$HOME/Library/Caches/gmp-mpfr-sys`
  * on Windows: inside `{FOLDERID_LocalAppData}\gmp-mpfr-sys`

To use a different directory, you can set the environment variable
`GMP_MPFR_SYS_CACHE` to the desired cache directory. Setting the
`GMP_MPFR_SYS_CACHE` variable to an empty string or to a single
underscore (`"_"`) will disable caching.

[*Cargo.toml*]: https://doc.rust-lang.org/cargo/guide/dependencies.html
[GMP]: https://gmplib.org/
[GNU GPL]: https://www.gnu.org/licenses/gpl-3.0.html
[GNU LGPL]: https://www.gnu.org/licenses/lgpl-3.0.en.html
[GNU]: https://www.gnu.org/
[MPC]: https://www.multiprecision.org/
[MPFR]: https://www.mpfr.org/
[`Complex`]: https://docs.rs/rug/latest/rug/struct.Complex.html
[`Float`]: https://docs.rs/rug/latest/rug/struct.Float.html
[`Integer`]: https://docs.rs/rug/latest/rug/struct.Integer.html
[`MPFR_RNDN`]: C/MPFR/constant.MPFR_Basics.html#Rounding-Modes
[`Rational`]: https://docs.rs/rug/latest/rug/struct.Rational.html
[`enum MPFR_RND_T`]: C/MPFR/constant.MPFR_Basics.html#index-mpfr_005frnd_005ft
[`mp_set_memory_functions`]: C/GMP/constant.Custom_Allocation.html#index-mp_005fset_005fmemory_005ffunctions
[`mpz_init`]: C/GMP/constant.Integer_Functions.html#index-mpz_005finit
[msys]: https://www.msys2.org/
[rug crate]: https://crates.io/crates/rug
[sys crate]: https://crates.io/crates/gmp-mpfr-sys
*/
#![no_std]
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/gmp-mpfr-sys/~1.7")]
#![doc(html_logo_url = "data:image/svg+xml;base64,
PHN2ZyB3aWR0aD0iMTI4IiBoZWlnaHQ9IjEyOCIgdmVyc2lvbj0iMS4xIiB2aWV3Qm94PSIwIDAgMzMuODY3IDMzLjg2NyIgeG1s
bnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48ZyB0cmFuc2Zvcm09InRyYW5zbGF0ZSgwIC0yNjMuMTMpIj48Y2lyY2xl
IGN4PSIxNi45MzMiIGN5PSIyODAuMDciIHI9IjE2LjkzMyIgZmlsbD0iI2Y3ZjFhMSIvPjxnIGZpbGw9IiMwMDcyYjIiIHN0cm9r
ZS13aWR0aD0iLjI2NDU4cHgiPjxnIHN0cm9rZT0iIzAwMCI+PGcgYXJpYS1sYWJlbD0iNiI+PHBhdGggZD0ibTE0LjM2MSAyNzgu
NzFjMC42NjA0IDAgMS4yODQxIDAuMjc1MTYgMS4yODQxIDEuMzk0MiAwIDEuMjI5MS0wLjU4NzAyIDEuNjE0My0xLjI0NzQgMS42
MTQzLTAuNTY4NjggMC0xLjI2NTgtMC4zODUyMy0xLjUyMjYtMi4wMTc5IDAuMzY2ODktMC42OTcwOSAwLjkzNTU2LTAuOTkwNiAx
LjQ4NTktMC45OTA2em0wLjExMDA3IDUuMzU2NmMyLjIwMTMgMCA0LjAzNTgtMS40Njc2IDQuMDM1OC0zLjk2MjRzLTEuNTQwOS0z
LjU5NTUtMy41MjIxLTMuNTk1NWMtMC42MjM3MSAwLTEuNjE0MyAwLjQwMzU4LTIuMTgzIDEuMTU1NyAwLjEyODQxLTIuMzQ4MSAx
LjAwODktMy4xMzY5IDIuMTQ2My0zLjEzNjkgMC42NjA0IDAgMS4zOTQyIDAuNDAzNTcgMS43NjExIDAuODA3MTVsMS42NTEtMS44
NzExYy0wLjc3MDQ2LTAuNzcwNDYtMS45ODEyLTEuNDY3Ni0zLjYzMjItMS40Njc2LTIuNDk0OCAwLTQuODA2MiAyLjAxNzktNC44
MDYyIDYuMjM3MSAwIDQuMjE5MiAyLjMxMTQgNS44MzM1IDQuNTQ5NCA1LjgzMzV6IiBzdHJva2U9Im5vbmUiLz48L2c+PGcgdHJh
bnNmb3JtPSJyb3RhdGUoMTUuNTE1KSIgYXJpYS1sYWJlbD0iMiI+PHBhdGggZD0ibTk4LjAyOCAyNjcuOTVoNS4wNDYxdi0xLjM5
OThoLTEuNjAzYy0wLjMyNzM4IDAtMC44MjQwOSAwLjA0NTEtMS4xOTY2IDAuMDkwMyAxLjI3NTYtMS4yNTMxIDIuNDQ5Ny0yLjQy
NzEgMi40NDk3LTMuNzI1MyAwLTEuMzY2LTAuOTU5NTYtMi4yNjkxLTIuMzcwNy0yLjI2OTEtMS4wMDQ3IDAtMS42ODIgMC4zOTUx
MS0yLjM3MDcgMS4xNzRsMC44NjkyNCAwLjg1Nzk2YzAuMzYxMjQtMC4zODM4MyAwLjc1NjM2LTAuNzMzNzggMS4yNzU2LTAuNzMz
NzggMC42MjA4OSAwIDEuMDE2IDAuMzgzODIgMS4wMTYgMS4wODM3IDAgMS4wMDQ3LTEuMzA5NSAyLjI5MTYtMy4xMTU3IDMuODk0
N3oiIGZpbGwtb3BhY2l0eT0iLjk3MjU1IiBzdHJva2U9Im5vbmUiLz48L2c+PGcgdHJhbnNmb3JtPSJyb3RhdGUoLTExLjAzMyki
IGFyaWEtbGFiZWw9IjgiPjxwYXRoIGQ9Im0tMzguOTIgMjkwLjc2YzEuMjc0MiAwIDIuMTIzNy0wLjc0MDgzIDIuMTIzNy0xLjcw
ODggMC0wLjgzOTYyLTAuNTAzNzctMS4zMDM5LTEuMDg2Ni0xLjYyOTh2LTAuMDM5NWMwLjQwNDk5LTAuMjk2MzMgMC44Mzk2MS0w
LjgxOTg2IDAuODM5NjEtMS40NDIyIDAtMS4wMTc0LTAuNzIxMDgtMS42OTktMS44NDcxLTEuNjk5LTEuMDg2NiAwLTEuODk2NSAw
LjY1MTkzLTEuODk2NSAxLjY2OTMgMCAwLjY2MTgxIDAuMzg1MjMgMS4xMjYxIDAuODY5MjQgMS40NzE4djAuMDM5NWMtMC41OTI2
NyAwLjMxNjA5LTEuMTM1OSAwLjgyOTc0LTEuMTM1OSAxLjYxMDEgMCAxLjAxNzQgMC45MDg3NiAxLjcyODYgMi4xMzM2IDEuNzI4
NnptMC40MTQ4Ny0zLjY2NDdjLTAuNzAxMzItMC4yNzY1Ny0xLjI2NDQtMC41NTMxNS0xLjI2NDQtMS4xODUzIDAtMC41NDMyOCAw
LjM3NTM2LTAuODU5MzcgMC44NTkzNy0wLjg1OTM3IDAuNTgyNzkgMCAwLjkyODUxIDAuNDA0OTkgMC45Mjg1MSAwLjk1ODE1IDAg
MC4zOTUxMS0wLjE4NzY4IDAuNzUwNzEtMC41MjM1MiAxLjA4NjZ6bS0wLjM5NTExIDIuODU0N2MtMC42NDIwNiAwLTEuMTU1Ny0w
LjQxNDg2LTEuMTU1Ny0xLjAzNzIgMC0wLjQ4NDAxIDAuMjg2NDYtMC44ODkgMC42ODE1Ny0xLjE3NTUgMC44NDk0OSAwLjM0NTcy
IDEuNTExMyAwLjU5MjY3IDEuNTExMyAxLjI3NDIgMCAwLjU4Mjc5LTAuNDM0NjIgMC45MzgzOS0xLjAzNzIgMC45MzgzOXoiIGZp
bGwtb3BhY2l0eT0iLjk0MTE4IiBzdHJva2U9Im5vbmUiLz48L2c+PGcgdHJhbnNmb3JtPSJyb3RhdGUoNi41MDA4KSIgYXJpYS1s
YWJlbD0iMyI+PHBhdGggZD0ibTM5LjMwMiAyODMuNjRjMS4wMzI5IDAgMS44ODgxLTAuNTc1NzMgMS44ODgxLTEuNTU3OSAwLTAu
NzExMi0wLjQ4MjYtMS4xNjg0LTEuMTE3Ni0xLjMzNzd2LTAuMDMzOWMwLjU4NDItMC4yMjg2IDAuOTM5OC0wLjYzNSAwLjkzOTgt
MS4yMzYxIDAtMC45MTQ0LTAuNzExMi0xLjQyMjQtMS43NDQxLTEuNDIyNC0wLjY0MzQ3IDAtMS4xNTk5IDAuMjcwOTMtMS42MTcx
IDAuNjc3MzNsMC40OTk1MyAwLjYwMTE0YzAuMzMwMi0wLjMwNDggMC42NjA0LTAuNTA4IDEuMDgzNy0wLjUwOCAwLjQ5MTA3IDAg
MC43OTU4NyAwLjI3MDkzIDAuNzk1ODcgMC43MTk2NiAwIDAuNDk5NTQtMC4zNDcxMyAwLjg2MzYtMS40MDU1IDAuODYzNnYwLjcx
MTJjMS4yMjc3IDAgMS41ODMzIDAuMzU1NiAxLjU4MzMgMC45MTQ0IDAgMC41MDgtMC40MDY0IDAuODEyODEtMC45OTA2IDAuODEy
ODEtMC41NDE4NyAwLTAuOTU2NzMtMC4yNjI0Ny0xLjI3ODUtMC41OTI2N2wtMC40NjU2NyAwLjYyNjUzYzAuMzgxIDAuNDIzMzQg
MC45NTY3MyAwLjc2MiAxLjgyODggMC43NjJ6IiBmaWxsLW9wYWNpdHk9Ii44Nzg0MyIgc3Ryb2tlPSJub25lIi8+PC9nPjxnIHRy
YW5zZm9ybT0icm90YXRlKDguMzU2KSIgYXJpYS1sYWJlbD0iMSI+PHBhdGggZD0ibTQ2LjQwNSAyNjguOWgzLjI0Mjd2LTAuNzk1
ODdoLTEuMDU4M3YtNC41ODg5aC0wLjcyODEzYy0wLjMzODY3IDAuMjAzMi0wLjcxMTIgMC4zMzg2Ny0xLjI0NDYgMC40NDAyN3Yw
LjYwOTZoMC45OTA2djMuNTM5MWgtMS4yMDIzeiIgZmlsbC1vcGFjaXR5PSIuNzUyOTQiIHN0cm9rZT0ibm9uZSIvPjwvZz48ZyB0
cmFuc2Zvcm09InJvdGF0ZSgxMi44NjEpIiBhcmlhLWxhYmVsPSI4Ij48cGF0aCBkPSJtODUuMDM2IDI2MS42M2MxLjA5MjIgMCAx
LjgyMDMtMC42MzUgMS44MjAzLTEuNDY0NyAwLTAuNzE5NjctMC40MzE4LTEuMTE3Ni0wLjkzMTMzLTEuMzk3di0wLjAzMzljMC4z
NDcxMy0wLjI1NCAwLjcxOTY3LTAuNzAyNzMgMC43MTk2Ny0xLjIzNjEgMC0wLjg3MjA3LTAuNjE4MDctMS40NTYzLTEuNTgzMy0x
LjQ1NjMtMC45MzEzMyAwLTEuNjI1NiAwLjU1ODgtMS42MjU2IDEuNDMwOSAwIDAuNTY3MjYgMC4zMzAyIDAuOTY1MiAwLjc0NTA3
IDEuMjYxNXYwLjAzMzljLTAuNTA4IDAuMjcwOTMtMC45NzM2NyAwLjcxMTItMC45NzM2NyAxLjM4MDEgMCAwLjg3MjA3IDAuNzc4
OTMgMS40ODE3IDEuODI4OCAxLjQ4MTd6bTAuMzU1Ni0zLjE0MTFjLTAuNjAxMTMtMC4yMzcwNy0xLjA4MzctMC40NzQxNC0xLjA4
MzctMS4wMTYgMC0wLjQ2NTY3IDAuMzIxNzMtMC43MzY2IDAuNzM2Ni0wLjczNjYgMC40OTk1MyAwIDAuNzk1ODcgMC4zNDcxMyAw
Ljc5NTg3IDAuODIxMjYgMCAwLjMzODY3LTAuMTYwODcgMC42NDM0Ny0wLjQ0ODczIDAuOTMxMzR6bS0wLjMzODY3IDIuNDQ2OWMt
MC41NTAzMyAwLTAuOTkwNi0wLjM1NTYtMC45OTA2LTAuODg5IDAtMC40MTQ4NiAwLjI0NTUzLTAuNzYyIDAuNTg0Mi0xLjAwNzUg
MC43MjgxMyAwLjI5NjMzIDEuMjk1NCAwLjUwOCAxLjI5NTQgMS4wOTIyIDAgMC40OTk1My0wLjM3MjUzIDAuODA0MzMtMC44ODkg
MC44MDQzM3oiIGZpbGwtb3BhY2l0eT0iLjYyNzQ1IiBzdHJva2U9Im5vbmUiLz48L2c+PGcgdHJhbnNmb3JtPSJyb3RhdGUoNC4z
MDk5KSIgYXJpYS1sYWJlbD0iNSI+PHBhdGggZD0ibTQ2LjM0MSAyODkuNDljMC45OTA2IDAgMS44OTY1LTAuNjc3MzQgMS44OTY1
LTEuODU0MiAwLTEuMTU5OS0wLjc3MDQ3LTEuNjg0OS0xLjY5MzMtMS42ODQ5LTAuMjc5NCAwLTAuNDgyNiAwLjA2NzctMC43MTEy
IDAuMTc3OGwwLjExMDA3LTEuMzAzOWgyLjAzMnYtMC44MjEyN2gtMi44Nzg3bC0wLjE2MDg3IDIuNjU4NSAwLjQ2NTY3IDAuMjk2
MzNjMC4zMjE3My0wLjIxMTY3IDAuNTE2NDctMC4zMDQ4IDAuODYzNi0wLjMwNDggMC41OTI2NyAwIDAuOTkwNiAwLjM2NDA3IDAu
OTkwNiAxLjAwNzUgMCAwLjY1MTk0LTAuNDQwMjcgMS4wMzI5LTEuMDQxNCAxLjAzMjktMC41NDE4NyAwLTAuOTM5OC0wLjI3MDk0
LTEuMjYxNS0wLjU3NTc0bC0wLjQ0ODczIDAuNjI2NTRjMC4zOTc5MyAwLjM5NzkzIDAuOTY1MiAwLjc0NTA3IDEuODM3MyAwLjc0
NTA3eiIgZmlsbC1vcGFjaXR5PSIuNTAxOTYiIHN0cm9rZT0ibm9uZSIvPjwvZz48ZyBmaWxsLW9wYWNpdHk9Ii4zNzY0NyIgYXJp
YS1sYWJlbD0iMyI+PHBhdGggZD0ibTkuODg1OSAyOTMuNDZjMS4wMzI5IDAgMS44ODgxLTAuNTc1NzQgMS44ODgxLTEuNTU3OSAw
LTAuNzExMi0wLjQ4MjYtMS4xNjg0LTEuMTE3Ni0xLjMzNzd2LTAuMDMzOWMwLjU4NDItMC4yMjg2IDAuOTM5OC0wLjYzNSAwLjkz
OTgtMS4yMzYxIDAtMC45MTQ0LTAuNzExMi0xLjQyMjQtMS43NDQxLTEuNDIyNC0wLjY0MzQ3IDAtMS4xNTk5IDAuMjcwOTQtMS42
MTcxIDAuNjc3MzRsMC40OTk1MyAwLjYwMTEzYzAuMzMwMi0wLjMwNDggMC42NjA0LTAuNTA4IDEuMDgzNy0wLjUwOCAwLjQ5MTA3
IDAgMC43OTU4NyAwLjI3MDkzIDAuNzk1ODcgMC43MTk2NyAwIDAuNDk5NTMtMC4zNDcxMyAwLjg2MzYtMS40MDU1IDAuODYzNnYw
LjcxMTJjMS4yMjc3IDAgMS41ODMzIDAuMzU1NiAxLjU4MzMgMC45MTQ0IDAgMC41MDgtMC40MDY0IDAuODEyOC0wLjk5MDYgMC44
MTI4LTAuNTQxODcgMC0wLjk1NjczLTAuMjYyNDctMS4yNzg1LTAuNTkyNjdsLTAuNDY1NjcgMC42MjY1NGMwLjM4MSAwLjQyMzMz
IDAuOTU2NzMgMC43NjIgMS44Mjg4IDAuNzYyeiIgc3Ryb2tlPSJub25lIi8+PC9nPjxnIHRyYW5zZm9ybT0icm90YXRlKC0xMS4z
NTIpIiBhcmlhLWxhYmVsPSIwIj48cGF0aCBkPSJtLTUxLjQ3MSAyNzYuMTdjMS4xMTc2IDAgMS44Mjg4LTAuOTk5MDcgMS44Mjg4
LTIuODE5NCAwLTEuODExOS0wLjcxMTItMi43Njg2LTEuODI4OC0yLjc2ODYtMS4xMTc2IDAtMS44Mjg4IDAuOTQ4MjYtMS44Mjg4
IDIuNzY4NiAwIDEuODIwMyAwLjcxMTIgMi44MTk0IDEuODI4OCAyLjgxOTR6bTAtMC43NjJjLTAuNTE2NDcgMC0wLjg5NzQ3LTAu
NTMzNC0wLjg5NzQ3LTIuMDU3NHMwLjM4MS0yLjAwNjYgMC44OTc0Ny0yLjAwNjZjMC41MjQ5MyAwIDAuODk3NDcgMC40ODI2IDAu
ODk3NDcgMi4wMDY2cy0wLjM3MjUzIDIuMDU3NC0wLjg5NzQ3IDIuMDU3NHoiIGZpbGwtb3BhY2l0eT0iLjI1MDk4IiBzdHJva2U9
Im5vbmUiLz48L2c+PGcgdHJhbnNmb3JtPSJyb3RhdGUoMjIuNTA2KSIgYXJpYS1sYWJlbD0iNyI+PHBhdGggZD0ibTExOC4xNSAy
NDMuMDhoMC45OTA2YzAuMDkzMS0yLjA5OTcgMC4zNDcxMy0zLjIwODkgMS42NDI1LTQuNzkyMXYtMC41OTI2NmgtMy42MTUzdjAu
ODIxMjZoMi41NTY5Yy0xLjA3NTMgMS40NjQ3LTEuNDczMiAyLjY1MDEtMS41NzQ4IDQuNTYzNXoiIGZpbGwtb3BhY2l0eT0iLjEy
NTQ5IiBzdHJva2U9Im5vbmUiLz48L2c+PGcgdHJhbnNmb3JtPSJyb3RhdGUoLTkuNzI3MykiIGFyaWEtbGFiZWw9IjEiPjxwYXRo
IGQ9Im0tMTguMjk5IDI4Mi43OWgzLjI0Mjd2LTAuNzk1ODdoLTEuMDU4M3YtNC41ODg5aC0wLjcyODEzYy0wLjMzODY3IDAuMjAz
Mi0wLjcxMTIgMC4zMzg2Ni0xLjI0NDYgMC40NDAyNnYwLjYwOTZoMC45OTA2djMuNTM5MWgtMS4yMDIzeiIgZmlsbC1vcGFjaXR5
PSIuMDYyNzQ1IiBzdHJva2U9Im5vbmUiLz48L2c+PC9nPjxnIGFyaWEtbGFiZWw9Ii4iPjxwYXRoIGQ9Im0yMC45MiAyODMuOThj
MC42NTQ3NiAwIDEuMTI4OS0wLjUxOTI5IDEuMTI4OS0xLjE3NCAwLTAuNjU0NzYtMC40NzQxMy0xLjE3NC0xLjEyODktMS4xNzQt
MC42NTQ3NiAwLTEuMTI4OSAwLjUxOTI5LTEuMTI4OSAxLjE3NCAwIDAuNjU0NzUgMC40NzQxMyAxLjE3NCAxLjEyODkgMS4xNzR6
Ii8+PC9nPjwvZz48L2c+PC9zdmc+Cg==
")]
#![doc(test(attr(deny(warnings))))]
#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![warn(unsafe_op_in_unsafe_fn)]
#![allow(clippy::missing_safety_doc, clippy::useless_conversion)]

pub mod gmp;
#[cfg(feature = "mpc")]
pub mod mpc;
#[cfg(feature = "mpfr")]
pub mod mpfr;

pub mod C;

#[cfg(test)]
mod tests {
    use core::ffi::c_char;
    use core::slice;
    use core::str;

    pub unsafe fn str_from_cstr<'a>(cstr: *const c_char) -> &'a str {
        let s = unsafe { slice::from_raw_parts(cstr.cast(), libc::strlen(cstr)) };
        str::from_utf8(s).expect("version not utf8")
    }
}
