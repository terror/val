// Copyright © 2016–2026 Trevor Spiteri

// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU Lesser General Public License and
// a copy of the GNU General Public License along with this program. If not, see
// <https://www.gnu.org/licenses/>.

/*!
# Arbitrary-precision numbers

Rug provides integers and floating-point numbers with arbitrary precision and
correct rounding:

  * [`Integer`] is a bignum integer with arbitrary precision,
  * [`Rational`] is a bignum rational number with arbitrary precision,
  * [`Float`] is a multi-precision floating-point number with correct rounding,
    and
  * [`Complex`] is a multi-precision complex number with correct rounding.

Rug is a high-level interface to the following [GNU] libraries:

  * [GMP] for integers and rational numbers,
  * [MPFR] for floating-point numbers, and
  * [MPC] for complex numbers.

Rug is free software: you can redistribute it and/or modify it under the terms
of the GNU Lesser General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version. See the full text of the [GNU LGPL] and [GNU GPL] for details.

You are also free to use the examples in this documentation without any
restrictions; the examples are in the public domain.

## Quick example

```rust
# #[cfg(feature = "integer")] {
use rug::{Assign, Integer};
let mut int = Integer::new();
assert_eq!(int, 0);
int.assign(14);
assert_eq!(int, 14);

let decimal = "98_765_432_109_876_543_210";
int.assign(Integer::parse(decimal).unwrap());
assert!(int > 100_000_000);

let hex_160 = "ffff0000ffff0000ffff0000ffff0000ffff0000";
int.assign(Integer::parse_radix(hex_160, 16).unwrap());
assert_eq!(int.significant_bits(), 160);
int = (int >> 128) - 1;
assert_eq!(int, 0xfffe_ffff_u32);
# }
```

  * <code>[Integer]::[new][Integer::new]</code> creates a new [`Integer`]
    intialized to zero.
  * To assign values to Rug types, we use the [`Assign`] trait and its method
    [`Assign::assign`]. We do not use the [assignment operator `=`][assignment]
    as that would drop the left-hand-side operand and replace it with a
    right-hand-side operand of the same type, which is not what we want here.
  * Arbitrary precision numbers can hold numbers that are too large to fit in a
    primitive type. To assign such a number to the large types, we use strings
    rather than primitives; in the example this is done using
    <code>[Integer]::[parse][Integer::parse]</code> and
    <code>[Integer]::[parse\_radix][Integer::parse_radix]</code>.
  * We can compare Rug types to primitive types or to other Rug types using the
    normal comparison operators, for example `int > 100_000_000`.
  * Most arithmetic operations are supported with Rug types and primitive types
    on either side of the operator, for example `int >> 128`.

## Using with primitive types

With Rust primitive types, arithmetic operators usually operate on two values of
the same type, for example `12i32 + 5i32`. Unlike primitive types, conversion to
and from Rug types can be expensive, so the arithmetic operators are overloaded
to work on many combinations of Rug types and primitives. The following are
provided:

 1. Where they make sense, all arithmetic operators are overloaded to work with
    Rug types and the primitives [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], [`f32`] and [`f64`].
 2. Where they make sense, conversions using the [`From`] trait and assignments
    using the [`Assign`] trait are supported for all the primitives in 1 above
    as well as [`bool`], [`isize`] and [`usize`].
 3. Comparisons between Rug types and all the numeric primitives listed in 1 and
    2 above are supported.
 4. For [`Rational`] numbers, conversions and comparisons are also supported for
    tuples containing two integer primitives: the first is the numerator and the
    second is the denominator which must not be zero. The two primitives do not
    need to be of the same type.
 5. For [`Complex`] numbers, conversions and comparisons are also supported for
    tuples containing two primitives: the first is the real part and the second
    is the imaginary part. The two primitives do not need to be of the same
    type.

## Operators

Operators are overloaded to work on Rug types alone or on a combination of Rug
types and Rust primitives. When at least one operand is an owned value of a Rug
type, the operation will consume that value and return a value of the Rug type.
For example

```rust
# #[cfg(feature = "integer")] {
use rug::Integer;
let a = Integer::from(10);
let b = 5 - a;
assert_eq!(b, 5 - 10);
# }
```

Here `a` is consumed by the subtraction, and `b` is an owned [`Integer`].

If on the other hand there are no owned Rug types and there are references
instead, the returned value is not the final value, but an
incomplete-computation value. For example

```rust
# #[cfg(feature = "integer")] {
use rug::Integer;
let (a, b) = (Integer::from(10), Integer::from(20));
let incomplete = &a - &b;
// This would fail to compile: assert_eq!(incomplete, -10);
let sub = Integer::from(incomplete);
assert_eq!(sub, -10);
# }
```

Here `a` and `b` are not consumed, and `incomplete` is not the final value. It
still needs to be converted or assigned into an [`Integer`]. This is covered in
more detail in the [*Incomplete-computation values*] section.

### Shifting operations

The left shift `<<` and right shift `>>` operators support shifting by negative
values, for example `a << 5` is equivalent to `a >> -5`.

The shifting operators are also supported for the [`Float`] and [`Complex`]
number types, where they are equivalent to multiplication or division by a power
of two. Only the exponent of the value is affected; the mantissa is unchanged.

### Exponentiation

Exponentiation (raising to a power) does not have a dedicated operator in Rust.
In order to perform exponentiation of Rug types, the [`Pow`][ops::Pow] trait has
to be brought into scope, for example

```rust
# #[cfg(feature = "integer")] {
use rug::ops::Pow;
use rug::Integer;
let base = Integer::from(10);
let power = base.pow(5);
assert_eq!(power, 100_000);
# }
```

### Compound assignments to right-hand-side operands

Traits are provided for compound assignment to right-hand-side operands. This
can be useful for non-commutative operations like subtraction. The names of the
traits and their methods are similar to Rust compound assignment traits, with
the suffix “`Assign`” replaced with “`From`”. For example the counterpart to
[`SubAssign`][core::ops::SubAssign] is [`SubFrom`][ops::SubFrom]:

```rust
# #[cfg(feature = "integer")] {
use rug::ops::SubFrom;
use rug::Integer;
let mut rhs = Integer::from(10);
// set rhs = 100 - rhs
rhs.sub_from(100);
assert_eq!(rhs, 90);
# }
```

## Incomplete-computation values

There are two main reasons why operations like `&a - &b` do not perform a
complete computation and return a Rug type:

 1. Sometimes we need to assign the result to an object that already exists.
    Since Rug types require memory allocations, this can help reduce the number
    of allocations. (While the allocations might not affect performance
    noticeably for computationally intensive functions, they can have a much
    more significant effect on faster functions like addition.)
 2. For the [`Float`] and [`Complex`] number types, we need to know the
    precision when we create a value, and the operation itself does not convey
    information about what precision is desired for the result.

There are two things that can be done with incomplete-computation values:

 1. Assign them to an existing object without unnecessary allocations. This is
    usually achieved using the [`Assign`] trait or a similar method, for example
    <code>int.[assign][Assign::assign]\(incomplete)</code> and
    <code>float.[assign\_round][ops::AssignRound::assign_round]\(incomplete, [Round][float::Round]::[Up][float::Round::Up])</code>.
 2. Convert them to the final value using the [`Complete`] trait, the [`From`]
    trait or a similar method. For example incomplete integers can be completed
    using <code>incomplete.[complete][Complete::complete]\()</code> or
    <code>[Integer]::[from][From::from]\(incomplete)</code>. Incomplete
    floating-point numbers can be completed using
    <code>incomplete.[complete][ops::CompleteRound::complete]\(53)</code> or
    <code>[Float]::[with_val][Float::with_val]\(53, incomplete)</code> since the
    precision has to be specified.

Let us consider a couple of examples.

```rust
# #[cfg(feature = "integer")] {
use rug::{Assign, Integer};
let mut buffer = Integer::new();
// ... buffer can be used and reused ...
let (a, b) = (Integer::from(10), Integer::from(20));
let incomplete = &a - &b;
buffer.assign(incomplete);
assert_eq!(buffer, -10);
# }
```

Here the assignment from `incomplete` into `buffer` does not require an
allocation unless the result does not fit in the current capacity of `buffer`.
If `&a - &b` returned an [`Integer`] instead, then an allocation would take
place even if it is not necessary.

```rust
# #[cfg(feature = "float")] {
use rug::float::Constant;
use rug::Float;
// x has a precision of 10 bits
let x = Float::with_val(10, 180);
// y has a precision of 50 bits
let y = Float::with_val(50, Constant::Pi);
let incomplete = &x / &y;
// z has a precision of 45 bits
let z = Float::with_val(45, incomplete);
assert!(57.295 < z && z < 57.296);
# }
```

The precision to use for the result depends on the requirements of the algorithm
being implemented. Here `z` is created with a precision of 45.

Many operations can return incomplete-computation values, for example

  * unary operators applied to references, for example `-&int`
  * binary operators applied to two references, for example `&int1 + &int2`
  * binary operators applied to a primitive and a reference, for example `&int *
    10`
  * methods that take a reference, for example
    <code>int.[abs\_ref][Integer::abs_ref]\()</code>
  * methods that take two references, for example
    <code>int1.[gcd\_ref][Integer::gcd_ref]\(\&int2)</code>
  * string parsing, for example
    <code>[Integer]::[parse][Integer::parse]\(\"12\")</code>

These operations return objects that can be stored in temporary variables like
`incomplete` in the last few code examples. However, the names of the types are
not public, and consequently, the incomplete-computation values cannot be for
example stored in a struct. If you need to store the value in a struct, complete
it to its final type and value.

## Using Rug

Rug is available on [crates.io][rug crate]. To use Rug in your crate, add it as
a dependency inside [*Cargo.toml*]:

```toml
[dependencies]
rug = "1.30"
```

Rug requires rustc version 1.85.0 or later.

Rug also depends on the [GMP], [MPFR] and [MPC] libraries through the low-level
FFI bindings in the [gmp-mpfr-sys crate][sys crate], which needs some setup to
build; the [gmp-mpfr-sys documentation][sys] has some details on usage under
[GNU/Linux][sys gnu], [macOS][sys mac] and [Windows][sys win].

## Optional features

The Rug crate has six optional features:

 1. `integer`, enabled by default. Required for the [`Integer`] type and its
    supporting features.
 2. `rational`, enabled by default. Required for the [`Rational`] number type
    and its supporting features. This feature requires the `integer` feature.
 3. `float`, enabled by default. Required for the [`Float`] type and its
    supporting features.
 4. `complex`, enabled by default. Required for the [`Complex`] number type and
    its supporting features. This feature requires the `float` feature.
 5. `rand`, enabled by default. Required for the [`RandState`][rand::RandState]
    type and its supporting features. This feature requires the `integer`
    feature.
 6. `std`, enabled by default. This is for features that are not possible under
    `no_std`, such as methods that return [`String`].
 7. `serde`, disabled by default. This provides serialization support for the
    [`Integer`], [`Rational`], [`Float`] and [`Complex`] number types, providing
    that they are enabled. This feature requires the `std` feature and the
    [serde crate].

The first six optional features are enabled by default; to use features
selectively, you can add the dependency like this to [*Cargo.toml*]:

```toml
[dependencies.rug]
version = "1.30"
default-features = false
features = ["integer", "float", "std"]
```

Here only the `integer`, `float` and `rand` features are enabled. If none of the
features are selected, the [gmp-mpfr-sys crate][sys crate] is not required and
thus not enabled. In that case, only the [`Assign`] trait and the traits that
are in the [`ops`] module are provided by the crate.

## Experimental optional features

It is not considered a breaking change if the following experimental features
are removed. The removal of experimental features would however require a minor
version bump. Similarly, on a minor version bump, optional dependencies can be
updated to an incompatible newer version.

 1. `num-traits`, disabled by default. This implements some traits from the
    [*num-traits* crate] and the [*num-integer* crate]. (The plan is to promote
    this to an optional feature once the [*num-traits* crate] and the
    [*num-integer* crate] reach version 1.0.0.)
 2. `num-complex`, disabled by default. This adds conversion methods to and from
    complex floats as provided by the [*num-complex* crate]. (The plan is to
    promote this to an optional feature once the [*num-complex* crate] reaces
    version 1.0.0.)
 3. `nightly-float`, disabled by default. This requires the nightly compiler,
    and implements some operations with the experimental [`f16`] and [`f128`]
    primitives. (The plan is to always implement the operations and remove this
    experimental feature once the primitives are stabilized.)
 4. `borsh`, disabled by default. This provides serialization support using the
    [*borsh* crate]. (The plan is to promote this to an optional feature.)

[*Cargo.toml*]: https://doc.rust-lang.org/cargo/guide/dependencies.html
[*Incomplete-computation values*]: #incomplete-computation-values
[*borsh* crate]: https://crates.io/crates/borsh
[*num-complex* crate]: https://crates.io/crates/num-complex
[*num-integer* crate]: https://crates.io/crates/num-integer
[*num-traits* crate]: https://crates.io/crates/num-traits
[GMP]: https://gmplib.org/
[GNU GPL]: https://www.gnu.org/licenses/gpl-3.0.html
[GNU LGPL]: https://www.gnu.org/licenses/lgpl-3.0.en.html
[GNU]: https://www.gnu.org/
[MPC]: https://www.multiprecision.org/mpc/
[MPFR]: https://www.mpfr.org/
[`f128`]: https://doc.rust-lang.org/nightly/std/primitive.f128.html
[`f16`]: https://doc.rust-lang.org/nightly/std/primitive.f16.html
[assignment]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#assignment-expressions
[rug crate]: https://crates.io/crates/rug
[serde crate]: https://crates.io/crates/serde
[sys crate]: https://crates.io/crates/gmp-mpfr-sys
[sys gnu]: gmp_mpfr_sys#building-on-gnulinux
[sys mac]: gmp_mpfr_sys#building-on-macos
[sys win]: gmp_mpfr_sys#building-on-windows
[sys]: gmp_mpfr_sys
*/
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/rug/~1.30")]
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
#![cfg_attr(feature = "nightly-float", feature(f16, f128))]
#![warn(unsafe_op_in_unsafe_fn)]
// allowed to deal with e.g. 1i32.into(): c_long which can be i32 or i64
#![allow(clippy::useless_conversion)]
#[macro_use]
mod macros;
mod ext;
#[cfg(any(feature = "integer", feature = "float"))]
mod misc;
mod ops_prim;
#[cfg(any(feature = "serde", feature = "borsh"))]
mod serdeize;

