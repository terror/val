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
Arbitrary-precision integers.

This module provides support for arbitrary-precision integers of type
[`Integer`]. Instances of [`Integer`] always have a heap allocation for the bit
data; if you want a temporary small integer without heap allocation, you can use
the [`MiniInteger`] type.

# Examples

```rust
use rug::integer::MiniInteger;
use rug::Assign;
use rug::Integer;
let mut int = Integer::from(10);
assert_eq!(int, 10);
let small = MiniInteger::from(-15);
// `small` can be borrowed like an `Integer` in the following line:
int.assign(small.borrow().abs_ref());
assert_eq!(int, 15);
```

[`Integer`]: crate::Integer
*/

pub(crate) mod arith;
pub(crate) mod big;
mod borrow;
#[cfg(feature = "borsh")]
mod borsh;
mod casts;
mod cmp;
mod division;
#[cfg(feature = "num-traits")]
mod impl_num_traits;
#[cfg(all(target_pointer_width = "64", not(windows)))]
mod long64;
pub(crate) mod mini;
#[cfg(feature = "serde")]
mod serde;
pub(crate) mod small;
#[cfg(test)]
mod tests;
mod traits;

pub use crate::integer::big::{IsPrime, ParseIntegerError, UnsignedPrimitive};
pub use crate::integer::borrow::BorrowInteger;
#[cfg(all(target_pointer_width = "64", not(windows)))]
pub use crate::integer::long64::IntegerExt64;
pub use crate::integer::mini::{MiniInteger, ToMini};
#[allow(deprecated)]
pub use crate::integer::small::{SmallInteger, ToSmall};

use core::ffi::c_int;

/**
An error which can be returned when a checked conversion from [`Integer`] fails.

# Examples

```rust
use rug::integer::TryFromIntegerError;
use rug::Integer;
// This is negative and cannot be converted to u32.
let i = Integer::from(-5);
let error: TryFromIntegerError = match u32::try_from(&i) {
    Ok(_) => unreachable!(),
    Err(error) => error,
};
println!("Error: {}", error);
```

[`Integer`]: crate::Integer
*/
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TryFromIntegerError {
    _unused: (),
}

/**
The ordering of digits inside a [slice], and bytes inside a digit.

# Examples

```rust
use rug::integer::Order;
use rug::Integer;

let i = Integer::from(0x0102_0304);
let mut buf: [u16; 4] = [0; 4];

// most significant 16-bit digit first, little endian digits
i.write_digits(&mut buf, Order::MsfLe);
assert_eq!(buf, [0, 0, 0x0102u16.to_le(), 0x0304u16.to_le()]);
// least significant 16-bit digit first, big endian digits
i.write_digits(&mut buf, Order::LsfBe);
assert_eq!(buf, [0x0304u16.to_be(), 0x0102u16.to_be(), 0, 0]);
```
*/
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Order {
    /// Least significant digit first, with each digit in the target’s
    /// endianness.
    Lsf,
    /// Least significant digit first, with little endian digits.
    LsfLe,
    /// Least significant digit first, with big endian digits.
    LsfBe,
    /// Most significant digit first, with each digit in the target’s
    /// endianness.
    Msf,
    /// Most significant digit first, with little endian digits.
    MsfLe,
    /// Most significant digit first, with big endian digits.
    MsfBe,
}

impl Order {
    #[inline]
    fn order(self) -> c_int {
        match self {
            Order::Lsf | Order::LsfLe | Order::LsfBe => -1,
            Order::Msf | Order::MsfLe | Order::MsfBe => 1,
        }
    }
    #[inline]
    fn endian(self) -> c_int {
        match self {
            Order::Lsf | Order::Msf => 0,
            Order::LsfLe | Order::MsfLe => -1,
            Order::LsfBe | Order::MsfBe => 1,
        }
    }
}
