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

use crate::ext::xmpz;
use crate::integer::ParseIntegerError;
use crate::ops::{DivRounding, RemRounding};
use crate::{Assign, Integer};
use az::{CheckedCast, StrictCast};
use num_integer::{ExtendedGcd, Integer as NumInteger, Roots};
use num_traits::Num;
use num_traits::cast::{FromPrimitive, ToPrimitive};
use num_traits::identities::{ConstZero, One, Zero};
use num_traits::ops::checked::{CheckedDiv, CheckedRem};
use num_traits::ops::euclid::{CheckedEuclid, Euclid};
use num_traits::ops::mul_add::{MulAdd, MulAddAssign};
use num_traits::sign::Signed;

impl Zero for Integer {
    #[inline]
    fn zero() -> Self {
        Integer::new()
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.is_zero()
    }

    #[inline]
    fn set_zero(&mut self) {
        xmpz::set_0(self);
    }
}

impl ConstZero for Integer {
    const ZERO: Integer = Integer::ZERO;
}

impl One for Integer {
    #[inline]
    fn one() -> Self {
        Integer::from(1u8)
    }

    #[inline]
    fn set_one(&mut self) {
        xmpz::set_1(self);
    }

    #[inline]
    fn is_one(&self) -> bool {
        xmpz::is_1(self)
    }
}

impl Num for Integer {
    type FromStrRadixErr = ParseIntegerError;

    #[inline]
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntegerError> {
        Integer::from_str_radix(src, radix.strict_cast())
    }
}

impl Signed for Integer {
    #[inline]
    fn abs(&self) -> Self {
        self.abs_ref().into()
    }

    #[inline]
    fn abs_sub(&self, other: &Self) -> Self {
        if *self <= *other {
            Integer::new()
        } else {
            (self - other).into()
        }
    }

    #[inline]
    fn signum(&self) -> Self {
        self.signum_ref().into()
    }

    #[inline]
    fn is_positive(&self) -> bool {
        self.is_positive()
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.is_negative()
    }
}

impl MulAdd for Integer {
    type Output = Integer;

    #[inline]
    fn mul_add(self, a: Integer, b: Integer) -> Integer {
        &self * &a + b
    }
}

impl MulAddAssign for Integer {
    #[inline]
    fn mul_add_assign(&mut self, a: Integer, b: Integer) {
        *self = &*self * &a + b;
    }
}

impl MulAdd<&Integer, &Integer> for Integer {
    type Output = Integer;

    #[inline]
    fn mul_add(self, a: &Integer, b: &Integer) -> Integer {
        self * a + b
    }
}

impl MulAddAssign<&Integer, &Integer> for Integer {
    #[inline]
    fn mul_add_assign(&mut self, a: &Integer, b: &Integer) {
        *self *= a;
        *self += b;
    }
}

