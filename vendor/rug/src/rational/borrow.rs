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

use crate::Rational;
use crate::misc;
use core::fmt::{
    Binary, Debug, Display, Formatter, LowerHex, Octal, Pointer, Result as FmtResult, UpperHex,
};
use core::marker::PhantomData;
use core::ops::Deref;
use gmp_mpfr_sys::gmp::mpq_t;

/// Used to get a reference to a [`Rational`] number.
///
/// The struct implements <code>[Deref]\<[Target][Deref::Target] = [Rational]></code>.
///
/// No memory is unallocated when this struct is dropped.
///
/// # Examples
///
/// ```rust
/// use rug::rational::{BorrowRational, MiniRational};
/// use rug::Rational;
/// let r = Rational::from((42, 3));
/// let neg: BorrowRational = r.as_neg();
/// // r is still valid
/// assert_eq!(r, MiniRational::from((42, 3)));
/// assert_eq!(*neg, MiniRational::from((-42, 3)));
/// ```
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BorrowRational<'a> {
    inner: mpq_t,
    phantom: PhantomData<&'a Rational>,
}

impl<'a> BorrowRational<'a> {
    /// Create a borrow from a raw [GMP rational number][mpq_t].
    ///
    /// # Safety
    ///
    ///   * The value must be initialized as a valid [`mpq_t`].
    ///   * The [`mpq_t`] type can be considered as a kind of pointer, so there
    ///     can be multiple copies of it. [`BorrowRational`] cannot mutate the
    ///     value, so there can be other copies, but none of them are allowed to
    ///     mutate the value.
    ///   * The lifetime is obtained from the return type. The user must ensure
    ///     the value remains valid for the duration of the lifetime.
    ///   * The numerator and denominator must be in canonical form, as the rest
    ///     of the library assumes that they are. Most GMP functions leave the
    ///     rational number in canonical form, but assignment functions do not.
    ///     Check the [GMP documentation][gmp mpq] for details.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rational::BorrowRational;
    /// use rug::Rational;
    /// let r = Rational::from((42, 3));
    /// // Safety: r.as_raw() is a valid pointer.
    /// let raw = unsafe { *r.as_raw() };
    /// // Safety: r is still valid when borrow is used.
    /// let borrow = unsafe { BorrowRational::from_raw(raw) };
    /// assert_eq!(r, *borrow);
    /// ```
    ///
    /// [gmp mpq]: gmp_mpfr_sys::C::GMP::Rational_Number_Functions#index-Rational-number-functions
    pub const unsafe fn from_raw(raw: mpq_t) -> BorrowRational<'a> {
        BorrowRational {
            inner: raw,
            phantom: PhantomData,
        }
    }

    /// Gets a reference to [`Rational`] from a `BorrowRational`.
    ///
    /// This is equivalent to taking the reference of the dereferencing operator
    /// `*` or to <code>[Deref]::[deref][Deref::deref]</code>, but can also be
    /// used in constant context. Unless required in constant context, use the
    /// operator or trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the unary `*` operator and the
    /// [`Deref`] trait are usable in constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use core::ops::Deref;
    /// use core::ptr;
    /// use rug::rational::{BorrowRational, MiniRational};
    /// use rug::Rational;
    ///
    /// let i = Rational::from((-5, 7));
    /// let b = i.as_recip();
    /// let using_method: &Rational = BorrowRational::const_deref(&b);
    /// let using_operator: &Rational = &*b;
    /// let using_trait: &Rational = b.deref();
    /// assert_eq!(*using_trait, MiniRational::from((-7, 5)));
    /// assert!(ptr::eq(using_method, using_operator));
    /// assert!(ptr::eq(using_method, using_trait));
    /// ```
    ///
    /// This method can be used to create a constant reference.
    ///
    /// ```rust
    /// use rug::integer::MiniInteger;
    /// use rug::rational::{BorrowRational, MiniRational};
    /// use rug::Rational;
    ///
    /// const TWO_INT: MiniInteger = MiniInteger::const_from_i32(2);
    /// const TWO_MINI: MiniRational = MiniRational::const_from_integer(TWO_INT);
    /// const TWO_BORROW: BorrowRational = TWO_MINI.borrow();
    /// const TWO: &Rational = BorrowRational::const_deref(&TWO_BORROW);
    /// assert_eq!(*TWO, 2);
    /// ```
    ///
    /// Creating a constant with [`MiniRational`][crate::rational::MiniRational]
    /// is rather limited. If the constant cannot be initialized that way, we
    /// can set up the limbs directly, though it is more complicated.
    ///
    /// ```rust
    /// use gmp_mpfr_sys::gmp;
    /// use gmp_mpfr_sys::gmp::{limb_t, mpq_t};
    /// use rug::rational::BorrowRational;
    /// use rug::{Integer, Rational};
    ///
    /// const NUMER_LIMBS: [limb_t; 2] = [0, 5];
    /// const DENOM_LIMBS: [limb_t; 1] = [3];
    /// const MPQ: mpq_t = unsafe {
    ///     mpq_t {
    ///         num: gmp::MPZ_ROINIT_N(NUMER_LIMBS.as_ptr().cast_mut(), -2),
    ///         den: gmp::MPZ_ROINIT_N(DENOM_LIMBS.as_ptr().cast_mut(), 1),
    ///     }
    /// };
    /// // Safety: MPQ will remain valid, and will not be changed.
    /// const BORROW: BorrowRational = unsafe { BorrowRational::from_raw(MPQ) };
    /// const R: &Rational = BorrowRational::const_deref(&BORROW);
    /// let numer_check =
    ///     -((Integer::from(NUMER_LIMBS[1]) << gmp::NUMB_BITS) + NUMER_LIMBS[0]);
    /// let denom_check = Integer::from(DENOM_LIMBS[0]);
    /// let check = Rational::from((&numer_check, &denom_check));
    /// assert_eq!(*R, check);
    /// assert_eq!(*R.numer(), *check.numer());
    /// assert_eq!(*R.denom(), *check.denom());
    /// ```
    #[inline]
    pub const fn const_deref<'b>(b: &'b BorrowRational<'a>) -> &'b Rational {
        let ptr = misc::cast_ptr(&b.inner);
        // Safety: the inner pointer is valid for the duration of the lifetime.
        unsafe { &*ptr }
    }
}

impl Deref for BorrowRational<'_> {
    type Target = Rational;
    #[inline]
    fn deref(&self) -> &Rational {
        BorrowRational::const_deref(self)
    }
}

impl Pointer for BorrowRational<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Pointer::fmt(&&**self, f)
    }
}

impl Display for BorrowRational<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&**self, f)
    }
}

impl Debug for BorrowRational<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&**self, f)
    }
}

impl Binary for BorrowRational<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Binary::fmt(&**self, f)
    }
}

impl Octal for BorrowRational<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Octal::fmt(&**self, f)
    }
}

impl LowerHex for BorrowRational<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerHex::fmt(&**self, f)
    }
}

impl UpperHex for BorrowRational<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperHex::fmt(&**self, f)
    }
}

// Safety: mpq_t is thread safe as guaranteed by the GMP library.
unsafe impl Send for BorrowRational<'_> {}
unsafe impl Sync for BorrowRational<'_> {}
