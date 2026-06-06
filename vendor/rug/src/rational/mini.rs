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

use crate::ext::xmpq;
use crate::integer::{MiniInteger, ToMini};
use crate::misc;
use crate::rational::BorrowRational;
use crate::{Assign, Rational};
use az::Cast;
use core::ffi::c_int;
use core::fmt::{
    Binary, Debug, Display, Formatter, LowerHex, Octal, Result as FmtResult, UpperHex,
};
use core::mem;
use core::mem::MaybeUninit;
#[allow(unused_imports)]
use core::ops::Deref;
use core::ptr::NonNull;
use gmp_mpfr_sys::gmp;
use gmp_mpfr_sys::gmp::{limb_t, mpq_t, mpz_t};

const LIMBS_IN_SMALL: usize = (128 / gmp::LIMB_BITS) as usize;
type Limbs = [MaybeUninit<limb_t>; LIMBS_IN_SMALL];

/**
A small rational number that does not require any memory allocation.

This can be useful when you have a numerator and denominator that are primitive
integer-types such as [`i64`] or [`u8`], and you need a reference to a
[`Rational`].

Although no allocation is required, setting the value of a `MiniRational` does
require some computation, as the numerator and denominator need to be
canonicalized.

The [`borrow`][Self::borrow] method returns an object that can be coerced to a
[`Rational`], as it implements
<code>[Deref]\<[Target][Deref::Target] = [Rational]></code>.

# Examples

```rust
use rug::rational::MiniRational;
use rug::Rational;
// `a` requires a heap allocation
let mut a = Rational::from((100, 13));
// `b` can reside on the stack
let b = MiniRational::from((-100, 21));
a /= &*b.borrow();
assert_eq!(*a.numer(), -21);
assert_eq!(*a.denom(), 13);
```
*/
#[derive(Clone, Copy)]
pub struct MiniRational {
    inner: mpq_t,
    // numerator is first in limbs if inner.num.d <= inner.den.d
    first_limbs: Limbs,
    last_limbs: Limbs,
}

static_assert!(mem::size_of::<Limbs>() == 16);

// SAFETY: mpq_t is thread safe as guaranteed by the GMP library.
unsafe impl Send for MiniRational {}
unsafe impl Sync for MiniRational {}

impl Default for MiniRational {
    #[inline]
    fn default() -> Self {
        MiniRational::new()
    }
}

impl Display for MiniRational {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&*self.borrow(), f)
    }
}

impl Debug for MiniRational {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&*self.borrow(), f)
    }
}

impl Binary for MiniRational {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Binary::fmt(&*self.borrow(), f)
    }
}

impl Octal for MiniRational {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Octal::fmt(&*self.borrow(), f)
    }
}

impl LowerHex for MiniRational {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerHex::fmt(&*self.borrow(), f)
    }
}

impl UpperHex for MiniRational {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperHex::fmt(&*self.borrow(), f)
    }
}

