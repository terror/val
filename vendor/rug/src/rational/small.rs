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

use crate::integer::{MiniInteger, ToSmall};
use crate::{Assign, Rational};
use core::fmt::{Debug, Formatter, Result as FmtResult};
use core::marker::PhantomData;
use core::ops::Deref;
use gmp_mpfr_sys::gmp::limb_t;

/**
A small rational number that did not require any memory allocation until version 1.23.0.

Because of a [soundness issue], this has been deprecated and replaced by
[`MiniRational`]. To fix the soundness issue, this struct now uses allocations
like [`Rational`] itself, so it is less efficient than [`MiniRational`].

The `SmallRational` type can be coerced to a [`Rational`], as it implements
<code>[Deref]\<[Target][Deref::Target] = [Rational]></code>.

# Examples

```rust
#![allow(deprecated)]

use rug::rational::SmallRational;
use rug::Rational;
// `a` requires a heap allocation
let mut a = Rational::from((100, 13));
// `b` can reside on the stack
let b = SmallRational::from((-100, 21));
a /= &*b;
assert_eq!(*a.numer(), -21);
assert_eq!(*a.denom(), 13);
```

[`MiniRational`]: crate::rational::MiniRational
[soundness issue]: https://gitlab.com/tspiteri/rug/-/issues/52
*/
#[deprecated(since = "1.23.0", note = "use `MiniRational` instead")]
#[derive(Clone)]
pub struct SmallRational {
    inner: Option<Rational>,
    // for !Sync
    phantom: PhantomData<*const limb_t>,
}

unsafe impl Send for SmallRational {}

impl Default for SmallRational {
    #[inline]
    fn default() -> Self {
        SmallRational::new()
    }
}

impl Debug for SmallRational {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.inner {
            Some(r) => Debug::fmt(r, f),
            None => Debug::fmt(Rational::ZERO, f),
        }
    }
}

impl SmallRational {
    /// Creates a [`SmallRational`] with value 0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![allow(deprecated)]
    ///
    /// use rug::rational::SmallRational;
    /// let r = SmallRational::new();
    /// // Use r as if it were Rational.
    /// assert_eq!(*r.numer(), 0);
    /// assert_eq!(*r.denom(), 1);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        SmallRational {
            inner: None,
            phantom: PhantomData,
        }
    }

    /// Returns a mutable reference to a [`Rational`] number for simple
    /// operations that do not need to allocate more space for the numerator or
    /// denominator.
    ///
    /// # Safety
    ///
    /// It is undefined behavior to perform operations that reallocate the
    /// internal data of the referenced [`Rational`] number or to swap it with
    /// another number, although it is allowed to swap the numerator and
    /// denominator allocations, such as in the reciprocal operation
    /// [`recip_mut`].
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
    /// use rug::rational::SmallRational;
    /// let mut r = SmallRational::from((-15i32, 47i32));
    /// let num_capacity = r.numer().capacity();
    /// let den_capacity = r.denom().capacity();
    /// // reciprocating this will not require reallocations
    /// unsafe {
    ///     r.as_nonreallocating_rational().recip_mut();
    /// }
    /// assert_eq!(*r, SmallRational::from((-47, 15)));
    /// assert_eq!(r.numer().capacity(), num_capacity);
    /// assert_eq!(r.denom().capacity(), den_capacity);
    /// ```
    ///
    /// [`recip_mut`]: `Rational::recip_mut`
    #[inline]
    pub unsafe fn as_nonreallocating_rational(&mut self) -> &mut Rational {
        if self.inner.is_none() {
            *self = SmallRational {
                inner: Some(Rational::new()),
                phantom: PhantomData,
            };
        }
        match &mut self.inner {
            Some(r) => r,
            None => unreachable!(),
        }
    }

    /// Creates a [`SmallRational`] from a numerator and denominator, assuming
    /// they are in canonical form.
    ///
    /// # Safety
    ///
    /// This method leads to undefined behavior if `den` is zero or if `num` and
    /// `den` have common factors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![allow(deprecated)]
    ///
    /// use rug::rational::SmallRational;
    /// let from_unsafe = unsafe { SmallRational::from_canonical(-13, 10) };
    /// // from_safe is canonicalized to the same form as from_unsafe
    /// let from_safe = SmallRational::from((130, -100));
    /// assert_eq!(from_unsafe.numer(), from_safe.numer());
    /// assert_eq!(from_unsafe.denom(), from_safe.denom());
    /// ```
    pub unsafe fn from_canonical<Num: ToSmall, Den: ToSmall>(num: Num, den: Den) -> Self {
        let num = MiniInteger::from(num);
        let den = MiniInteger::from(den);
        SmallRational {
            inner: Some(unsafe { Rational::from_canonical(num, den) }),
            phantom: PhantomData,
        }
    }

    /// Assigns a numerator and denominator to a [`SmallRational`], assuming
    /// they are in canonical form.
    ///
    /// # Safety
    ///
    /// This method leads to undefined behavior if `den` is zero or negative, or
    /// if `num` and `den` have common factors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![allow(deprecated)]
    ///
    /// use rug::rational::SmallRational;
    /// use rug::Assign;
    /// let mut a = SmallRational::new();
    /// unsafe {
    ///     a.assign_canonical(-13, 10);
    /// }
    /// // b is canonicalized to the same form as a
    /// let mut b = SmallRational::new();
    /// b.assign((130, -100));
    /// assert_eq!(a.numer(), b.numer());
    /// assert_eq!(a.denom(), b.denom());
    /// ```
    pub unsafe fn assign_canonical<Num: ToSmall, Den: ToSmall>(&mut self, num: Num, den: Den) {
        let mut num = MiniInteger::from(num);
        let mut den = MiniInteger::from(den);
        unsafe {
            self.as_nonreallocating_rational()
                .assign_canonical(num.borrow_excl(), den.borrow_excl());
        }
    }
}

