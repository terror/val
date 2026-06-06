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
#[allow(deprecated)]
use crate::integer::SmallInteger;
use crate::integer::big;
use crate::integer::{MiniInteger, ParseIntegerError, TryFromIntegerError};
use crate::misc::StringLike;
use crate::{Assign, Integer};
use az::{Az, CheckedCast};
use core::error::Error;
use core::fmt::{
    Binary, Debug, Display, Formatter, LowerHex, Octal, Result as FmtResult, UpperHex,
};
use core::hash::{Hash, Hasher};
use core::mem;
use core::mem::MaybeUninit;
use core::str::FromStr;
use gmp_mpfr_sys::gmp::limb_t;

impl Default for Integer {
    #[inline]
    fn default() -> Integer {
        Integer::new()
    }
}

impl Clone for Integer {
    #[inline]
    fn clone(&self) -> Integer {
        unsafe {
            let mut dst = MaybeUninit::uninit();
            xmpz::init_set(dst.as_mut_ptr(), self);
            dst.assume_init()
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &Integer) {
        self.assign(source);
    }
}

impl Drop for Integer {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            xmpz::clear(self);
        }
    }
}

impl Hash for Integer {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().size.hash(state);
        self.inner_data().hash(state);
    }
}

impl Assign for Integer {
    #[inline]
    fn assign(&mut self, src: Integer) {
        drop(mem::replace(self, src));
    }
}

impl Assign<&Integer> for Integer {
    #[inline]
    fn assign(&mut self, src: &Integer) {
        xmpz::set(self, src);
    }
}

impl From<&Integer> for Integer {
    #[inline]
    fn from(val: &Integer) -> Self {
        val.clone()
    }
}

impl Assign<MiniInteger> for Integer {
    #[inline]
    fn assign(&mut self, mut src: MiniInteger) {
        self.assign(src.borrow_excl());
    }
}

impl Assign<&MiniInteger> for Integer {
    #[inline]
    fn assign(&mut self, src: &MiniInteger) {
        self.assign(&*src.borrow());
    }
}

impl From<MiniInteger> for Integer {
    #[inline]
    fn from(mut src: MiniInteger) -> Self {
        Integer::from(src.borrow_excl())
    }
}

impl From<&MiniInteger> for Integer {
    #[inline]
    fn from(src: &MiniInteger) -> Self {
        Integer::from(&*src.borrow())
    }
}

#[allow(deprecated)]
impl Assign<SmallInteger> for Integer {
    #[inline]
    fn assign(&mut self, src: SmallInteger) {
        self.assign(&*src);
    }
}

#[allow(deprecated)]
impl Assign<&SmallInteger> for Integer {
    #[inline]
    fn assign(&mut self, src: &SmallInteger) {
        self.assign(&**src);
    }
}

#[allow(deprecated)]
impl From<SmallInteger> for Integer {
    #[inline]
    fn from(src: SmallInteger) -> Self {
        Integer::from(&*src)
    }
}

#[allow(deprecated)]
impl From<&SmallInteger> for Integer {
    #[inline]
    fn from(src: &SmallInteger) -> Self {
        Integer::from(&**src)
    }
}

macro_rules! try_from {
    ($T:ty) => {
        impl TryFrom<Integer> for $T {
            type Error = TryFromIntegerError;
            #[inline]
            fn try_from(value: Integer) -> Result<Self, TryFromIntegerError> {
                TryFrom::try_from(&value)
            }
        }

        impl TryFrom<&Integer> for $T {
            type Error = TryFromIntegerError;
            #[inline]
            fn try_from(value: &Integer) -> Result<Self, TryFromIntegerError> {
                value
                    .checked_cast()
                    .ok_or(TryFromIntegerError { _unused: () })
            }
        }
    };
}

try_from! { i8 }
try_from! { i16 }
try_from! { i32 }
try_from! { i64 }
try_from! { i128 }
try_from! { isize }
try_from! { u8 }
try_from! { u16 }
try_from! { u32 }
try_from! { u64 }
try_from! { u128 }
try_from! { usize }

macro_rules! unsafe_assign {
    ($T:ty, $set:path, $init_set:path) => {
        impl Assign<$T> for Integer {
            #[inline]
            fn assign(&mut self, src: $T) {
                $set(self, src);
            }
        }

        impl Assign<&$T> for Integer {
            #[inline]
            fn assign(&mut self, src: &$T) {
                self.assign(*src);
            }
        }

        impl From<$T> for Integer {
            #[inline]
            fn from(src: $T) -> Self {
                unsafe {
                    let mut dst = MaybeUninit::uninit();
                    $init_set(dst.as_mut_ptr(), src);
                    dst.assume_init()
                }
            }
        }
    };
}

macro_rules! assign {
    ($T:ty as $U:ty) => {
        impl Assign<$T> for Integer {
            #[inline]
            fn assign(&mut self, src: $T) {
                self.assign(src.az::<$U>());
            }
        }

        impl Assign<&$T> for Integer {
            #[inline]
            fn assign(&mut self, src: &$T) {
                self.assign((*src).az::<$U>());
            }
        }

        impl From<$T> for Integer {
            #[inline]
            fn from(src: $T) -> Self {
                Integer::from(src.az::<$U>())
            }
        }
    };
}