impl MiniRational {
    /// Creates a [`MiniRational`] with value 0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rational::MiniRational;
    /// let r = MiniRational::new();
    /// let b = r.borrow();
    /// // Use b as if it were Rational.
    /// assert_eq!(*b.numer(), 0);
    /// assert_eq!(*b.denom(), 1);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        let d = NonNull::dangling();
        MiniRational {
            inner: mpq_t {
                num: mpz_t {
                    alloc: LIMBS_IN_SMALL as c_int,
                    size: 0,
                    d,
                },
                den: mpz_t {
                    alloc: LIMBS_IN_SMALL as c_int,
                    size: 1,
                    d,
                },
            },
            first_limbs: small_limbs![],
            last_limbs: small_limbs![1],
        }
    }

    /// Creates a [`MiniRational`] from a [`MiniInteger`].
    ///
    /// This is equivalent to `MiniRational::from(val)`, but can also be used in
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
    /// use rug::integer::MiniInteger;
    /// use rug::rational::{BorrowRational, MiniRational};
    /// use rug::Rational;
    ///
    /// const TWO_INT: MiniInteger = MiniInteger::const_from_i8(2i8);
    /// const TWO_MINI: MiniRational = MiniRational::const_from_integer(TWO_INT);
    /// const TWO_BORROW: BorrowRational = TWO_MINI.borrow();
    /// const TWO: &Rational = BorrowRational::const_deref(&TWO_BORROW);
    /// assert_eq!(*TWO, 2);
    ///
    /// const HALF_BORROW: BorrowRational = TWO.as_recip();
    /// const HALF: &Rational = BorrowRational::const_deref(&HALF_BORROW);
    /// assert_eq!(*HALF, MiniRational::from((1, 2)));
    /// ```
    #[inline]
    pub const fn const_from_integer(val: MiniInteger) -> Self {
        let MiniInteger {
            inner: mut num_inner,
            limbs: num_limbs,
        } = val;
        let MiniInteger {
            inner: mut den_inner,
            limbs: den_limbs,
        } = MiniInteger::const_from_u8(1);
        let d = NonNull::dangling();
        // remove d pointer relation
        num_inner.d = d;
        den_inner.d = d;
        MiniRational {
            inner: mpq_t {
                num: num_inner,
                den: den_inner,
            },
            first_limbs: num_limbs,
            last_limbs: den_limbs,
        }
    }

    /// Creates a [`MiniRational`] from a numerator [`MiniInteger`] and a
    /// denominator [`MiniInteger`], assuming they are in canonical form.
    ///
    /// # Planned deprecation
    ///
    /// This method will be deprecated when the
    /// [`from_canonical`][Self::from_canonical] method is usable in constant
    /// context.
    ///
    /// # Safety
    ///
    /// This method leads to undefined behavior if `den` is zero or negative, or
    /// if `num` and `den` have common factors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::MiniInteger;
    /// use rug::rational::{BorrowRational, MiniRational};
    /// use rug::Rational;
    ///
    /// const NEG_TWO_INT: MiniInteger = MiniInteger::const_from_i8(-2);
    /// const THREE_INT: MiniInteger = MiniInteger::const_from_i8(3);
    /// const NEG_TWO_THIRDS_MINI: MiniRational = unsafe {
    ///     MiniRational::const_from_canonical(NEG_TWO_INT, THREE_INT)
    /// };
    /// const NEG_TWO_THIRDS_BORROW: BorrowRational = NEG_TWO_THIRDS_MINI.borrow();
    /// const NEG_TWO_THIRDS: &Rational = BorrowRational::const_deref(&NEG_TWO_THIRDS_BORROW);
    /// assert_eq!(*NEG_TWO_THIRDS, MiniRational::from((-2, 3)));
    /// ```
    pub const unsafe fn const_from_canonical(num: MiniInteger, den: MiniInteger) -> Self {
        let MiniInteger {
            inner: mut num_inner,
            limbs: num_limbs,
        } = num;
        let MiniInteger {
            inner: mut den_inner,
            limbs: den_limbs,
        } = den;
        let d = NonNull::dangling();
        // remove d pointer relation
        num_inner.d = d;
        den_inner.d = d;
        MiniRational {
            inner: mpq_t {
                num: num_inner,
                den: den_inner,
            },
            first_limbs: num_limbs,
            last_limbs: den_limbs,
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
    /// use rug::rational::MiniRational;
    /// let mut r = MiniRational::from((-15i32, 47i32));
    /// let (num_capacity, den_capacity) = {
    ///     let b = r.borrow();
    ///     (b.numer().capacity(), b.denom().capacity())
    /// };
    /// // reciprocating this will not require reallocations
    /// unsafe {
    ///     r.as_nonreallocating_rational().recip_mut();
    /// }
    /// let after = r.borrow();
    /// assert_eq!(*after, MiniRational::from((-47, 15)));
    /// assert_eq!(after.numer().capacity(), num_capacity);
    /// assert_eq!(after.denom().capacity(), den_capacity);
    /// ```
    ///
    /// [`recip_mut`]: `Rational::recip_mut`
    #[inline]
    pub unsafe fn as_nonreallocating_rational(&mut self) -> &mut Rational {
        // Update num.d and den.d to point to limbs.
        let first = NonNull::<[MaybeUninit<limb_t>]>::from(&self.first_limbs[..]).cast();
        let last = NonNull::<[MaybeUninit<limb_t>]>::from(&self.last_limbs[..]).cast();
        let (num_d, den_d) = if self.num_is_first() {
            (first, last)
        } else {
            (last, first)
        };
        self.inner.num.d = num_d;
        self.inner.den.d = den_d;
        let ptr = misc::cast_ptr_mut(&mut self.inner);
        // SAFETY: since inner.num.d and inner.den.d point to the limbs, it is
        // in a consistent state.
        unsafe { &mut *ptr }
    }

    /// Borrows the rational number.
    ///
    /// The returned object implements
    /// <code>[Deref]\<[Target][Deref::Target] = [Rational]></code>.
    ///
    /// The borrow lasts until the returned object exits scope. Multiple borrows
    /// can be taken at the same time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rational::MiniRational;
    /// use rug::Rational;
    /// let r = MiniRational::from((-13i32, 5i32));
    /// let b = r.borrow();
    /// let abs_ref = Rational::from(b.abs_ref());
    /// assert_eq!(*abs_ref.numer(), 13);
    /// assert_eq!(*abs_ref.denom(), 5);
    /// ```
    #[inline]
    pub const fn borrow(&self) -> BorrowRational<'_> {
        let first_d: *const Limbs = &self.first_limbs;
        let last_d: *const Limbs = &self.last_limbs;
        let (num_d, den_d) = if self.num_is_first() {
            (first_d, last_d)
        } else {
            (last_d, first_d)
        };
        // SAFETY: Since num_d and den_d point to the limbs, the mpq_t is in a
        // consistent state. Also, the lifetime of the BorrowRational is the
        // lifetime of self, which covers the limbs.
        unsafe {
            BorrowRational::from_raw(mpq_t {
                num: mpz_t {
                    alloc: self.inner.num.alloc,
                    size: self.inner.num.size,
                    d: NonNull::new_unchecked(num_d.cast_mut().cast()),
                },
                den: mpz_t {
                    alloc: self.inner.den.alloc,
                    size: self.inner.den.size,
                    d: NonNull::new_unchecked(den_d.cast_mut().cast()),
                },
            })
        }
    }

    /// Borrows the rational number exclusively.
    ///
    /// This is similar to the [`borrow`][Self::borrow] method, but it requires
    /// exclusive access to the underlying [`MiniRational`]; the returned
    /// reference can however be shared. The exclusive access is required to
    /// reduce the amount of housekeeping necessary, providing a more efficient
    /// operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rational::MiniRational;
    /// use rug::Rational;
    /// let mut r = MiniRational::from((-13i32, 5i32));
    /// let b = r.borrow_excl();
    /// let abs_ref = Rational::from(b.abs_ref());
    /// assert_eq!(*abs_ref.numer(), 13);
    /// assert_eq!(*abs_ref.denom(), 5);
    /// ```
    #[inline]
    pub fn borrow_excl(&mut self) -> &Rational {
        // SAFETY: since the return is a const reference, there will be no reallocation
        unsafe { &*self.as_nonreallocating_rational() }
    }

    /// Creates a [`MiniRational`] from a numerator and denominator, assuming
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
    /// use rug::rational::MiniRational;
    /// let from_unsafe = unsafe { MiniRational::from_canonical(-13, 10) };
    /// // from_safe is canonicalized to the same form as from_unsafe
    /// let from_safe = MiniRational::from((130, -100));
    /// let unsafe_borrow = from_unsafe.borrow();
    /// let safe_borrow = from_safe.borrow();
    /// assert_eq!(unsafe_borrow.numer(), safe_borrow.numer());
    /// assert_eq!(unsafe_borrow.denom(), safe_borrow.denom());
    /// ```
    pub unsafe fn from_canonical<Num: ToMini, Den: ToMini>(num: Num, den: Den) -> Self {
        let mut num_size = 0;
        let mut den_size = 0;
        let mut num_limbs: Limbs = small_limbs![];
        let mut den_limbs: Limbs = small_limbs![];
        num.copy(&mut num_size, &mut num_limbs);
        den.copy(&mut den_size, &mut den_limbs);
        // since inner.num.d == inner.den.d, first_limbs are num_limbs
        let d = NonNull::dangling();
        MiniRational {
            inner: mpq_t {
                num: mpz_t {
                    alloc: LIMBS_IN_SMALL.cast(),
                    size: num_size,
                    d,
                },
                den: mpz_t {
                    alloc: LIMBS_IN_SMALL.cast(),
                    size: den_size,
                    d,
                },
            },
            first_limbs: num_limbs,
            last_limbs: den_limbs,
        }
    }

    /// Assigns a numerator and denominator to a [`MiniRational`], assuming
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
    /// use rug::rational::MiniRational;
    /// use rug::Assign;
    /// let mut a = MiniRational::new();
    /// unsafe {
    ///     a.assign_canonical(-13, 10);
    /// }
    /// // b is canonicalized to the same form as a
    /// let mut b = MiniRational::new();
    /// b.assign((130, -100));
    /// let a_borrow = a.borrow();
    /// let b_borrow = b.borrow();
    /// assert_eq!(a_borrow.numer(), b_borrow.numer());
    /// assert_eq!(a_borrow.denom(), b_borrow.denom());
    /// ```
    pub unsafe fn assign_canonical<Num: ToMini, Den: ToMini>(&mut self, num: Num, den: Den) {
        // make num is first
        self.inner.den.d = self.inner.num.d;
        num.copy(&mut self.inner.num.size, &mut self.first_limbs);
        den.copy(&mut self.inner.den.size, &mut self.last_limbs);
    }

    #[inline]
    const fn num_is_first(&self) -> bool {
        // SAFETY: num.d and den.d were created either from the same dangling
        // pointer, or from fields in the same struct
        let num_ptr = self.inner.num.d.as_ptr();
        let den_ptr = self.inner.den.d.as_ptr();
        unsafe { num_ptr.offset_from(den_ptr) <= 0 }
    }
}