pub mod ops;

/**
Assigns to a number from another value.

# Examples

Implementing the trait:

```rust
use rug::Assign;
struct I(i32);
impl Assign<i16> for I {
    fn assign(&mut self, rhs: i16) {
        self.0 = rhs.into();
    }
}
let mut i = I(0);
i.assign(42_i16);
assert_eq!(i.0, 42);
```

Performing an assignment operation using the trait:

```rust
# #[cfg(feature = "integer")] {
use rug::{Assign, Integer};
let mut i = Integer::from(15);
assert_eq!(i, 15);
i.assign(23);
assert_eq!(i, 23);
# }
```
*/
pub trait Assign<Src = Self> {
    /// Peforms the assignement.
    fn assign(&mut self, src: Src);
}

/**
Completes an [incomplete-computation value][icv].

# Examples

Implementing the trait:

```rust
# #[cfg(feature = "integer")] {
use rug::{Complete, Integer};
struct LazyPow4<'a>(&'a Integer);
impl Complete for LazyPow4<'_> {
    type Completed = Integer;
    fn complete(self) -> Integer {
        self.0.clone().square().square()
    }
}

assert_eq!(LazyPow4(&Integer::from(3)).complete(), 3i32.pow(4));
# }
```

Completing an [incomplete-computation value][icv]:

```rust
# #[cfg(feature = "integer")] {
use rug::{Complete, Integer};
let incomplete = Integer::fibonacci(12);
let complete = incomplete.complete();
assert_eq!(complete, 144);
# }
```

[icv]: crate#incomplete-computation-values
*/
pub trait Complete {
    /// The type of the completed operation.
    type Completed;

