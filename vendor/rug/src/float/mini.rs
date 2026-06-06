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

use crate::ext::xmpfr;
use crate::float::BorrowFloat;
use crate::float::{self, Special};
use crate::misc;
use crate::{Assign, Float};
use az::{Az, WrappingCast};
use core::ffi::c_int;
use core::fmt::{
    Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, Result as FmtResult, UpperExp,
    UpperHex,
};
use core::mem;
use core::mem::MaybeUninit;
#[allow(unused_imports)]
use core::ops::Deref;
use core::ptr::NonNull;
use gmp_mpfr_sys::gmp;
use gmp_mpfr_sys::gmp::limb_t;
use gmp_mpfr_sys::mpfr::{exp_t, mpfr_t, prec_t};

const LIMBS_IN_SMALL: usize = (128 / gmp::LIMB_BITS) as usize;
type Limbs = [MaybeUninit<limb_t>; LIMBS_IN_SMALL];

/**
A small float that does not require any memory allocation.

This can be useful when you have a primitive number type but need a reference to
a [`Float`]. The `MiniFloat` will have a precision according to the type of the
primitive used to set its value.

  * [`bool`]: the `MiniFloat` will have the [minimum possible
    precision][crate::float::prec_min].
  * [`i8`], [`u8`]: the `MiniFloat` will have eight bits of precision.
  * [`i16`], [`u16`]: the `MiniFloat` will have 16 bits of precision.
  * [`i32`], [`u32`]: the `MiniFloat` will have 32 bits of precision.
  * [`i64`], [`u64`]: the `MiniFloat` will have 64 bits of precision.
  * [`i128`], [`u128`]: the `MiniFloat` will have 128 bits of precision.
  * [`isize`], [`usize`]: the `MiniFloat` will have 32 or 64 bits of precision,
    depending on the platform.
  * [`f32`]: the `MiniFloat` will have 24 bits of precision.
  * [`f64`]: the `MiniFloat` will have 53 bits of precision.
  * [`Special`]: the `MiniFloat` will have the [minimum possible
    precision][crate::float::prec_min].

The [`borrow`][Self::borrow] method returns an object that can be coerced to a
[`Float`], as it implements
<code>[Deref]\<[Target][Deref::Target] = [Float]></code>.

# Examples

```rust
use rug::float::MiniFloat;
use rug::Float;
// `a` requires a heap allocation, has 53-bit precision
let mut a = Float::with_val(53, 250);
// `b` can reside on the stack
let b = MiniFloat::from(-100f64);
a += &*b.borrow();
assert_eq!(a, 150);
// another computation:
a *= &*b.borrow();
assert_eq!(a, -15000);
```
*/
#[derive(Clone, Copy)]
pub struct MiniFloat {
    pub(crate) inner: mpfr_t,
    pub(crate) limbs: Limbs,
}

static_assert!(mem::size_of::<Limbs>() == 16);

// SAFETY: mpfr_t is thread safe as guaranteed by the MPFR library.
unsafe impl Send for MiniFloat {}
unsafe impl Sync for MiniFloat {}

impl Default for MiniFloat {
    #[inline]
    fn default() -> Self {
        MiniFloat::new()
    }
}

impl Display for MiniFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&*self.borrow(), f)
    }
}

impl Debug for MiniFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&*self.borrow(), f)
    }
}

impl LowerExp for MiniFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerExp::fmt(&*self.borrow(), f)
    }
}

impl UpperExp for MiniFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperExp::fmt(&*self.borrow(), f)
    }
}

impl Binary for MiniFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Binary::fmt(&*self.borrow(), f)
    }
}

impl Octal for MiniFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Octal::fmt(&*self.borrow(), f)
    }
}

impl LowerHex for MiniFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerHex::fmt(&*self.borrow(), f)
    }
}

impl UpperHex for MiniFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperHex::fmt(&*self.borrow(), f)
    }
}