impl Assign<MiniInteger> for MiniRational {
    #[inline]
    fn assign(&mut self, src: MiniInteger) {
        // make num is first
        self.inner.den.d = self.inner.num.d;
        self.inner.num.size = src.inner.size;
        self.first_limbs = src.limbs;
        self.inner.den.size = 1;
        self.last_limbs[0] = MaybeUninit::new(1);
    }
}

impl From<MiniInteger> for MiniRational {
    #[inline]
    fn from(src: MiniInteger) -> Self {
        MiniRational::const_from_integer(src)
    }
}

impl<Num: ToMini> Assign<Num> for MiniRational {
    #[inline]
    fn assign(&mut self, src: Num) {
        // make num is first
        self.inner.den.d = self.inner.num.d;
        src.copy(&mut self.inner.num.size, &mut self.first_limbs);
        self.inner.den.size = 1;
        self.last_limbs[0] = MaybeUninit::new(1);
    }
}

impl<Num: ToMini> From<Num> for MiniRational {
    fn from(src: Num) -> Self {
        let mut num_size = 0;
        let mut num_limbs = small_limbs![];
        src.copy(&mut num_size, &mut num_limbs);
        // since inner.num.d == inner.den.d, first_limbs are num_limbs
        let d = NonNull::dangling();
        MiniRational {
            inner: mpq_t {
                num: mpz_t {
                    alloc: LIMBS_IN_SMALL.cast(),
                    size: num_size,
                    d,
                },
                den: mpz_t {
                    alloc: LIMBS_IN_SMALL.cast(),
                    size: 1,
                    d,
                },
            },
            first_limbs: num_limbs,
            last_limbs: small_limbs![1],
        }
    }
}

