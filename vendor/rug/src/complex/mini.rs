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

use crate::complex::BorrowComplex;
use crate::ext::xmpfr;
use crate::float;
use crate::float::{MiniFloat, ToMini};
use crate::misc;
use crate::{Assign, Complex};
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
use gmp_mpfr_sys::mpc::mpc_t;
use gmp_mpfr_sys::mpfr::{mpfr_t, prec_t};
#[cfg(feature = "num-complex")]
use num_complex::Complex as NumComplex;

const LIMBS_IN_SMALL: usize = (128 / gmp::LIMB_BITS) as usize;
type Limbs = [MaybeUninit<limb_t>; LIMBS_IN_SMALL];

/**
A small complex number that does not require any memory allocation.

This can be useful when you have real and imaginary numbers that are primitive
integers or floats and you need a reference to a [`Complex`].

The `MiniComplex` will have a precision according to the types of the
primitives used to set its real and imaginary parts. Note that if different
types are used to set the parts, the parts can have different precisions.

  * [`bool`]: the part will have the [minimum possible
    precision][crate::float::prec_min].
  * [`i8`], [`u8`]: the part will have eight bits of precision.
  * [`i16`], [`u16`]: the part will have 16 bits of precision.
  * [`i32`], [`u32`]: the part will have 32 bits of precision.
  * [`i64`], [`u64`]: the part will have 64 bits of precision.
  * [`i128`], [`u128`]: the part will have 128 bits of precision.
  * [`isize`], [`usize`]: the part will have 32 or 64 bits of precision,
    depending on the platform.
  * [`f32`]: the part will have 24 bits of precision.
  * [`f64`]: the part will have 53 bits of precision.
  * [`Special`][crate::float::Special]: the part will have the [minimum possible
    precision][crate::float::prec_min].

The [`borrow`][Self::borrow] method returns an object that can be coerced to a
[`Complex`], as it implements
<code>[Deref]\<[Target][Deref::Target] = [Complex]></code>.

# Examples

```rust
use rug::complex::MiniComplex;
use rug::Complex;
// `a` requires a heap allocation
let mut a = Complex::with_val(53, (1, 2));
// `b` can reside on the stack
let b = MiniComplex::from((-10f64, -20.5f64));
a += &*b.borrow();
assert_eq!(*a.real(), -9);
assert_eq!(*a.imag(), -18.5);
```
*/
#[derive(Clone, Copy)]
pub struct MiniComplex {
    inner: mpc_t,
    // real part is first in limbs if inner.re.d <= inner.im.d
    first_limbs: Limbs,
    last_limbs: Limbs,
}

static_assert!(mem::size_of::<Limbs>() == 16);

// SAFETY: mpc_t is thread safe as guaranteed by the MPC library.
unsafe impl Send for MiniComplex {}
unsafe impl Sync for MiniComplex {}

impl Default for MiniComplex {
    #[inline]
    fn default() -> Self {
        MiniComplex::new()
    }
}

impl Display for MiniComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&*self.borrow(), f)
    }
}

impl Debug for MiniComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&*self.borrow(), f)
    }
}

impl LowerExp for MiniComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerExp::fmt(&*self.borrow(), f)
    }
}

impl UpperExp for MiniComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperExp::fmt(&*self.borrow(), f)
    }
}

impl Binary for MiniComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Binary::fmt(&*self.borrow(), f)
    }
}

impl Octal for MiniComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Octal::fmt(&*self.borrow(), f)
    }
}

impl LowerHex for MiniComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerHex::fmt(&*self.borrow(), f)
    }
}

impl UpperHex for MiniComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperHex::fmt(&*self.borrow(), f)
    }
}

