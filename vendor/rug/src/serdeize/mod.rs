// Copyright © 2025 Kartik Soneji

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

#[cfg(all(feature = "serde", any(feature = "integer", feature = "float")))]
pub mod serde;

#[cfg(all(feature = "borsh", any(feature = "integer", feature = "float")))]
pub mod borsh;

#[cfg(any(feature = "integer", feature = "float"))]
pub mod data;

#[cfg(any(feature = "integer", feature = "float"))]
#[allow(unused_imports)]
pub use data::{Data, PrecReq, PrecVal};

#[allow(dead_code)]
pub fn check_range<T>(name: &'static str, val: T, min: T, max: T) -> Result<(), String>
where
    T: Copy + core::fmt::Display + Ord,
{
    if val < min {
        Err(format!("{name} {val} less than minimum {min}"))
    } else if val > max {
        Err(format!("{name} {val} greater than maximum {max}"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
#[cfg(any(feature = "integer", feature = "float"))]
pub mod test {
    #[cfg(feature = "serde")]
    pub use super::serde::test::*;

    #[cfg(feature = "borsh")]
    pub use super::borsh::test::*;

    #[cfg(all(feature = "borsh", feature = "complex"))]
    pub fn assert_eq_float_handle_nan(a: &crate::Float, b: &crate::Float) {
        if a.is_nan() || b.is_nan() {
            assert!(a.is_nan());
            assert!(b.is_nan());
        } else {
            assert_eq!(a, b);
        }
    }
}
