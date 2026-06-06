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
Arbitrary-precision rational numbers.

This module provides support for arbitrary-precision rational numbers of type
[`Rational`][crate::Rational].
*/

mod arith;
pub(crate) mod big;
mod borrow;
#[cfg(feature = "borsh")]
mod borsh;
mod casts;
mod cmp;
#[cfg(feature = "num-traits")]
mod impl_num_traits;
mod mini;
#[cfg(feature = "serde")]
mod serde;
mod small;
#[cfg(test)]
mod tests;
mod traits;

pub use crate::rational::big::ParseRationalError;
pub use crate::rational::borrow::BorrowRational;
pub use crate::rational::mini::MiniRational;
#[allow(deprecated)]
pub use crate::rational::small::SmallRational;
use core::error::Error;
use core::fmt::{Display, Formatter, Result as FmtResult};

/**
An error which can be returned when a checked conversion from a floating-point
number to a [`Rational`][crate::Rational] number fails.

# Examples

```rust
use rug::rational::TryFromFloatError;
use rug::Rational;
// This is not finite and cannot be converted to Rational.
let inf = 1.0f32 / 0.0;
let error: TryFromFloatError = match Rational::try_from(inf) {
    Ok(_) => unreachable!(),
    Err(error) => error,
};
println!("Error: {}", error);
```
*/
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TryFromFloatError {
    pub(crate) _unused: (),
}

impl TryFromFloatError {
    fn desc(&self) -> &str {
        "conversion of infinite or NaN value attempted"
    }
}

impl Display for TryFromFloatError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(self.desc(), f)
    }
}

impl Error for TryFromFloatError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        self.desc()
    }
}