impl MiniComplex {
    /// Creates a [`MiniComplex`] with value 0 and the [minimum possible
    /// precision][crate::float::prec_min].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::complex::MiniComplex;
    /// let c = MiniComplex::new();
    /// // Borrow c as if it were Complex.
    /// assert_eq!(*c.borrow(), 0);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        let d = NonNull::dangling();
        MiniComplex {
            inner: mpc_t {
                re: mpfr_t {
                    prec: float::prec_min() as prec_t,
                    sign: 1,
                    exp: xmpfr::EXP_ZERO,
                    d,
                },
                im: mpfr_t {
                    prec: float::prec_min() as prec_t,
                    sign: 1,
                    exp: xmpfr::EXP_ZERO,
                    d,
                },
            },
            first_limbs: small_limbs![],
            last_limbs: small_limbs![],
        }
    }

    /// Creates a [`MiniComplex`] from a [`MiniFloat`] real part.
    ///
    /// This is equivalent to `MiniComplex::from(real)`, but can also be used in
    /// constant context. Unless required in constant context, use the [`From`]
    /// trait instead.
    ///
    /// The precision of the imaginary part is set to the precision of the real
    /// part.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::complex::{BorrowComplex, MiniComplex};
    /// use rug::float::MiniFloat;
    /// use rug::Complex;
    ///
    /// const TWO_FLOAT: MiniFloat = MiniFloat::const_from_i8(2i8);
    /// const TWO_MINI: MiniComplex = MiniComplex::const_from_real(TWO_FLOAT);
    /// const TWO_BORROW: BorrowComplex = TWO_MINI.borrow();
    /// const TWO: &Complex = BorrowComplex::const_deref(&TWO_BORROW);
    /// assert_eq!(*TWO, 2);
    /// assert_eq!(TWO.prec(), (i8::BITS, i8::BITS));
    /// ```
    #[inline]
    pub const fn const_from_real(real: MiniFloat) -> Self {
        let MiniFloat { inner, limbs } = real;
        MiniComplex {
            inner: mpc_t {
                re: inner,
                im: mpfr_t {
                    prec: inner.prec,
                    sign: 1,
                    exp: xmpfr::EXP_ZERO,
                    d: inner.d,
                },
            },
            first_limbs: limbs,
            last_limbs: small_limbs![],
        }
    }

    /// Creates a [`MiniComplex`] from two [`MiniFloat`] parts.
    ///
    /// This is equivalent to `MiniComplex::from((real, imag))`, but can also be
    /// used in constant context. Unless required in constant context, use the
    /// [`From`] trait instead.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the [`From`] trait is usable in
    /// constant context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::complex::{BorrowComplex, MiniComplex};
    /// use rug::float::MiniFloat;
    /// use rug::Complex;
    ///
    /// const TWO_FLOAT: MiniFloat = MiniFloat::const_from_i8(2i8);
    /// const THOUSAND_FLOAT: MiniFloat = MiniFloat::const_from_i16(1000i16);
    /// const TWO_1000I_MINI: MiniComplex =
    ///     MiniComplex::const_from_parts(TWO_FLOAT, THOUSAND_FLOAT);
    /// const TWO_1000I_BORROW: BorrowComplex = TWO_1000I_MINI.borrow();
    /// const TWO_1000I: &Complex = BorrowComplex::const_deref(&TWO_1000I_BORROW);
    /// assert_eq!(*TWO_1000I, (2, 1000));
    /// assert_eq!(TWO_1000I.prec(), (i8::BITS, i16::BITS));
    /// ```
    #[inline]
    pub const fn const_from_parts(real: MiniFloat, imag: MiniFloat) -> Self {
        let MiniFloat {
            inner: mut re_inner,
            limbs: re_limbs,
        } = real;
        let MiniFloat {
            inner: mut im_inner,
            limbs: im_limbs,
        } = imag;
        let d = NonNull::dangling();
        // remove d pointer relation
        re_inner.d = d;
        im_inner.d = d;
        MiniComplex {
            inner: mpc_t {
                re: re_inner,
                im: im_inner,
            },
            first_limbs: re_limbs,
            last_limbs: im_limbs,
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
    /// use rug::complex::MiniComplex;
    /// let mut c = MiniComplex::from((1.0f32, 3.0f32));
    /// // rotation does not change the precision
    /// unsafe {
    ///     c.as_nonreallocating_complex().mul_i_mut(false);
    /// }
    /// assert_eq!(*c.borrow(), (-3.0, 1.0));
    /// ```
    #[inline]
    pub unsafe fn as_nonreallocating_complex(&mut self) -> &mut Complex {
        // Update re.d and im.d to point to limbs.
        let first = NonNull::<[MaybeUninit<limb_t>]>::from(&self.first_limbs[..]).cast();
        let last = NonNull::<[MaybeUninit<limb_t>]>::from(&self.last_limbs[..]).cast();
        let (re_d, im_d) = if self.re_is_first() {
            (first, last)
        } else {
            (last, first)
        };
        self.inner.re.d = re_d;
        self.inner.im.d = im_d;
        let ptr = misc::cast_ptr_mut(&mut self.inner);
        // SAFETY: since inner.re.d and inner.im.d point to the limbs, it is
        // in a consistent state.
        unsafe { &mut *ptr }
    }

    /// Borrows the complex number.
    ///
    /// The returned object implements
    /// <code>[Deref]\<[Target][Deref::Target] = [Complex]></code>.
    ///
    /// The borrow lasts until the returned object exits scope. Multiple borrows
    /// can be taken at the same time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::complex::MiniComplex;
    /// use rug::Complex;
    /// let c = MiniComplex::from((-13f64, 5.5f64));
    /// let b = c.borrow();
    /// let conj = Complex::with_val(53, b.conj_ref());
    /// assert_eq!(*conj.real(), -13);
    /// assert_eq!(*conj.imag(), -5.5);
    /// ```
    #[inline]
    pub const fn borrow(&self) -> BorrowComplex<'_> {
        let first_d: *const Limbs = &self.first_limbs;
        let last_d: *const Limbs = &self.last_limbs;
        let (re_d, im_d) = if self.re_is_first() {
            (first_d, last_d)
        } else {
            (last_d, first_d)
        };
        // SAFETY: Since re_d and im_d point to the limbs, the mpc_t is in a
        // consistent state. Also, the lifetime of the BorrowComplex is the
        // lifetime of self, which covers the limbs.
        unsafe {
            BorrowComplex::from_raw(mpc_t {
                re: mpfr_t {
                    prec: self.inner.re.prec,
                    sign: self.inner.re.sign,
                    exp: self.inner.re.exp,
                    d: NonNull::new_unchecked(re_d.cast_mut().cast()),
                },
                im: mpfr_t {
                    prec: self.inner.im.prec,
                    sign: self.inner.im.sign,
                    exp: self.inner.im.exp,
                    d: NonNull::new_unchecked(im_d.cast_mut().cast()),
                },
            })
        }
    }

    /// Borrows the complex number exclusively.
    ///
    /// This is similar to the [`borrow`][Self::borrow] method, but it requires
    /// exclusive access to the underlying [`MiniComplex`]; the returned
    /// reference can however be shared. The exclusive access is required to
    /// reduce the amount of housekeeping necessary, providing a more efficient
    /// operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::complex::MiniComplex;
    /// use rug::Complex;
    /// let mut c = MiniComplex::from((-13f64, 5.5f64));
    /// let b = c.borrow_excl();
    /// let conj = Complex::with_val(53, b.conj_ref());
    /// assert_eq!(*conj.real(), -13);
    /// assert_eq!(*conj.imag(), -5.5);
    /// ```
    #[inline]
    pub fn borrow_excl(&mut self) -> &Complex {
        // SAFETY: since the return is a const reference, there will be no reallocation
        unsafe { &*self.as_nonreallocating_complex() }
    }

    #[inline]
    const fn re_is_first(&self) -> bool {
        // SAFETY: re.d and im.d were created either from the same dangling
        // pointer, or from fields in the same struct
        let re_ptr = self.inner.re.d.as_ptr();
        let im_ptr = self.inner.im.d.as_ptr();
        unsafe { re_ptr.offset_from(im_ptr) <= 0 }
    }
}