impl MiniFloat {
    /// Creates a [`MiniFloat`] with value 0 and the [minimum possible
    /// precision][crate::float::prec_min].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::MiniFloat;
    /// let f = MiniFloat::new();
    /// // Borrow f as if it were Float.
    /// assert_eq!(*f.borrow(), 0);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        MiniFloat {
            inner: mpfr_t {
                prec: float::prec_min() as prec_t,
                sign: 1,
                exp: xmpfr::EXP_ZERO,
                d: NonNull::dangling(),
            },
            limbs: small_limbs![],
        }
    }

    /// Creates a [`MiniFloat`] from a [`bool`].
    ///
    /// This is equivalent to `MiniFloat::from(val)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float;
    /// use rug::float::{BorrowFloat, MiniFloat};
    /// use rug::Float;
    ///
    /// const ONE_MINI: MiniFloat = MiniFloat::const_from_bool(true);
    /// const ONE_BORROW: BorrowFloat = ONE_MINI.borrow();
    /// const ONE: &Float = BorrowFloat::const_deref(&ONE_BORROW);
    /// assert_eq!(*ONE, 1);
    /// assert_eq!(ONE.prec(), float::prec_min());
    /// ```
    #[inline]
    pub const fn const_from_bool(val: bool) -> Self {
        let (prec, sign, exp, limbs) = from_bool(val);
        MiniFloat {
            inner: mpfr_t {
                prec,
                sign,
                exp,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniFloat`] from a [`i8`].
    ///
    /// This is equivalent to `MiniFloat::from(val)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::{BorrowFloat, MiniFloat};
    /// use rug::Float;
    ///
    /// const TWO_MINI: MiniFloat = MiniFloat::const_from_i8(2i8);
    /// const TWO_BORROW: BorrowFloat = TWO_MINI.borrow();
    /// const TWO: &Float = BorrowFloat::const_deref(&TWO_BORROW);
    /// assert_eq!(*TWO, 2);
    /// assert_eq!(TWO.prec(), i8::BITS);
    /// ```
    #[inline]
    pub const fn const_from_i8(val: i8) -> Self {
        let (prec, sign, exp, limbs) = from_i8(val);
        MiniFloat {
            inner: mpfr_t {
                prec,
                sign,
                exp,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniFloat`] from a [`i16`].
    ///
    /// This is equivalent to `MiniFloat::from(val)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::{BorrowFloat, MiniFloat};
    /// use rug::Float;
    ///
    /// const KIBI_MINI: MiniFloat = MiniFloat::const_from_i16(1i16 << 10);
    /// const KIBI_BORROW: BorrowFloat = KIBI_MINI.borrow();
    /// const KIBI: &Float = BorrowFloat::const_deref(&KIBI_BORROW);
    /// assert_eq!(*KIBI, 1i16 << 10);
    /// assert_eq!(KIBI.prec(), i16::BITS);
    /// ```
    #[inline]
    pub const fn const_from_i16(val: i16) -> Self {
        let (prec, sign, exp, limbs) = from_i16(val);
        MiniFloat {
            inner: mpfr_t {
                prec,
                sign,
                exp,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniFloat`] from a [`i32`].
    ///
    /// This is equivalent to `MiniFloat::from(val)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::{BorrowFloat, MiniFloat};
    /// use rug::Float;
    ///
    /// const MEBI_MINI: MiniFloat = MiniFloat::const_from_i32(1i32 << 20);
    /// const MEBI_BORROW: BorrowFloat = MEBI_MINI.borrow();
    /// const MEBI: &Float = BorrowFloat::const_deref(&MEBI_BORROW);
    /// assert_eq!(*MEBI, 1i32 << 20);
    /// assert_eq!(MEBI.prec(), i32::BITS);
    /// ```
    #[inline]
    pub const fn const_from_i32(val: i32) -> Self {
        let (prec, sign, exp, limbs) = from_i32(val);
        MiniFloat {
            inner: mpfr_t {
                prec,
                sign,
                exp,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniFloat`] from a [`i64`].
    ///
    /// This is equivalent to `MiniFloat::from(val)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::{BorrowFloat, MiniFloat};
    /// use rug::Float;
    ///
    /// const TEBI_MINI: MiniFloat = MiniFloat::const_from_i64(1i64 << 40);
    /// const TEBI_BORROW: BorrowFloat = TEBI_MINI.borrow();
    /// const TEBI: &Float = BorrowFloat::const_deref(&TEBI_BORROW);
    /// assert_eq!(*TEBI, 1i64 << 40);
    /// assert_eq!(TEBI.prec(), i64::BITS);
    /// ```
    #[inline]
    pub const fn const_from_i64(val: i64) -> Self {
        let (prec, sign, exp, limbs) = from_i64(val);
        MiniFloat {
            inner: mpfr_t {
                prec,
                sign,
                exp,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniFloat`] from a [`i128`].
    ///
    /// This is equivalent to `MiniFloat::from(val)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::{BorrowFloat, MiniFloat};
    /// use rug::Float;
    ///
    /// const YOBI_MINI: MiniFloat = MiniFloat::const_from_i128(1i128 << 80);
    /// const YOBI_BORROW: BorrowFloat = YOBI_MINI.borrow();
    /// const YOBI: &Float = BorrowFloat::const_deref(&YOBI_BORROW);
    /// assert_eq!(*YOBI, 1i128 << 80);
    /// assert_eq!(YOBI.prec(), i128::BITS);
    /// ```
    #[inline]
    pub const fn const_from_i128(val: i128) -> Self {
        let (prec, sign, exp, limbs) = from_i128(val);
        MiniFloat {
            inner: mpfr_t {
                prec,
                sign,
                exp,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniFloat`] from a [`isize`].
    ///
    /// This is equivalent to `MiniFloat::from(val)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::{BorrowFloat, MiniFloat};
    /// use rug::Float;
    ///
    /// const KIBI_MINI: MiniFloat = MiniFloat::const_from_isize(1isize << 10);
    /// const KIBI_BORROW: BorrowFloat = KIBI_MINI.borrow();
    /// const KIBI: &Float = BorrowFloat::const_deref(&KIBI_BORROW);
    /// assert_eq!(*KIBI, 1isize << 10);
    /// assert_eq!(KIBI.prec(), isize::BITS);
    /// ```
    #[inline]
    pub const fn const_from_isize(val: isize) -> Self {
        let (prec, sign, exp, limbs) = from_isize(val);
        MiniFloat {
            inner: mpfr_t {
                prec,
                sign,
                exp,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniFloat`] from a [`u8`].
    ///
    /// This is equivalent to `MiniFloat::from(val)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::{BorrowFloat, MiniFloat};
    /// use rug::Float;
    ///
    /// const TWO_MINI: MiniFloat = MiniFloat::const_from_u8(2u8);
    /// const TWO_BORROW: BorrowFloat = TWO_MINI.borrow();
    /// const TWO: &Float = BorrowFloat::const_deref(&TWO_BORROW);
    /// assert_eq!(*TWO, 2);
    /// assert_eq!(TWO.prec(), u8::BITS);
    /// ```
    #[inline]
    pub const fn const_from_u8(val: u8) -> Self {
        let (prec, sign, exp, limbs) = from_u8(val);
        MiniFloat {
            inner: mpfr_t {
                prec,
                sign,
                exp,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniFloat`] from a [`u16`].
    ///
    /// This is equivalent to `MiniFloat::from(val)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::{BorrowFloat, MiniFloat};
    /// use rug::Float;
    ///
    /// const KIBI_MINI: MiniFloat = MiniFloat::const_from_u16(1u16 << 10);
    /// const KIBI_BORROW: BorrowFloat = KIBI_MINI.borrow();
    /// const KIBI: &Float = BorrowFloat::const_deref(&KIBI_BORROW);
    /// assert_eq!(*KIBI, 1u16 << 10);
    /// assert_eq!(KIBI.prec(), u16::BITS);
    /// ```
    #[inline]
    pub const fn const_from_u16(val: u16) -> Self {
        let (prec, sign, exp, limbs) = from_u16(val);
        MiniFloat {
            inner: mpfr_t {
                prec,
                sign,
                exp,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniFloat`] from a [`u32`].
    ///
    /// This is equivalent to `MiniFloat::from(val)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::{BorrowFloat, MiniFloat};
    /// use rug::Float;
    ///
    /// const MEBI_MINI: MiniFloat = MiniFloat::const_from_u32(1u32 << 20);
    /// const MEBI_BORROW: BorrowFloat = MEBI_MINI.borrow();
    /// const MEBI: &Float = BorrowFloat::const_deref(&MEBI_BORROW);
    /// assert_eq!(*MEBI, 1u32 << 20);
    /// assert_eq!(MEBI.prec(), u32::BITS);
    /// ```
    #[inline]
    pub const fn const_from_u32(val: u32) -> Self {
        let (prec, sign, exp, limbs) = from_u32(val);
        MiniFloat {
            inner: mpfr_t {
                prec,
                sign,
                exp,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniFloat`] from a [`u64`].
    ///
    /// This is equivalent to `MiniFloat::from(val)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::{BorrowFloat, MiniFloat};
    /// use rug::Float;
    ///
    /// const TEBI_MINI: MiniFloat = MiniFloat::const_from_u64(1u64 << 40);
    /// const TEBI_BORROW: BorrowFloat = TEBI_MINI.borrow();
    /// const TEBI: &Float = BorrowFloat::const_deref(&TEBI_BORROW);
    /// assert_eq!(*TEBI, 1u64 << 40);
    /// assert_eq!(TEBI.prec(), u64::BITS);
    /// ```
    #[inline]
    pub const fn const_from_u64(val: u64) -> Self {
        let (prec, sign, exp, limbs) = from_u64(val);
        MiniFloat {
            inner: mpfr_t {
                prec,
                sign,
                exp,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniFloat`] from a [`u128`].
    ///
    /// This is equivalent to `MiniFloat::from(val)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::{BorrowFloat, MiniFloat};
    /// use rug::Float;
    ///
    /// const YOBI_MINI: MiniFloat = MiniFloat::const_from_u128(1u128 << 80);
    /// const YOBI_BORROW: BorrowFloat = YOBI_MINI.borrow();
    /// const YOBI: &Float = BorrowFloat::const_deref(&YOBI_BORROW);
    /// assert_eq!(*YOBI, 1u128 << 80);
    /// assert_eq!(YOBI.prec(), u128::BITS);
    /// ```
    #[inline]
    pub const fn const_from_u128(val: u128) -> Self {
        let (prec, sign, exp, limbs) = from_u128(val);
        MiniFloat {
            inner: mpfr_t {
                prec,
                sign,
                exp,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniFloat`] from a [`usize`].
    ///
    /// This is equivalent to `MiniFloat::from(val)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::{BorrowFloat, MiniFloat};
    /// use rug::Float;
    ///
    /// const KIBI_MINI: MiniFloat = MiniFloat::const_from_usize(1usize << 10);
    /// const KIBI_BORROW: BorrowFloat = KIBI_MINI.borrow();
    /// const KIBI: &Float = BorrowFloat::const_deref(&KIBI_BORROW);
    /// assert_eq!(*KIBI, 1usize << 10);
    /// assert_eq!(KIBI.prec(), usize::BITS);
    /// ```
    #[inline]
    pub const fn const_from_usize(val: usize) -> Self {
        let (prec, sign, exp, limbs) = from_usize(val);
        MiniFloat {
            inner: mpfr_t {
                prec,
                sign,
                exp,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    #[cfg(feature = "nightly-float")]
    /// Creates a [`MiniFloat`] from an [`f16`].
    ///
    /// This is equivalent to `MiniFloat::from(val)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![feature(f16)]
    ///
    /// use rug::float::{BorrowFloat, MiniFloat};
    /// use rug::Float;
    ///
    /// const TWO_HALF_MINI: MiniFloat = MiniFloat::const_from_f16(2.5);
    /// const TWO_HALF_BORROW: BorrowFloat = TWO_HALF_MINI.borrow();
    /// const TWO_HALF: &Float = BorrowFloat::const_deref(&TWO_HALF_BORROW);
    /// assert_eq!(*TWO_HALF, 2.5);
    /// assert_eq!(TWO_HALF.prec(), f16::MANTISSA_DIGITS);
    /// ```
    #[inline]
    pub const fn const_from_f16(val: f16) -> Self {
        let (prec, sign, exp, limbs) = from_f16(val);
        MiniFloat {
            inner: mpfr_t {
                prec,
                sign,
                exp,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniFloat`] from an [`f32`].
    ///
    /// This is equivalent to `MiniFloat::from(val)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::{BorrowFloat, MiniFloat};
    /// use rug::Float;
    ///
    /// const TWO_HALF_MINI: MiniFloat = MiniFloat::const_from_f32(2.5);
    /// const TWO_HALF_BORROW: BorrowFloat = TWO_HALF_MINI.borrow();
    /// const TWO_HALF: &Float = BorrowFloat::const_deref(&TWO_HALF_BORROW);
    /// assert_eq!(*TWO_HALF, 2.5);
    /// assert_eq!(TWO_HALF.prec(), f32::MANTISSA_DIGITS);
    /// ```
    #[inline]
    pub const fn const_from_f32(val: f32) -> Self {
        let (prec, sign, exp, limbs) = from_f32(val);
        MiniFloat {
            inner: mpfr_t {
                prec,
                sign,
                exp,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniFloat`] from an [`f64`].
    ///
    /// This is equivalent to `MiniFloat::from(val)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::{BorrowFloat, MiniFloat};
    /// use rug::Float;
    ///
    /// const TWO_HALF_MINI: MiniFloat = MiniFloat::const_from_f64(2.5);
    /// const TWO_HALF_BORROW: BorrowFloat = TWO_HALF_MINI.borrow();
    /// const TWO_HALF: &Float = BorrowFloat::const_deref(&TWO_HALF_BORROW);
    /// assert_eq!(*TWO_HALF, 2.5);
    /// assert_eq!(TWO_HALF.prec(), f64::MANTISSA_DIGITS);
    /// ```
    #[inline]
    pub const fn const_from_f64(val: f64) -> Self {
        let (prec, sign, exp, limbs) = from_f64(val);
        MiniFloat {
            inner: mpfr_t {
                prec,
                sign,
                exp,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    #[cfg(feature = "nightly-float")]
    /// Creates a [`MiniFloat`] from an [`f128`].
    ///
    /// This is equivalent to `MiniFloat::from(val)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![feature(f128)]
    ///
    /// use rug::float::{BorrowFloat, MiniFloat};
    /// use rug::Float;
    ///
    /// const TWO_HALF_MINI: MiniFloat = MiniFloat::const_from_f128(2.5);
    /// const TWO_HALF_BORROW: BorrowFloat = TWO_HALF_MINI.borrow();
    /// const TWO_HALF: &Float = BorrowFloat::const_deref(&TWO_HALF_BORROW);
    /// assert_eq!(*TWO_HALF, 2.5);
    /// assert_eq!(TWO_HALF.prec(), f128::MANTISSA_DIGITS);
    /// ```
    #[inline]
    pub const fn const_from_f128(val: f128) -> Self {
        let (prec, sign, exp, limbs) = from_f128(val);
        MiniFloat {
            inner: mpfr_t {
                prec,
                sign,
                exp,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniFloat`] from a [`Special`] value.
    ///
    /// This is equivalent to `MiniFloat::from(val)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float;
    /// use rug::float::{BorrowFloat, MiniFloat, Special};
    /// use rug::Float;
    ///
    /// const INF_MINI: MiniFloat = MiniFloat::const_from_special(Special::Infinity);
    /// const INF_BORROW: BorrowFloat = INF_MINI.borrow();
    /// const INF: &Float = BorrowFloat::const_deref(&INF_BORROW);
    /// assert!(INF.is_infinite());
    /// assert_eq!(INF.prec(), float::prec_min());
    /// ```
    #[inline]
    pub const fn const_from_special(val: Special) -> Self {
        let (prec, sign, exp, limbs) = from_special(val);
        MiniFloat {
            inner: mpfr_t {
                prec,
                sign,
                exp,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Returns a mutable reference to a [`Float`] for simple operations that do
    /// not need to change the precision of the number.
    ///
    /// # Safety
    ///
    /// It is undefined behavior modify the precision of the referenced
    /// [`Float`] or to swap it with another number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::MiniFloat;
    /// let mut f = MiniFloat::from(1.0f32);
    /// // addition does not change the precision
    /// unsafe {
    ///     *f.as_nonreallocating_float() += 2.0;
    /// }
    /// assert_eq!(*f.borrow(), 3.0);
    /// ```
    #[inline]
    pub unsafe fn as_nonreallocating_float(&mut self) -> &mut Float {
        // Update d to point to limbs.
        self.inner.d = NonNull::<[MaybeUninit<limb_t>]>::from(&self.limbs[..]).cast();
        let ptr = misc::cast_ptr_mut(&mut self.inner);
        // SAFETY: since inner.d points to the limbs, it is in a consistent state.
        unsafe { &mut *ptr }
    }

    /// Borrows the floating-point number.
    ///
    /// The returned object implements
    /// <code>[Deref]\<[Target][Deref::Target] = [Float]></code>.
    ///
    /// The borrow lasts until the returned object exits scope. Multiple borrows
    /// can be taken at the same time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::MiniFloat;
    /// use rug::Float;
    /// let f = MiniFloat::from(-13i32);
    /// let b = f.borrow();
    /// let abs_ref = b.abs_ref();
    /// assert_eq!(Float::with_val(53, abs_ref), 13);
    /// ```
    #[inline]
    pub const fn borrow(&self) -> BorrowFloat<'_> {
        // SAFETY: Since d points to the limbs, the mpfr_t is in a consistent
        // state. Also, the lifetime of the BorrowFloat is the lifetime of self,
        // which covers the limbs.
        let d: *const Limbs = &self.limbs;
        unsafe {
            BorrowFloat::from_raw(mpfr_t {
                prec: self.inner.prec,
                sign: self.inner.sign,
                exp: self.inner.exp,
                d: NonNull::new_unchecked(d.cast_mut().cast()),
            })
        }
    }

    /// Borrows the floating-point number exclusively.
    ///
    /// This is similar to the [`borrow`][Self::borrow] method, but it requires
    /// exclusive access to the underlying [`MiniFloat`]; the returned reference
    /// can however be shared. The exclusive access is required to reduce the
    /// amount of housekeeping necessary, providing a more efficient operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::MiniFloat;
    /// use rug::Float;
    /// let mut f = MiniFloat::from(-13i32);
    /// let b = f.borrow_excl();
    /// let abs_ref = b.abs_ref();
    /// assert_eq!(Float::with_val(53, abs_ref), 13);
    /// ```
    #[inline]
    pub fn borrow_excl(&mut self) -> &Float {
        // SAFETY: since the return is a const reference, there will be no reallocation
        unsafe { &*self.as_nonreallocating_float() }
    }
}

/// Types implementing this trait can be converted to [`MiniFloat`].
///
/// The following are implemented when `T` implements `ToMini`:
///   * <code>[Assign]\<T> for [MiniFloat]</code>
///   * <code>[From]\<T> for [MiniFloat]</code>
///
/// This trait is sealed and cannot be implemented for more types; it is
/// implemented for [`bool`], for the integer types [`i8`], [`i16`], [`i32`],
/// [`i64`], [`i128`], [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`]
/// and [`usize`], and for the floating-point types [`f32`] and [`f64`].
pub trait ToMini: SealedToMini {}

pub trait SealedToMini: Copy {
    fn copy(self, inner: &mut mpfr_t, limbs: &mut Limbs);
}

macro_rules! unsafe_signed {
    ($I:ty, $fn:ident, $fnu:ident) => {
        impl ToMini for $I {}

        impl SealedToMini for $I {
            #[inline]
            fn copy(self, inner: &mut mpfr_t, limbs: &mut Limbs) {
                self.unsigned_abs().copy(inner, limbs);
                if self < 0 {
                    inner.sign = -1;
                }
            }
        }

        #[inline]
        const fn $fn(val: $I) -> (prec_t, c_int, exp_t, Limbs) {
            let unsigned_abs = val.unsigned_abs();
            let (prec, mut sign, exp, limbs) = $fnu(unsigned_abs);
            if val < 0 {
                sign = -1;
            }
            (prec, sign, exp, limbs)
        }
    };
}

macro_rules! unsafe_unsigned_limb {
    ($U:ty, $fn:ident) => {
        impl ToMini for $U {}

        impl SealedToMini for $U {
            #[inline]
            fn copy(self, inner: &mut mpfr_t, limbs: &mut Limbs) {
                inner.prec = <$U>::BITS.az();
                inner.sign = 1;
                if self == 0 {
                    inner.exp = xmpfr::EXP_ZERO;
                } else {
                    let leading = self.leading_zeros();
                    let limb_leading = leading + (gmp::LIMB_BITS as u32) - <$U>::BITS;
                    inner.exp = (<$U>::BITS - leading) as exp_t;
                    limbs[0] = MaybeUninit::new(limb_t::from(self) << limb_leading);
                }
            }
        }

        #[inline]
        const fn $fn(val: $U) -> (prec_t, c_int, exp_t, Limbs) {
            let prec = <$U>::BITS as prec_t;
            let sign = 1;
            if val == 0 {
                (prec, sign, xmpfr::EXP_ZERO, small_limbs![])
            } else {
                let leading = val.leading_zeros();
                let limb_leading = leading + (gmp::LIMB_BITS as u32) - <$U>::BITS;
                let exp = (<$U>::BITS - leading) as exp_t;
                let limb = (val as limb_t) << limb_leading;
                (prec, sign, exp, small_limbs![limb])
            }
        }
    };
}

impl ToMini for bool {}

impl SealedToMini for bool {
    #[inline]
    fn copy(self, inner: &mut mpfr_t, limbs: &mut Limbs) {
        inner.prec = float::prec_min().az();
        inner.sign = 1;
        if !self {
            inner.exp = xmpfr::EXP_ZERO
        } else {
            inner.exp = 1;
            limbs[0] = MaybeUninit::new(1 << (limb_t::BITS - 1));
        }
    }
}

#[inline]
const fn from_bool(val: bool) -> (prec_t, c_int, exp_t, Limbs) {
    let prec = float::prec_min() as prec_t;
    let sign = 1;
    if !val {
        (prec, sign, xmpfr::EXP_ZERO, small_limbs![])
    } else {
        (prec, sign, 1, small_limbs![1 << (limb_t::BITS - 1)])
    }
}

unsafe_signed! { i8, from_i8, from_u8 }
unsafe_signed! { i16, from_i16, from_u16 }
unsafe_signed! { i32, from_i32, from_u32 }
unsafe_signed! { i64, from_i64, from_u64 }
unsafe_signed! { i128, from_i128, from_u128 }
unsafe_signed! { isize, from_isize, from_usize }

unsafe_unsigned_limb! { u8, from_u8 }
unsafe_unsigned_limb! { u16, from_u16 }
unsafe_unsigned_limb! { u32, from_u32 }
#[cfg(gmp_limb_bits_64)]
unsafe_unsigned_limb! { u64, from_u64 }

#[cfg(gmp_limb_bits_32)]
impl ToMini for u64 {}

#[cfg(gmp_limb_bits_32)]
impl SealedToMini for u64 {
    #[inline]
    fn copy(self, inner: &mut mpfr_t, limbs: &mut Limbs) {
        inner.prec = u64::BITS.az();
        inner.sign = 1;
        if self == 0 {
            inner.exp = xmpfr::EXP_ZERO;
        } else {
            let leading = self.leading_zeros();
            let sval = self << leading;
            inner.exp = (u64::BITS - leading) as exp_t;
            limbs[0] = MaybeUninit::new(sval.wrapping_cast());
            limbs[1] = MaybeUninit::new((sval >> 32).wrapping_cast());
        }
    }
}

#[cfg(gmp_limb_bits_32)]
#[inline]
const fn from_u64(val: u64) -> (prec_t, c_int, exp_t, Limbs) {
    let prec = u64::BITS as prec_t;
    let sign = 1;
    if val == 0 {
        (prec, sign, xmpfr::EXP_ZERO, small_limbs![])
    } else {
        let leading = val.leading_zeros();
        let sval = val << leading;
        let exp = (u64::BITS - leading) as exp_t;
        (
            prec,
            sign,
            exp,
            small_limbs![sval as limb_t, (sval >> 32) as limb_t],
        )
    }
}

impl ToMini for u128 {}

impl SealedToMini for u128 {
    #[inline]
    fn copy(self, inner: &mut mpfr_t, limbs: &mut Limbs) {
        inner.prec = u128::BITS.az();
        inner.sign = 1;
        if self == 0 {
            inner.exp = xmpfr::EXP_ZERO;
        } else {
            let leading = self.leading_zeros();
            let sval = self << leading;
            inner.exp = (u128::BITS - leading) as exp_t;
            #[cfg(gmp_limb_bits_64)]
            {
                limbs[0] = MaybeUninit::new(sval.wrapping_cast());
                limbs[1] = MaybeUninit::new((sval >> 64).wrapping_cast());
            }
            #[cfg(gmp_limb_bits_32)]
            {
                limbs[0] = MaybeUninit::new(sval.wrapping_cast());
                limbs[1] = MaybeUninit::new((sval >> 32).wrapping_cast());
                limbs[2] = MaybeUninit::new((sval >> 64).wrapping_cast());
                limbs[3] = MaybeUninit::new((sval >> 96).wrapping_cast());
            }
        }
    }
}

#[cfg(gmp_limb_bits_64)]
#[inline]
const fn from_u128(val: u128) -> (prec_t, c_int, exp_t, Limbs) {
    let prec = u128::BITS as prec_t;
    let sign = 1;
    if val == 0 {
        (prec, sign, xmpfr::EXP_ZERO, small_limbs![])
    } else {
        let leading = val.leading_zeros();
        let sval = val << leading;
        let exp = (u128::BITS - leading) as exp_t;
        (
            prec,
            sign,
            exp,
            small_limbs![sval as limb_t, (sval >> 64) as limb_t],
        )
    }
}

#[cfg(gmp_limb_bits_32)]
#[inline]
const fn from_u128(val: u128) -> (prec_t, c_int, exp_t, Limbs) {
    let prec = u128::BITS as prec_t;
    let sign = 1;
    if val == 0 {
        (prec, sign, xmpfr::EXP_ZERO, small_limbs![])
    } else {
        let leading = val.leading_zeros();
        let sval = val << leading;
        let exp = (u128::BITS - leading) as exp_t;
        (
            prec,
            sign,
            exp,
            small_limbs![
                sval as limb_t,
                (sval >> 32) as limb_t,
                (sval >> 64) as limb_t,
                (sval >> 96) as limb_t
            ],
        )
    }
}

impl ToMini for usize {}

impl SealedToMini for usize {
    #[inline]
    fn copy(self, inner: &mut mpfr_t, limbs: &mut Limbs) {
        #[cfg(target_pointer_width = "32")]
        {
            let val = self.az::<u32>();
            val.copy(inner, limbs);
        }
        #[cfg(target_pointer_width = "64")]
        {
            let val = self.az::<u64>();
            val.copy(inner, limbs);
        }
    }
}

#[cfg(target_pointer_width = "32")]
#[inline]
const fn from_usize(val: usize) -> (prec_t, c_int, exp_t, Limbs) {
    from_u32(val as u32)
}

#[cfg(target_pointer_width = "64")]
#[inline]
const fn from_usize(val: usize) -> (prec_t, c_int, exp_t, Limbs) {
    from_u64(val as u64)
}

macro_rules! from_float {
    (fn $method:ident($Float:ident); $limbs_for:ident($Uns:ident)) => {
        const fn $method(val: $Float) -> (prec_t, c_int, exp_t, Limbs) {
            const SIGN_MASK: $Uns = 1 << ($Uns::BITS - 1);
            const IMPLICIT: $Uns = 1 << ($Float::MANTISSA_DIGITS - 1);
            const MANT_MASK: $Uns = IMPLICIT - 1;
            const EXP_MASK: $Uns = !(SIGN_MASK | MANT_MASK);

            let val: $Uns = $Float::to_bits(val);
            let prec = $Float::MANTISSA_DIGITS as prec_t;
            let sign = if (val & SIGN_MASK) == 0 { 1 } else { -1 };
            let exp_bits = val & EXP_MASK;
            let mant_bits = val & MANT_MASK;
            match exp_bits {
                0 => {
                    if mant_bits == 0 {
                        // zero

                        (prec, sign, xmpfr::EXP_ZERO, small_limbs![])
                    } else {
                        // subnormal

                        // If val is equal to MANT_MASK, we have max subnormal.
                        // Minimum normal is 10000... with MIN_EXP.
                        // Maximum subnormal is 01111... with MIN_EXP.
                        // So for maximum subnormal, we need exp to be MIN_EXP - 1.
                        // For every other leading zero, we need exp to be smaller by 1.
                        const MAX_SUBNORMAL_LEADING: u32 = MANT_MASK.leading_zeros();
                        let leading = mant_bits.leading_zeros();
                        let exp = $Float::MIN_EXP as exp_t
                            - 1
                            - (leading - MAX_SUBNORMAL_LEADING) as exp_t;
                        let shifted = mant_bits << leading;
                        (prec, sign, exp, $limbs_for(shifted))
                    }
                }
                EXP_MASK => {
                    if mant_bits == 0 {
                        // inf

                        (prec, sign, xmpfr::EXP_INF, small_limbs![])
                    } else {
                        // NaN

                        (prec, sign, xmpfr::EXP_NAN, small_limbs![])
                    }
                }
                _ => {
                    // normal

                    // When biased_exp is 1, we want exp to be MIN_EXP.
                    let biased_exp = (exp_bits >> (prec - 1)) as exp_t;
                    let exp = biased_exp - 1 + $Float::MIN_EXP as exp_t;
                    let with_implicit = mant_bits | IMPLICIT;
                    let shifted = with_implicit << ($Uns::BITS - $Float::MANTISSA_DIGITS);
                    (prec, sign, exp, $limbs_for(shifted))
                }
            }
        }
    };
}

#[cfg(feature = "nightly-float")]
impl ToMini for f16 {}

#[cfg(feature = "nightly-float")]
impl SealedToMini for f16 {
    #[inline]
    fn copy(self, inner: &mut mpfr_t, limbs: &mut Limbs) {
        (inner.prec, inner.sign, inner.exp, *limbs) = from_f16(self);
    }
}

#[cfg(feature = "nightly-float")]
#[inline]
const fn limbs_for_16(mant_bits: u16) -> Limbs {
    let limb = (mant_bits as limb_t) << (limb_t::BITS - u16::BITS);
    small_limbs![limb]
}

#[cfg(feature = "nightly-float")]
from_float! { fn from_f16(f16); limbs_for_16(u16) }

impl ToMini for f32 {}

impl SealedToMini for f32 {
    #[inline]
    fn copy(self, inner: &mut mpfr_t, limbs: &mut Limbs) {
        (inner.prec, inner.sign, inner.exp, *limbs) = from_f32(self);
    }
}

#[inline]
const fn limbs_for_32(mant_bits: u32) -> Limbs {
    let limb = (mant_bits as limb_t) << (limb_t::BITS - u32::BITS);
    small_limbs![limb]
}

from_float! { fn from_f32(f32); limbs_for_32(u32) }

impl ToMini for f64 {}

impl SealedToMini for f64 {
    #[inline]
    fn copy(self, inner: &mut mpfr_t, limbs: &mut Limbs) {
        (inner.prec, inner.sign, inner.exp, *limbs) = from_f64(self);
    }
}

#[inline]
const fn limbs_for_64(mant_bits: u64) -> Limbs {
    #[cfg(gmp_limb_bits_64)]
    {
        small_limbs![mant_bits]
    }
    #[cfg(gmp_limb_bits_32)]
    {
        small_limbs![mant_bits as limb_t, (mant_bits >> 32) as limb_t]
    }
}

from_float! { fn from_f64(f64); limbs_for_64(u64) }

#[cfg(feature = "nightly-float")]
impl ToMini for f128 {}

#[cfg(feature = "nightly-float")]
impl SealedToMini for f128 {
    #[inline]
    fn copy(self, inner: &mut mpfr_t, limbs: &mut Limbs) {
        (inner.prec, inner.sign, inner.exp, *limbs) = from_f128(self);
    }
}

#[cfg(feature = "nightly-float")]
#[inline]
const fn limbs_for_128(mant_bits: u128) -> Limbs {
    #[cfg(gmp_limb_bits_64)]
    {
        small_limbs![mant_bits as limb_t, (mant_bits >> 64) as limb_t]
    }
    #[cfg(gmp_limb_bits_32)]
    {
        small_limbs![
            mant_bits as limb_t,
            (mant_bits >> 32) as limb_t,
            (mant_bits >> 64) as limb_t,
            (mant_bits >> 96) as limb_t
        ]
    }
}

#[cfg(feature = "nightly-float")]
from_float! { fn from_f128(f128); limbs_for_128(u128) }

impl ToMini for Special {}

impl SealedToMini for Special {
    #[inline]
    fn copy(self, inner: &mut mpfr_t, _limbs: &mut Limbs) {
        inner.prec = float::prec_min().az();
        (inner.sign, inner.exp) = match self {
            Special::Zero => (1, xmpfr::EXP_ZERO),
            Special::NegZero => (-1, xmpfr::EXP_ZERO),
            Special::Infinity => (1, xmpfr::EXP_INF),
            Special::NegInfinity => (-1, xmpfr::EXP_INF),
            Special::Nan => (1, xmpfr::EXP_NAN),
        }
    }
}

#[inline]
const fn from_special(val: Special) -> (prec_t, c_int, exp_t, Limbs) {
    let prec = float::prec_min() as prec_t;
    match val {
        Special::Zero => (prec, 1, xmpfr::EXP_ZERO, small_limbs![]),
        Special::NegZero => (prec, -1, xmpfr::EXP_ZERO, small_limbs![]),
        Special::Infinity => (prec, 1, xmpfr::EXP_INF, small_limbs![]),
        Special::NegInfinity => (prec, -1, xmpfr::EXP_INF, small_limbs![]),
        Special::Nan => (prec, 1, xmpfr::EXP_NAN, small_limbs![]),
    }
}

impl<T: ToMini> Assign<T> for MiniFloat {
    #[inline]
    fn assign(&mut self, src: T) {
        src.copy(&mut self.inner, &mut self.limbs);
    }
}

impl<T: ToMini> From<T> for MiniFloat {
    #[inline]
    fn from(src: T) -> Self {
        let mut inner = mpfr_t {
            prec: 0,
            sign: 0,
            exp: 0,
            d: NonNull::dangling(),
        };
        let mut limbs = small_limbs![];
        src.copy(&mut inner, &mut limbs);
        MiniFloat { inner, limbs }
    }
}

impl Assign<&Self> for MiniFloat {
    #[inline]
    fn assign(&mut self, other: &Self) {
        self.clone_from(other);
    }
}

impl Assign for MiniFloat {
    #[inline]
    fn assign(&mut self, other: Self) {
        *self = other;
    }
}

#[inline]
pub(crate) unsafe fn unchecked_get_unshifted_u8(small: &MiniFloat) -> u8 {
    debug_assert!(small.borrow().prec() >= 8);
    debug_assert!(small.borrow().is_normal());
    (unsafe { small.limbs[0].assume_init() } >> (gmp::LIMB_BITS - 8)).wrapping_cast()
}

#[inline]
pub(crate) unsafe fn unchecked_get_unshifted_u16(small: &MiniFloat) -> u16 {
    debug_assert!(small.borrow().prec() >= 16);
    debug_assert!(small.borrow().is_normal());
    (unsafe { small.limbs[0].assume_init() } >> (gmp::LIMB_BITS - 16)).wrapping_cast()
}

#[inline]
pub(crate) unsafe fn unchecked_get_unshifted_u32(small: &MiniFloat) -> u32 {
    debug_assert!(small.borrow().prec() >= 32);
    debug_assert!(small.borrow().is_normal());
    #[cfg(gmp_limb_bits_32)]
    {
        unsafe { small.limbs[0].assume_init() }
    }
    #[cfg(gmp_limb_bits_64)]
    {
        (unsafe { small.limbs[0].assume_init() } >> 32).wrapping_cast()
    }
}

#[inline]
pub(crate) unsafe fn unchecked_get_unshifted_u64(small: &MiniFloat) -> u64 {
    debug_assert!(small.borrow().prec() >= 64);
    debug_assert!(small.borrow().is_normal());
    #[cfg(gmp_limb_bits_32)]
    {
        u64::from(unsafe { small.limbs[0].assume_init() })
            | (u64::from(unsafe { small.limbs[1].assume_init() }) << 32)
    }
    #[cfg(gmp_limb_bits_64)]
    {
        unsafe { small.limbs[0].assume_init() }
    }
}

#[inline]
pub(crate) unsafe fn unchecked_get_unshifted_u128(small: &MiniFloat) -> u128 {
    debug_assert!(small.borrow().prec() >= 128);
    debug_assert!(small.borrow().is_normal());
    #[cfg(gmp_limb_bits_32)]
    {
        u128::from(unsafe { small.limbs[0].assume_init() })
            | (u128::from(unsafe { small.limbs[1].assume_init() }) << 32)
            | (u128::from(unsafe { small.limbs[2].assume_init() }) << 64)
            | (u128::from(unsafe { small.limbs[3].assume_init() }) << 96)
    }
    #[cfg(gmp_limb_bits_64)]
    {
        u128::from(unsafe { small.limbs[0].assume_init() })
            | (u128::from(unsafe { small.limbs[1].assume_init() }) << 64)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use crate::float;
    use crate::float::{FreeCache, MiniFloat, Special};
    use crate::{Assign, Float};

    #[test]
    fn check_assign() {
        let mut f = MiniFloat::from(-1.0f32);
        assert_eq!(*f.borrow_excl(), -1.0);
        f.assign(-2.0f64);
        assert_eq!(*f.borrow_excl(), -2.0);
        let other = MiniFloat::from(4u8);
        f.assign(&other);
        assert_eq!(*f.borrow_excl(), 4);
        f.assign(5i8);
        assert_eq!(*f.borrow_excl(), 5);
        f.assign(other);
        assert_eq!(*f.borrow_excl(), 4);
        f.assign(6u16);
        assert_eq!(*f.borrow_excl(), 6);
        f.assign(-6i16);
        assert_eq!(*f.borrow_excl(), -6);
        f.assign(6u32);
        assert_eq!(*f.borrow_excl(), 6);
        f.assign(-6i32);
        assert_eq!(*f.borrow_excl(), -6);
        f.assign(6u64);
        assert_eq!(*f.borrow_excl(), 6);
        f.assign(-6i64);
        assert_eq!(*f.borrow_excl(), -6);
        f.assign(6u128);
        assert_eq!(*f.borrow_excl(), 6);
        f.assign(-6i128);
        assert_eq!(*f.borrow_excl(), -6);
        f.assign(6usize);
        assert_eq!(*f.borrow_excl(), 6);
        f.assign(-6isize);
        assert_eq!(*f.borrow_excl(), -6);
        f.assign(0u32);
        assert_eq!(*f.borrow_excl(), 0);
        f.assign(Special::Infinity);
        assert!(f.borrow_excl().is_infinite() && f.borrow_excl().is_sign_positive());
        f.assign(Special::NegZero);
        assert!(f.borrow_excl().is_zero() && f.borrow_excl().is_sign_negative());
        f.assign(Special::NegInfinity);
        assert!(f.borrow_excl().is_infinite() && f.borrow_excl().is_sign_negative());
        f.assign(Special::Zero);
        assert!(f.borrow_excl().is_zero() && f.borrow_excl().is_sign_positive());
        f.assign(Special::Nan);
        assert!(f.borrow_excl().is_nan());

        float::free_cache(FreeCache::All);
    }

    #[test]
    fn check_traits() {
        assert!(MiniFloat::default().borrow_excl().is_zero());

        let mini = MiniFloat::from(-5.2f64);
        let check = Float::with_val(53, -5.2f64);
        assert_eq!(format!("{mini}"), format!("{check}"));
        assert_eq!(format!("{mini:?}"), format!("{check:?}"));
        assert_eq!(format!("{mini:e}"), format!("{check:e}"));
        assert_eq!(format!("{mini:E}"), format!("{check:E}"));
        assert_eq!(format!("{mini:b}"), format!("{check:b}"));
        assert_eq!(format!("{mini:o}"), format!("{check:o}"));
        assert_eq!(format!("{mini:x}"), format!("{check:x}"));
        assert_eq!(format!("{mini:X}"), format!("{check:X}"));
    }

    #[cfg(feature = "nightly-float")]
    #[test]
    fn check_from_f16() {
        let vals = [
            0.0,
            -0.0,
            1.0,
            core::f16::consts::PI,
            f16::MIN,
            f16::MAX,
            f16::INFINITY,
            f16::NEG_INFINITY,
            f16::MIN_POSITIVE,
            -f16::MIN_POSITIVE,
            f16::from_bits(1),
            -f16::from_bits(1),
            f16::MIN_POSITIVE - f16::from_bits(1),
            f16::from_bits(1) - f16::MIN_POSITIVE,
        ];
        for &val in &vals {
            let mut mini = MiniFloat::const_from_f16(val);
            let f = mini.borrow_excl();
            assert_eq!(*f, val);
            assert_eq!(f.is_sign_positive(), val.is_sign_positive());
            assert_eq!(f.to_f16(), val);
        }
        let mut mini = MiniFloat::const_from_f16(f16::NAN);
        let f = mini.borrow_excl();
        assert!(f.is_nan());
        assert_eq!(f.is_sign_positive(), f16::NAN.is_sign_positive());
        let mut mini = MiniFloat::const_from_f16(-f16::NAN);
        let f = mini.borrow_excl();
        assert!(f.is_nan());
        assert_ne!(f.is_sign_positive(), f16::NAN.is_sign_positive());
    }

    #[test]
    fn check_from_f32() {
        let vals = [
            0.0,
            -0.0,
            1.0,
            core::f32::consts::PI,
            f32::MIN,
            f32::MAX,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::MIN_POSITIVE,
            -f32::MIN_POSITIVE,
            f32::from_bits(1),
            -f32::from_bits(1),
            f32::MIN_POSITIVE - f32::from_bits(1),
            f32::from_bits(1) - f32::MIN_POSITIVE,
        ];
        for &val in &vals {
            let mut mini = MiniFloat::const_from_f32(val);
            let f = mini.borrow_excl();
            assert_eq!(*f, val);
            assert_eq!(f.is_sign_positive(), val.is_sign_positive());
            assert_eq!(f.to_f32(), val);
        }
        let mut mini = MiniFloat::const_from_f32(f32::NAN);
        let f = mini.borrow_excl();
        assert!(f.is_nan());
        assert_eq!(f.is_sign_positive(), f32::NAN.is_sign_positive());
        let mut mini = MiniFloat::const_from_f32(-f32::NAN);
        let f = mini.borrow_excl();
        assert!(f.is_nan());
        assert_ne!(f.is_sign_positive(), f32::NAN.is_sign_positive());
    }

    #[test]
    fn check_from_f64() {
        let vals = [
            0.0,
            -0.0,
            1.0,
            core::f64::consts::PI,
            f64::MIN,
            f64::MAX,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::MIN_POSITIVE,
            -f64::MIN_POSITIVE,
            f64::from_bits(1),
            -f64::from_bits(1),
            f64::MIN_POSITIVE - f64::from_bits(1),
            f64::from_bits(1) - f64::MIN_POSITIVE,
        ];
        for &val in &vals {
            let mut mini = MiniFloat::const_from_f64(val);
            let f = mini.borrow_excl();
            assert_eq!(*f, val);
            assert_eq!(f.is_sign_positive(), val.is_sign_positive());
            assert_eq!(f.to_f64(), val);
        }
        let mut mini = MiniFloat::const_from_f64(f64::NAN);
        let f = mini.borrow_excl();
        assert!(f.is_nan());
        assert_eq!(f.is_sign_positive(), f64::NAN.is_sign_positive());
        let mut mini = MiniFloat::const_from_f64(-f64::NAN);
        let f = mini.borrow_excl();
        assert!(f.is_nan());
        assert_ne!(f.is_sign_positive(), f64::NAN.is_sign_positive());
    }

    #[cfg(feature = "nightly-float")]
    #[test]
    fn check_from_f128() {
        let vals = [
            0.0,
            -0.0,
            1.0,
            core::f128::consts::PI,
            f128::MIN,
            f128::MAX,
            f128::INFINITY,
            f128::NEG_INFINITY,
            f128::MIN_POSITIVE,
            -f128::MIN_POSITIVE,
            f128::from_bits(1),
            -f128::from_bits(1),
            f128::MIN_POSITIVE - f128::from_bits(1),
            f128::from_bits(1) - f128::MIN_POSITIVE,
        ];
        for &val in &vals {
            let mut mini = MiniFloat::const_from_f128(val);
            let f = mini.borrow_excl();
            assert_eq!(*f, val);
            assert_eq!(f.is_sign_positive(), val.is_sign_positive());
            assert_eq!(f.to_f128(), val);
        }
        let mut mini = MiniFloat::const_from_f128(f128::NAN);
        let f = mini.borrow_excl();
        assert!(f.is_nan());
        assert_eq!(f.is_sign_positive(), f128::NAN.is_sign_positive());
        let mut mini = MiniFloat::const_from_f128(-f128::NAN);
        let f = mini.borrow_excl();
        assert!(f.is_nan());
        assert_ne!(f.is_sign_positive(), f128::NAN.is_sign_positive());
    }

    macro_rules! compare_conv {
        ($T:ident, $prec:expr, $fn:ident, [$($val:expr),+] $( as $U:ident)?) => {
            for &val in &[$($val),+] {
                let a = MiniFloat::from(val);
                let b = MiniFloat::$fn(val);
                let mut c = MiniFloat::new();
                c.assign(val);
                assert_eq!(*a.borrow(), val $(as $U)?);
                assert_eq!(*b.borrow(), val $(as $U)?);
                assert_eq!(*c.borrow(), val $(as $U)?);
                assert_eq!(a.borrow().prec(), $prec);
                assert_eq!(b.borrow().prec(), $prec);
                assert_eq!(c.borrow().prec(), $prec);
                assert_eq!(a.borrow().is_sign_positive(), b.borrow().is_sign_positive());
                assert_eq!(a.borrow().is_sign_positive(), c.borrow().is_sign_positive());
            }
        };
    }

    #[test]
    fn check_equiv_convs() {
        compare_conv!(bool, 1, const_from_bool, [false, true] as u8);
        compare_conv!(i8, i8::BITS, const_from_i8, [i8::MIN, 0, i8::MAX]);
        compare_conv!(i16, i16::BITS, const_from_i16, [i16::MIN, 0, i16::MAX]);
        compare_conv!(i32, i32::BITS, const_from_i32, [i32::MIN, 0, i32::MAX]);
        compare_conv!(i64, i64::BITS, const_from_i64, [i64::MIN, 0, i64::MAX]);
        compare_conv!(i128, i128::BITS, const_from_i128, [i128::MIN, 0, i128::MAX]);
        compare_conv!(
            isize,
            isize::BITS,
            const_from_isize,
            [isize::MIN, 0, isize::MAX]
        );
        compare_conv!(u8, u8::BITS, const_from_u8, [0, u8::MAX]);
        compare_conv!(u16, u16::BITS, const_from_u16, [0, u16::MAX]);
        compare_conv!(u32, u32::BITS, const_from_u32, [0, u32::MAX]);
        compare_conv!(u64, u64::BITS, const_from_u64, [0, u64::MAX]);
        compare_conv!(u128, u128::BITS, const_from_u128, [0, u128::MAX]);
        compare_conv!(usize, usize::BITS, const_from_usize, [0, usize::MAX]);
        compare_conv!(
            f32,
            f32::MANTISSA_DIGITS,
            const_from_f32,
            [f32::MIN, 0.0, f32::MAX, f32::INFINITY]
        );
        compare_conv!(
            f64,
            f64::MANTISSA_DIGITS,
            const_from_f64,
            [f64::MIN, 0.0, f64::MAX, f64::INFINITY]
        );
        compare_conv!(
            Special,
            1,
            const_from_special,
            [Special::NegZero, Special::Infinity]
        );
    }
}
