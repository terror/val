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

use crate::integer::{MiniInteger, ToMini};
use crate::{Assign, Integer};
use core::fmt::{Debug, Formatter, Result as FmtResult};
use core::marker::PhantomData;
use core::ops::Deref;
use gmp_mpfr_sys::gmp::limb_t;

/**
A small integer that did not require any memory allocation until version 1.23.0.

Because of a [soundness issue], this has been deprecated and replaced by
[`MiniInteger`]. To fix the soundness issue, this struct now uses allocations
like [`Integer`] itself, so it is less efficient than [`MiniInteger`].

The `SmallInteger` type can be coerced to an [`Integer`], as it implements
<code>[Deref]\<[Target][Deref::Target] = [Integer]></code>.

# Examples

```rust
#![allow(deprecated)]

use rug::integer::SmallInteger;
use rug::Integer;
// `a` requires a heap allocation
let mut a = Integer::from(250);
// `b` can reside on the stack
let b = SmallInteger::from(-100);
a.lcm_mut(&b);
assert_eq!(a, 500);
// another computation:
a.lcm_mut(&SmallInteger::from(30));
assert_eq!(a, 1500);
```

[soundness issue]: https://gitlab.com/tspiteri/rug/-/issues/52
*/
#[deprecated(since = "1.23.0", note = "use `MiniInteger` instead")]
#[repr(transparent)]
#[derive(Clone)]
pub struct SmallInteger {
    inner: Integer,
    // for !Sync
    phantom: PhantomData<*const limb_t>,
}

unsafe impl Send for SmallInteger {}

impl Default for SmallInteger {
    #[inline]
    fn default() -> Self {
        SmallInteger::new()
    }
}

impl Debug for SmallInteger {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&self.inner, f)
    }
}

impl SmallInteger {
    /// Creates a [`SmallInteger`] with value 0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![allow(deprecated)]
    ///
    /// use rug::integer::SmallInteger;
    /// let i = SmallInteger::new();
    /// // Borrow i as if it were Integer.
    /// assert_eq!(*i, 0);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        SmallInteger {
            inner: Integer::new(),
            phantom: PhantomData,
        }
    }

    /// Returns a mutable reference to an [`Integer`] for simple operations that
    /// do not need to allocate more space for the number.
    ///
    /// # Safety
    ///
    /// It is undefined behavior to perform operations that reallocate the
    /// internal data of the referenced [`Integer`] or to swap it with another
    /// number.
    ///
    /// Some GMP functions swap the allocations of their target operands;
    /// calling such functions with the mutable reference returned by this
    /// method can lead to undefined behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![allow(deprecated)]
    ///
    /// use rug::integer::SmallInteger;
    /// use rug::Assign;
    /// let mut i = SmallInteger::from(1u64);
    /// let capacity = i.capacity();
    /// // another u64 will not require a reallocation
    /// unsafe {
    ///     i.as_nonreallocating_integer().assign(2u64);
    /// }
    /// assert_eq!(*i, 2);
    /// assert_eq!(i.capacity(), capacity);
    /// ```
    #[inline]
    pub unsafe fn as_nonreallocating_integer(&mut self) -> &mut Integer {
        &mut self.inner
    }
}

impl Deref for SmallInteger {
    type Target = Integer;
    #[inline]
    fn deref(&self) -> &Integer {
        &self.inner
    }
}

/// Types implementing this trait can be converted to [`SmallInteger`].
///
/// The following are implemented when `T` implements `ToSmall`:
///   * <code>[Assign][`Assign`]\<T> for [SmallInteger][`SmallInteger`]</code>
///   * <code>[From][`From`]\<T> for [SmallInteger][`SmallInteger`]</code>
///
/// This trait is sealed and cannot be implemented for more types; it is
/// implemented for [`bool`] and for the integer types [`i8`], [`i16`], [`i32`],
/// [`i64`], [`i128`], [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`]
/// and [`usize`].
#[deprecated(since = "1.23.0", note = "`ToMini` instead")]
pub trait ToSmall: ToMini {}
impl<T: ToMini> ToSmall for T {}

impl<T: ToSmall> Assign<T> for SmallInteger {
    #[inline]
    fn assign(&mut self, src: T) {
        let mut mini = MiniInteger::from(src);
        self.inner.assign(mini.borrow_excl())
    }
}

impl<T: ToSmall> From<T> for SmallInteger {
    #[inline]
    fn from(src: T) -> Self {
        let mut mini = MiniInteger::from(src);
        SmallInteger {
            inner: Integer::from(mini.borrow_excl()),
            phantom: PhantomData,
        }
    }
}

impl Assign<&Self> for SmallInteger {
    #[inline]
    fn assign(&mut self, other: &Self) {
        self.clone_from(other);
    }
}

impl Assign for SmallInteger {
    #[inline]
    fn assign(&mut self, other: Self) {
        *self = other;
    }
}

#[cfg(test)]
mod tests {
    use crate::Assign;
    use crate::integer::SmallInteger;

    #[test]
    fn check_assign() {
        let mut i = SmallInteger::from(-1i32);
        assert_eq!(*i, -1);
        let other = SmallInteger::from(2i32);
        i.assign(&other);
        assert_eq!(*i, 2);
        i.assign(6u8);
        assert_eq!(*i, 6);
        i.assign(-6i8);
        assert_eq!(*i, -6);
        i.assign(other);
        assert_eq!(*i, 2);
        i.assign(6u16);
        assert_eq!(*i, 6);
        i.assign(-6i16);
        assert_eq!(*i, -6);
        i.assign(6u32);
        assert_eq!(*i, 6);
        i.assign(-6i32);
        assert_eq!(*i, -6);
        i.assign(0xf_0000_0006u64);
        assert_eq!(*i, 0xf_0000_0006u64);
        i.assign(-0xf_0000_0006i64);
        assert_eq!(*i, -0xf_0000_0006i64);
        i.assign((6u128 << 64) | 7u128);
        assert_eq!(*i, (6u128 << 64) | 7u128);
        i.assign((-6i128 << 64) | 7i128);
        assert_eq!(*i, (-6i128 << 64) | 7i128);
        i.assign(6usize);
        assert_eq!(*i, 6);
        i.assign(-6isize);
        assert_eq!(*i, -6);
        i.assign(0u32);
        assert_eq!(*i, 0);
    }
}