    /// Completes the operation.
    fn complete(self) -> Self::Completed;

    /// Completes the operation and stores the result in a target.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "integer")] {
    /// use rug::{Complete, Integer};
    /// let mut complete = Integer::new();
    /// Integer::fibonacci(12).complete_into(&mut complete);
    /// assert_eq!(complete, 144);
    /// # }
    /// ```
    #[inline]
    fn complete_into<T>(self, target: &mut T)
    where
        Self: Sized,
        T: Assign<Self>,
    {
        target.assign(self);
    }
}

#[cfg(feature = "integer")]
pub mod integer;
#[cfg(feature = "integer")]
pub use crate::integer::big::Integer;

#[cfg(feature = "rational")]
pub mod rational;
#[cfg(feature = "rational")]
pub use crate::rational::big::Rational;

#[cfg(feature = "float")]
pub mod float;
#[cfg(feature = "float")]
pub use crate::float::big::Float;

#[cfg(feature = "complex")]
pub mod complex;
#[cfg(feature = "complex")]
pub use crate::complex::big::Complex;

#[cfg(feature = "rand")]
pub mod rand;

pub use az;

// Used by doc tests. Not public API.
#[doc(hidden)]
pub mod private {
    #[cfg(any(feature = "integer", feature = "float"))]
    pub use crate::misc::{cast_ptr, cast_ptr_mut};
}

