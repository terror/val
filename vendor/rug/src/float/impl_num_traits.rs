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

use crate::Float;
#[cfg(feature = "integer")]
use crate::Integer;
#[cfg(feature = "integer")]
use az::{CheckedAs, CheckedCast};
use core::cmp::Ordering;
#[cfg(feature = "integer")]
use num_traits::cast::ToPrimitive;
use num_traits::float::TotalOrder;
use num_traits::ops::inv::Inv;
use num_traits::ops::mul_add::{MulAdd, MulAddAssign};

impl Inv for Float {
    type Output = Self;

    #[inline]
    fn inv(self) -> Self::Output {
        self.recip()
    }
}

impl MulAdd for Float {
    type Output = Float;

    #[inline]
    fn mul_add(self, a: Float, b: Float) -> Float {
        self.mul_add(&a, &b)
    }
}

impl MulAddAssign for Float {
    #[inline]
    fn mul_add_assign(&mut self, a: Float, b: Float) {
        self.mul_add_mut(&a, &b);
    }
}

impl MulAdd<&Float, &Float> for Float {
    type Output = Float;

    #[inline]
    fn mul_add(self, a: &Float, b: &Float) -> Float {
        self.mul_add(a, b)
    }
}

impl MulAddAssign<&Float, &Float> for Float {
    #[inline]
    fn mul_add_assign(&mut self, a: &Float, b: &Float) {
        self.mul_add_mut(a, b);
    }
}

#[cfg(feature = "integer")]
impl ToPrimitive for Float {
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        self.checked_as::<Integer>()
            .and_then(CheckedCast::checked_cast)
    }
    #[inline]
    fn to_u64(&self) -> Option<u64> {
        self.checked_as::<Integer>()
            .and_then(CheckedCast::checked_cast)
    }
    #[inline]
    fn to_isize(&self) -> Option<isize> {
        self.checked_as::<Integer>()
            .and_then(CheckedCast::checked_cast)
    }
    #[inline]
    fn to_i8(&self) -> Option<i8> {
        self.checked_as::<Integer>()
            .and_then(CheckedCast::checked_cast)
    }
    #[inline]
    fn to_i16(&self) -> Option<i16> {
        self.checked_as::<Integer>()
            .and_then(CheckedCast::checked_cast)
    }
    #[inline]
    fn to_i32(&self) -> Option<i32> {
        self.checked_as::<Integer>()
            .and_then(CheckedCast::checked_cast)
    }
    #[inline]
    fn to_i128(&self) -> Option<i128> {
        self.checked_as::<Integer>()
            .and_then(CheckedCast::checked_cast)
    }
    #[inline]
    fn to_usize(&self) -> Option<usize> {
        self.checked_as::<Integer>()
            .and_then(CheckedCast::checked_cast)
    }
    #[inline]
    fn to_u8(&self) -> Option<u8> {
        self.checked_as::<Integer>()
            .and_then(CheckedCast::checked_cast)
    }
    #[inline]
    fn to_u16(&self) -> Option<u16> {
        self.checked_as::<Integer>()
            .and_then(CheckedCast::checked_cast)
    }
    #[inline]
    fn to_u32(&self) -> Option<u32> {
        self.checked_as::<Integer>()
            .and_then(CheckedCast::checked_cast)
    }
    #[inline]
    fn to_u128(&self) -> Option<u128> {
        self.checked_as::<Integer>()
            .and_then(CheckedCast::checked_cast)
    }
    #[inline]
    fn to_f32(&self) -> Option<f32> {
        Some(self.to_f32())
    }
    #[inline]
    fn to_f64(&self) -> Option<f64> {
        Some(self.to_f64())
    }
}

impl TotalOrder for Float {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.total_cmp(other)
    }
}