assign! { i8 as i32 }
assign! { i16 as i32 }
unsafe_assign! { i32, xmpz::set_i32, xmpz::init_set_i32 }
unsafe_assign! { i64, xmpz::set_i64, xmpz::init_set_i64 }
unsafe_assign! { i128, xmpz::set_i128, xmpz::init_set_i128 }
#[cfg(target_pointer_width = "32")]
assign! { isize as i32 }
#[cfg(target_pointer_width = "64")]
assign! { isize as i64 }

assign! { bool as u32 }
assign! { u8 as u32 }
assign! { u16 as u32 }
unsafe_assign! { u32, xmpz::set_u32, xmpz::init_set_u32 }
unsafe_assign! { u64, xmpz::set_u64, xmpz::init_set_u64 }
unsafe_assign! { u128, xmpz::set_u128, xmpz::init_set_u128 }
#[cfg(target_pointer_width = "32")]
assign! { usize as u32 }
#[cfg(target_pointer_width = "64")]
assign! { usize as u64 }

impl FromStr for Integer {
    type Err = ParseIntegerError;
    #[inline]
    fn from_str(src: &str) -> Result<Integer, ParseIntegerError> {
        Ok(Integer::from(Integer::parse(src)?))
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_radix(self, f, 10, false, "")
    }
}

impl Debug for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_radix(self, f, 10, false, "")
    }
}

impl Binary for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_radix(self, f, 2, false, "0b")
    }
}

impl Octal for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_radix(self, f, 8, false, "0o")
    }
}

impl LowerHex for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_radix(self, f, 16, false, "0x")
    }
}

impl UpperHex for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_radix(self, f, 16, true, "0x")
    }
}

fn fmt_radix(
    i: &Integer,
    f: &mut Formatter<'_>,
    radix: i32,
    to_upper: bool,
    prefix: &str,
) -> FmtResult {
    let mut s = StringLike::new_malloc();
    big::append_to_string(&mut s, i, radix, to_upper);
    let st = s.as_str();
    let (neg, buf) = if let Some(stripped) = st.strip_prefix('-') {
        (true, stripped)
    } else {
        (false, st)
    };
    f.pad_integral(!neg, prefix, buf)
}

#[cfg(feature = "float")]
mod float_conv {
    use crate::float;
    use crate::{Float, Integer};
    use core::fmt::{Formatter, LowerExp, Result as FmtResult, UpperExp};

    fn float_from_int(i: &Integer) -> Float {
        let abs = i.as_abs();
        let mut prec = abs.significant_bits();
        // avoid copying trailing zeros
        if let Some(zeros) = abs.find_one(0) {
            prec -= zeros;
        }
        prec = prec.max(float::prec_min());
        Float::with_val(prec, i)
    }

    impl LowerExp for Integer {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            LowerExp::fmt(&float_from_int(self), f)
        }
    }

    impl UpperExp for Integer {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            UpperExp::fmt(&float_from_int(self), f)
        }
    }
}

/// Provides a reference to the underlying digits as
/// <code>\&[\[][slice][limb_t][`limb_t`][\]][slice]</code>. See
/// [`as_limbs`][Integer::as_limbs].
impl AsRef<[limb_t]> for Integer {
    #[inline]
    fn as_ref(&self) -> &[limb_t] {
        self.as_limbs()
    }
}

impl TryFromIntegerError {
    fn desc(&self) -> &str {
        "out of range conversion attempted"
    }
}

impl Display for TryFromIntegerError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(self.desc(), f)
    }
}

impl Error for TryFromIntegerError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        self.desc()
    }
}

// Safety: mpz_t is thread safe as guaranteed by the GMP library.
unsafe impl Send for Integer {}
unsafe impl Sync for Integer {}

#[cfg(test)]
mod tests {
    use crate::{Assign, Integer};

    #[test]
    fn check_assign() {
        let mut i = Integer::from(1);
        assert_eq!(i, 1);
        let other = Integer::from(2);
        i.assign(&other);
        assert_eq!(i, 2);
        i.assign(-other);
        assert_eq!(i, -2);
    }

    macro_rules! check_fallible_conversions_helper {
        ($int:ident, $bits:expr, $I:ty, $U:ty) => {{
            const I_MIN: $I = -1 << ($bits - 1);
            const I_MAX: $I = -1 - I_MIN;
            $int.assign(I_MIN);
            assert_eq!(<$I>::try_from(&$int).ok(), Some(I_MIN));
            $int -= 1;
            assert!(<$I>::try_from(&$int).is_err());
            $int.assign(I_MAX);
            assert_eq!(<$I>::try_from(&$int).ok(), Some(I_MAX));
            $int += 1;
            assert!(<$I>::try_from(&$int).is_err());

            const U_MIN: $U = 0;
            const U_MAX: $U = !0;
            $int.assign(U_MIN);
            assert_eq!(<$U>::try_from(&$int).ok(), Some(U_MIN));
            $int -= 1;
            assert!(<$U>::try_from(&$int).is_err());
            $int.assign(U_MAX);
            assert_eq!(<$U>::try_from(&$int).ok(), Some(U_MAX));
            $int += 1;
            assert!(<$U>::try_from(&$int).is_err());
        }};
    }

    #[test]
    fn check_fallible_conversions() {
        let mut int = Integer::new();
        check_fallible_conversions_helper!(int, 8, i8, u8);
        check_fallible_conversions_helper!(int, 16, i16, u16);
        check_fallible_conversions_helper!(int, 32, i32, u32);
        check_fallible_conversions_helper!(int, 64, i64, u64);
        check_fallible_conversions_helper!(int, 128, i128, u128);
    }
}
