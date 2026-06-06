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

use crate::integer::BorrowInteger;
use crate::misc;
use crate::misc::NegAbs;
use crate::{Assign, Integer};
use az::{Az, Cast, WrappingCast};
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
use gmp_mpfr_sys::gmp::{limb_t, mpz_t};

pub const LIMBS_IN_SMALL: usize = (128 / gmp::LIMB_BITS) as usize;
pub type Limbs = [MaybeUninit<limb_t>; LIMBS_IN_SMALL];

/**
A small integer that does not require any memory allocation.

This can be useful when you have a primitive integer type such as [`u64`] or
[`i8`], but need a reference to an [`Integer`].

If there are functions that take a [`u32`] or [`i32`] directly instead of an
[`Integer`] reference, using them can still be faster than using a
`MiniInteger`; the functions would still need to check for the size of an
[`Integer`] obtained using `MiniInteger`.

The [`borrow`][Self::borrow] method returns an object that can be coerced to an
[`Integer`], as it implements
<code>[Deref]\<[Target][Deref::Target] = [Integer]></code>.

# Examples

```rust
use rug::integer::MiniInteger;
use rug::Integer;
// `a` requires a heap allocation
let mut a = Integer::from(250);
// `b` can reside on the stack
let b = MiniInteger::from(-100);
a.lcm_mut(&b.borrow());
assert_eq!(a, 500);
// another computation:
a.lcm_mut(&MiniInteger::from(30).borrow());
assert_eq!(a, 1500);
```
*/
#[derive(Clone, Copy)]
pub struct MiniInteger {
    pub(crate) inner: mpz_t,
    pub(crate) limbs: Limbs,
}

static_assert!(mem::size_of::<Limbs>() == 16);

// SAFETY: mpz_t is thread safe as guaranteed by the GMP library.
unsafe impl Send for MiniInteger {}
unsafe impl Sync for MiniInteger {}

impl Default for MiniInteger {
    #[inline]
    fn default() -> Self {
        MiniInteger::new()
    }
}

impl Display for MiniInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&*self.borrow(), f)
    }
}

impl Debug for MiniInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&*self.borrow(), f)
    }
}

impl Binary for MiniInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Binary::fmt(&*self.borrow(), f)
    }
}

impl Octal for MiniInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Octal::fmt(&*self.borrow(), f)
    }
}

impl LowerHex for MiniInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerHex::fmt(&*self.borrow(), f)
    }
}

impl UpperHex for MiniInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperHex::fmt(&*self.borrow(), f)
    }
}

