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

#[cfg(feature = "integer")]
use crate::Integer;
#[cfg(feature = "rational")]
use crate::Rational;
use crate::ext::xmpfr;
use crate::float::mini;
use crate::float::{MiniFloat, Round};
use crate::{Assign, Float};
#[allow(deprecated)]
use az::UnwrappedCast;
use az::{Cast, CheckedAs, CheckedCast, SaturatingCast, StrictCast, WrappingAs};
use core::cmp::Ordering;

macro_rules! cast_int_uint_common {
    ($Prim:ty) => {
        impl Cast<$Prim> for Float {
            #[inline]
            fn cast(self) -> $Prim {
                (&self).cast()
            }
        }

        impl Cast<$Prim> for &'_ Float {
            #[inline]
            fn cast(self) -> $Prim {
                self.strict_cast()
            }
        }

        impl CheckedCast<$Prim> for Float {
            #[inline]
            fn checked_cast(self) -> Option<$Prim> {
                (&self).checked_cast()
            }
        }

        impl SaturatingCast<$Prim> for Float {
            #[inline]
            fn saturating_cast(self) -> $Prim {
                (&self).saturating_cast()
            }
        }

        impl SaturatingCast<$Prim> for &'_ Float {
            fn saturating_cast(self) -> $Prim {
                if self.is_nan() {
                    panic!("NaN");
                }
                match self.checked_cast() {
                    Some(val) => val,
                    None => {
                        if self.is_sign_negative() {
                            <$Prim>::MIN
                        } else {
                            <$Prim>::MAX
                        }
                    }
                }
            }
        }

        impl StrictCast<$Prim> for Float {
            #[inline]
            fn strict_cast(self) -> $Prim {
                (&self).strict_cast()
            }
        }

        impl StrictCast<$Prim> for &'_ Float {
            #[inline]
            fn strict_cast(self) -> $Prim {
                if self.is_nan() {
                    panic!("NaN");
                }
                self.checked_cast().expect("overflow")
            }
        }

        #[allow(deprecated)]
        impl UnwrappedCast<$Prim> for Float {
            #[inline]
            fn unwrapped_cast(self) -> $Prim {
                (&self).strict_cast()
            }
        }

        #[allow(deprecated)]
        impl UnwrappedCast<$Prim> for &'_ Float {
            #[inline]
            fn unwrapped_cast(self) -> $Prim {
                self.strict_cast()
            }
        }
    };
}

macro_rules! cast_int {
    ($Prim:ty, $U:ty, $nbits:expr, $unchecked_get:path) => {
        cast_int_uint_common! { $Prim }

        impl CheckedCast<$Prim> for &'_ Float {
            fn checked_cast(self) -> Option<$Prim> {
                if !self.is_finite() {
                    return None;
                }

                const ZERO: $Prim = 0;
                let mut small = MiniFloat::from(ZERO);
                // Safety: assigning a value will not change the precision,
                // so there is no reallocation.
                unsafe {
                    small
                        .as_nonreallocating_float()
                        .assign(self.round_even_ref());
                }
                let bor = small.borrow_excl();
                // We already checked for NaN, so we can use mpfr::sgn.
                debug_assert!(!bor.is_nan());
                let cmp0 = xmpfr::sgn_not_nan(bor);
                match cmp0 {
                    Ordering::Less => match bor.get_exp() {
                        None => None,
                        Some(exp) if exp > $nbits => None,
                        Some(exp) => {
                            // Safety:
                            //  1. small is normal, so we can get the number.
                            //  2. Since it is a normal integer, exp > 0
                            debug_assert!(bor.is_normal());
                            let abs = unsafe { $unchecked_get(&small) >> ($nbits - exp) };
                            if abs > <$Prim>::MIN.wrapping_as::<$U>() {
                                None
                            } else {
                                Some(abs.wrapping_as::<$Prim>().wrapping_neg())
                            }
                        }
                    },
                    Ordering::Equal => Some(0),
                    Ordering::Greater => match bor.get_exp() {
                        None => None,
                        Some(exp) if exp >= $nbits => None,
                        Some(exp) => {
                            // Safety:
                            //  1. small is normal, so we can get the number.
                            //  2. Since it is a normal integer, exp > 0
                            debug_assert!(bor.is_normal());
                            let abs = unsafe { $unchecked_get(&small) >> ($nbits - exp) };
                            // We have already checked that exp < $nbits, so
                            // the value fits.
                            Some(abs.wrapping_as::<$Prim>())
                        }
                    },
                }
            }
        }
    };
}