impl<Num: ToMini, Den: ToMini> Assign<(Num, Den)> for MiniRational {
    fn assign(&mut self, src: (Num, Den)) {
        assert!(!src.1.is_zero(), "division by zero");
        // make num is first
        self.inner.den.d = self.inner.num.d;
        src.0.copy(&mut self.inner.num.size, &mut self.first_limbs);
        src.1.copy(&mut self.inner.den.size, &mut self.last_limbs);
        // SAFETY: canonicalization will never need to make a number larger.
        xmpq::canonicalize(unsafe { self.as_nonreallocating_rational() });
    }
}

impl<Num: ToMini, Den: ToMini> From<(Num, Den)> for MiniRational {
    fn from(src: (Num, Den)) -> Self {
        assert!(!src.1.is_zero(), "division by zero");
        let d = NonNull::dangling();
        let mut ret = MiniRational {
            inner: mpq_t {
                num: mpz_t {
                    alloc: LIMBS_IN_SMALL.cast(),
                    size: 0,
                    d,
                },
                den: mpz_t {
                    alloc: LIMBS_IN_SMALL.cast(),
                    size: 0,
                    d,
                },
            },
            first_limbs: small_limbs![],
            last_limbs: small_limbs![],
        };
        src.0.copy(&mut ret.inner.num.size, &mut ret.first_limbs);
        src.1.copy(&mut ret.inner.den.size, &mut ret.last_limbs);
        // SAFETY: canonicalization will never need to make a number larger.
        xmpq::canonicalize(unsafe { ret.as_nonreallocating_rational() });
        ret
    }
}

