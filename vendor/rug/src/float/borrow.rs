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

use crate::Float;
use crate::misc;
use core::fmt::{
    Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, Pointer, Result as FmtResult,
    UpperExp, UpperHex,
};
use core::marker::PhantomData;
use core::ops::Deref;
use gmp_mpfr_sys::mpfr::mpfr_t;

/// Used to get a reference to a [`Float`].
///
/// The struct implements <code>[Deref]\<[Target][Deref::Target] = [Float]></code>.
///
/// No memory is unallocated when this struct is dropped.
///
/// # Examples
///
/// ```rust
/// use rug::float::BorrowFloat;
/// use rug::Float;
/// let f = Float::with_val(53, 4.2);
/// let neg: BorrowFloat = f.as_neg();
/// // f is still valid
/// assert_eq!(f, 4.2);
/// assert_eq!(*neg, -4.2);
/// ```
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BorrowFloat<'a> {
    inner: mpfr_t,
    phantom: PhantomData<&'a Float>,
}

impl<'a> BorrowFloat<'a> {
    /// Create a borrow from a raw [MPFR floating-point number][mpfr_t].
    ///
    /// # Safety
    ///
    ///   * The value must be initialized as a valid [`mpfr_t`].
    ///   * The [`mpfr_t`] type can be considered as a kind of pointer, so there
    ///     can be multiple copies of it. [`BorrowFloat`] cannot mutate the
    ///     value, so there can be other copies, but none of them are allowed to
    ///     mutate the value.
    ///   * The lifetime is obtained from the return type. The user must ensure
    ///     the value remains valid for the duration of the lifetime.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::BorrowFloat;
    /// use rug::Float;
    /// let f = Float::with_val(53, 4.2);
    /// // Safety: f.as_raw() is a valid pointer.
    /// let raw = unsafe { *f.as_raw() };
    /// // Safety: f is still valid when borrow is used.
    /// let borrow = unsafe { BorrowFloat::from_raw(raw) };
    /// assert_eq!(f, *borrow);
    /// ```
    pub const unsafe fn from_raw(raw: mpfr_t) -> BorrowFloat<'a> {
        BorrowFloat {
            inner: raw,
            phantom: PhantomData,
        }
    }

    #[cfg(feature = "complex")]
    pub(crate) const fn into_raw(b: BorrowFloat<'a>) -> mpfr_t {
        b.inner
    }

    /// Gets a reference to [`Float`] from a `BorrowFloat`.
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
    /// use rug::float::BorrowFloat;
    /// use rug::Float;
    ///
    /// let f = Float::with_val(53, 23.5);
    /// let b = f.as_neg();
    /// let using_method: &Float = BorrowFloat::const_deref(&b);
    /// let using_operator: &Float = &*b;
    /// let using_trait: &Float = b.deref();
    /// assert_eq!(*using_method, -23.5);
    /// assert!(ptr::eq(using_method, using_operator));
    /// assert!(ptr::eq(using_method, using_trait));
    /// ```
    ///
    /// This method can be used to create a constant reference.
    ///
    /// ```rust
    /// use rug::float::{BorrowFloat, MiniFloat};
    /// use rug::Float;
    ///
    /// const TWO_HALF_MINI: MiniFloat = MiniFloat::const_from_f32(2.5);
    /// const TWO_HALF_BORROW: BorrowFloat = TWO_HALF_MINI.borrow();
    /// const TWO_HALF: &Float = BorrowFloat::const_deref(&TWO_HALF_BORROW);
    /// assert_eq!(*TWO_HALF, 2.5);
    /// ```
    ///
    /// If the constant cannot be initialized using
    /// [`MiniFloat`][crate::float::MiniFloat], we can set up the limbs
    /// directly, though it is more complicated.
    ///
    /// ```rust
    /// use core::ptr::NonNull;
    /// use gmp_mpfr_sys::gmp::limb_t;
    /// use gmp_mpfr_sys::mpfr::{mpfr_t, prec_t};
    /// use rug::float::BorrowFloat;
    /// use rug::Float;
    ///
    /// const LIMBS: [limb_t; 2] = [5, 1 << (limb_t::BITS - 1)];
    /// const LIMBS_PTR: *const [limb_t; 2] = &LIMBS;
    /// const MANTISSA_DIGITS: u32 = limb_t::BITS * 2;
    /// const MPFR: mpfr_t = mpfr_t {
    ///     prec: MANTISSA_DIGITS as prec_t,
    ///     sign: -1,
    ///     exp: 1,
    ///     d: unsafe { NonNull::new_unchecked(LIMBS_PTR.cast_mut().cast()) },
    /// };
    /// // Safety: MPFR will remain valid, and will not be changed.
    /// const BORROW: BorrowFloat = unsafe { BorrowFloat::from_raw(MPFR) };
    /// const F: &Float = BorrowFloat::const_deref(&BORROW);
    /// let lsig = Float::with_val(MANTISSA_DIGITS, 5) >> (MANTISSA_DIGITS - 1);
    /// let msig = 1u32;
    /// let check = -(lsig + msig);
    /// assert_eq!(*F, check);
    /// ```
    #[inline]
    pub const fn const_deref<'b>(b: &'b BorrowFloat<'a>) -> &'b Float {
        let ptr = misc::cast_ptr(&b.inner);
        // Safety: the inner pointer is valid for the duration of the lifetime.
        unsafe { &*ptr }
    }
}

impl Deref for BorrowFloat<'_> {
    type Target = Float;
    #[inline]
    fn deref(&self) -> &Float {
        BorrowFloat::const_deref(self)
    }
}

impl Pointer for BorrowFloat<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Pointer::fmt(&&**self, f)
    }
}

impl Display for BorrowFloat<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&**self, f)
    }
}

impl Debug for BorrowFloat<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&**self, f)
    }
}

impl Binary for BorrowFloat<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Binary::fmt(&**self, f)
    }
}

impl Octal for BorrowFloat<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Octal::fmt(&**self, f)
    }
}

impl LowerHex for BorrowFloat<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerHex::fmt(&**self, f)
    }
}

impl UpperHex for BorrowFloat<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperHex::fmt(&**self, f)
    }
}

impl LowerExp for BorrowFloat<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerExp::fmt(&**self, f)
    }
}

impl UpperExp for BorrowFloat<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperExp::fmt(&**self, f)
    }
}

// Safety: mpfr_t is thread safe as guaranteed by the MPFR library.
unsafe impl Send for BorrowFloat<'_> {}
unsafe impl Sync for BorrowFloat<'_> {}
