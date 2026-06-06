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

use crate::complex::{BorrowComplex, MiniComplex};
use crate::float::{MiniFloat, Special, ToSmall};
use crate::{Assign, Complex};
use core::fmt::{Debug, Formatter, Result as FmtResult};
use core::marker::PhantomData;
use core::mem;
use core::ops::Deref;
use gmp_mpfr_sys::gmp::limb_t;

const ZERO_MINI: MiniComplex = MiniComplex::new();
const ZERO_BORROW: BorrowComplex = ZERO_MINI.borrow();
const ZERO: &Complex = BorrowComplex::const_deref(&ZERO_BORROW);

/**
A small complex number that did not require any memory allocation until version 1.23.0.

Because of a [soundness issue], this has been deprecated and replaced by
[`MiniComplex`]. To fix the soundness issue, this struct now uses allocations
like [`Complex`] itself, so it is less efficient than [`MiniComplex`].

The `SmallComplex` type can be coerced to a [`Complex`], as it implements
<code>[Deref]\<[Target][Deref::Target] = [Complex]></code>.

# Examples

```rust
#![allow(deprecated)]

use rug::complex::SmallComplex;
use rug::Complex;
// `a` requires a heap allocation
let mut a = Complex::with_val(53, (1, 2));
// `b` can reside on the stack
let b = SmallComplex::from((-10f64, -20.5f64));
a += &*b;
assert_eq!(*a.real(), -9);
assert_eq!(*a.imag(), -18.5);
```

[soundness issue]: https://gitlab.com/tspiteri/rug/-/issues/52
*/
#[deprecated(since = "1.23.0", note = "use `MiniComplex` instead")]
#[derive(Clone)]
pub struct SmallComplex {
    inner: Option<Complex>,
    // for !Sync
    phantom: PhantomData<*const limb_t>,
}

unsafe impl Send for SmallComplex {}

impl Default for SmallComplex {
    #[inline]
    fn default() -> Self {
        SmallComplex::new()
    }
}

impl Debug for SmallComplex {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.inner {
            Some(c) => Debug::fmt(c, f),
            None => Debug::fmt(ZERO, f),
        }
    }
}

impl SmallComplex {
    /// Creates a [`SmallComplex`] with value 0 and the [minimum possible
    /// precision][crate::float::prec_min].
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![allow(deprecated)]
    ///
    /// use rug::complex::SmallComplex;
    /// let c = SmallComplex::new();
    /// // Borrow c as if it were Complex.
    /// assert_eq!(*c, 0);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        SmallComplex {
            inner: None,
            phantom: PhantomData,
        }
    }

    /// Returns a mutable reference to a [`Complex`] number for simple
    /// operations that do not need to change the precision of the real or
    /// imaginary part.
    ///
    /// # Safety
    ///
    /// It is undefined behavior to modify the precision of the referenced
    /// [`Complex`] number or to swap it with another number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![allow(deprecated)]
    ///
    /// use rug::complex::SmallComplex;
    /// let mut c = SmallComplex::from((1.0f32, 3.0f32));
    /// // rotation does not change the precision
    /// unsafe {
    ///     c.as_nonreallocating_complex().mul_i_mut(false);
    /// }
    /// assert_eq!(*c, (-3.0, 1.0));
    /// ```
    #[inline]
    pub unsafe fn as_nonreallocating_complex(&mut self) -> &mut Complex {
        if self.inner.is_none() {
            *self = SmallComplex {
                inner: Some(Complex::new(ZERO.prec())),
                phantom: PhantomData,
            };
        }
        match &mut self.inner {
            Some(c) => c,
            None => unreachable!(),
        }
    }
}

impl Deref for SmallComplex {
    type Target = Complex;
    #[inline]
    fn deref(&self) -> &Complex {
        match &self.inner {
            Some(c) => c,
            None => ZERO,
        }
    }
}

impl<Re: ToSmall> Assign<Re> for SmallComplex {
    fn assign(&mut self, src: Re) {
        let mut mini = MiniFloat::from(src);
        let src = mini.borrow_excl();
        unsafe {
            let dst = self.as_nonreallocating_complex();
            dst.mut_real().set_prec(src.prec());
            dst.mut_real().assign(src);
            dst.mut_imag().set_prec(src.prec());
            dst.mut_imag().assign(Special::Zero);
        }
    }
}