impl Assign<MiniFloat> for MiniComplex {
    #[inline]
    fn assign(&mut self, src: MiniFloat) {
        // make re is first
        self.inner.im.d = self.inner.re.d;
        self.inner.re.prec = src.inner.prec;
        self.inner.re.sign = src.inner.sign;
        self.inner.re.exp = src.inner.exp;
        self.inner.im.prec = src.inner.prec;
        self.inner.im.sign = 1;
        self.inner.im.exp = xmpfr::EXP_ZERO;
        self.first_limbs = src.limbs;
    }
}

impl From<MiniFloat> for MiniComplex {
    #[inline]
    fn from(src: MiniFloat) -> Self {
        MiniComplex::const_from_real(src)
    }
}

impl Assign<(MiniFloat, MiniFloat)> for MiniComplex {
    #[inline]
    fn assign(&mut self, src: (MiniFloat, MiniFloat)) {
        // make re is first
        self.inner.im.d = self.inner.re.d;
        self.inner.re.prec = src.0.inner.prec;
        self.inner.re.sign = src.0.inner.sign;
        self.inner.re.exp = src.0.inner.exp;
        self.inner.im.prec = src.1.inner.prec;
        self.inner.im.sign = src.1.inner.sign;
        self.inner.im.exp = src.1.inner.exp;
        self.first_limbs = src.0.limbs;
        self.last_limbs = src.1.limbs;
    }
}