macro_rules! cast_uint {
    ($Prim:ty, $nbits:expr, $unchecked_get:path) => {
        cast_int_uint_common! { $Prim }

        impl CheckedCast<$Prim> for &'_ Float {
            fn checked_cast(self) -> Option<$Prim> {
                if !self.is_finite() {
                    return None;
                }

                const ZERO: $Prim = 0;
                let mut small = MiniFloat::from(ZERO);
                // Safety: assigning a value will not change the precision,
                // so there is no reallocation.
                unsafe {
                    small
                        .as_nonreallocating_float()
                        .assign(self.round_even_ref());
                }
                let bor = small.borrow_excl();
                // We already checked for NaN, so we can use mpfr::sgn.
                debug_assert!(!bor.is_nan());
                let cmp0 = xmpfr::sgn_not_nan(bor);
                match cmp0 {
                    Ordering::Less => None,
                    Ordering::Equal => Some(0),
                    Ordering::Greater => match bor.get_exp() {
                        None => None,
                        Some(exp) if exp > $nbits => None,
                        Some(exp) => {
                            // Safety:
                            //  1. small is normal, so we can get the number.
                            //  2. Since it is a normal integer, exp > 0
                            debug_assert!(bor.is_normal());
                            Some(unsafe { $unchecked_get(&small) >> ($nbits - exp) })
                        }
                    },
                }
            }
        }
    };
}

cast_int! { i8, u8, 8, mini::unchecked_get_unshifted_u8 }
cast_int! { i16, u16, 16, mini::unchecked_get_unshifted_u16 }
cast_int! { i32, u32, 32, mini::unchecked_get_unshifted_u32 }
cast_int! { i64, u64, 64, mini::unchecked_get_unshifted_u64 }
cast_int! { i128, u128, 128, mini::unchecked_get_unshifted_u128 }

cast_int_uint_common! { isize }

impl CheckedCast<isize> for &'_ Float {
    #[inline]
    fn checked_cast(self) -> Option<isize> {
        #[cfg(target_pointer_width = "32")]
        {
            self.checked_as::<i32>().map(az::cast)
        }
        #[cfg(target_pointer_width = "64")]
        {
            self.checked_as::<i64>().map(az::cast)
        }
    }
}

cast_uint! { u8, 8, mini::unchecked_get_unshifted_u8 }
cast_uint! { u16, 16, mini::unchecked_get_unshifted_u16 }
cast_uint! { u32, 32, mini::unchecked_get_unshifted_u32 }
cast_uint! { u64, 64, mini::unchecked_get_unshifted_u64 }
cast_uint! { u128, 128, mini::unchecked_get_unshifted_u128 }

cast_int_uint_common! { usize }

impl CheckedCast<usize> for &'_ Float {
    #[inline]
    fn checked_cast(self) -> Option<usize> {
        #[cfg(target_pointer_width = "32")]
        {
            self.checked_as::<u32>().map(az::cast)
        }
        #[cfg(target_pointer_width = "64")]
        {
            self.checked_as::<u64>().map(az::cast)
        }
    }
}

#[cfg(feature = "nightly-float")]
impl Cast<f16> for Float {
    #[inline]
    fn cast(self) -> f16 {
        (&self).cast()
    }
}

#[cfg(feature = "nightly-float")]
impl Cast<f16> for &'_ Float {
    #[inline]
    fn cast(self) -> f16 {
        self.to_f16_round(Round::Nearest)
    }
}

impl Cast<f32> for Float {
    #[inline]
    fn cast(self) -> f32 {
        (&self).cast()
    }
}