impl ToPrimitive for Integer {
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        self.checked_cast()
    }
    #[inline]
    fn to_u64(&self) -> Option<u64> {
        self.checked_cast()
    }
    #[inline]
    fn to_isize(&self) -> Option<isize> {
        self.checked_cast()
    }
    #[inline]
    fn to_i8(&self) -> Option<i8> {
        self.checked_cast()
    }
    #[inline]
    fn to_i16(&self) -> Option<i16> {
        self.checked_cast()
    }
    #[inline]
    fn to_i32(&self) -> Option<i32> {
        self.checked_cast()
    }
    #[inline]
    fn to_i128(&self) -> Option<i128> {
        self.checked_cast()
    }
    #[inline]
    fn to_usize(&self) -> Option<usize> {
        self.checked_cast()
    }
    #[inline]
    fn to_u8(&self) -> Option<u8> {
        self.checked_cast()
    }
    #[inline]
    fn to_u16(&self) -> Option<u16> {
        self.checked_cast()
    }
    #[inline]
    fn to_u32(&self) -> Option<u32> {
        self.checked_cast()
    }
    #[inline]
    fn to_u128(&self) -> Option<u128> {
        self.checked_cast()
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

impl FromPrimitive for Integer {
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

impl NumInteger for Integer {
    #[inline]
    fn div_floor(&self, other: &Self) -> Self {
        DivRounding::div_floor(self, other).into()
    }
    #[inline]
    fn mod_floor(&self, other: &Self) -> Self {
        RemRounding::rem_floor(self, other).into()
    }
    #[inline]
    fn gcd(&self, other: &Self) -> Self {
        self.gcd_ref(other).into()
    }
    #[inline]
    fn lcm(&self, other: &Self) -> Self {
        self.lcm_ref(other).into()
    }
    #[inline]
    fn divides(&self, other: &Self) -> bool {
        other.is_divisible(self)
    }
    #[inline]
    fn is_multiple_of(&self, other: &Self) -> bool {
        other.is_divisible(self)
    }
    #[inline]
    fn is_even(&self) -> bool {
        self.is_even()
    }
    #[inline]
    fn is_odd(&self) -> bool {
        self.is_odd()
    }
    #[inline]
    fn div_rem(&self, other: &Self) -> (Self, Self) {
        self.div_rem_ref(other).into()
    }

    #[inline]
    fn div_ceil(&self, other: &Self) -> Self {
        DivRounding::div_ceil(self, other).into()
    }
    #[inline]
    fn gcd_lcm(&self, other: &Self) -> (Self, Self) {
        let gcd = Integer::from(self.gcd_ref(other));
        let lcm = if gcd.is_zero() || other.is_zero() {
            Integer::new()
        } else {
            Integer::from(self.div_exact_ref(&gcd)) * other
        };
        (gcd, lcm)
    }
    #[inline]
    fn extended_gcd(&self, other: &Self) -> ExtendedGcd<Self> {
        let mut gcdc = ExtendedGcd::<Integer> {
            gcd: Integer::new(),
            x: Integer::new(),
            y: Integer::new(),
        };
        (&mut gcdc.gcd, &mut gcdc.x, &mut gcdc.y).assign(self.extended_gcd_ref(other));
        gcdc
    }
    #[inline]
    fn extended_gcd_lcm(&self, other: &Self) -> (ExtendedGcd<Self>, Self) {
        let gcdc = NumInteger::extended_gcd(self, other);
        let lcm = if gcdc.gcd.is_zero() || other.is_zero() {
            Integer::new()
        } else {
            Integer::from(self.div_exact_ref(&gcdc.gcd)) * other
        };
        (gcdc, lcm)
    }
    #[inline]
    fn div_mod_floor(&self, other: &Self) -> (Self, Self) {
        self.div_rem_floor_ref(other).into()
    }
    #[inline]
    fn next_multiple_of(&self, other: &Self) -> Self {
        let mut i = Integer::from(RemRounding::rem_floor(self, other));
        if i.is_zero() {
            i.assign(self);
            i
        } else {
            other - i + self
        }
    }
    #[inline]
    fn prev_multiple_of(&self, other: &Self) -> Self {
        let mut i = Integer::from(RemRounding::rem_floor(self, other));
        if i.is_zero() {
            i.assign(self);
            i
        } else {
            self - i
        }
    }
    #[inline]
    fn dec(&mut self) {
        *self -= 1u32;
    }
    #[inline]
    fn inc(&mut self) {
        *self += 1u32;
    }
}

impl Roots for Integer {
    #[inline]
    fn nth_root(&self, n: u32) -> Self {
        self.root_ref(n).into()
    }

    #[inline]
    fn sqrt(&self) -> Self {
        self.sqrt_ref().into()
    }

    #[inline]
    fn cbrt(&self) -> Self {
        self.root_ref(3).into()
    }
}

impl CheckedDiv for Integer {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        if v.is_zero() {
            None
        } else {
            Some(Integer::from(self / v))
        }
    }
}

impl CheckedRem for Integer {
    fn checked_rem(&self, v: &Self) -> Option<Self> {
        if v.is_zero() {
            None
        } else {
            Some(Integer::from(self % v))
        }
    }
}

impl Euclid for Integer {
    fn div_euclid(&self, v: &Self) -> Self {
        Integer::from(DivRounding::div_euc(self, v))
    }

    fn rem_euclid(&self, v: &Self) -> Self {
        Integer::from(RemRounding::rem_euc(self, v))
    }

    fn div_rem_euclid(&self, v: &Self) -> (Self, Self) {
        <(Integer, Integer)>::from(self.div_rem_euc_ref(v))
    }
}

impl CheckedEuclid for Integer {
    fn checked_div_euclid(&self, v: &Self) -> Option<Self> {
        if v.is_zero() {
            None
        } else {
            Some(Integer::from(DivRounding::div_euc(self, v)))
        }
    }

    fn checked_rem_euclid(&self, v: &Self) -> Option<Self> {
        if v.is_zero() {
            None
        } else {
            Some(Integer::from(RemRounding::rem_euc(self, v)))
        }
    }

    fn checked_div_rem_euclid(&self, v: &Self) -> Option<(Self, Self)> {
        if v.is_zero() {
            None
        } else {
            Some(<(Integer, Integer)>::from(self.div_rem_euc_ref(v)))
        }
    }
}