#[cfg(any(feature = "integer", feature = "float"))]
mod static_assertions {
    use gmp_mpfr_sys::gmp::{LIMB_BITS, NAIL_BITS, NUMB_BITS, limb_t};

    static_assert!(NAIL_BITS == 0);
    static_assert!(NUMB_BITS == LIMB_BITS);
    static_assert!(cfg!(target_pointer_width = "32") ^ cfg!(target_pointer_width = "64"));
    static_assert!(cfg!(gmp_limb_bits_32) ^ cfg!(gmp_limb_bits_64));
    #[cfg(gmp_limb_bits_64)]
    static_assert!(NUMB_BITS == 64);
    #[cfg(gmp_limb_bits_32)]
    static_assert!(NUMB_BITS == 32);
    static_assert!(NUMB_BITS % 8 == 0);
    static_assert!(limb_t::BITS == NUMB_BITS as u32);
}

#[cfg(all(test, any(feature = "integer", feature = "float")))]
mod tests {
    pub const U8: &[u8] = &[0, 1, 100, 101, i8::MAX as u8 + 1, u8::MAX];
    pub const I8: &[i8] = &[i8::MIN, -101, -100, -1, 0, 1, 100, 101, i8::MAX];
    pub const U16: &[u16] = &[0, 1, 1000, 1001, i16::MAX as u16 + 1, u16::MAX];
    pub const I16: &[i16] = &[i16::MIN, -1001, -1000, -1, 0, 1, 1000, 1001, i16::MAX];
    pub const U32: &[u32] = &[0, 1, 1000, 1001, i32::MAX as u32 + 1, u32::MAX];
    pub const I32: &[i32] = &[i32::MIN, -1001, -1000, -1, 0, 1, 1000, 1001, i32::MAX];
    pub const U64: &[u64] = &[
        0,
        1,
        1000,
        1001,
        i32::MAX as u64 + 1,
        u32::MAX as u64 + 1,
        u64::MAX,
    ];
    pub const I64: &[i64] = &[
        i64::MIN,
        -(u32::MAX as i64) - 1,
        i32::MIN as i64 - 1,
        -1001,
        -1000,
        -1,
        0,
        1,
        1000,
        1001,
        i32::MAX as i64 + 1,
        u32::MAX as i64 + 1,
        i64::MAX,
    ];
    pub const U128: &[u128] = &[
        0,
        1,
        1000,
        1001,
        i32::MAX as u128 + 1,
        u32::MAX as u128 + 1,
        i64::MAX as u128 + 1,
        u64::MAX as u128 + 1,
        u128::MAX,
    ];
    pub const I128: &[i128] = &[
        i128::MIN,
        -(u64::MAX as i128) - 1,
        i64::MIN as i128 - 1,
        -(u32::MAX as i128) - 1,
        i32::MIN as i128 - 1,
        -1001,
        -1000,
        -1,
        0,
        1,
        1000,
        1001,
        i32::MAX as i128 + 1,
        u32::MAX as i128 + 1,
        i64::MAX as i128 + 1,
        u64::MAX as i128 + 1,
        i128::MAX,
    ];
    pub const USIZE: &[usize] = &[0, 1, 1000, 1001, isize::MAX as usize + 1, usize::MAX];
    pub const ISIZE: &[isize] = &[isize::MIN, -1001, -1000, -1, 0, 1, 1000, 1001, isize::MAX];
    #[cfg(any(feature = "rational", feature = "float"))]
    pub const F32: &[f32] = &[
        -f32::NAN,
        f32::NEG_INFINITY,
        f32::MIN,
        -12.0e30,
        -2.0,
        -1.0 - f32::EPSILON,
        -1.0,
        -f32::MIN_POSITIVE,
        -f32::MIN_POSITIVE * f32::EPSILON,
        -0.0,
        0.0,
        f32::MIN_POSITIVE * f32::EPSILON,
        f32::MIN_POSITIVE,
        1.0,
        1.0 + f32::EPSILON,
        2.0,
        12.0e30,
        f32::MAX,
        f32::INFINITY,
        f32::NAN,
    ];
    #[cfg(any(feature = "rational", feature = "float"))]
    pub const F64: &[f64] = &[
        -f64::NAN,
        f64::NEG_INFINITY,
        f64::MIN,
        -12.0e43,
        -2.0,
        -1.0 - f64::EPSILON,
        -1.0,
        -f64::MIN_POSITIVE,
        -f64::MIN_POSITIVE * f64::EPSILON,
        -0.0,
        0.0,
        f64::MIN_POSITIVE * f64::EPSILON,
        f64::MIN_POSITIVE,
        1.0,
        1.0 + f64::EPSILON,
        2.0,
        12.0e43,
        f64::MAX,
        f64::INFINITY,
        f64::NAN,
    ];
}