impl Deref for SmallRational {
    type Target = Rational;
    #[inline]
    fn deref(&self) -> &Rational {
        match &self.inner {
            Some(r) => r,
            None => Rational::ZERO,
        }
    }
}

impl<Num: ToSmall> Assign<Num> for SmallRational {
    #[inline]
    fn assign(&mut self, src: Num) {
        let mut mini = MiniInteger::from(src);
        unsafe {
            self.as_nonreallocating_rational()
                .assign(mini.borrow_excl());
        }
    }
}

impl<Num: ToSmall> From<Num> for SmallRational {
    fn from(src: Num) -> Self {
        let mut mini = MiniInteger::from(src);
        SmallRational {
            inner: Some(Rational::from(mini.borrow_excl())),
            phantom: PhantomData,
        }
    }
}

impl<Num: ToSmall, Den: ToSmall> Assign<(Num, Den)> for SmallRational {
    fn assign(&mut self, src: (Num, Den)) {
        assert!(!src.1.is_zero(), "division by zero");
        let mut num = MiniInteger::from(src.0);
        let mut den = MiniInteger::from(src.1);
        unsafe {
            self.as_nonreallocating_rational()
                .assign((num.borrow_excl(), den.borrow_excl()));
        }
    }
}

impl<Num: ToSmall, Den: ToSmall> From<(Num, Den)> for SmallRational {
    fn from(src: (Num, Den)) -> Self {
        assert!(!src.1.is_zero(), "division by zero");
        let mut num = MiniInteger::from(src.0);
        let mut den = MiniInteger::from(src.1);
        SmallRational {
            inner: Some(Rational::from((num.borrow_excl(), den.borrow_excl()))),
            phantom: PhantomData,
        }
    }
}

impl Assign<&Self> for SmallRational {
    #[inline]
    fn assign(&mut self, other: &Self) {
        self.clone_from(other);
    }
}

impl Assign for SmallRational {
    #[inline]
    fn assign(&mut self, other: Self) {
        *self = other;
    }
}

#[cfg(test)]
mod tests {
    use crate::Assign;
    use crate::rational::SmallRational;

    #[test]
    fn check_assign() {
        let mut r = SmallRational::from((1, 2));
        assert_eq!(*r, SmallRational::from((1, 2)));
        r.assign(3);
        assert_eq!(*r, 3);
        let other = SmallRational::from((4, 5));
        r.assign(&other);
        assert_eq!(*r, SmallRational::from((4, 5)));
        r.assign((6, 7));
        assert_eq!(*r, SmallRational::from((6, 7)));
        r.assign(other);
        assert_eq!(*r, SmallRational::from((4, 5)));
    }

    fn swapped_parts(small: &SmallRational) -> bool {
        unsafe {
            let num = (*small.numer().as_raw()).d;
            let den = (*small.denom().as_raw()).d;
            num > den
        }
    }

    #[test]
    fn check_swapped_parts() {
        let mut r = SmallRational::from((2, 3));
        assert_eq!(*r, SmallRational::from((2, 3)));
        assert_eq!(*r.clone(), *r);
        let mut orig_swapped_parts = swapped_parts(&r);
        unsafe {
            r.as_nonreallocating_rational().recip_mut();
        }
        assert_eq!(*r, SmallRational::from((3, 2)));
        assert_eq!(*r.clone(), *r);
        assert!(swapped_parts(&r) != orig_swapped_parts);

        unsafe {
            r.assign_canonical(5, 7);
        }
        assert_eq!(*r, SmallRational::from((5, 7)));
        assert_eq!(*r.clone(), *r);
        orig_swapped_parts = swapped_parts(&r);
        unsafe {
            r.as_nonreallocating_rational().recip_mut();
        }
        assert_eq!(*r, SmallRational::from((7, 5)));
        assert_eq!(*r.clone(), *r);
        assert!(swapped_parts(&r) != orig_swapped_parts);

        r.assign(2);
        assert_eq!(*r, 2);
        assert_eq!(*r.clone(), *r);
        orig_swapped_parts = swapped_parts(&r);
        unsafe {
            r.as_nonreallocating_rational().recip_mut();
        }
        assert_eq!(*r, SmallRational::from((1, 2)));
        assert_eq!(*r.clone(), *r);
        assert!(swapped_parts(&r) != orig_swapped_parts);

        r.assign((3, -5));
        assert_eq!(*r, SmallRational::from((-3, 5)));
        assert_eq!(*r.clone(), *r);
        orig_swapped_parts = swapped_parts(&r);
        unsafe {
            r.as_nonreallocating_rational().recip_mut();
        }
        assert_eq!(*r, SmallRational::from((-5, 3)));
        assert_eq!(*r.clone(), *r);
        assert!(swapped_parts(&r) != orig_swapped_parts);
    }
}