impl Cast<f32> for &'_ Float {
    #[inline]
    fn cast(self) -> f32 {
        self.to_f32_round(Round::Nearest)
    }
}

impl Cast<f64> for Float {
    #[inline]
    fn cast(self) -> f64 {
        (&self).cast()
    }
}

impl Cast<f64> for &'_ Float {
    #[inline]
    fn cast(self) -> f64 {
        self.to_f64_round(Round::Nearest)
    }
}

#[cfg(feature = "nightly-float")]
impl Cast<f128> for Float {
    #[inline]
    fn cast(self) -> f128 {
        (&self).cast()
    }
}

#[cfg(feature = "nightly-float")]
impl Cast<f128> for &'_ Float {
    #[inline]
    fn cast(self) -> f128 {
        self.to_f128_round(Round::Nearest)
    }
}

#[cfg(feature = "integer")]
impl Cast<Integer> for Float {
    #[inline]
    fn cast(self) -> Integer {
        (&self).cast()
    }
}

#[cfg(feature = "integer")]
impl Cast<Integer> for &'_ Float {
    #[inline]
    fn cast(self) -> Integer {
        self.checked_cast().expect("not finite")
    }
}

#[cfg(feature = "integer")]
impl CheckedCast<Integer> for Float {
    #[inline]
    fn checked_cast(self) -> Option<Integer> {
        (&self).checked_cast()
    }
}

#[cfg(feature = "integer")]
impl CheckedCast<Integer> for &'_ Float {
    #[inline]
    fn checked_cast(self) -> Option<Integer> {
        self.to_integer_round(Round::Nearest).map(|x| x.0)
    }
}

#[cfg(feature = "integer")]
impl StrictCast<Integer> for Float {
    #[inline]
    fn strict_cast(self) -> Integer {
        (&self).strict_cast()
    }
}

#[cfg(feature = "integer")]
impl StrictCast<Integer> for &'_ Float {
    #[inline]
    fn strict_cast(self) -> Integer {
        self.checked_cast().expect("not finite")
    }
}

#[cfg(feature = "integer")]
#[allow(deprecated)]
impl UnwrappedCast<Integer> for Float {
    #[inline]
    fn unwrapped_cast(self) -> Integer {
        (&self).strict_cast()
    }
}

#[cfg(feature = "integer")]
#[allow(deprecated)]
impl UnwrappedCast<Integer> for &'_ Float {
    #[inline]
    fn unwrapped_cast(self) -> Integer {
        self.strict_cast()
    }
}

#[cfg(feature = "rational")]
impl Cast<Rational> for Float {
    #[inline]
    fn cast(self) -> Rational {
        (&self).cast()
    }
}

#[cfg(feature = "rational")]
impl Cast<Rational> for &'_ Float {
    #[inline]
    fn cast(self) -> Rational {
        self.checked_cast().expect("not finite")
    }
}

#[cfg(feature = "rational")]
impl CheckedCast<Rational> for Float {
    #[inline]
    fn checked_cast(self) -> Option<Rational> {
        (&self).checked_cast()
    }
}

#[cfg(feature = "rational")]
impl CheckedCast<Rational> for &'_ Float {
    #[inline]
    fn checked_cast(self) -> Option<Rational> {
        if !self.is_finite() {
            return None;
        }
        let mut r = Rational::new();
        xmpfr::get_q(&mut r, self);
        Some(r)
    }
}

#[cfg(feature = "rational")]
impl StrictCast<Rational> for Float {
    #[inline]
    fn strict_cast(self) -> Rational {
        (&self).strict_cast()
    }
}

#[cfg(feature = "rational")]
impl StrictCast<Rational> for &'_ Float {
    #[inline]
    fn strict_cast(self) -> Rational {
        self.checked_cast().expect("not finite")
    }
}

#[cfg(feature = "rational")]
#[allow(deprecated)]
impl UnwrappedCast<Rational> for Float {
    #[inline]
    fn unwrapped_cast(self) -> Rational {
        (&self).strict_cast()
    }
}