impl MiniInteger {
    /// Creates a [`MiniInteger`] with value 0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::MiniInteger;
    /// let i = MiniInteger::new();
    /// // Borrow i as if it were Integer.
    /// assert_eq!(*i.borrow(), 0);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        MiniInteger {
            inner: mpz_t {
                alloc: LIMBS_IN_SMALL as c_int,
                size: 0,
                d: NonNull::dangling(),
            },
            limbs: small_limbs![],
        }
    }

    /// Creates a [`MiniInteger`] from a [`bool`].
    ///
    /// This is equivalent to `MiniInteger::from(val)`, but can also be used in
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
    /// use rug::integer::{BorrowInteger, MiniInteger};
    /// use rug::Integer;
    ///
    /// const ONE_MINI: MiniInteger = MiniInteger::const_from_bool(true);
    /// const ONE_BORROW: BorrowInteger = ONE_MINI.borrow();
    /// const ONE: &Integer = BorrowInteger::const_deref(&ONE_BORROW);
    /// assert_eq!(*ONE, 1);
    /// ```
    #[inline]
    pub const fn const_from_bool(val: bool) -> Self {
        let (size, limbs) = from_bool(val);
        MiniInteger {
            inner: mpz_t {
                alloc: LIMBS_IN_SMALL as c_int,
                size,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniInteger`] from a [`i8`].
    ///
    /// This is equivalent to `MiniInteger::from(val)`, but can also be used in
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
    /// use rug::integer::{BorrowInteger, MiniInteger};
    /// use rug::Integer;
    ///
    /// const TWO_MINI: MiniInteger = MiniInteger::const_from_i8(2i8);
    /// const TWO_BORROW: BorrowInteger = TWO_MINI.borrow();
    /// const TWO: &Integer = BorrowInteger::const_deref(&TWO_BORROW);
    /// assert_eq!(*TWO, 2);
    /// ```
    #[inline]
    pub const fn const_from_i8(val: i8) -> Self {
        let (size, limbs) = from_i8(val);
        MiniInteger {
            inner: mpz_t {
                alloc: LIMBS_IN_SMALL as c_int,
                size,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniInteger`] from a [`i16`].
    ///
    /// This is equivalent to `MiniInteger::from(val)`, but can also be used in
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
    /// use rug::integer::{BorrowInteger, MiniInteger};
    /// use rug::Integer;
    ///
    /// const KIBI_MINI: MiniInteger = MiniInteger::const_from_i16(1i16 << 10);
    /// const KIBI_BORROW: BorrowInteger = KIBI_MINI.borrow();
    /// const KIBI: &Integer = BorrowInteger::const_deref(&KIBI_BORROW);
    /// assert_eq!(*KIBI, 1i16 << 10);
    /// ```
    #[inline]
    pub const fn const_from_i16(val: i16) -> Self {
        let (size, limbs) = from_i16(val);
        MiniInteger {
            inner: mpz_t {
                alloc: LIMBS_IN_SMALL as c_int,
                size,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniInteger`] from a [`i32`].
    ///
    /// This is equivalent to `MiniInteger::from(val)`, but can also be used in
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
    /// use rug::integer::{BorrowInteger, MiniInteger};
    /// use rug::Integer;
    ///
    /// const MEBI_MINI: MiniInteger = MiniInteger::const_from_i32(1i32 << 20);
    /// const MEBI_BORROW: BorrowInteger = MEBI_MINI.borrow();
    /// const MEBI: &Integer = BorrowInteger::const_deref(&MEBI_BORROW);
    /// assert_eq!(*MEBI, 1i32 << 20);
    /// ```
    #[inline]
    pub const fn const_from_i32(val: i32) -> Self {
        let (size, limbs) = from_i32(val);
        MiniInteger {
            inner: mpz_t {
                alloc: LIMBS_IN_SMALL as c_int,
                size,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniInteger`] from a [`i64`].
    ///
    /// This is equivalent to `MiniInteger::from(val)`, but can also be used in
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
    /// use rug::integer::{BorrowInteger, MiniInteger};
    /// use rug::Integer;
    ///
    /// const TEBI_MINI: MiniInteger = MiniInteger::const_from_i64(1i64 << 40);
    /// const TEBI_BORROW: BorrowInteger = TEBI_MINI.borrow();
    /// const TEBI: &Integer = BorrowInteger::const_deref(&TEBI_BORROW);
    /// assert_eq!(*TEBI, 1i64 << 40);
    /// ```
    #[inline]
    pub const fn const_from_i64(val: i64) -> Self {
        let (size, limbs) = from_i64(val);
        MiniInteger {
            inner: mpz_t {
                alloc: LIMBS_IN_SMALL as c_int,
                size,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniInteger`] from a [`i128`].
    ///
    /// This is equivalent to `MiniInteger::from(val)`, but can also be used in
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
    /// use rug::integer::{BorrowInteger, MiniInteger};
    /// use rug::Integer;
    ///
    /// const YOBI_MINI: MiniInteger = MiniInteger::const_from_i128(1i128 << 80);
    /// const YOBI_BORROW: BorrowInteger = YOBI_MINI.borrow();
    /// const YOBI: &Integer = BorrowInteger::const_deref(&YOBI_BORROW);
    /// assert_eq!(*YOBI, 1i128 << 80);
    /// ```
    #[inline]
    pub const fn const_from_i128(val: i128) -> Self {
        let (size, limbs) = from_i128(val);
        MiniInteger {
            inner: mpz_t {
                alloc: LIMBS_IN_SMALL as c_int,
                size,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniInteger`] from a [`isize`].
    ///
    /// This is equivalent to `MiniInteger::from(val)`, but can also be used in
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
    /// use rug::integer::{BorrowInteger, MiniInteger};
    /// use rug::Integer;
    ///
    /// const KIBI_MINI: MiniInteger = MiniInteger::const_from_isize(1isize << 10);
    /// const KIBI_BORROW: BorrowInteger = KIBI_MINI.borrow();
    /// const KIBI: &Integer = BorrowInteger::const_deref(&KIBI_BORROW);
    /// assert_eq!(*KIBI, 1isize << 10);
    /// ```
    #[inline]
    pub const fn const_from_isize(val: isize) -> Self {
        let (size, limbs) = from_isize(val);
        MiniInteger {
            inner: mpz_t {
                alloc: LIMBS_IN_SMALL as c_int,
                size,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniInteger`] from a [`u8`].
    ///
    /// This is equivalent to `MiniInteger::from(val)`, but can also be used in
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
    /// use rug::integer::{BorrowInteger, MiniInteger};
    /// use rug::Integer;
    ///
    /// const TWO_MINI: MiniInteger = MiniInteger::const_from_u8(2u8);
    /// const TWO_BORROW: BorrowInteger = TWO_MINI.borrow();
    /// const TWO: &Integer = BorrowInteger::const_deref(&TWO_BORROW);
    /// assert_eq!(*TWO, 2);
    /// ```
    #[inline]
    pub const fn const_from_u8(val: u8) -> Self {
        let (size, limbs) = from_u8(val);
        MiniInteger {
            inner: mpz_t {
                alloc: LIMBS_IN_SMALL as c_int,
                size,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniInteger`] from a [`u16`].
    ///
    /// This is equivalent to `MiniInteger::from(val)`, but can also be used in
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
    /// use rug::integer::{BorrowInteger, MiniInteger};
    /// use rug::Integer;
    ///
    /// const KIBI_MINI: MiniInteger = MiniInteger::const_from_u16(1u16 << 10);
    /// const KIBI_BORROW: BorrowInteger = KIBI_MINI.borrow();
    /// const KIBI: &Integer = BorrowInteger::const_deref(&KIBI_BORROW);
    /// assert_eq!(*KIBI, 1u16 << 10);
    /// ```
    #[inline]
    pub const fn const_from_u16(val: u16) -> Self {
        let (size, limbs) = from_u16(val);
        MiniInteger {
            inner: mpz_t {
                alloc: LIMBS_IN_SMALL as c_int,
                size,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniInteger`] from a [`u32`].
    ///
    /// This is equivalent to `MiniInteger::from(val)`, but can also be used in
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
    /// use rug::integer::{BorrowInteger, MiniInteger};
    /// use rug::Integer;
    ///
    /// const MEBI_MINI: MiniInteger = MiniInteger::const_from_u32(1u32 << 20);
    /// const MEBI_BORROW: BorrowInteger = MEBI_MINI.borrow();
    /// const MEBI: &Integer = BorrowInteger::const_deref(&MEBI_BORROW);
    /// assert_eq!(*MEBI, 1u32 << 20);
    /// ```
    #[inline]
    pub const fn const_from_u32(val: u32) -> Self {
        let (size, limbs) = from_u32(val);
        MiniInteger {
            inner: mpz_t {
                alloc: LIMBS_IN_SMALL as c_int,
                size,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniInteger`] from a [`u64`].
    ///
    /// This is equivalent to `MiniInteger::from(val)`, but can also be used in
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
    /// use rug::integer::{BorrowInteger, MiniInteger};
    /// use rug::Integer;
    ///
    /// const TEBI_MINI: MiniInteger = MiniInteger::const_from_u64(1u64 << 40);
    /// const TEBI_BORROW: BorrowInteger = TEBI_MINI.borrow();
    /// const TEBI: &Integer = BorrowInteger::const_deref(&TEBI_BORROW);
    /// assert_eq!(*TEBI, 1u64 << 40);
    /// ```
    #[inline]
    pub const fn const_from_u64(val: u64) -> Self {
        let (size, limbs) = from_u64(val);
        MiniInteger {
            inner: mpz_t {
                alloc: LIMBS_IN_SMALL as c_int,
                size,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniInteger`] from a [`u128`].
    ///
    /// This is equivalent to `MiniInteger::from(val)`, but can also be used in
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
    /// use rug::integer::{BorrowInteger, MiniInteger};
    /// use rug::Integer;
    ///
    /// const YOBI_MINI: MiniInteger = MiniInteger::const_from_u128(1u128 << 80);
    /// const YOBI_BORROW: BorrowInteger = YOBI_MINI.borrow();
    /// const YOBI: &Integer = BorrowInteger::const_deref(&YOBI_BORROW);
    /// assert_eq!(*YOBI, 1u128 << 80);
    /// ```
    #[inline]
    pub const fn const_from_u128(val: u128) -> Self {
        let (size, limbs) = from_u128(val);
        MiniInteger {
            inner: mpz_t {
                alloc: LIMBS_IN_SMALL as c_int,
                size,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }

    /// Creates a [`MiniInteger`] from a [`usize`].
    ///
    /// This is equivalent to `MiniInteger::from(val)`, but can also be used in
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
    /// use rug::integer::{BorrowInteger, MiniInteger};
    /// use rug::Integer;
    ///
    /// const KIBI_MINI: MiniInteger = MiniInteger::const_from_usize(1usize << 10);
    /// const KIBI_BORROW: BorrowInteger = KIBI_MINI.borrow();
    /// const KIBI: &Integer = BorrowInteger::const_deref(&KIBI_BORROW);
    /// assert_eq!(*KIBI, 1usize << 10);
    /// ```
    #[inline]
    pub const fn const_from_usize(val: usize) -> Self {
        let (size, limbs) = from_usize(val);
        MiniInteger {
            inner: mpz_t {
                alloc: LIMBS_IN_SMALL as c_int,
                size,
                d: NonNull::dangling(),
            },
            limbs,
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
    /// use rug::integer::MiniInteger;
    /// use rug::Assign;
    /// let mut i = MiniInteger::from(1u64);
    /// let capacity = i.borrow().capacity();
    /// // another u64 will not require a reallocation
    /// unsafe {
    ///     i.as_nonreallocating_integer().assign(2u64);
    /// }
    /// assert_eq!(*i.borrow(), 2);
    /// assert_eq!(i.borrow().capacity(), capacity);
    /// ```
    #[inline]
    pub unsafe fn as_nonreallocating_integer(&mut self) -> &mut Integer {
        // Update d to point to limbs.
        self.inner.d = NonNull::<[MaybeUninit<limb_t>]>::from(&self.limbs[..]).cast();
        let ptr = misc::cast_ptr_mut(&mut self.inner);
        // SAFETY: since inner.d points to the limbs, it is in a consistent state.
        unsafe { &mut *ptr }
    }

    /// Borrows the integer.
    ///
    /// The returned object implements
    /// <code>[Deref]\<[Target][Deref::Target] = [Integer]></code>.
    ///
    /// The borrow lasts until the returned object exits scope. Multiple borrows
    /// can be taken at the same time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::MiniInteger;
    /// use rug::Integer;
    /// let i = MiniInteger::from(-13i32);
    /// let b = i.borrow();
    /// let abs_ref = b.abs_ref();
    /// assert_eq!(Integer::from(abs_ref), 13);
    /// ```
    #[inline]
    pub const fn borrow(&self) -> BorrowInteger<'_> {
        // SAFETY: Since d points to the limbs, the mpz_t is in a consistent
        // state. Also, the lifetime of the BorrowInteger is the lifetime of
        // self, which covers the limbs.
        let d: *const Limbs = &self.limbs;
        unsafe {
            BorrowInteger::from_raw(mpz_t {
                alloc: self.inner.alloc,
                size: self.inner.size,
                d: NonNull::new_unchecked(d.cast_mut().cast()),
            })
        }
    }

    /// Borrows the integer exclusively.
    ///
    /// This is similar to the [`borrow`][Self::borrow] method, but it requires
    /// exclusive access to the underlying [`MiniInteger`]; the returned
    /// reference can however be shared. The exclusive access is required to
    /// reduce the amount of housekeeping necessary, providing a more efficient
    /// operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::MiniInteger;
    /// use rug::Integer;
    /// let mut i = MiniInteger::from(-13i32);
    /// let b = i.borrow_excl();
    /// let abs_ref = b.abs_ref();
    /// assert_eq!(Integer::from(abs_ref), 13);
    /// ```
    #[inline]
    pub fn borrow_excl(&mut self) -> &Integer {
        // SAFETY: since the return is a const reference, there will be no reallocation
        unsafe { &*self.as_nonreallocating_integer() }
    }
}

/// Types implementing this trait can be converted to [`MiniInteger`].
///
/// The following are implemented when `T` implements `ToMini`:
///   * <code>[Assign][`Assign`]\<T> for [MiniInteger][`MiniInteger`]</code>
///   * <code>[From][`From`]\<T> for [MiniInteger][`MiniInteger`]</code>
///
/// This trait is sealed and cannot be implemented for more types; it is
/// implemented for [`bool`] and for the integer types [`i8`], [`i16`], [`i32`],
/// [`i64`], [`i128`], [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`]
/// and [`usize`].
pub trait ToMini: SealedToMini {}

pub trait SealedToMini: Sized {
    fn copy(self, size: &mut c_int, limbs: &mut Limbs);
    fn is_zero(&self) -> bool;
}

macro_rules! is_zero {
    () => {
        #[inline]
        fn is_zero(&self) -> bool {
            *self == 0
        }
    };
}

macro_rules! signed {
    ($I:ty, $fn:ident, $fnu:ident) => {
        impl ToMini for $I {}

        impl SealedToMini for $I {
            #[inline]
            fn copy(self, size: &mut c_int, limbs: &mut Limbs) {
                let (neg, abs) = self.neg_abs();
                abs.copy(size, limbs);
                if neg {
                    *size = -*size;
                }
            }

            is_zero! {}
        }

        #[inline]
        const fn $fn(val: $I) -> (c_int, Limbs) {
            let unsigned_abs = val.unsigned_abs();
            let (size, limbs) = $fnu(unsigned_abs);
            let size = if val < 0 { -size } else { size };
            (size, limbs)
        }
    };
}

macro_rules! one_limb {
    ($U:ty, $fn:ident) => {
        impl ToMini for $U {}

        impl SealedToMini for $U {
            #[inline]
            fn copy(self, size: &mut c_int, limbs: &mut Limbs) {
                if self == 0 {
                    *size = 0;
                } else {
                    *size = 1;
                    limbs[0] = MaybeUninit::new(self.into());
                }
            }

            is_zero! {}
        }

        #[inline]
        const fn $fn(val: $U) -> (c_int, Limbs) {
            if val == 0 {
                (0, small_limbs![])
            } else {
                (1, small_limbs![val as limb_t])
            }
        }
    };
}

signed! { i8, from_i8, from_u8 }
signed! { i16, from_i16, from_u16 }
signed! { i32, from_i32, from_u32 }
signed! { i64, from_i64, from_u64 }
signed! { i128, from_i128, from_u128 }
signed! { isize, from_isize, from_usize }

impl ToMini for bool {}

impl SealedToMini for bool {
    #[inline]
    fn copy(self, size: &mut c_int, limbs: &mut Limbs) {
        if self {
            *size = 1;
            limbs[0] = MaybeUninit::new(1);
        } else {
            *size = 0;
        }
    }

    #[inline]
    fn is_zero(&self) -> bool {
        !*self
    }
}

#[inline]
const fn from_bool(val: bool) -> (c_int, Limbs) {
    if val {
        (1, small_limbs![1])
    } else {
        (0, small_limbs![])
    }
}

one_limb! { u8, from_u8 }
one_limb! { u16, from_u16 }
one_limb! { u32, from_u32 }

#[cfg(gmp_limb_bits_64)]
one_limb! { u64, from_u64 }

#[cfg(gmp_limb_bits_32)]
impl ToMini for u64 {}

#[cfg(gmp_limb_bits_32)]
impl SealedToMini for u64 {
    #[inline]
    fn copy(self, size: &mut c_int, limbs: &mut Limbs) {
        if self == 0 {
            *size = 0;
        } else if self <= 0xffff_ffff {
            *size = 1;
            limbs[0] = MaybeUninit::new(self.wrapping_cast());
        } else {
            *size = 2;
            limbs[0] = MaybeUninit::new(self.wrapping_cast());
            limbs[1] = MaybeUninit::new((self >> 32).wrapping_cast());
        }
    }

    is_zero! {}
}

#[cfg(gmp_limb_bits_32)]
#[inline]
const fn from_u64(val: u64) -> (c_int, Limbs) {
    if val == 0 {
        (0, small_limbs![])
    } else if val <= 0xffff_ffff {
        (1, small_limbs![val as limb_t])
    } else {
        (2, small_limbs![val as limb_t, (val >> 32) as limb_t])
    }
}

impl ToMini for u128 {}

impl SealedToMini for u128 {
    #[cfg(gmp_limb_bits_64)]
    #[inline]
    fn copy(self, size: &mut c_int, limbs: &mut Limbs) {
        if self == 0 {
            *size = 0;
        } else if self <= 0xffff_ffff_ffff_ffff {
            *size = 1;
            limbs[0] = MaybeUninit::new(self.wrapping_cast());
        } else {
            *size = 2;
            limbs[0] = MaybeUninit::new(self.wrapping_cast());
            limbs[1] = MaybeUninit::new((self >> 64).wrapping_cast());
        }
    }

    #[cfg(gmp_limb_bits_32)]
    #[inline]
    fn copy(self, size: &mut c_int, limbs: &mut Limbs) {
        if self == 0 {
            *size = 0;
        } else if self <= 0xffff_ffff {
            *size = 1;
            limbs[0] = MaybeUninit::new(self.wrapping_cast());
        } else if self <= 0xffff_ffff_ffff_ffff {
            *size = 2;
            limbs[0] = MaybeUninit::new(self.wrapping_cast());
            limbs[1] = MaybeUninit::new((self >> 32).wrapping_cast());
        } else if self <= 0xffff_ffff_ffff_ffff_ffff_ffff {
            *size = 3;
            limbs[0] = MaybeUninit::new(self.wrapping_cast());
            limbs[1] = MaybeUninit::new((self >> 32).wrapping_cast());
            limbs[2] = MaybeUninit::new((self >> 64).wrapping_cast());
        } else {
            *size = 4;
            limbs[0] = MaybeUninit::new(self.wrapping_cast());
            limbs[1] = MaybeUninit::new((self >> 32).wrapping_cast());
            limbs[2] = MaybeUninit::new((self >> 64).wrapping_cast());
            limbs[3] = MaybeUninit::new((self >> 96).wrapping_cast());
        }
    }

    is_zero! {}
}

#[cfg(gmp_limb_bits_64)]
#[inline]
const fn from_u128(val: u128) -> (c_int, Limbs) {
    if val == 0 {
        (0, small_limbs![])
    } else if val <= 0xffff_ffff_ffff_ffff {
        (1, small_limbs![val as limb_t])
    } else {
        (2, small_limbs![val as limb_t, (val >> 64) as limb_t])
    }
}

#[cfg(gmp_limb_bits_32)]
#[inline]
const fn from_u128(val: u128) -> (c_int, Limbs) {
    if val == 0 {
        (0, small_limbs![])
    } else if val <= 0xffff_ffff {
        (1, small_limbs![val as limb_t])
    } else if val <= 0xffff_ffff_ffff_ffff {
        (2, small_limbs![val as limb_t, (val >> 32) as limb_t])
    } else if val <= 0xffff_ffff_ffff_ffff_ffff_ffff {
        (
            3,
            small_limbs![val as limb_t, (val >> 32) as limb_t, (val >> 64) as limb_t],
        )
    } else {
        (
            4,
            small_limbs![
                val as limb_t,
                (val >> 32) as limb_t,
                (val >> 64) as limb_t,
                (val >> 96) as limb_t
            ],
        )
    }
}

impl ToMini for usize {}

impl SealedToMini for usize {
    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn copy(self, size: &mut c_int, limbs: &mut Limbs) {
        self.az::<u32>().copy(size, limbs);
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn copy(self, size: &mut c_int, limbs: &mut Limbs) {
        self.az::<u64>().copy(size, limbs);
    }

    is_zero! {}
}

#[cfg(target_pointer_width = "32")]
#[inline]
const fn from_usize(val: usize) -> (c_int, Limbs) {
    from_u32(val as u32)
}

#[cfg(target_pointer_width = "64")]
#[inline]
const fn from_usize(val: usize) -> (c_int, Limbs) {
    from_u64(val as u64)
}

impl<T: ToMini> Assign<T> for MiniInteger {
    #[inline]
    fn assign(&mut self, src: T) {
        src.copy(&mut self.inner.size, &mut self.limbs);
    }
}

impl<T: ToMini> From<T> for MiniInteger {
    #[inline]
    fn from(src: T) -> Self {
        let mut size = 0;
        let mut limbs = small_limbs![];
        src.copy(&mut size, &mut limbs);
        MiniInteger {
            inner: mpz_t {
                alloc: LIMBS_IN_SMALL.cast(),
                size,
                d: NonNull::dangling(),
            },
            limbs,
        }
    }
}

impl Assign<&Self> for MiniInteger {
    #[inline]
    fn assign(&mut self, other: &Self) {
        self.clone_from(other);
    }
}

impl Assign for MiniInteger {
    #[inline]
    fn assign(&mut self, other: Self) {
        *self = other;
    }
}

#[cfg(test)]
mod tests {
    use crate::integer::MiniInteger;
    use crate::{Assign, Integer};

    #[test]
    fn check_assign() {
        let mut i = MiniInteger::from(-1i32);
        assert_eq!(*i.borrow_excl(), -1);
        let other = MiniInteger::from(2i32);
        i.assign(&other);
        assert_eq!(*i.borrow_excl(), 2);
        i.assign(6u8);
        assert_eq!(*i.borrow_excl(), 6);
        i.assign(-6i8);
        assert_eq!(*i.borrow_excl(), -6);
        i.assign(other);
        assert_eq!(*i.borrow_excl(), 2);
        i.assign(6u16);
        assert_eq!(*i.borrow_excl(), 6);
        i.assign(-6i16);
        assert_eq!(*i.borrow_excl(), -6);
        i.assign(6u32);
        assert_eq!(*i.borrow_excl(), 6);
        i.assign(-6i32);
        assert_eq!(*i.borrow_excl(), -6);
        i.assign(0xf_0000_0006u64);
        assert_eq!(*i.borrow_excl(), 0xf_0000_0006u64);
        i.assign(-0xf_0000_0006i64);
        assert_eq!(*i.borrow_excl(), -0xf_0000_0006i64);
        i.assign((6u128 << 64) | 7u128);
        assert_eq!(*i.borrow_excl(), (6u128 << 64) | 7u128);
        i.assign((-6i128 << 64) | 7i128);
        assert_eq!(*i.borrow_excl(), (-6i128 << 64) | 7i128);
        i.assign(6usize);
        assert_eq!(*i.borrow_excl(), 6);
        i.assign(-6isize);
        assert_eq!(*i.borrow_excl(), -6);
        i.assign(0u32);
        assert_eq!(*i.borrow_excl(), 0);
    }

    #[test]
    fn check_traits() {
        assert!(MiniInteger::default().borrow_excl().is_zero());

        let mini = MiniInteger::from(-5);
        let check = Integer::from(-5);
        assert_eq!(format!("{mini}"), format!("{check}"));
        assert_eq!(format!("{mini:?}"), format!("{check:?}"));
        assert_eq!(format!("{mini:b}"), format!("{check:b}"));
        assert_eq!(format!("{mini:o}"), format!("{check:o}"));
        assert_eq!(format!("{mini:x}"), format!("{check:x}"));
        assert_eq!(format!("{mini:X}"), format!("{check:X}"));
    }

    macro_rules! compare_conv {
        ($T:ident, $fn:ident, [$($val:expr),+] $( as $U:ident)?) => {
            for &val in &[$($val),+] {
                let a = MiniInteger::from(val);
                let b = MiniInteger::$fn(val);
                let mut c = MiniInteger::new();
                c.assign(val);
                assert_eq!(*a.borrow(), val $(as $U)?);
                assert_eq!(*b.borrow(), val $(as $U)?);
                assert_eq!(*c.borrow(), val $(as $U)?);
            }
        };
    }

    #[test]
    fn check_equiv_convs() {
        compare_conv!(bool, const_from_bool, [false, true] as u8);
        compare_conv!(i8, const_from_i8, [i8::MIN, 0, i8::MAX]);
        compare_conv!(i16, const_from_i16, [i16::MIN, 0, i16::MAX]);
        compare_conv!(i32, const_from_i32, [i32::MIN, 0, i32::MAX]);
        compare_conv!(i64, const_from_i64, [i64::MIN, 0, i64::MAX]);
        compare_conv!(i128, const_from_i128, [i128::MIN, 0, i128::MAX]);
        compare_conv!(isize, const_from_isize, [isize::MIN, 0, isize::MAX]);
        compare_conv!(u8, const_from_u8, [0, u8::MAX]);
        compare_conv!(u16, const_from_u16, [0, u16::MAX]);
        compare_conv!(u32, const_from_u32, [0, u32::MAX]);
        compare_conv!(u64, const_from_u64, [0, u64::MAX]);
        compare_conv!(u128, const_from_u128, [0, u128::MAX]);
        compare_conv!(usize, const_from_usize, [0, usize::MAX]);
    }
}