impl<Re: ToSmall> From<Re> for SmallComplex {
    fn from(src: Re) -> Self {
        let mut mini = MiniFloat::from(src);
        let src = mini.borrow_excl();
        SmallComplex {
            inner: Some(Complex::with_val(src.prec(), src)),
            phantom: PhantomData,
        }
    }
}

impl<Re: ToSmall, Im: ToSmall> Assign<(Re, Im)> for SmallComplex {
    fn assign(&mut self, src: (Re, Im)) {
        let mut re = MiniFloat::from(src.0);
        let mut im = MiniFloat::from(src.1);
        let re = re.borrow_excl();
        let im = im.borrow_excl();
        unsafe {
            let dst = self.as_nonreallocating_complex();
            dst.mut_real().set_prec(re.prec());
            dst.mut_real().assign(re);
            dst.mut_imag().set_prec(im.prec());
            dst.mut_imag().assign(im);
        }
    }
}

impl<Re: ToSmall, Im: ToSmall> From<(Re, Im)> for SmallComplex {
    fn from(src: (Re, Im)) -> Self {
        let mut re = MiniFloat::from(src.0);
        let mut im = MiniFloat::from(src.1);
        let re = re.borrow_excl();
        let im = im.borrow_excl();
        SmallComplex {
            inner: Some(Complex::with_val((re.prec(), im.prec()), (re, im))),
            phantom: PhantomData,
        }
    }
}

impl Assign<&Self> for SmallComplex {
    #[inline]
    fn assign(&mut self, other: &Self) {
        self.clone_from(other);
    }
}

impl Assign for SmallComplex {
    #[inline]
    fn assign(&mut self, other: Self) {
        drop(mem::replace(self, other));
    }
}

#[cfg(test)]
mod tests {
    use crate::Assign;
    use crate::complex::SmallComplex;
    use crate::float;
    use crate::float::FreeCache;

    #[test]
    fn check_assign() {
        let mut c = SmallComplex::from((1.0, 2.0));
        assert_eq!(*c, (1.0, 2.0));
        c.assign(3.0);
        assert_eq!(*c, (3.0, 0.0));
        let other = SmallComplex::from((4.0, 5.0));
        c.assign(&other);
        assert_eq!(*c, (4.0, 5.0));
        c.assign((6.0, 7.0));
        assert_eq!(*c, (6.0, 7.0));
        c.assign(other);
        assert_eq!(*c, (4.0, 5.0));

        float::free_cache(FreeCache::All);
    }

    fn swapped_parts(small: &SmallComplex) -> bool {
        unsafe {
            let re = (*small.real().as_raw()).d;
            let im = (*small.imag().as_raw()).d;
            re > im
        }
    }

    #[test]
    fn check_swapped_parts() {
        let mut c = SmallComplex::from((1, 2));
        assert_eq!(*c, (1, 2));
        assert_eq!(*c.clone(), *c);
        let mut orig_swapped_parts = swapped_parts(&c);
        unsafe {
            c.as_nonreallocating_complex().mul_i_mut(false);
        }
        assert_eq!(*c, (-2, 1));
        assert_eq!(*c.clone(), *c);
        assert!(swapped_parts(&c) != orig_swapped_parts);

        c.assign(12);
        assert_eq!(*c, 12);
        assert_eq!(*c.clone(), *c);
        orig_swapped_parts = swapped_parts(&c);
        unsafe {
            c.as_nonreallocating_complex().mul_i_mut(false);
        }
        assert_eq!(*c, (0, 12));
        assert_eq!(*c.clone(), *c);
        assert!(swapped_parts(&c) != orig_swapped_parts);

        c.assign((4, 5));
        assert_eq!(*c, (4, 5));
        assert_eq!(*c.clone(), *c);
        orig_swapped_parts = swapped_parts(&c);
        unsafe {
            c.as_nonreallocating_complex().mul_i_mut(false);
        }
        assert_eq!(*c, (-5, 4));
        assert_eq!(*c.clone(), *c);
        assert!(swapped_parts(&c) != orig_swapped_parts);
    }
}