#[cfg(feature = "rational")]
#[allow(deprecated)]
impl UnwrappedCast<Rational> for &'_ Float {
    #[inline]
    fn unwrapped_cast(self) -> Rational {
        self.strict_cast()
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use crate::{Assign, Float};
    use az::{Az, SaturatingAs, SaturatingCast};
    use core::borrow::Borrow;
    use core::fmt::Debug;
    use core::ops::{Add, Sub};

    fn check_integer<T>(min: T, max: T)
    where
        T: Copy + Debug + Eq + Add<Output = T> + Sub<Output = T> + From<bool>,
        Float: Assign<T>,
        for<'a> &'a Float: SaturatingCast<T>,
    {
        let min_float = Float::with_val(128, min);
        let max_float = Float::with_val(128, max);
        let one = T::from(true);

        // min is even
        assert_eq!(
            (min_float.clone() - 1f32).borrow().saturating_as::<T>(),
            min
        );
        assert_eq!(
            (min_float.clone() - 0.5f32).borrow().saturating_as::<T>(),
            min
        );
        assert_eq!(min_float.borrow().saturating_as::<T>(), min);
        assert_eq!(
            (min_float.clone() + 0.5f32).borrow().saturating_as::<T>(),
            min
        );
        assert_eq!((min_float + 1f32).borrow().saturating_as::<T>(), min + one);
        // max is odd
        assert_eq!(
            (max_float.clone() - 1f32).borrow().saturating_as::<T>(),
            max - one
        );
        assert_eq!(
            (max_float.clone() - 0.5f32).borrow().saturating_as::<T>(),
            max - one
        );
        assert_eq!(max_float.borrow().saturating_as::<T>(), max);
        assert_eq!(
            (max_float.clone() + 0.5f32).borrow().saturating_as::<T>(),
            max
        );
        assert_eq!((max_float + 1f32).borrow().saturating_as::<T>(), max);
    }

    #[test]
    fn check_integers() {
        check_integer(i8::MIN, i8::MAX);
        check_integer(i16::MIN, i16::MAX);
        check_integer(i32::MIN, i32::MAX);
        check_integer(i64::MIN, i64::MAX);
        check_integer(i128::MIN, i128::MAX);
        check_integer(isize::MIN, isize::MAX);
        check_integer(u8::MIN, u8::MAX);
        check_integer(u16::MIN, u16::MAX);
        check_integer(u32::MIN, u32::MAX);
        check_integer(u64::MIN, u64::MAX);
        check_integer(u128::MIN, u128::MAX);
        check_integer(usize::MIN, usize::MAX);
    }

    #[test]
    fn check_floats() {
        let f32_min_pos_subnormal: Float = Float::with_val(128, 1) >> (126 + 23);
        let f32_min_pos_normal: Float = Float::with_val(128, 1) >> 126;
        let f32_max: Float = Float::with_val(128, (1u32 << 24) - 1) << (127 - 23);
        let f64_min_pos_subnormal: Float = Float::with_val(128, 1) >> (1022 + 52);
        let f64_min_pos_normal: Float = Float::with_val(128, 1) >> 1022;
        let f64_max: Float = Float::with_val(128, (1u64 << 53) - 1) << (1023 - 52);
        let zero: Float = Float::new(1);
        let one: Float = Float::with_val(1, 1);
        let two_point5: Float = Float::with_val(3, 2.5);
        let f32_overflow: Float = Float::with_val(1, 1) << 128;
        let f64_overflow: Float = Float::with_val(1, 1) << 1024;

        assert_eq!(
            (*f32_overflow.as_neg()).borrow().az::<f32>(),
            f32::NEG_INFINITY
        );
        assert_eq!((*f32_max.as_neg()).borrow().az::<f32>(), -f32::MAX);
        assert_eq!((*two_point5.as_neg()).borrow().az::<f32>(), -2.5f32);
        assert_eq!((*one.as_neg()).borrow().az::<f32>(), -1f32);
        assert_eq!(
            (*f32_min_pos_normal.as_neg()).borrow().az::<f32>(),
            -f32::MIN_POSITIVE
        );
        assert_eq!(
            (*f32_min_pos_subnormal.as_neg()).borrow().az::<f32>(),
            -f32::from_bits(1)
        );
        assert_eq!((*zero.as_neg()).borrow().az::<f32>(), 0f32);
        assert!((*zero.as_neg()).borrow().az::<f32>().is_sign_negative());
        assert!(zero.borrow().az::<f32>().is_sign_positive());
        assert_eq!(zero.borrow().az::<f32>(), 0f32);
        assert_eq!(
            (*f32_min_pos_subnormal.as_neg()).borrow().az::<f32>(),
            -f32::from_bits(1)
        );
        assert_eq!(
            (*f32_min_pos_normal.as_neg()).borrow().az::<f32>(),
            -f32::MIN_POSITIVE
        );
        assert_eq!(one.borrow().az::<f32>(), 1f32);
        assert_eq!(two_point5.borrow().az::<f32>(), 2.5f32);
        assert_eq!(f32_max.borrow().az::<f32>(), f32::MAX);
        assert_eq!(f32_overflow.borrow().az::<f32>(), f32::INFINITY);

        assert_eq!(
            (*f64_overflow.as_neg()).borrow().az::<f64>(),
            f64::NEG_INFINITY
        );
        assert_eq!((*f64_max.as_neg()).borrow().az::<f64>(), -f64::MAX);
        assert_eq!((*two_point5.as_neg()).borrow().az::<f64>(), -2.5f64);
        assert_eq!((*one.as_neg()).borrow().az::<f64>(), -1f64);
        assert_eq!(
            (*f64_min_pos_normal.as_neg()).borrow().az::<f64>(),
            -f64::MIN_POSITIVE
        );
        assert_eq!(
            (*f64_min_pos_subnormal.as_neg()).borrow().az::<f64>(),
            -f64::from_bits(1)
        );
        assert_eq!((*zero.as_neg()).borrow().az::<f64>(), 0f64);
        assert!((*zero.as_neg()).borrow().az::<f64>().is_sign_negative());
        assert!(zero.borrow().az::<f64>().is_sign_positive());
        assert_eq!(zero.borrow().az::<f64>(), 0f64);
        assert_eq!(
            (*f64_min_pos_subnormal.as_neg()).borrow().az::<f64>(),
            -f64::from_bits(1)
        );
        assert_eq!(
            (*f64_min_pos_normal.as_neg()).borrow().az::<f64>(),
            -f64::MIN_POSITIVE
        );
        assert_eq!(one.borrow().az::<f64>(), 1f64);
        assert_eq!(two_point5.borrow().az::<f64>(), 2.5f64);
        assert_eq!(f64_max.borrow().az::<f64>(), f64::MAX);
        assert_eq!(f64_overflow.borrow().az::<f64>(), f64::INFINITY);
    }

    #[cfg(feature = "nightly-float")]
    #[test]
    fn check_issue_80() {
        let x = f16::MIN_POSITIVE * (2.0 - f16::EPSILON);
        let mut x = Float::with_val(11, x);
        x *= 0.5;
        assert_eq!(x.to_f16() as f32, x.to_f32() as f16 as f32);
        assert_eq!(x.to_f16(), f16::MIN_POSITIVE);
        x *= 0.5;
        assert_eq!(x.to_f16(), f16::MIN_POSITIVE / 2.0);
        x >>= f16::MANTISSA_DIGITS;
        assert_eq!(x.to_f16(), 0f16);

        let x = f128::MIN_POSITIVE * (2.0 - f128::EPSILON);
        let mut x = Float::with_val(f128::MANTISSA_DIGITS, x);
        x *= 0.5;
        assert_eq!(x.to_f128(), f128::MIN_POSITIVE);
        x *= 0.5;
        assert_eq!(x.to_f128(), f128::MIN_POSITIVE / 2.0);
        x >>= f128::MANTISSA_DIGITS;
        assert_eq!(x.to_f128(), 0f128);
    }
}