impl From<(MiniFloat, MiniFloat)> for MiniComplex {
    #[inline]
    fn from(src: (MiniFloat, MiniFloat)) -> Self {
        MiniComplex::const_from_parts(src.0, src.1)
    }
}

impl<Re: ToMini> Assign<Re> for MiniComplex {
    fn assign(&mut self, src: Re) {
        // make re is first
        self.inner.im.d = self.inner.re.d;
        src.copy(&mut self.inner.re, &mut self.first_limbs);
        self.inner.im.prec = self.inner.re.prec;
        self.inner.im.sign = 1;
        self.inner.im.exp = xmpfr::EXP_ZERO;
    }
}

impl<Re: ToMini> From<Re> for MiniComplex {
    fn from(src: Re) -> Self {
        let re = MiniFloat::from(src);
        MiniComplex::const_from_real(re)
    }
}

impl<Re: ToMini, Im: ToMini> Assign<(Re, Im)> for MiniComplex {
    fn assign(&mut self, src: (Re, Im)) {
        // make re is first
        self.inner.im.d = self.inner.re.d;
        src.0.copy(&mut self.inner.re, &mut self.first_limbs);
        src.1.copy(&mut self.inner.im, &mut self.last_limbs);
    }
}

impl<Re: ToMini, Im: ToMini> From<(Re, Im)> for MiniComplex {
    #[inline]
    fn from(src: (Re, Im)) -> Self {
        let re = MiniFloat::from(src.0);
        let im = MiniFloat::from(src.1);
        MiniComplex::const_from_parts(re, im)
    }
}

#[cfg(feature = "num-complex")]
impl<T: ToMini> Assign<NumComplex<T>> for MiniComplex {
    fn assign(&mut self, src: NumComplex<T>) {
        // make re is first
        self.inner.im.d = self.inner.re.d;
        src.re.copy(&mut self.inner.re, &mut self.first_limbs);
        src.im.copy(&mut self.inner.im, &mut self.last_limbs);
    }
}

#[cfg(feature = "num-complex")]
impl<T: ToMini> From<NumComplex<T>> for MiniComplex {
    #[inline]
    fn from(src: NumComplex<T>) -> Self {
        let re = MiniFloat::from(src.re);
        let im = MiniFloat::from(src.im);
        MiniComplex::const_from_parts(re, im)
    }
}