impl Assign<&Self> for MiniRational {
    #[inline]
    fn assign(&mut self, other: &Self) {
        self.clone_from(other);
    }
}

impl Assign for MiniRational {
    #[inline]
    fn assign(&mut self, other: Self) {
        *self = other;
    }
}

#[cfg(test)]
mod tests {
    use crate::integer::MiniInteger;
    use crate::rational::MiniRational;
    use crate::{Assign, Rational};

    #[test]
    fn check_assign() {
        let mut r = MiniRational::from((1, 2));
        assert_eq!(*r.borrow_excl(), MiniRational::from((1, 2)));
        r.assign(3);
        assert_eq!(*r.borrow_excl(), 3);
        let other = MiniRational::from((4, 5));
        r.assign(&other);
        assert_eq!(*r.borrow_excl(), MiniRational::from((4, 5)));
        r.assign((6, 7));
        assert_eq!(*r.borrow_excl(), MiniRational::from((6, 7)));
        r.assign(other);
        assert_eq!(*r.borrow_excl(), MiniRational::from((4, 5)));
    }

    fn swapped_parts(small: &MiniRational) -> bool {
        unsafe {
            let borrow = small.borrow();
            let num = (*borrow.numer().as_raw()).d;
            let den = (*borrow.denom().as_raw()).d;
            num > den
        }
    }

