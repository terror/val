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

use crate::Complex;
use crate::misc;
use core::fmt::{
    Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, Pointer, Result as FmtResult,
    UpperExp, UpperHex,
};
use core::marker::PhantomData;
use core::ops::Deref;
use gmp_mpfr_sys::mpc::mpc_t;

/// Used to get a reference to a [`Complex`] number.
///
/// The struct implements <code>[Deref]\<[Target][Deref::Target] = [Complex]></code>.
///
/// No memory is unallocated when this struct is dropped.
///
/// # Examples
///
/// ```rust
/// use rug::complex::BorrowComplex;
/// use rug::Complex;
/// let c = Complex::with_val(53, (4.2, -2.3));
/// let neg: BorrowComplex = c.as_neg();
/// // c is still valid
/// assert_eq!(c, (4.2, -2.3));
/// assert_eq!(*neg, (-4.2, 2.3));
/// ```
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BorrowComplex<'a> {
    inner: mpc_t,
    phantom: PhantomData<&'a Complex>,
}

impl<'a> BorrowComplex<'a> {
    /// Create a borrow from a raw [MPC complex number][mpc_t].
    ///
    /// # Safety
    ///
    ///   * The value must be initialized as a valid [`mpc_t`].
    ///   * The [`mpc_t`] type can be considered as a kind of pointer, so there
    ///     can be multiple copies of it. [`BorrowComplex`] cannot mutate the
    ///     value, so there can be other copies, but none of them are allowed to
    ///     mutate the value.
    ///   * The lifetime is obtained from the return type. The user must ensure
    ///     the value remains valid for the duration of the lifetime.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::complex::BorrowComplex;
    /// use rug::Complex;
    /// let c = Complex::with_val(53, (4.2, -2.3));
    /// // Safety: c.as_raw() is a valid pointer.
    /// let raw = unsafe { *c.as_raw() };
    /// // Safety: c is still valid when borrow is used.
    /// let borrow = unsafe { BorrowComplex::from_raw(raw) };
    /// assert_eq!(c, *borrow);
    /// ```
    // unsafe because the lifetime is obtained from return type
    pub const unsafe fn from_raw(raw: mpc_t) -> BorrowComplex<'a> {
        BorrowComplex {
            inner: raw,
            phantom: PhantomData,
        }
    }

    /// Gets a reference to [`Complex`] from a `BorrowComplex`.
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
    /// use rug::complex::BorrowComplex;
    /// use rug::Complex;
    ///
    /// let c = Complex::with_val(53, (23.5, -12.25));
    /// let b = c.as_conj();
    /// let using_method: &Complex = BorrowComplex::const_deref(&b);
    /// let using_operator: &Complex = &*b;
    /// let using_trait: &Complex = b.deref();
    /// assert_eq!(*using_method, (23.5, 12.25));
    /// assert!(ptr::eq(using_method, using_operator));
    /// assert!(ptr::eq(using_method, using_trait));
    /// ```
    ///
    /// This method can be used to create a constant reference.
    ///
    /// ```rust
    /// use rug::complex::{BorrowComplex, MiniComplex};
    /// use rug::float::MiniFloat;
    /// use rug::Complex;
    ///
    /// const TWO_FLOAT: MiniFloat = MiniFloat::const_from_i8(2);
    /// const THOUSAND_FLOAT: MiniFloat = MiniFloat::const_from_i16(1000);
    /// const TWO_1000I_MINI: MiniComplex =
    ///     MiniComplex::const_from_parts(TWO_FLOAT, THOUSAND_FLOAT);
    /// const TWO_1000I_BORROW: BorrowComplex = TWO_1000I_MINI.borrow();
    /// const TWO_1000I: &Complex = BorrowComplex::const_deref(&TWO_1000I_BORROW);
    /// assert_eq!(*TWO_1000I, (2, 1000));
    /// ```
    ///
    /// If the constant cannot be initialized using
    /// [`MiniComplex`][crate::complex::MiniComplex], we can set up the limbs
    /// directly, though it is more complicated.
    ///
    /// ```rust
    /// use core::ptr::NonNull;
    /// use gmp_mpfr_sys::gmp::limb_t;
    /// use gmp_mpfr_sys::mpfr::{mpfr_t, prec_t};
    /// use gmp_mpfr_sys::mpc::mpc_t;
    /// use rug::complex::BorrowComplex;
    /// use rug::{Complex, Float};
    ///
    /// const LIMBS: [limb_t; 2] = [5, 1 << (limb_t::BITS - 1)];
    /// const LIMBS_PTR: *const [limb_t; 2] = &LIMBS;
    /// const MANTISSA_DIGITS: u32 = limb_t::BITS * 2;
    /// const MPC: mpc_t = mpc_t {
    ///     re: mpfr_t {
    ///         prec: MANTISSA_DIGITS as prec_t,
    ///         sign: -1,
    ///         exp: 1,
    ///         d: unsafe { NonNull::new_unchecked(LIMBS_PTR.cast_mut().cast()) },
    ///     },
    ///     im: mpfr_t {
    ///         prec: MANTISSA_DIGITS as prec_t,
    ///         sign: 1,
    ///         exp: 1,
    ///         d: unsafe { NonNull::new_unchecked(LIMBS_PTR.cast_mut().cast()) },
    ///     },
    /// };
    /// // Safety: MPFR will remain valid, and will not be changed.
    /// const BORROW: BorrowComplex = unsafe { BorrowComplex::from_raw(MPC) };
    /// const C: &Complex = BorrowComplex::const_deref(&BORROW);
    /// let lsig = Float::with_val(MANTISSA_DIGITS, 5) >> (MANTISSA_DIGITS - 1);
    /// let msig = 1u32;
    /// let val = lsig + msig;
    /// let check = Complex::from((-val.clone(), val));
    /// assert_eq!(*C, check);
    /// ```
    #[inline]
    pub const fn const_deref<'b>(b: &'b BorrowComplex<'a>) -> &'b Complex {
        let ptr = misc::cast_ptr(&b.inner);
        // Safety: the inner pointer is valid for the duration of the lifetime.
        unsafe { &*ptr }
    }
}

impl Deref for BorrowComplex<'_> {
    type Target = Complex;
    #[inline]
    fn deref(&self) -> &Complex {
        BorrowComplex::const_deref(self)
    }
}

impl Pointer for BorrowComplex<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Pointer::fmt(&&**self, f)
    }
}

impl Display for BorrowComplex<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&**self, f)
    }
}

impl Debug for BorrowComplex<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&**self, f)
    }
}

impl Binary for BorrowComplex<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Binary::fmt(&**self, f)
    }
}

impl Octal for BorrowComplex<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Octal::fmt(&**self, f)
    }
}

impl LowerHex for BorrowComplex<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerHex::fmt(&**self, f)
    }
}

impl UpperHex for BorrowComplex<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperHex::fmt(&**self, f)
    }
}

impl LowerExp for BorrowComplex<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerExp::fmt(&**self, f)
    }
}

impl UpperExp for BorrowComplex<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperExp::fmt(&**self, f)
    }
}

// Safety: mpc_t is thread safe as guaranteed by the MPC library.
unsafe impl Send for BorrowComplex<'_> {}
unsafe impl Sync for BorrowComplex<'_> {}
