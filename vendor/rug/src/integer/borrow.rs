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

use crate::Integer;
use crate::misc;
use core::fmt::{
    Binary, Debug, Display, Formatter, LowerHex, Octal, Pointer, Result as FmtResult, UpperHex,
};
#[cfg(feature = "float")]
use core::fmt::{LowerExp, UpperExp};
use core::marker::PhantomData;
use core::ops::Deref;
use gmp_mpfr_sys::gmp::mpz_t;

/// Used to get a reference to an [`Integer`].
///
/// The struct implements <code>[Deref]\<[Target][Deref::Target] = [Integer]></code>.
///
/// No memory is unallocated when this struct is dropped.
///
/// # Examples
///
/// ```rust
/// use rug::integer::BorrowInteger;
/// use rug::Integer;
/// let i = Integer::from(42);
/// let neg: BorrowInteger = i.as_neg();
/// // i is still valid
/// assert_eq!(i, 42);
/// assert_eq!(*neg, -42);
/// ```
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BorrowInteger<'a> {
    inner: mpz_t,
    phantom: PhantomData<&'a Integer>,
}

impl<'a> BorrowInteger<'a> {
    /// Create a borrow from a raw [GMP integer][mpz_t].
    ///
    /// # Safety
    ///
    ///   * The value must be initialized as a valid [`mpz_t`].
    ///   * The [`mpz_t`] type can be considered as a kind of pointer, so there
    ///     can be multiple copies of it. [`BorrowInteger`] cannot mutate the
    ///     value, so there can be other copies, but none of them are allowed to
    ///     mutate the value.
    ///   * The lifetime is obtained from the return type. The user must ensure
    ///     the value remains valid for the duration of the lifetime.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::BorrowInteger;
    /// use rug::Integer;
    /// let i = Integer::from(42);
    /// // Safety: i.as_raw() is a valid pointer.
    /// let raw = unsafe { *i.as_raw() };
    /// // Safety: i is still valid when borrow is used.
    /// let borrow = unsafe { BorrowInteger::from_raw(raw) };
    /// assert_eq!(i, *borrow);
    /// ```
    #[inline]
    pub const unsafe fn from_raw(raw: mpz_t) -> BorrowInteger<'a> {
        BorrowInteger {
            inner: raw,
            phantom: PhantomData,
        }
    }

    /// Gets a reference to [`Integer`] from a `BorrowInteger`.
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
    /// use rug::integer::BorrowInteger;
    /// use rug::Integer;
    ///
    /// let i = Integer::from(23);
    /// let b = i.as_neg();
    /// let using_method: &Integer = BorrowInteger::const_deref(&b);
    /// let using_operator: &Integer = &*b;
    /// let using_trait: &Integer = b.deref();
    /// assert_eq!(*using_method, -23);
    /// assert!(ptr::eq(using_method, using_operator));
    /// assert!(ptr::eq(using_method, using_trait));
    /// ```
    ///
    /// This method can be used to create a constant reference.
    ///
    /// ```rust
    /// use rug::integer::{BorrowInteger, MiniInteger};
    /// use rug::Integer;
    ///
    /// const TWO_MINI: MiniInteger = MiniInteger::const_from_i32(2);
    /// const TWO_BORROW: BorrowInteger = TWO_MINI.borrow();
    /// const TWO: &Integer = BorrowInteger::const_deref(&TWO_BORROW);
    /// assert_eq!(*TWO, 2);
    /// ```
    ///
    /// If the constant cannot be initialized using
    /// [`MiniInteger`][crate::integer::MiniInteger], we can set up the limbs
    /// directly, though it is more complicated.
    ///
    /// ```rust
    /// use gmp_mpfr_sys::gmp;
    /// use gmp_mpfr_sys::gmp::{limb_t, mpz_t};
    /// use rug::integer::BorrowInteger;
    /// use rug::Integer;
    ///
    /// const LIMBS: [limb_t; 2] = [123, 456];
    /// const MPZ: mpz_t =
    ///     unsafe { gmp::MPZ_ROINIT_N(LIMBS.as_ptr().cast_mut(), -2) };
    /// // Safety: MPZ will remain valid, and will not be changed.
    /// const BORROW: BorrowInteger = unsafe { BorrowInteger::from_raw(MPZ) };
    /// const I: &Integer = BorrowInteger::const_deref(&BORROW);
    /// let check = -((Integer::from(LIMBS[1]) << gmp::NUMB_BITS) + LIMBS[0]);
    /// assert_eq!(*I, check);
    /// ```
    #[inline]
    pub const fn const_deref<'b>(b: &'b BorrowInteger<'a>) -> &'b Integer {
        let ptr = misc::cast_ptr(&b.inner);
        // Safety: the inner pointer is valid for the duration of the lifetime.
        unsafe { &*ptr }
    }
}

impl Deref for BorrowInteger<'_> {
    type Target = Integer;
    #[inline]
    fn deref(&self) -> &Integer {
        BorrowInteger::const_deref(self)
    }
}

impl Pointer for BorrowInteger<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Pointer::fmt(&&**self, f)
    }
}

impl Display for BorrowInteger<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&**self, f)
    }
}

impl Debug for BorrowInteger<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&**self, f)
    }
}

impl Binary for BorrowInteger<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Binary::fmt(&**self, f)
    }
}

impl Octal for BorrowInteger<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Octal::fmt(&**self, f)
    }
}

impl LowerHex for BorrowInteger<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerHex::fmt(&**self, f)
    }
}

impl UpperHex for BorrowInteger<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperHex::fmt(&**self, f)
    }
}

#[cfg(feature = "float")]
impl LowerExp for BorrowInteger<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerExp::fmt(&**self, f)
    }
}

#[cfg(feature = "float")]
impl UpperExp for BorrowInteger<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperExp::fmt(&**self, f)
    }
}

// Safety: mpz_t is thread safe as guaranteed by the GMP library.
unsafe impl Send for BorrowInteger<'_> {}
unsafe impl Sync for BorrowInteger<'_> {}
