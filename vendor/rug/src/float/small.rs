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

#![allow(deprecated)]

use crate::float::{BorrowFloat, MiniFloat, ToMini};
use crate::{Assign, Float};
use core::fmt::{Debug, Formatter, Result as FmtResult};
use core::marker::PhantomData;
use core::ops::Deref;
use gmp_mpfr_sys::gmp::limb_t;

static ZERO_MINI: MiniFloat = MiniFloat::new();
static ZERO_BORROW: BorrowFloat = ZERO_MINI.borrow();
static ZERO: &Float = BorrowFloat::const_deref(&ZERO_BORROW);

/**
A small float that did not require any memory allocation until version 1.23.0.

Because of a [soundness issue], this has been deprecated and replaced by
[`MiniFloat`]. To fix the soundness issue, this struct now uses allocations
like [`Float`] itself, so it is less efficient than [`MiniFloat`].

The `SmallFloat` type can be coerced to a [`Float`], as it implements
<code>[Deref]\<[Target][Deref::Target] = [Float]></code>.

# Examples

```rust
#![allow(deprecated)]

use rug::float::SmallFloat;
use rug::Float;
// `a` requires a heap allocation, has 53-bit precision
let mut a = Float::with_val(53, 250);
// `b` can reside on the stack
let b = SmallFloat::from(-100f64);
a += &*b;
assert_eq!(a, 150);
// another computation:
a *= &*b;
assert_eq!(a, -15000);
```

[soundness issue]: https://gitlab.com/tspiteri/rug/-/issues/52
*/
#[deprecated(since = "1.23.0", note = "use `MiniFloat` instead")]
#[derive(Clone)]
pub struct SmallFloat {
    inner: Option<Float>,
    // for !Sync
    phantom: PhantomData<*const limb_t>,
}

unsafe impl Send for SmallFloat {}

impl Default for SmallFloat {
    #[inline]
    fn default() -> Self {
        SmallFloat::new()
    }
}

impl Debug for SmallFloat {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.inner {
            Some(fl) => Debug::fmt(fl, f),
            None => Debug::fmt(ZERO, f),
        }
    }
}

impl SmallFloat {
    /// Creates a [`SmallFloat`] with value 0 and the [minimum possible
    /// precision][crate::float::prec_min].
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![allow(deprecated)]
    ///
    /// use rug::float::SmallFloat;
    /// let f = SmallFloat::new();
    /// // Borrow f as if it were Float.
    /// assert_eq!(*f, 0);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        SmallFloat {
            inner: None,
            phantom: PhantomData,
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
    /// #![allow(deprecated)]
    ///
    /// use rug::float::SmallFloat;
    /// let mut f = SmallFloat::from(1.0f32);
    /// // addition does not change the precision
    /// unsafe {
    ///     *f.as_nonreallocating_float() += 2.0;
    /// }
    /// assert_eq!(*f, 3.0);
    /// ```
    #[inline]
    pub unsafe fn as_nonreallocating_float(&mut self) -> &mut Float {
        if self.inner.is_none() {
            *self = SmallFloat {
                inner: Some(Float::new(ZERO.prec())),
                phantom: PhantomData,
            };
        }
        match &mut self.inner {
            Some(f) => f,
            None => unreachable!(),
        }
    }
}

impl Deref for SmallFloat {
    type Target = Float;
    #[inline]
    fn deref(&self) -> &Float {
        match &self.inner {
            Some(f) => f,
            None => ZERO,
        }
    }
}

/// Types implementing this trait can be converted to [`SmallFloat`].
///
/// The following are implemented when `T` implements `ToSmall`:
///   * <code>[Assign]\<T> for [SmallFloat]</code>
///   * <code>[From]\<T> for [SmallFloat]</code>
///
/// This trait is sealed and cannot be implemented for more types; it is
/// implemented for [`bool`], for the integer types [`i8`], [`i16`], [`i32`],
/// [`i64`], [`i128`], [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`]
/// and [`usize`], and for the floating-point types [`f32`] and [`f64`].
#[deprecated(since = "1.23.0", note = "`ToMini` instead")]
pub trait ToSmall: ToMini {}
impl<T: ToMini> ToSmall for T {}

impl<T: ToSmall> Assign<T> for SmallFloat {
    #[inline]
    fn assign(&mut self, src: T) {
        let mut mini = MiniFloat::from(src);
        unsafe {
            let dst = self.as_nonreallocating_float();
            let src = mini.borrow_excl();
            dst.set_prec(src.prec());
            dst.assign(src);
        }
    }
}

impl<T: ToSmall> From<T> for SmallFloat {
    #[inline]
    fn from(src: T) -> Self {
        let mut mini = MiniFloat::from(src);
        let src = mini.borrow_excl();
        SmallFloat {
            inner: Some(Float::with_val(src.prec(), src)),
            phantom: PhantomData,
        }
    }
}

impl Assign<&Self> for SmallFloat {
    #[inline]
    fn assign(&mut self, other: &Self) {
        self.clone_from(other);
    }
}

impl Assign for SmallFloat {
    #[inline]
    fn assign(&mut self, other: Self) {
        *self = other;
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use crate::Assign;
    use crate::float;
    use crate::float::{FreeCache, SmallFloat, Special};

    #[test]
    fn check_assign() {
        let mut f = SmallFloat::from(-1.0f32);
        assert_eq!(*f, -1.0);
        f.assign(-2.0f64);
        assert_eq!(*f, -2.0);
        let other = SmallFloat::from(4u8);
        f.assign(&other);
        assert_eq!(*f, 4);
        f.assign(5i8);
        assert_eq!(*f, 5);
        f.assign(other);
        assert_eq!(*f, 4);
        f.assign(6u16);
        assert_eq!(*f, 6);
        f.assign(-6i16);
        assert_eq!(*f, -6);
        f.assign(6u32);
        assert_eq!(*f, 6);
        f.assign(-6i32);
        assert_eq!(*f, -6);
        f.assign(6u64);
        assert_eq!(*f, 6);
        f.assign(-6i64);
        assert_eq!(*f, -6);
        f.assign(6u128);
        assert_eq!(*f, 6);
        f.assign(-6i128);
        assert_eq!(*f, -6);
        f.assign(6usize);
        assert_eq!(*f, 6);
        f.assign(-6isize);
        assert_eq!(*f, -6);
        f.assign(0u32);
        assert_eq!(*f, 0);
        f.assign(Special::Infinity);
        assert!(f.is_infinite() && f.is_sign_positive());
        f.assign(Special::NegZero);
        assert!(f.is_zero() && f.is_sign_negative());
        f.assign(Special::NegInfinity);
        assert!(f.is_infinite() && f.is_sign_negative());
        f.assign(Special::Zero);
        assert!(f.is_zero() && f.is_sign_positive());
        f.assign(Special::Nan);
        assert!(f.is_nan());

        float::free_cache(FreeCache::All);
    }
}
