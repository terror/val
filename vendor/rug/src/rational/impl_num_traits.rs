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

use crate::ext::xmpq;
use crate::ext::xmpz;
use crate::{Assign, Integer, Rational};
use az::CheckedCast;
use num_traits::cast::{FromPrimitive, ToPrimitive};
use num_traits::identities::{One, Zero};
use num_traits::ops::inv::Inv;
use num_traits::ops::mul_add::{MulAdd, MulAddAssign};

impl Zero for Rational {
    #[inline]
    fn zero() -> Self {
        Rational::new()
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.is_zero()
    }

    #[inline]
    fn set_zero(&mut self) {
        xmpq::set_0(self);
    }
}

impl One for Rational {
    #[inline]
    fn one() -> Self {
        Rational::from(1u8)
    }

    #[inline]
    fn set_one(&mut self) {
        self.assign(1u32);
    }

    #[inline]
    fn is_one(&self) -> bool {
        xmpz::is_1(self.numer()) && xmpz::is_1(self.denom())
    }
}

impl Inv for Rational {
    type Output = Self;

    #[inline]
    fn inv(self) -> Self::Output {
        self.recip()
    }
}

impl MulAdd for Rational {
    type Output = Rational;

    #[inline]
    fn mul_add(self, a: Rational, b: Rational) -> Rational {
        self * a + b
    }
}

impl MulAddAssign for Rational {
    #[inline]
    fn mul_add_assign(&mut self, a: Rational, b: Rational) {
        *self *= a;
        *self += b;
    }
}

impl MulAdd<&Rational, &Rational> for Rational {
    type Output = Rational;

    #[inline]
    fn mul_add(self, a: &Rational, b: &Rational) -> Rational {
        self * a + b
    }
}

impl MulAddAssign<&Rational, &Rational> for Rational {
    #[inline]
    fn mul_add_assign(&mut self, a: &Rational, b: &Rational) {
        *self *= a;
        *self += b;
    }
}

impl ToPrimitive for Rational {
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        let trunc = Integer::from(self.trunc_ref());
        trunc.checked_cast()
    }
    #[inline]
    fn to_u64(&self) -> Option<u64> {
        let trunc = Integer::from(self.trunc_ref());
        trunc.checked_cast()
    }
    #[inline]
    fn to_isize(&self) -> Option<isize> {
        let trunc = Integer::from(self.trunc_ref());
        trunc.checked_cast()
    }
    #[inline]
    fn to_i8(&self) -> Option<i8> {
        let trunc = Integer::from(self.trunc_ref());
        trunc.checked_cast()
    }
    #[inline]
    fn to_i16(&self) -> Option<i16> {
        let trunc = Integer::from(self.trunc_ref());
        trunc.checked_cast()
    }
    #[inline]
    fn to_i32(&self) -> Option<i32> {
        let trunc = Integer::from(self.trunc_ref());
        trunc.checked_cast()
    }
    #[inline]
    fn to_i128(&self) -> Option<i128> {
        let trunc = Integer::from(self.trunc_ref());
        trunc.checked_cast()
    }
    #[inline]
    fn to_usize(&self) -> Option<usize> {
        let trunc = Integer::from(self.trunc_ref());
        trunc.checked_cast()
    }
    #[inline]
    fn to_u8(&self) -> Option<u8> {
        let trunc = Integer::from(self.trunc_ref());
        trunc.checked_cast()
    }
    #[inline]
    fn to_u16(&self) -> Option<u16> {
        let trunc = Integer::from(self.trunc_ref());
        trunc.checked_cast()
    }
    #[inline]
    fn to_u32(&self) -> Option<u32> {
        let trunc = Integer::from(self.trunc_ref());
        trunc.checked_cast()
    }
    #[inline]
    fn to_u128(&self) -> Option<u128> {
        let trunc = Integer::from(self.trunc_ref());
        trunc.checked_cast()
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

impl FromPrimitive for Rational {
    #[inline]
    fn from_i64(n: i64) -> Option<Self> {
        Some(n.into())
    }
    #[inline]
    fn from_u64(n: u64) -> Option<Self> {
        Some(n.into())
    }
    #[inline]
    fn from_isize(n: isize) -> Option<Self> {
        Some(n.into())
    }
    #[inline]
    fn from_i8(n: i8) -> Option<Self> {
        Some(n.into())
    }
    #[inline]
    fn from_i16(n: i16) -> Option<Self> {
        Some(n.into())
    }
    #[inline]
    fn from_i32(n: i32) -> Option<Self> {
        Some(n.into())
    }
    #[inline]
    fn from_i128(n: i128) -> Option<Self> {
        Some(n.into())
    }
    #[inline]
    fn from_usize(n: usize) -> Option<Self> {
        Some(n.into())
    }
    #[inline]
    fn from_u8(n: u8) -> Option<Self> {
        Some(n.into())
    }
    #[inline]
    fn from_u16(n: u16) -> Option<Self> {
        Some(n.into())
    }
    #[inline]
    fn from_u32(n: u32) -> Option<Self> {
        Some(n.into())
    }
    #[inline]
    fn from_u128(n: u128) -> Option<Self> {
        Some(n.into())
    }
    #[inline]
    fn from_f32(n: f32) -> Option<Self> {
        Self::from_f32(n)
    }
    #[inline]
    fn from_f64(n: f64) -> Option<Self> {
        Self::from_f64(n)
    }
}