    #[test]
    fn check_swapped_parts() {
        let mut r = MiniRational::from((2, 3));
        assert_eq!(*r.borrow_excl(), MiniRational::from((2, 3)));
        assert_eq!(*r.clone().borrow_excl(), r);
        let mut orig_swapped_parts = swapped_parts(&r);
        unsafe {
            r.as_nonreallocating_rational().recip_mut();
        }
        assert_eq!(*r.borrow_excl(), MiniRational::from((3, 2)));
        assert_eq!(*r.clone().borrow_excl(), r);
        assert!(swapped_parts(&r) != orig_swapped_parts);

        unsafe {
            r.assign_canonical(5, 7);
        }
        assert_eq!(*r.borrow_excl(), MiniRational::from((5, 7)));
        assert_eq!(*r.clone().borrow_excl(), r);
        orig_swapped_parts = swapped_parts(&r);
        unsafe {
            r.as_nonreallocating_rational().recip_mut();
        }
        assert_eq!(*r.borrow_excl(), MiniRational::from((7, 5)));
        assert_eq!(*r.clone().borrow_excl(), r);
        assert!(swapped_parts(&r) != orig_swapped_parts);

        r.assign(2);
        assert_eq!(*r.borrow_excl(), 2);
        assert_eq!(*r.clone().borrow_excl(), r);
        orig_swapped_parts = swapped_parts(&r);
        unsafe {
            r.as_nonreallocating_rational().recip_mut();
        }
        assert_eq!(*r.borrow_excl(), MiniRational::from((1, 2)));
        assert_eq!(*r.clone().borrow_excl(), r);
        assert!(swapped_parts(&r) != orig_swapped_parts);

        r.assign((3, -5));
        assert_eq!(*r.borrow_excl(), MiniRational::from((-3, 5)));
        assert_eq!(*r.clone().borrow_excl(), r);
        orig_swapped_parts = swapped_parts(&r);
        unsafe {
            r.as_nonreallocating_rational().recip_mut();
        }
        assert_eq!(*r.borrow_excl(), MiniRational::from((-5, 3)));
        assert_eq!(*r.clone().borrow_excl(), r);
        assert!(swapped_parts(&r) != orig_swapped_parts);
    }

    #[test]
    fn check_traits() {
        assert!(MiniRational::default().borrow_excl().is_zero());

        let mini = MiniRational::from((14, -10));
        let check = Rational::from((14, -10));
        assert_eq!(format!("{mini}"), format!("{check}"));
        assert_eq!(format!("{mini:?}"), format!("{check:?}"));
        assert_eq!(format!("{mini:b}"), format!("{check:b}"));
        assert_eq!(format!("{mini:o}"), format!("{check:o}"));
        assert_eq!(format!("{mini:x}"), format!("{check:x}"));
        assert_eq!(format!("{mini:X}"), format!("{check:X}"));
    }

    macro_rules! compare_conv {
        ($T:ident, [$($val:expr),+] $( as $U:ident)?) => {
            for &val in &[$($val),+] {
                let integer = MiniInteger::from(val);
                let a = MiniRational::from(integer);
                let b = MiniRational::const_from_integer(integer);
                let mut c = MiniInteger::new();
                c.assign(integer);
                assert_eq!(*a.borrow(), val $(as $U)?);
                assert_eq!(*b.borrow(), val $(as $U)?);
                assert_eq!(*c.borrow(), val $(as $U)?);
            }
        };
    }

    #[test]
    fn check_equiv_convs() {
        compare_conv!(bool, [false, true] as u8);
        compare_conv!(i8, [i8::MIN, 0, i8::MAX]);
        compare_conv!(i16, [i16::MIN, 0, i16::MAX]);
        compare_conv!(i32, [i32::MIN, 0, i32::MAX]);
        compare_conv!(i64, [i64::MIN, 0, i64::MAX]);
        compare_conv!(i128, [i128::MIN, 0, i128::MAX]);
        compare_conv!(isize, [isize::MIN, 0, isize::MAX]);
        compare_conv!(u8, [0, u8::MAX]);
        compare_conv!(u16, [0, u16::MAX]);
        compare_conv!(u32, [0, u32::MAX]);
        compare_conv!(u64, [0, u64::MAX]);
        compare_conv!(u128, [0, u128::MAX]);
        compare_conv!(usize, [0, usize::MAX]);
    }
}