impl Assign<&Self> for MiniComplex {
    #[inline]
    fn assign(&mut self, other: &Self) {
        self.clone_from(other);
    }
}

impl Assign for MiniComplex {
    #[inline]
    fn assign(&mut self, other: Self) {
        *self = other;
    }
}

#[cfg(test)]
mod tests {
    use crate::complex::MiniComplex;
    use crate::float;
    use crate::float::{FreeCache, MiniFloat, Special};
    use crate::{Assign, Complex};

    #[test]
    fn check_assign() {
        let mut c = MiniComplex::from((1.0, 2.0));
        assert_eq!(*c.borrow_excl(), (1.0, 2.0));
        c.assign(3.0);
        assert_eq!(*c.borrow_excl(), (3.0, 0.0));
        let other = MiniComplex::from((4.0, 5.0));
        c.assign(&other);
        assert_eq!(*c.borrow_excl(), (4.0, 5.0));
        c.assign((6.0, 7.0));
        assert_eq!(*c.borrow_excl(), (6.0, 7.0));
        c.assign(other);
        assert_eq!(*c.borrow_excl(), (4.0, 5.0));

        float::free_cache(FreeCache::All);
    }

    fn swapped_parts(small: &MiniComplex) -> bool {
        unsafe {
            let borrow = small.borrow();
            let re = (*borrow.real().as_raw()).d;
            let im = (*borrow.imag().as_raw()).d;
            re > im
        }
    }

    #[test]
    fn check_swapped_parts() {
        let mut c = MiniComplex::from((1, 2));
        assert_eq!(*c.borrow_excl(), (1, 2));
        assert_eq!(*c.clone().borrow_excl(), c);
        let mut orig_swapped_parts = swapped_parts(&c);
        unsafe {
            assert_eq!(c.borrow_excl().real().prec(), c.borrow_excl().imag().prec());
            c.as_nonreallocating_complex().mul_i_mut(false);
        }
        assert_eq!(*c.borrow_excl(), (-2, 1));
        assert_eq!(*c.clone().borrow_excl(), c);
        assert!(swapped_parts(&c) != orig_swapped_parts);

        c.assign(12);
        assert_eq!(*c.borrow_excl(), 12);
        assert_eq!(*c.clone().borrow_excl(), c);
        orig_swapped_parts = swapped_parts(&c);
        unsafe {
            assert_eq!(c.borrow_excl().real().prec(), c.borrow_excl().imag().prec());
            c.as_nonreallocating_complex().mul_i_mut(false);
        }
        assert_eq!(*c.borrow_excl(), (0, 12));
        assert_eq!(*c.clone().borrow_excl(), c);
        assert!(swapped_parts(&c) != orig_swapped_parts);

        c.assign((4, 5));
        assert_eq!(*c.borrow_excl(), (4, 5));
        assert_eq!(*c.clone().borrow_excl(), c);
        orig_swapped_parts = swapped_parts(&c);
        unsafe {
            assert_eq!(c.borrow_excl().real().prec(), c.borrow_excl().imag().prec());
            c.as_nonreallocating_complex().mul_i_mut(false);
        }
        assert_eq!(*c.borrow_excl(), (-5, 4));
        assert_eq!(*c.clone().borrow_excl(), c);
        assert!(swapped_parts(&c) != orig_swapped_parts);
    }

    #[test]
    fn check_traits() {
        assert!(MiniComplex::default().borrow_excl().is_zero());

        let mini = MiniComplex::from((-5.2f64, 4u128));
        let check = Complex::with_val((53, 128), (-5.2f64, 4u128));
        assert_eq!(format!("{mini}"), format!("{check}"));
        assert_eq!(format!("{mini:?}"), format!("{check:?}"));
        assert_eq!(format!("{mini:e}"), format!("{check:e}"));
        assert_eq!(format!("{mini:E}"), format!("{check:E}"));
        assert_eq!(format!("{mini:b}"), format!("{check:b}"));
        assert_eq!(format!("{mini:o}"), format!("{check:o}"));
        assert_eq!(format!("{mini:x}"), format!("{check:x}"));
        assert_eq!(format!("{mini:X}"), format!("{check:X}"));
    }

    macro_rules! compare_conv {
        ($T:ident, $prec:expr, [$($val:expr),+] $( as $U:ident)?) => {
            for &val in &[$($val),+] {
                let float = MiniFloat::from(val);
                let a = MiniComplex::from(float);
                let b = MiniComplex::const_from_real(float);
                let mut c = MiniComplex::new();
                c.assign(float);
                assert_eq!(*a.borrow(), val $(as $U)?);
                assert_eq!(*b.borrow(), val $(as $U)?);
                assert_eq!(*c.borrow(), val $(as $U)?);
                assert_eq!(a.borrow().prec(), ($prec, $prec));
                assert_eq!(b.borrow().prec(), ($prec, $prec));
                assert_eq!(c.borrow().prec(), ($prec, $prec));

                let one = MiniFloat::from(true);
                let a = MiniComplex::from((float, one));
                let b = MiniComplex::const_from_parts(float, one);
                let mut c = MiniComplex::new();
                c.assign((float, one));
                assert_eq!(*a.borrow(), (val $(as $U)?, 1));
                assert_eq!(*b.borrow(), (val $(as $U)?, 1));
                assert_eq!(*c.borrow(), (val $(as $U)?, 1));
                assert_eq!(a.borrow().prec(), ($prec, 1));
                assert_eq!(b.borrow().prec(), ($prec, 1));
                assert_eq!(c.borrow().prec(), ($prec, 1));

                let a = MiniComplex::from((one, float));
                let b = MiniComplex::const_from_parts(one, float);
                let mut c = MiniComplex::new();
                c.assign((one, float));
                assert_eq!(*a.borrow(), (1, val $(as $U)?));
                assert_eq!(*b.borrow(), (1, val $(as $U)?));
                assert_eq!(*c.borrow(), (1, val $(as $U)?));
                assert_eq!(a.borrow().prec(), (1, $prec));
                assert_eq!(b.borrow().prec(), (1, $prec));
                assert_eq!(c.borrow().prec(), (1, $prec));
            }
        };
    }

    #[test]
    fn check_equiv_convs() {
        compare_conv!(bool, 1, [false, true] as u8);
        compare_conv!(i8, i8::BITS, [i8::MIN, 0, i8::MAX]);
        compare_conv!(i16, i16::BITS, [i16::MIN, 0, i16::MAX]);
        compare_conv!(i32, i32::BITS, [i32::MIN, 0, i32::MAX]);
        compare_conv!(i64, i64::BITS, [i64::MIN, 0, i64::MAX]);
        compare_conv!(i128, i128::BITS, [i128::MIN, 0, i128::MAX]);
        compare_conv!(isize, isize::BITS, [isize::MIN, 0, isize::MAX]);
        compare_conv!(u8, u8::BITS, [0, u8::MAX]);
        compare_conv!(u16, u16::BITS, [0, u16::MAX]);
        compare_conv!(u32, u32::BITS, [0, u32::MAX]);
        compare_conv!(u64, u64::BITS, [0, u64::MAX]);
        compare_conv!(u128, u128::BITS, [0, u128::MAX]);
        compare_conv!(usize, usize::BITS, [0, usize::MAX]);
        compare_conv!(
            f32,
            f32::MANTISSA_DIGITS,
            [f32::MIN, 0.0, f32::MAX, f32::INFINITY]
        );
        compare_conv!(
            f64,
            f64::MANTISSA_DIGITS,
            [f64::MIN, 0.0, f64::MAX, f64::INFINITY]
        );
        compare_conv!(Special, 1, [Special::NegZero, Special::Infinity]);
    }
}
