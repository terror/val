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

/*!
Random number generation.

This module provides two main structs.

 1. [`RandState`] is thread safe and can be shared and sent across threads.
 2. [`ThreadRandState`] is suitable for use in a single thread.

[`RandState`] has constructors for Mersenne Twister and for linear congruential
random number generators. It can also be constructed using a custom random
number generator implementing the [`RandGen`] trait, which has to implement
[`Send`] and [`Sync`] too.

If you need a custom random number generator that cannot be shared or sent
across threads, you can use [`ThreadRandState`] instead. [`ThreadRandState`] can
be constructed using a custom random number generator implementing the
[`ThreadRandGen`] trait, which does *not* have to implement [`Send`] or
[`Sync`].

Both [`RandState`] and [`ThreadRandState`] implement the [`MutRandState`] trait
so that they can be used with methods like
<code>[Integer]::[random\_below][Integer::random_below]</code>.
*/

use crate::Integer;
#[cfg(feature = "std")]
use crate::misc;
use az::Cast;
#[cfg(feature = "std")]
use az::{StrictAs, StrictCast};
use core::ffi::c_ulong;
#[cfg(feature = "std")]
use core::ffi::c_void;
use core::marker::PhantomData;
#[cfg(feature = "std")]
use core::mem::ManuallyDrop;
use core::mem::MaybeUninit;
use core::ptr;
#[cfg(feature = "std")]
use core::ptr::NonNull;
use gmp_mpfr_sys::gmp;
#[cfg(feature = "std")]
use gmp_mpfr_sys::gmp::randseed_t;
use gmp_mpfr_sys::gmp::{limb_t, mpz_t, randfnptr_t, randstate_t};

/**
The state of a random number generator.

# Examples

```rust
use rug::rand::RandState;
let mut rand = RandState::new();
let u = rand.bits(32);
println!("32 random bits: {:032b}", u);
```
*/
#[derive(Debug)]
#[repr(transparent)]
pub struct RandState<'a> {
    inner: randstate_t,
    phantom: PhantomData<&'a dyn RandGen>,
}

impl Default for RandState<'_> {
    #[inline]
    fn default() -> RandState<'static> {
        RandState::new()
    }
}

impl Clone for RandState<'_> {
    #[inline]
    fn clone(&self) -> RandState<'static> {
        unsafe {
            let mut inner = MaybeUninit::uninit();
            gmp::randinit_set(inner.as_mut_ptr(), self.as_raw());
            let inner = inner.assume_init();
            // If algdata is ABORT_FUNCS, then boxed_clone must have returned None.
            if ptr::eq(inner.algdata, &ABORT_FUNCS) {
                panic!("`RandGen::boxed_clone` returned `None`");
            }
            RandState {
                inner,
                phantom: PhantomData,
            }
        }
    }
}

impl Drop for RandState<'_> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gmp::randclear(self.as_raw_mut());
        }
    }
}

unsafe impl Send for RandState<'_> {}
unsafe impl Sync for RandState<'_> {}

impl RandState<'_> {
    /// Creates a new random generator with a compromise between speed and
    /// randomness.
    ///
    /// Currently this is equivalent to [`new_mersenne_twister`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::RandState;
    /// let mut rand = RandState::new();
    /// let u = rand.bits(32);
    /// println!("32 random bits: {:032b}", u);
    /// ```
    ///
    /// [`new_mersenne_twister`]: RandState::new_mersenne_twister
    #[inline]
    pub fn new() -> RandState<'static> {
        RandState::new_mersenne_twister()
    }

    /// Creates a random generator with a Mersenne Twister algorithm.
    ///
    /// This algorithm is fast and has good randomness properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::RandState;
    /// let mut rand = RandState::new_mersenne_twister();
    /// let u = rand.bits(32);
    /// println!("32 random bits: {:032b}", u);
    /// ```
    pub fn new_mersenne_twister() -> RandState<'static> {
        unsafe {
            let mut inner = MaybeUninit::uninit();
            gmp::randinit_mt(inner.as_mut_ptr());
            RandState {
                inner: inner.assume_init(),
                phantom: PhantomData,
            }
        }
    }

    /// Creates a new random generator with a linear congruential algorithm
    /// <i>X</i> = (<i>a</i> × <i>X</i> + <i>c</i>) mod 2<sup><i>m</i></sup>.
    ///
    /// The low bits of <i>X</i> in this algorithm are not very random, so only
    /// the high half of each <i>X</i> is actually used, that is the higher
    /// <i>m</i>/2 bits.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::RandState;
    /// use rug::Integer;
    /// let a = match Integer::from_str_radix("292787ebd3329ad7e7575e2fd", 16) {
    ///     Ok(i) => i,
    ///     Err(_) => unreachable!(),
    /// };
    /// let c = 1;
    /// let m = 100;
    /// let mut rand = RandState::new_linear_congruential(&a, c, m);
    /// let u = rand.bits(32);
    /// println!("32 random bits: {:032b}", u);
    /// ```
    pub fn new_linear_congruential(a: &Integer, c: u32, m: u32) -> RandState<'static> {
        unsafe {
            let mut inner = MaybeUninit::uninit();
            gmp::randinit_lc_2exp(inner.as_mut_ptr(), a.as_raw(), c.into(), m.into());
            RandState {
                inner: inner.assume_init(),
                phantom: PhantomData,
            }
        }
    }

    /// Creates a new random generator with a linear congruential algorithm like
    /// the [`new_linear_congruential`] method.
    ///
    /// For the linear congruential algorithm <i>X</i> = (<i>a</i> × <i>X</i> +
    /// <i>c</i>) mod 2<sup><i>m</i></sup>, <i>a</i>, <i>c</i> and <i>m</i> are
    /// selected from a table such that at least <i>size</i> bits of each
    /// <i>X</i> will be used, that is <i>m</i>/2&nbsp;≥&nbsp;<i>size</i>. The
    /// table only has values for <i>size</i>&nbsp;≤&nbsp;128; [`None`] will be
    /// returned if the requested size is larger.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::RandState;
    /// let mut rand = match RandState::new_linear_congruential_size(100) {
    ///     Some(r) => r,
    ///     None => unreachable!(),
    /// };
    /// let u = rand.bits(32);
    /// println!("32 random bits: {:032b}", u);
    /// ```
    ///
    /// [`new_linear_congruential`]: RandState::new_linear_congruential
    pub fn new_linear_congruential_size(size: u32) -> Option<RandState<'static>> {
        unsafe {
            let mut inner = MaybeUninit::uninit();
            if gmp::randinit_lc_2exp_size(inner.as_mut_ptr(), size.into()) == 0 {
                None
            } else {
                Some(RandState {
                    inner: inner.assume_init(),
                    phantom: PhantomData,
                })
            }
        }
    }

    /// Creates a new custom random generator.
    ///
    /// If the custom random generator is cloned, the implemented trait method
    /// <code>[RandGen]::[boxed\_clone]</code> is called; this leads to panic if
    /// the method returns [`None`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::{RandGen, RandState};
    /// use rug::Integer;
    /// struct Seed;
    /// impl RandGen for Seed {
    ///     fn r#gen(&mut self) -> u32 {
    ///         // not really random
    ///         0x8CEF_7310
    ///     }
    /// }
    /// let mut seed = Seed;
    /// let mut rand = RandState::new_custom(&mut seed);
    /// let mut i = Integer::from(15);
    /// i.random_below_mut(&mut rand);
    /// println!("0 ≤ {} < 15", i);
    /// assert!(i < 15);
    /// ```
    ///
    /// [boxed\_clone]: RandGen::boxed_clone
    #[cfg(feature = "std")]
    pub fn new_custom(custom: &mut dyn RandGen) -> RandState<'_> {
        let b = Box::<&mut dyn RandGen>::new(custom);
        let r_ptr = NonNull::<&mut dyn RandGen>::from(Box::leak(b));
        let inner = randstate_t {
            seed: randseed_t {
                alloc: MaybeUninit::uninit(),
                size: MaybeUninit::uninit(),
                d: r_ptr.cast::<c_void>(),
            },
            alg: MaybeUninit::uninit(),
            algdata: &CUSTOM_FUNCS,
        };
        RandState {
            inner,
            phantom: PhantomData,
        }
    }

    /// Creates a new custom random generator.
    ///
    /// If the custom random generator is cloned, the implemented trait method
    /// <code>[RandGen]::[boxed\_clone]</code> is called; this leads to panic if
    /// the method returns [`None`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::{RandGen, RandState};
    /// use rug::Integer;
    /// struct Seed;
    /// impl RandGen for Seed {
    ///     fn r#gen(&mut self) -> u32 {
    ///         // not really random
    ///         0x8CEF_7310
    ///     }
    /// }
    /// let seed = Box::new(Seed);
    /// let mut rand = RandState::new_custom_boxed(seed);
    /// let mut i = Integer::from(15);
    /// i.random_below_mut(&mut rand);
    /// println!("0 ≤ {} < 15", i);
    /// assert!(i < 15);
    /// ```
    ///
    /// [boxed\_clone]: RandGen::boxed_clone
    #[cfg(feature = "std")]
    pub fn new_custom_boxed(custom: Box<dyn RandGen>) -> RandState<'static> {
        let b = Box::<Box<dyn RandGen>>::new(custom);
        let r_ptr = NonNull::<Box<dyn RandGen>>::from(Box::leak(b));
        let inner = randstate_t {
            seed: randseed_t {
                alloc: MaybeUninit::uninit(),
                size: MaybeUninit::uninit(),
                d: r_ptr.cast::<c_void>(),
            },
            alg: MaybeUninit::uninit(),
            algdata: &CUSTOM_BOXED_FUNCS,
        };
        RandState {
            inner,
            phantom: PhantomData,
        }
    }

    /// Creates a random generator from an initialized [GMP random
    /// generator][randstate_t].
    ///
    /// # Safety
    ///
    ///   * The value must be initialized as a valid [`randstate_t`].
    ///   * The [`randstate_t`] type can be considered as a kind of pointer, so
    ///     there can be multiple copies of it. Since this function takes over
    ///     ownership, no other copies of the passed value should exist.
    ///   * The object must be thread safe.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use core::mem::MaybeUninit;
    /// use gmp_mpfr_sys::gmp;
    /// use rug::rand::RandState;
    /// let mut rand = unsafe {
    ///     let mut raw = MaybeUninit::uninit();
    ///     gmp::randinit_default(raw.as_mut_ptr());
    ///     let raw = raw.assume_init();
    ///     // raw is initialized and unique
    ///     RandState::from_raw(raw)
    /// };
    /// let u = rand.bits(32);
    /// println!("32 random bits: {:032b}", u);
    /// // since rand is a RandState now, deallocation is automatic
    /// ```
    #[inline]
    pub unsafe fn from_raw(raw: randstate_t) -> RandState<'static> {
        RandState {
            inner: raw,
            phantom: PhantomData,
        }
    }

    /// Converts a random generator into a [GMP random generator][randstate_t].
    ///
    /// The returned object should be freed to avoid memory leaks.
    ///
    /// # Panics
    ///
    /// This method panics if the [`RandState`] object was created using
    /// [`new_custom`], as the borrow into the custom generator would be
    /// terminated once `self` is consumed. This would lead to undefined
    /// behavior if the returned object is used. This method does work with
    /// objects created using [`new_custom_boxed`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gmp_mpfr_sys::gmp;
    /// use rug::rand::RandState;
    /// let rand = RandState::new();
    /// let mut raw = rand.into_raw();
    /// unsafe {
    ///     let u = gmp::urandomb_ui(&mut raw, 32) as u32;
    ///     println!("32 random bits: {:032b}", u);
    ///     // free object to prevent memory leak
    ///     gmp::randclear(&mut raw);
    /// }
    /// ```
    ///
    /// [`new_custom_boxed`]: RandState::new_custom_boxed
    /// [`new_custom`]: RandState::new_custom
    #[cfg(feature = "std")]
    #[inline]
    pub fn into_raw(self) -> randstate_t {
        assert!(
            !ptr::eq(self.inner.algdata, &CUSTOM_FUNCS)
                && !ptr::eq(self.inner.algdata, &THREAD_CUSTOM_FUNCS),
            "cannot convert custom `RandState` into raw, \
             consider using `new_custom_boxed` instead of `new_custom`"
        );
        let m = ManuallyDrop::new(self);
        m.inner
    }

    /// Returns a pointer to the inner [GMP random generator][randstate_t].
    ///
    /// The returned pointer will be valid for as long as `self` is valid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::RandState;
    /// let mut rand = RandState::new();
    /// let raw_ptr = rand.as_raw();
    /// // There is not much you can do with an immutable randstate_t pointer.
    /// println!("pointer: {:p}", raw_ptr);
    /// let u = rand.bits(32);
    /// println!("32 random bits: {:032b}", u);
    /// ```
    #[inline]
    pub fn as_raw(&self) -> *const randstate_t {
        &self.inner
    }

    /// Returns an unsafe mutable pointer to the inner [GMP random
    /// generator][randstate_t].
    ///
    /// The returned pointer will be valid for as long as `self` is valid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gmp_mpfr_sys::gmp;
    /// use rug::rand::RandState;
    /// let mut rand = RandState::new();
    /// let raw_ptr = rand.as_raw_mut();
    /// unsafe {
    ///     let u1 = gmp::urandomb_ui(raw_ptr, 32) as u32;
    ///     println!("32 random bits: {:032b}", u1);
    /// }
    /// let u2 = rand.bits(32);
    /// println!("another 32 random bits: {:032b}", u2);
    /// ```
    #[inline]
    pub fn as_raw_mut(&mut self) -> *mut randstate_t {
        &mut self.inner
    }

    /// Converts a random generator into <code>[Box]\<dyn [RandGen]></code> if
    /// possible.
    ///
    /// If the conversion is not possible, <code>[Err]\(self)</code> is
    /// returned.
    ///
    /// This conversion is always possible when the random generator was created
    /// with [`new_custom_boxed`]. It is also possible if the generator was
    /// cloned, directly or indirectly, from another generator that was created
    /// with [`new_custom`] or [`new_custom_boxed`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::{RandGen, RandState};
    /// struct Seed;
    /// impl RandGen for Seed {
    ///     fn r#gen(&mut self) -> u32 {
    ///         // not really random
    ///         0x8CEF_7310
    ///     }
    /// }
    /// let seed = Box::new(Seed);
    /// let rand = RandState::new_custom_boxed(seed);
    /// let mut back_to_seed = rand.into_custom_boxed().unwrap();
    /// assert_eq!(back_to_seed.r#gen(), 0x8CEF_7310);
    /// ```
    ///
    /// [`new_custom_boxed`]: RandState::new_custom_boxed
    /// [`new_custom`]: RandState::new_custom
    #[cfg(feature = "std")]
    #[inline]
    pub fn into_custom_boxed(self) -> Result<Box<dyn RandGen>, Self> {
        if !ptr::eq(self.inner.algdata, &CUSTOM_BOXED_FUNCS) {
            return Err(self);
        }
        let m = ManuallyDrop::new(self);
        let r_ptr = m.inner.seed.d.cast::<Box<dyn RandGen>>().as_ptr();
        let boxed_box: Box<Box<dyn RandGen>> = unsafe { Box::from_raw(r_ptr) };
        Ok(*boxed_box)
    }

    /// Seeds the random generator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::RandState;
    /// use rug::Integer;
    /// let seed = Integer::from(123456);
    /// let mut rand = RandState::new();
    /// rand.seed(&seed);
    /// let u1a = rand.bits(32);
    /// let u1b = rand.bits(32);
    /// // reseed with the same seed
    /// rand.seed(&seed);
    /// let u2a = rand.bits(32);
    /// let u2b = rand.bits(32);
    /// assert_eq!(u1a, u2a);
    /// assert_eq!(u1b, u2b);
    /// ```
    #[inline]
    pub fn seed(&mut self, seed: &Integer) {
        unsafe {
            gmp::randseed(self.as_raw_mut(), seed.as_raw());
        }
    }

    /// Generates a random number with the specified number of bits.
    ///
    /// # Panics
    ///
    /// Panics if `bits` is greater than 32.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::RandState;
    /// let mut rand = RandState::new();
    /// let u = rand.bits(16);
    /// assert!(u < (1 << 16));
    /// println!("16 random bits: {:016b}", u);
    /// ```
    #[inline]
    pub fn bits(&mut self, bits: u32) -> u32 {
        assert!(bits <= 32, "bits out of range");
        unsafe { gmp::urandomb_ui(self.as_raw_mut(), bits.into()) }.cast()
    }

    /// Generates a random number below the given boundary value.
    ///
    /// This function can never return the maximum 32-bit value; in order to
    /// generate a 32-bit random value that covers the whole range, use the
    /// [`bits`] method with `bits` set to 32.
    ///
    /// # Panics
    ///
    /// Panics if the boundary value is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::RandState;
    /// let mut rand = RandState::new();
    /// let u = rand.below(10000);
    /// assert!(u < 10000);
    /// println!("0 ≤ {} < 10000", u);
    /// ```
    ///
    /// [`bits`]: RandState::bits
    #[inline]
    pub fn below(&mut self, bound: u32) -> u32 {
        assert_ne!(bound, 0, "cannot be below zero");
        unsafe { gmp::urandomm_ui(self.as_raw_mut(), bound.into()) }.cast()
    }
}

/**
The state of a random number generator that is suitable for a single thread
only.

This is similar to [`RandState`] but can only be used in a single thread.

# Examples

```rust
use rug::rand::ThreadRandState;
# struct Gen { _dummy: *const i32, seed: u64 }
# impl rug::rand::ThreadRandGen for Gen {
#     fn r#gen(&mut self) -> u32 {
#         self.seed = self.seed.wrapping_mul(0x5851_F42D_4C95_7F2D).wrapping_add(1);
#         (self.seed >> 32) as u32
#     }
# }
# fn create_generator() -> Gen { Gen { _dummy: &0i32, seed: 1 } }
let mut gener = create_generator();
let mut rand = ThreadRandState::new_custom(&mut gener);
let u = rand.bits(32);
println!("32 random bits: {:032b}", u);
```
*/
#[derive(Debug)]
#[repr(transparent)]
pub struct ThreadRandState<'a> {
    inner: randstate_t,
    phantom: PhantomData<&'a dyn ThreadRandGen>,
}

impl Clone for ThreadRandState<'_> {
    #[inline]
    fn clone(&self) -> ThreadRandState<'static> {
        unsafe {
            let mut inner = MaybeUninit::uninit();
            gmp::randinit_set(inner.as_mut_ptr(), self.as_raw());
            let inner = inner.assume_init();
            // If algdata is ABORT_FUNCS, then boxed_clone must have returned None.
            if ptr::eq(inner.algdata, &ABORT_FUNCS) {
                panic!("`ThreadRandGen::boxed_clone` returned `None`");
            }
            ThreadRandState {
                inner,
                phantom: PhantomData,
            }
        }
    }
}

impl Drop for ThreadRandState<'_> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gmp::randclear(self.as_raw_mut());
        }
    }
}

impl ThreadRandState<'_> {
    /// Creates a new custom random generator.
    ///
    /// This is similar to
    /// <code>[RandState]::[new\_custom][RandState::new_custom]</code>. The
    /// difference is that this method takes a [`ThreadRandGen`] that does not
    /// have to implement [`Send`] or [`Sync`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::{ThreadRandGen, ThreadRandState};
    /// use rug::Integer;
    /// // dummy pointer field to ensure Seed is not Send and not Sync
    /// struct Seed { _unused_ptr: *const () }
    /// impl ThreadRandGen for Seed {
    ///     fn r#gen(&mut self) -> u32 {
    ///         // not really random
    ///         0x8CEF_7310
    ///     }
    /// }
    /// let mut seed = Seed { _unused_ptr: &() };
    /// let mut rand = ThreadRandState::new_custom(&mut seed);
    /// let mut i = Integer::from(15);
    /// i.random_below_mut(&mut rand);
    /// println!("0 ≤ {} < 15", i);
    /// assert!(i < 15);
    /// ```
    #[cfg(feature = "std")]
    pub fn new_custom(custom: &mut dyn ThreadRandGen) -> ThreadRandState<'_> {
        let b = Box::<&mut dyn ThreadRandGen>::new(custom);
        let r_ptr = NonNull::<&mut dyn ThreadRandGen>::from(Box::leak(b));
        let inner = randstate_t {
            seed: randseed_t {
                alloc: MaybeUninit::uninit(),
                size: MaybeUninit::uninit(),
                d: r_ptr.cast::<c_void>(),
            },
            alg: MaybeUninit::uninit(),
            algdata: &THREAD_CUSTOM_FUNCS,
        };
        ThreadRandState {
            inner,
            phantom: PhantomData,
        }
    }

    /// Creates a new custom random generator.
    ///
    /// This is similar to
    /// <code>[RandState][`RandState`]::[new\_custom\_boxed][RandState::new_custom_boxed]</code>.
    /// The difference is that this method takes a [`ThreadRandGen`] that does
    /// not have to implement [`Send`] or [`Sync`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::{ThreadRandGen, ThreadRandState};
    /// use rug::Integer;
    /// // dummy pointer field to ensure Seed is not Send and not Sync
    /// struct Seed { _unused_ptr: *const () }
    /// impl ThreadRandGen for Seed {
    ///     fn r#gen(&mut self) -> u32 {
    ///         // not really random
    ///         0x8CEF_7310
    ///     }
    /// }
    /// let seed = Box::new(Seed { _unused_ptr: &() });
    /// let mut rand = ThreadRandState::new_custom_boxed(seed);
    /// let mut i = Integer::from(15);
    /// i.random_below_mut(&mut rand);
    /// println!("0 ≤ {} < 15", i);
    /// assert!(i < 15);
    /// ```
    #[cfg(feature = "std")]
    pub fn new_custom_boxed(custom: Box<dyn ThreadRandGen>) -> ThreadRandState<'static> {
        let b = Box::<Box<dyn ThreadRandGen>>::new(custom);
        let r_ptr = NonNull::<Box<dyn ThreadRandGen>>::from(Box::leak(b));
        let inner = randstate_t {
            seed: randseed_t {
                alloc: MaybeUninit::uninit(),
                size: MaybeUninit::uninit(),
                d: r_ptr.cast::<c_void>(),
            },
            alg: MaybeUninit::uninit(),
            algdata: &THREAD_CUSTOM_BOXED_FUNCS,
        };
        ThreadRandState {
            inner,
            phantom: PhantomData,
        }
    }

    /// Creates a random generator from an initialized [GMP random
    /// generator][randstate_t].
    ///
    /// This is similar to
    /// <code>[RandState]::[from\_raw][RandState::from_raw]</code>, but the
    /// object does not need to be thread safe. You *can* use this method if the
    /// object is thread safe, but in that case
    /// <code>[RandState]::[from\_raw][RandState::from_raw]</code> is probably
    /// better as it allows the returned object to be shared and transferred
    /// across threads.
    ///
    /// # Safety
    ///
    ///   * The value must be initialized as a valid [`randstate_t`].
    ///   * The [`randstate_t`] type can be considered as a kind of pointer, so
    ///     there can be multiple copies of it. Since this function takes over
    ///     ownership, no other copies of the passed value should exist.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use core::mem::MaybeUninit;
    /// use gmp_mpfr_sys::gmp;
    /// use rug::rand::ThreadRandState;
    /// let mut rand = unsafe {
    ///     let mut raw = MaybeUninit::uninit();
    ///     gmp::randinit_default(raw.as_mut_ptr());
    ///     let raw = raw.assume_init();
    ///     // raw is initialized and unique
    ///     ThreadRandState::from_raw(raw)
    /// };
    /// let u = rand.bits(32);
    /// println!("32 random bits: {:032b}", u);
    /// // since rand is a ThreadRandState now, deallocation is automatic
    /// ```
    #[inline]
    pub unsafe fn from_raw(raw: randstate_t) -> ThreadRandState<'static> {
        ThreadRandState {
            inner: raw,
            phantom: PhantomData,
        }
    }

    /// Converts a random generator into a [GMP random generator][randstate_t].
    ///
    /// The returned object should be freed to avoid memory leaks.
    ///
    /// This is similar to
    /// <code>[RandState]::[into\_raw][RandState::into_raw]</code>, but the
    /// returned object is not thread safe. Notably, it should *not* be used in
    /// <code>[RandState]::[from\_raw][RandState::from_raw]</code>.
    ///
    /// # Panics
    ///
    /// This method panics if the [`ThreadRandState`] object was created using
    /// [`new_custom`], as the borrow into the custom generator would be
    /// terminated once `self` is consumed. This would lead to undefined
    /// behavior if the returned object is used. This method does work with
    /// objects created using [`new_custom_boxed`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gmp_mpfr_sys::gmp;
    /// use rug::rand::ThreadRandState;
    /// # struct Gen { _dummy: *const i32, seed: u64 }
    /// # impl rug::rand::ThreadRandGen for Gen {
    /// #     fn r#gen(&mut self) -> u32 {
    /// #         self.seed = self.seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
    /// #         (self.seed >> 32) as u32
    /// #     }
    /// # }
    /// # fn create_generator() -> Gen { Gen { _dummy: &0i32, seed: 1 } }
    /// let gener = Box::new(create_generator());
    /// let rand = ThreadRandState::new_custom_boxed(gener);
    /// let mut raw = rand.into_raw();
    /// unsafe {
    ///     let u = gmp::urandomb_ui(&mut raw, 32) as u32;
    ///     println!("32 random bits: {:032b}", u);
    ///     // free object to prevent memory leak
    ///     gmp::randclear(&mut raw);
    /// }
    /// ```
    ///
    /// [`new_custom_boxed`]: ThreadRandState::new_custom_boxed
    /// [`new_custom`]: ThreadRandState::new_custom
    #[cfg(feature = "std")]
    #[inline]
    pub fn into_raw(self) -> randstate_t {
        assert!(
            !ptr::eq(self.inner.algdata, &CUSTOM_FUNCS)
                && !ptr::eq(self.inner.algdata, &THREAD_CUSTOM_FUNCS),
            "cannot convert custom `ThreadRandState` into raw, \
             consider using `new_custom_boxed` instead of `new_custom`"
        );
        let m = ManuallyDrop::new(self);
        m.inner
    }

    /// Returns a pointer to the inner [GMP random generator][randstate_t].
    ///
    /// The returned pointer will be valid for as long as `self` is valid.
    ///
    /// This is similar to
    /// <code>[RandState]::[as\_raw][RandState::as_raw]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::ThreadRandState;
    /// # struct Gen { _dummy: *const i32, seed: u64 }
    /// # impl rug::rand::ThreadRandGen for Gen {
    /// #     fn r#gen(&mut self) -> u32 {
    /// #         self.seed = self.seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
    /// #         (self.seed >> 32) as u32
    /// #     }
    /// # }
    /// # fn create_generator() -> Gen { Gen { _dummy: &0i32, seed: 1 } }
    /// let mut gener = create_generator();
    /// let mut rand = ThreadRandState::new_custom(&mut gener);
    /// let raw_ptr = rand.as_raw();
    /// // There is not much you can do with an immutable randstate_t pointer.
    /// println!("pointer: {:p}", raw_ptr);
    /// let u = rand.bits(32);
    /// println!("32 random bits: {:032b}", u);
    /// ```
    #[inline]
    pub fn as_raw(&self) -> *const randstate_t {
        &self.inner
    }

    /// Returns an unsafe mutable pointer to the inner [GMP random
    /// generator][randstate_t].
    ///
    /// The returned pointer will be valid for as long as `self` is valid.
    ///
    /// This is similar to
    /// <code>[RandState]::[as\_raw\_mut][RandState::as_raw_mut]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gmp_mpfr_sys::gmp;
    /// use rug::rand::ThreadRandState;
    /// # struct Gen { _dummy: *const i32, seed: u64 }
    /// # impl rug::rand::ThreadRandGen for Gen {
    /// #     fn r#gen(&mut self) -> u32 {
    /// #         self.seed = self.seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
    /// #         (self.seed >> 32) as u32
    /// #     }
    /// # }
    /// # fn create_generator() -> Gen { Gen { _dummy: &0i32, seed: 1 } }
    /// let mut gener = create_generator();
    /// let mut rand = ThreadRandState::new_custom(&mut gener);
    /// let raw_ptr = rand.as_raw_mut();
    /// unsafe {
    ///     let u1 = gmp::urandomb_ui(raw_ptr, 32) as u32;
    ///     println!("32 random bits: {:032b}", u1);
    /// }
    /// let u2 = rand.bits(32);
    /// println!("another 32 random bits: {:032b}", u2);
    /// ```
    #[inline]
    pub fn as_raw_mut(&mut self) -> *mut randstate_t {
        &mut self.inner
    }

    /// Converts a random generator into <code>[Box]\<dyn [ThreadRandGen]></code> if possible.
    ///
    /// If the conversion is not possible, <code>[Err]\(self)</code> is
    /// returned.
    ///
    /// This conversion is always possible when the random generator was created
    /// with [`new_custom_boxed`]. It is also possible if the generator was
    /// cloned, directly or indirectly, from another generator that was created
    /// with [`new_custom`] or [`new_custom_boxed`].
    ///
    /// This is similar to
    /// <code>[RandState]::[into\_custom\_boxed][RandState::into_custom_boxed]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::{ThreadRandGen, ThreadRandState};
    /// struct Seed;
    /// impl ThreadRandGen for Seed {
    ///     fn r#gen(&mut self) -> u32 {
    ///         // not really random
    ///         0x8CEF_7310
    ///     }
    /// }
    /// let seed = Box::new(Seed);
    /// let rand = ThreadRandState::new_custom_boxed(seed);
    /// let mut back_to_seed = rand.into_custom_boxed().unwrap();
    /// assert_eq!(back_to_seed.r#gen(), 0x8CEF_7310);
    /// ```
    ///
    /// [`new_custom_boxed`]: ThreadRandState::new_custom_boxed
    /// [`new_custom`]: ThreadRandState::new_custom
    #[cfg(feature = "std")]
    #[inline]
    pub fn into_custom_boxed(self) -> Result<Box<dyn ThreadRandGen>, Self> {
        if !ptr::eq(self.inner.algdata, &THREAD_CUSTOM_BOXED_FUNCS) {
            return Err(self);
        }
        let m = ManuallyDrop::new(self);
        let r_ptr = m.inner.seed.d.cast::<Box<dyn ThreadRandGen>>().as_ptr();
        let boxed_box: Box<Box<dyn ThreadRandGen>> = unsafe { Box::from_raw(r_ptr) };
        Ok(*boxed_box)
    }

    /// Seeds the random generator.
    ///
    /// This is similar to <code>[RandState]::[seed][RandState::seed]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::ThreadRandState;
    /// use rug::Integer;
    /// # use az::WrappingCast;
    /// # struct Gen { _dummy: *const i32, seed: u64 }
    /// # impl rug::rand::ThreadRandGen for Gen {
    /// #     fn r#gen(&mut self) -> u32 {
    /// #         self.seed = self.seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
    /// #         (self.seed >> 32) as u32
    /// #     }
    /// #     fn seed(&mut self, seed: &Integer) { self.seed = seed.wrapping_cast() }
    /// # }
    /// # fn create_generator_with_seed() -> Gen { Gen { _dummy: &0i32, seed: 1 } }
    /// let mut gener = create_generator_with_seed();
    /// let seed = Integer::from(123456);
    /// let mut rand = ThreadRandState::new_custom(&mut gener);
    /// rand.seed(&seed);
    /// let u1a = rand.bits(32);
    /// let u1b = rand.bits(32);
    /// // reseed with the same seed
    /// rand.seed(&seed);
    /// let u2a = rand.bits(32);
    /// let u2b = rand.bits(32);
    /// assert_eq!(u1a, u2a);
    /// assert_eq!(u1b, u2b);
    /// ```
    #[inline]
    pub fn seed(&mut self, seed: &Integer) {
        unsafe {
            gmp::randseed(self.as_raw_mut(), seed.as_raw());
        }
    }

    /// Generates a random number with the specified number of bits.
    ///
    /// This is similar to <code>[RandState]::[bits][RandState::bits]</code>.
    ///
    /// # Panics
    ///
    /// Panics if `bits` is greater than 32.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::ThreadRandState;
    /// # struct Gen { _dummy: *const i32, seed: u64 }
    /// # impl rug::rand::ThreadRandGen for Gen {
    /// #     fn r#gen(&mut self) -> u32 {
    /// #         self.seed = self.seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
    /// #         (self.seed >> 32) as u32
    /// #     }
    /// # }
    /// # fn create_generator() -> Gen { Gen { _dummy: &0i32, seed: 1 } }
    /// let mut gener = create_generator();
    /// let mut rand = ThreadRandState::new_custom(&mut gener);
    /// let u = rand.bits(16);
    /// assert!(u < (1 << 16));
    /// println!("16 random bits: {:016b}", u);
    /// ```
    #[inline]
    pub fn bits(&mut self, bits: u32) -> u32 {
        assert!(bits <= 32, "bits out of range");
        unsafe { gmp::urandomb_ui(self.as_raw_mut(), bits.into()) }.cast()
    }

    /// Generates a random number below the given boundary value.
    ///
    /// This is similar to <code>[RandState]::[below][RandState::below]</code>.
    ///
    /// # Panics
    ///
    /// Panics if the boundary value is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::ThreadRandState;
    /// # struct Gen { _dummy: *const i32, seed: u64 }
    /// # impl rug::rand::ThreadRandGen for Gen {
    /// #     fn r#gen(&mut self) -> u32 {
    /// #         self.seed = self.seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
    /// #         (self.seed >> 32) as u32
    /// #     }
    /// # }
    /// # fn create_generator() -> Gen { Gen { _dummy: &0i32, seed: 1 } }
    /// let mut gener = create_generator();
    /// let mut rand = ThreadRandState::new_custom(&mut gener);
    /// let u = rand.below(10000);
    /// assert!(u < 10000);
    /// println!("0 ≤ {} < 10000", u);
    /// ```
    #[inline]
    pub fn below(&mut self, bound: u32) -> u32 {
        assert_ne!(bound, 0, "cannot be below zero");
        unsafe { gmp::urandomm_ui(self.as_raw_mut(), bound.into()) }.cast()
    }
}

/**
Custom random number generator to be used with [`RandState`].

The methods implemented for this trait, as well as possible destructors, can be
used by FFI callback functions. If these methods panic, they can cause the
program to abort.

# Examples

```rust
use rug::rand::{RandGen, RandState};
use rug::Integer;
struct SimpleGenerator {
    seed: u64,
}
impl RandGen for SimpleGenerator {
    fn r#gen(&mut self) -> u32 {
        // linear congruential algorithm with m = 64
        const A: u64 = 0x5851_F42D_4C95_7F2D;
        const C: u64 = 1;
        self.seed = self.seed.wrapping_mul(A).wrapping_add(C);
        (self.seed >> 32) as u32
    }
    fn seed(&mut self, seed: &Integer) {
        self.seed = seed.to_u64_wrapping();
    }
}
let mut gener = SimpleGenerator { seed: 1 };
let mut state = RandState::new_custom(&mut gener);
assert_eq!(state.bits(32), 0x5851_F42D);
assert_eq!(state.bits(32), 0xC0B1_8CCF);
```
*/
pub trait RandGen: Send + Sync {
    /// Gets a random 32-bit unsigned integer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::RandGen;
    /// struct SimpleGenerator {
    ///     seed: u64,
    /// }
    /// impl RandGen for SimpleGenerator {
    ///     fn r#gen(&mut self) -> u32 {
    ///         // linear congruential algorithm with m = 64
    ///         const A: u64 = 0x5851_F42D_4C95_7F2D;
    ///         const C: u64 = 1;
    ///         self.seed = self.seed.wrapping_mul(A).wrapping_add(C);
    ///         (self.seed >> 32) as u32
    ///     }
    /// }
    /// let mut rand = SimpleGenerator { seed: 1 };
    /// assert_eq!(rand.r#gen(), 0x5851_F42D);
    /// assert_eq!(rand.seed, 0x5851_F42D_4C95_7F2E);
    /// assert_eq!(rand.r#gen(), 0xC0B1_8CCF);
    /// assert_eq!(rand.seed, 0xC0B1_8CCF_4E25_2D17);
    /// ```
    fn r#gen(&mut self) -> u32;

    /// Gets up to 32 random bits.
    ///
    /// The default implementation simply calls the [`gen`][RandGen::gen] method
    /// once and returns the most significant required bits.
    ///
    /// This method can be overridden to store any unused bits for later use.
    /// This can be useful for example if the random number generation process
    /// is computationally expensive.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::RandGen;
    /// struct SimpleGenerator {
    ///     seed: u64,
    ///     buffer: u32,
    ///     len: u32,
    /// }
    /// impl RandGen for SimpleGenerator {
    ///     fn r#gen(&mut self) -> u32 {
    ///         // linear congruential algorithm with m = 64
    ///         const A: u64 = 0x5851_F42D_4C95_7F2D;
    ///         const C: u64 = 1;
    ///         self.seed = self.seed.wrapping_mul(A).wrapping_add(C);
    ///         (self.seed >> 32) as u32
    ///     }
    ///     fn gen_bits(&mut self, bits: u32) -> u32 {
    ///         let mut bits = match bits {
    ///             0 => return 0,
    ///             1..=31 => bits,
    ///             _ => return self.r#gen(),
    ///         };
    ///         let mut ret = 0;
    ///         if bits > self.len {
    ///             bits -= self.len;
    ///             ret |= self.buffer << bits;
    ///             self.buffer = self.r#gen();
    ///             self.len = 32;
    ///         }
    ///         self.len -= bits;
    ///         ret |= self.buffer >> self.len;
    ///         self.buffer &= !(!0 << self.len);
    ///         ret
    ///     }
    /// }
    /// let mut rand = SimpleGenerator {
    ///     seed: 1,
    ///     buffer: 0,
    ///     len: 0,
    /// };
    /// let (first_32, second_32) = (0x5851_F42D, 0xC0B1_8CCF);
    /// assert_eq!(rand.gen_bits(24), first_32 >> 8);
    /// assert_eq!(rand.gen_bits(24), ((first_32 & 0xFF) << 16) | (second_32 >> 16));
    /// assert_eq!(rand.gen_bits(16), second_32 & 0xFFFF);
    /// ```
    fn gen_bits(&mut self, bits: u32) -> u32 {
        let gener = self.r#gen();
        match bits {
            0 => 0,
            1..=31 => gener >> (32 - bits),
            _ => gener,
        }
    }

    /// Seeds the random number generator.
    ///
    /// The default implementation of this function does nothing.
    ///
    /// Note that the <code>[RandState]::[seed][RandState::seed]</code> method
    /// will pass its seed parameter exactly to this function without using it
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::{RandGen, RandState};
    /// use rug::{Assign, Integer};
    /// struct Seed {
    ///     inner: Integer,
    /// }
    /// impl RandGen for Seed {
    ///     fn r#gen(&mut self) -> u32 {
    ///         self.inner.to_u32_wrapping()
    ///     }
    ///     fn seed(&mut self, seed: &Integer) {
    ///         self.inner.assign(seed);
    ///     }
    /// }
    /// let mut seed = Seed {
    ///     inner: Integer::from(12),
    /// };
    /// let i = Integer::from(12345);
    /// {
    ///     let mut rand = RandState::new_custom(&mut seed);
    ///     rand.seed(&i);
    /// }
    /// assert_eq!(seed.inner, i);
    /// ```
    #[inline]
    fn seed(&mut self, seed: &Integer) {
        let _ = seed;
    }

    /// Optionally clones the random number generator.
    ///
    /// The default implementation returns [`None`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rand::RandGen;
    /// struct SimpleGenerator {
    ///     seed: u64,
    /// }
    /// impl RandGen for SimpleGenerator {
    ///     fn r#gen(&mut self) -> u32 {
    ///         // linear congruential algorithm with m = 64
    ///         const A: u64 = 0x5851_F42D_4C95_7F2D;
    ///         const C: u64 = 1;
    ///         self.seed = self.seed.wrapping_mul(A).wrapping_add(C);
    ///         (self.seed >> 32) as u32
    ///     }
    ///     fn boxed_clone(&self) -> Option<Box<dyn RandGen>> {
    ///         let other = SimpleGenerator { seed: self.seed };
    ///         let boxed = Box::new(other);
    ///         Some(boxed)
    ///     }
    /// }
    /// let mut rand = SimpleGenerator { seed: 1 };
    /// assert_eq!(rand.r#gen(), 0x5851_F42D);
    /// assert_eq!(rand.seed, 0x5851_F42D_4C95_7F2E);
    /// let mut other = rand.boxed_clone().unwrap();
    /// assert_eq!(rand.r#gen(), 0xC0B1_8CCF);
    /// assert_eq!(rand.seed, 0xC0B1_8CCF_4E25_2D17);
    /// assert_eq!(other.r#gen(), 0xC0B1_8CCF);
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    fn boxed_clone(&self) -> Option<Box<dyn RandGen>> {
        None
    }
}

/**
Custom random number generator to be used with [`ThreadRandState`].

The methods implemented for this trait, as well as possible destructors, can be
used by FFI callback functions. If these methods panic, they can cause the
program to abort.

This is similar to [`RandGen`] but can only be used in a single thread.

# Examples

```rust
# #[cfg(any())]
use rand::rngs::ThreadRng;
# #[cfg(any())]
use rand::thread_rng;
# #[cfg(any())]
use rand::RngCore;
# struct ThreadRng { _unused_ptr: *const i32, val: u32 }
# impl ThreadRng {
#     pub fn next_u32(&mut self) -> u32 { self.val = self.val.wrapping_add(1); self.val }
# }
# fn thread_rng() -> ThreadRng { ThreadRng { _unused_ptr: &0i32, val: !0 - 10 } }
use rug::rand::{ThreadRandGen, ThreadRandState};
struct Generator(ThreadRng);
impl ThreadRandGen for Generator {
    fn r#gen(&mut self) -> u32 {
        self.0.next_u32()
    }
}
let mut rng = Generator(thread_rng());
let mut state = ThreadRandState::new_custom(&mut rng);
let u = state.below(10000);
assert!(u < 10000);
println!("0 ≤ {} < 10000", u);
```

This would not compile, since `ThreadRng` is not [`Send`] and not [`Sync`].

```compile_fail
# #[cfg(any())]
use rand::rngs::ThreadRng;
# #[cfg(any())]
use rand::thread_rng;
# #[cfg(any())]
use rand::RngCore;
# struct ThreadRng(*const i32, u32);
# impl ThreadRng {
#     pub fn next_u32(&mut self) -> u32 { self.1 = self.1.wrapping_add(1); self.1 }
# }
# fn thread_rng() -> ThreadRng { ThreadRng(&0i32, !0 - 10) }
use rug::rand::RandGen;
struct Generator(ThreadRng);
impl RandGen for Generator {
    fn r#gen(&mut self) -> u32 {
        self.0.next_u32()
    }
}
```
*/
pub trait ThreadRandGen {
    /// Gets a random 32-bit unsigned integer.
    ///
    /// This is similar to <code>[RandGen]::[gen][RandGen::gen]</code>.
    fn r#gen(&mut self) -> u32;

    /// Gets up to 32 random bits.
    ///
    /// The default implementation simply calls the [`gen`][ThreadRandGen::gen]
    /// method once and returns the most significant required bits.
    ///
    /// This method can be overridden to store any unused bits for later use.
    /// This can be useful for example if the random number generation process
    /// is computationally expensive.
    ///
    /// This method is similar to
    /// <code>[RandGen]::[gen\_bits][RandGen::gen_bits]</code>.
    fn gen_bits(&mut self, bits: u32) -> u32 {
        let gener = self.r#gen();
        match bits {
            0 => 0,
            1..=32 => gener >> (32 - bits),
            _ => gener,
        }
    }

    /// Seeds the random number generator.
    ///
    /// The default implementation of this function does nothing.
    ///
    /// Note that the
    /// <code>[ThreadRandState]::[seed][ThreadRandState::seed]</code> method
    /// will pass its seed parameter exactly to this function without using it
    /// otherwise.
    ///
    /// This method is similar to <code>[RandGen]::[seed][RandGen::seed]</code>.
    #[inline]
    fn seed(&mut self, seed: &Integer) {
        let _ = seed;
    }

    /// Optionally clones the random number generator.
    ///
    /// The default implementation returns [`None`].
    ///
    /// This method is similar to
    /// <code>[RandGen]::[boxed\_clone][RandGen::boxed_clone]</code>.
    #[cfg(feature = "std")]
    #[inline]
    fn boxed_clone(&self) -> Option<Box<dyn ThreadRandGen>> {
        None
    }
}

static_assert_same_layout!(RandState<'_>, randstate_t);
static_assert_same_layout!(ThreadRandState<'_>, randstate_t);

static_assert_same_size!(RandState, Option<RandState>);
static_assert_same_size!(ThreadRandState, Option<ThreadRandState>);

// abort functions do not need a wrapper to abort on panic, they never panic and always abort
unsafe extern "C" fn abort_seed(_: *mut randstate_t, _: *const mpz_t) {
    unsafe {
        libc::abort();
    }
}

unsafe extern "C" fn abort_get(_: *mut randstate_t, _: *mut limb_t, _: c_ulong) {
    unsafe {
        libc::abort();
    }
}

unsafe extern "C" fn abort_clear(_: *mut randstate_t) {
    unsafe {
        libc::abort();
    }
}

unsafe extern "C" fn abort_iset(_: *mut randstate_t, _: *const randstate_t) {
    unsafe {
        libc::abort();
    }
}

#[cfg(feature = "std")]
unsafe extern "C" fn custom_seed(rstate: *mut randstate_t, seed: *const mpz_t) {
    let d = unsafe { (*rstate).seed.d };
    let r_ptr = d.cast::<&mut dyn RandGen>().as_ptr();
    let seed = misc::cast_ptr(seed);
    unsafe {
        (*r_ptr).seed(&*seed);
    }
}

#[cfg(feature = "std")]
unsafe extern "C" fn custom_get(rstate: *mut randstate_t, dest: *mut limb_t, nbits: c_ulong) {
    let d = unsafe { (*rstate).seed.d };
    let r_ptr = d.cast::<&mut dyn RandGen>().as_ptr();
    unsafe {
        gen_bits(*r_ptr, dest, nbits);
    }
}

#[cfg(feature = "std")]
unsafe extern "C" fn custom_clear(rstate: *mut randstate_t) {
    let d = unsafe { (*rstate).seed.d };
    let r_ptr = d.cast::<&mut dyn RandGen>().as_ptr();
    drop(unsafe { Box::from_raw(r_ptr) });
}

#[cfg(feature = "std")]
unsafe extern "C" fn custom_iset(dst: *mut randstate_t, src: *const randstate_t) {
    let d = unsafe { (*src).seed.d };
    let r_ptr = d.cast::<&mut dyn RandGen>().as_ptr();
    unsafe {
        gen_copy(*r_ptr, dst);
    }
}

#[cfg(feature = "std")]
unsafe extern "C" fn custom_boxed_seed(rstate: *mut randstate_t, seed: *const mpz_t) {
    let d = unsafe { (*rstate).seed.d };
    let r_ptr = d.cast::<Box<dyn RandGen>>().as_ptr();
    let seed = misc::cast_ptr(seed);
    unsafe {
        (*r_ptr).seed(&*seed);
    }
}

#[cfg(feature = "std")]
unsafe extern "C" fn custom_boxed_get(rstate: *mut randstate_t, dest: *mut limb_t, nbits: c_ulong) {
    let d = unsafe { (*rstate).seed.d };
    let r_ptr = d.cast::<Box<dyn RandGen>>().as_ptr();
    unsafe {
        gen_bits(&mut **r_ptr, dest, nbits);
    }
}

#[cfg(feature = "std")]
unsafe extern "C" fn custom_boxed_clear(rstate: *mut randstate_t) {
    let d = unsafe { (*rstate).seed.d };
    let r_ptr = d.cast::<Box<dyn RandGen>>().as_ptr();
    drop(unsafe { Box::from_raw(r_ptr) });
}

#[cfg(feature = "std")]
unsafe extern "C" fn custom_boxed_iset(dst: *mut randstate_t, src: *const randstate_t) {
    let d = unsafe { (*src).seed.d };
    let r_ptr = d.cast::<Box<dyn RandGen>>().as_ptr();
    unsafe {
        gen_copy(&**r_ptr, dst);
    }
}

#[cfg(feature = "std")]
unsafe extern "C" fn thread_custom_seed(rstate: *mut randstate_t, seed: *const mpz_t) {
    let d = unsafe { (*rstate).seed.d };
    let r_ptr = d.cast::<&mut dyn ThreadRandGen>().as_ptr();
    let seed = misc::cast_ptr(seed);
    unsafe {
        (*r_ptr).seed(&*seed);
    }
}

#[cfg(feature = "std")]
unsafe extern "C" fn thread_custom_get(
    rstate: *mut randstate_t,
    dest: *mut limb_t,
    nbits: c_ulong,
) {
    let d = unsafe { (*rstate).seed.d };
    let r_ptr = d.cast::<&mut dyn ThreadRandGen>().as_ptr();
    unsafe {
        thread_gen_bits(*r_ptr, dest, nbits);
    }
}

#[cfg(feature = "std")]
unsafe extern "C" fn thread_custom_clear(rstate: *mut randstate_t) {
    let d = unsafe { (*rstate).seed.d };
    let r_ptr = d.cast::<&mut dyn ThreadRandGen>().as_ptr();
    drop(unsafe { Box::from_raw(r_ptr) });
}

#[cfg(feature = "std")]
unsafe extern "C" fn thread_custom_iset(dst: *mut randstate_t, src: *const randstate_t) {
    let d = unsafe { (*src).seed.d };
    let r_ptr = d.cast::<&mut dyn ThreadRandGen>().as_ptr();
    unsafe {
        thread_gen_copy(*r_ptr, dst);
    }
}

#[cfg(feature = "std")]
unsafe extern "C" fn thread_custom_boxed_seed(rstate: *mut randstate_t, seed: *const mpz_t) {
    let d = unsafe { (*rstate).seed.d };
    let r_ptr = d.cast::<Box<dyn ThreadRandGen>>().as_ptr();
    let seed = misc::cast_ptr(seed);
    unsafe {
        (*r_ptr).seed(&*seed);
    }
}

#[cfg(feature = "std")]
unsafe extern "C" fn thread_custom_boxed_get(
    rstate: *mut randstate_t,
    dest: *mut limb_t,
    nbits: c_ulong,
) {
    let d = unsafe { (*rstate).seed.d };
    let r_ptr = d.cast::<Box<dyn ThreadRandGen>>().as_ptr();
    unsafe {
        thread_gen_bits(&mut **r_ptr, dest, nbits);
    }
}

#[cfg(feature = "std")]
unsafe extern "C" fn thread_custom_boxed_clear(rstate: *mut randstate_t) {
    let d = unsafe { (*rstate).seed.d };
    let r_ptr = d.cast::<Box<dyn ThreadRandGen>>().as_ptr();
    drop(unsafe { Box::from_raw(r_ptr) });
}

#[cfg(feature = "std")]
unsafe extern "C" fn thread_custom_boxed_iset(dst: *mut randstate_t, src: *const randstate_t) {
    let d = unsafe { (*src).seed.d };
    let r_ptr = d.cast::<Box<dyn ThreadRandGen>>().as_ptr();
    unsafe {
        thread_gen_copy(&**r_ptr, dst);
    }
}

#[cfg(feature = "std")]
#[cfg(gmp_limb_bits_64)]
unsafe fn gen_bits(gener: &mut dyn RandGen, dest: *mut limb_t, nbits: c_ulong) {
    let (limbs, rest) = (nbits / 64, nbits % 64);
    let limbs = limbs.strict_as::<isize>();
    for i in 0..limbs {
        let n = u64::from(gener.r#gen()) | (u64::from(gener.r#gen()) << 32);
        let n = n.strict_cast();
        unsafe {
            *dest.offset(i) = n;
        }
    }
    if rest >= 32 {
        let mut n = u64::from(gener.r#gen());
        if rest > 32 {
            let mask = !(!0 << (rest - 32));
            n |= u64::from(gener.gen_bits((rest - 32).strict_cast()) & mask) << 32;
        }
        let n = n.strict_cast();
        unsafe {
            *dest.offset(limbs) = n;
        }
    } else if rest > 0 {
        let mask = !(!0 << rest);
        let n = u64::from(gener.gen_bits(rest.strict_cast()) & mask);
        let n = n.strict_cast();
        unsafe {
            *dest.offset(limbs) = n;
        }
    }
}

#[cfg(feature = "std")]
#[cfg(gmp_limb_bits_32)]
unsafe fn gen_bits(gener: &mut dyn RandGen, dest: *mut limb_t, nbits: c_ulong) {
    let (limbs, rest) = (nbits / 32, nbits % 32);
    let limbs = limbs.strict_as::<isize>();
    for i in 0..limbs {
        let val = gener.r#gen().strict_cast();
        unsafe {
            *dest.offset(i) = val;
        }
    }
    if rest > 0 {
        let mask = !(!0 << rest);
        let val = (gener.gen_bits(rest.strict_cast()) & mask).strict_cast();
        unsafe {
            *dest.offset(limbs) = val;
        }
    }
}

#[cfg(feature = "std")]
unsafe fn gen_copy(gener: &dyn RandGen, dst: *mut randstate_t) {
    // Do not panic here if boxed_clone returns None, as panics cannot
    // cross FFI boundaries. Instead, set dst_ptr.seed.d to null.
    let (dst_r_ptr, funcs) = if let Some(other) = gener.boxed_clone() {
        let b = Box::<Box<dyn RandGen>>::new(other);
        let dst_r_ptr = NonNull::<Box<dyn RandGen>>::from(Box::leak(b));
        (dst_r_ptr, &CUSTOM_BOXED_FUNCS)
    } else {
        (NonNull::dangling(), &ABORT_FUNCS)
    };
    let dst = unsafe { &mut *dst };
    *dst = randstate_t {
        seed: randseed_t {
            alloc: MaybeUninit::uninit(),
            size: MaybeUninit::uninit(),
            d: dst_r_ptr.cast::<c_void>(),
        },
        alg: MaybeUninit::uninit(),
        algdata: funcs,
    };
}

#[cfg(feature = "std")]
#[cfg(gmp_limb_bits_64)]
unsafe fn thread_gen_bits(gener: &mut dyn ThreadRandGen, dest: *mut limb_t, nbits: c_ulong) {
    let (limbs, rest) = (nbits / 64, nbits % 64);
    let limbs = limbs.strict_as::<isize>();
    for i in 0..limbs {
        let n = u64::from(gener.r#gen()) | (u64::from(gener.r#gen()) << 32);
        let n = n.strict_cast();
        unsafe {
            *dest.offset(i) = n;
        }
    }
    if rest >= 32 {
        let mut n = u64::from(gener.r#gen());
        if rest > 32 {
            let mask = !(!0 << (rest - 32));
            n |= u64::from(gener.gen_bits((rest - 32).strict_cast()) & mask) << 32;
        }
        let n = n.strict_cast();
        unsafe {
            *dest.offset(limbs) = n;
        }
    } else if rest > 0 {
        let mask = !(!0 << rest);
        let n = u64::from(gener.gen_bits(rest.strict_cast()) & mask);
        let n = n.strict_cast();
        unsafe {
            *dest.offset(limbs) = n;
        }
    }
}

#[cfg(feature = "std")]
#[cfg(gmp_limb_bits_32)]
unsafe fn thread_gen_bits(gener: &mut dyn ThreadRandGen, dest: *mut limb_t, nbits: c_ulong) {
    let (limbs, rest) = (nbits / 32, nbits % 32);
    let limbs = limbs.strict_as::<isize>();
    for i in 0..limbs {
        let val = gener.r#gen().strict_cast();
        unsafe {
            *dest.offset(i) = val;
        }
    }
    if rest > 0 {
        let mask = !(!0 << rest);
        let val = (gener.gen_bits(rest.strict_cast()) & mask).strict_cast();
        unsafe {
            *dest.offset(limbs) = val;
        }
    }
}

#[cfg(feature = "std")]
unsafe fn thread_gen_copy(gener: &dyn ThreadRandGen, dst: *mut randstate_t) {
    // Do not panic here if boxed_clone returns None, as panics cannot
    // cross FFI boundaries. Instead, set dst_ptr.seed.d to null.
    let (dst_r_ptr, funcs) = if let Some(other) = gener.boxed_clone() {
        let b = Box::<Box<dyn ThreadRandGen>>::new(other);
        let dst_r_ptr = NonNull::<Box<dyn ThreadRandGen>>::from(Box::leak(b));
        (dst_r_ptr, &THREAD_CUSTOM_BOXED_FUNCS)
    } else {
        (NonNull::dangling(), &ABORT_FUNCS)
    };
    let dst = unsafe { &mut *dst };
    *dst = randstate_t {
        seed: randseed_t {
            alloc: MaybeUninit::uninit(),
            size: MaybeUninit::uninit(),
            d: dst_r_ptr.cast::<c_void>(),
        },
        alg: MaybeUninit::uninit(),
        algdata: funcs,
    };
}

static ABORT_FUNCS: randfnptr_t = randfnptr_t {
    seed: abort_seed,
    get: abort_get,
    clear: abort_clear,
    iset: abort_iset,
};

#[cfg(feature = "std")]
static CUSTOM_FUNCS: randfnptr_t = randfnptr_t {
    seed: custom_seed,
    get: custom_get,
    clear: custom_clear,
    iset: custom_iset,
};

#[cfg(feature = "std")]
static CUSTOM_BOXED_FUNCS: randfnptr_t = randfnptr_t {
    seed: custom_boxed_seed,
    get: custom_boxed_get,
    clear: custom_boxed_clear,
    iset: custom_boxed_iset,
};

#[cfg(feature = "std")]
static THREAD_CUSTOM_FUNCS: randfnptr_t = randfnptr_t {
    seed: thread_custom_seed,
    get: thread_custom_get,
    clear: thread_custom_clear,
    iset: thread_custom_iset,
};

#[cfg(feature = "std")]
static THREAD_CUSTOM_BOXED_FUNCS: randfnptr_t = randfnptr_t {
    seed: thread_custom_boxed_seed,
    get: thread_custom_boxed_get,
    clear: thread_custom_boxed_clear,
    iset: thread_custom_boxed_iset,
};

/// Used to pass the state of random number generators by mutable reference.
///
/// This trait is implemented by
///   1. [`RandState`], which is thread safe and implements [`Send`] and
///      [`Sync`].
///   2. [`ThreadRandState`], which can only be used in a single thread.
///
/// This trait is sealed and cannot be implemented for more types.
pub trait MutRandState: SealedMutRandState {}

mod hide {
    use gmp_mpfr_sys::gmp::randstate_t;
    #[repr(transparent)]
    pub struct Private<'a>(pub(crate) &'a mut randstate_t);
    pub trait SealedMutRandState {
        fn private(&mut self) -> Private<'_>;
    }
}
use self::hide::{Private, SealedMutRandState};

impl MutRandState for RandState<'_> {}
impl SealedMutRandState for RandState<'_> {
    #[inline]
    fn private(&mut self) -> Private<'_> {
        Private(&mut self.inner)
    }
}
impl MutRandState for ThreadRandState<'_> {}
impl SealedMutRandState for ThreadRandState<'_> {
    #[inline]
    fn private(&mut self) -> Private<'_> {
        Private(&mut self.inner)
    }
}

#[cfg(test)]
mod tests {
    use crate::rand::RandState;
    #[cfg(feature = "std")]
    use crate::rand::{RandGen, ThreadRandGen, ThreadRandState};
    #[cfg(feature = "std")]
    use az::{Az, Cast};
    #[cfg(feature = "std")]
    use core::ptr;
    #[cfg(feature = "std")]
    use gmp_mpfr_sys::gmp;

    #[cfg(feature = "std")]
    struct SimpleGenerator {
        seed: u64,
    }

    #[cfg(feature = "std")]
    impl RandGen for SimpleGenerator {
        fn r#gen(&mut self) -> u32 {
            self.seed = self
                .seed
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1);
            (self.seed >> 32).cast()
        }
        #[cfg(feature = "std")]
        fn boxed_clone(&self) -> Option<Box<dyn RandGen>> {
            let other = SimpleGenerator { seed: self.seed };
            let boxed = Box::new(other);
            Some(boxed)
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn check_custom_clone() {
        let mut gener = SimpleGenerator { seed: 1 };
        let third2;
        {
            let mut rand1 = RandState::new_custom(&mut gener);
            let mut rand2 = rand1.clone();
            let first1 = rand1.bits(32);
            let first2 = rand2.bits(32);
            assert_eq!(first1, first2);
            let second1 = rand1.bits(32);
            let second2 = rand2.bits(32);
            assert_eq!(second1, second2);
            assert_ne!(first1, second1);
            third2 = rand2.bits(32);
            assert_ne!(second2, third2);
        }
        let mut rand3 = RandState::new_custom_boxed(Box::new(gener));
        let mut rand4 = rand3.clone();
        let third3 = rand3.bits(32);
        let third4 = rand4.bits(32);
        assert_eq!(third2, third3);
        assert_eq!(third2, third4);
    }

    #[cfg(feature = "std")]
    struct NoCloneGenerator;

    #[cfg(feature = "std")]
    impl RandGen for NoCloneGenerator {
        fn r#gen(&mut self) -> u32 {
            0
        }
    }

    #[cfg(feature = "std")]
    #[test]
    #[should_panic(expected = "`RandGen::boxed_clone` returned `None`")]
    fn check_custom_no_clone() {
        let mut gener = NoCloneGenerator;
        let rand1 = RandState::new_custom(&mut gener);
        let _ = rand1.clone();
    }

    #[cfg(feature = "std")]
    #[test]
    #[should_panic(expected = "cannot convert custom `RandState` into raw")]
    fn check_custom_into_raw() {
        let mut gener = NoCloneGenerator;
        let rand1 = RandState::new_custom(&mut gener);
        let _ = rand1.into_raw();
    }

    #[cfg(feature = "std")]
    // include a dummy pointer to make !Send and !Sync
    struct ThreadSimpleGenerator {
        _dummy: *const i32,
        seed: u64,
    }

    #[cfg(feature = "std")]
    impl ThreadRandGen for ThreadSimpleGenerator {
        fn r#gen(&mut self) -> u32 {
            self.seed = self
                .seed
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1);
            (self.seed >> 32).cast()
        }
        #[cfg(feature = "std")]
        fn boxed_clone(&self) -> Option<Box<dyn ThreadRandGen>> {
            let other = ThreadSimpleGenerator {
                _dummy: ptr::null(),
                seed: self.seed,
            };
            let boxed = Box::new(other);
            Some(boxed)
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn thread_check_custom_clone() {
        let mut gener = ThreadSimpleGenerator {
            _dummy: ptr::null(),
            seed: 1,
        };
        let third2;
        {
            let mut rand1 = ThreadRandState::new_custom(&mut gener);
            let mut rand2 = rand1.clone();
            let first1 = rand1.bits(32);
            let first2 = rand2.bits(32);
            assert_eq!(first1, first2);
            let second1 = rand1.bits(32);
            let second2 = rand2.bits(32);
            assert_eq!(second1, second2);
            assert_ne!(first1, second1);
            third2 = rand2.bits(32);
            assert_ne!(second2, third2);
        }
        let mut rand3 = ThreadRandState::new_custom_boxed(Box::new(gener));
        let mut rand4 = rand3.clone();
        let third3 = rand3.bits(32);
        let third4 = rand4.bits(32);
        assert_eq!(third2, third3);
        assert_eq!(third2, third4);
    }

    #[cfg(feature = "std")]
    struct ThreadNoCloneGenerator;

    #[cfg(feature = "std")]
    impl ThreadRandGen for ThreadNoCloneGenerator {
        fn r#gen(&mut self) -> u32 {
            0
        }
    }

    #[cfg(feature = "std")]
    #[test]
    #[should_panic(expected = "`ThreadRandGen::boxed_clone` returned `None`")]
    fn thread_check_custom_no_clone() {
        let mut gener = ThreadNoCloneGenerator;
        let rand1 = ThreadRandState::new_custom(&mut gener);
        let _ = rand1.clone();
    }

    #[cfg(feature = "std")]
    #[test]
    #[should_panic(expected = "cannot convert custom `ThreadRandState` into raw")]
    fn thread_check_custom_into_raw() {
        let mut gener = ThreadNoCloneGenerator;
        let rand1 = ThreadRandState::new_custom(&mut gener);
        let _ = rand1.into_raw();
    }

    #[cfg(feature = "std")]
    #[test]
    fn thread_check_raw() {
        let mut check = RandState::new();
        // RandState is more restrictive than ThreadRandState; so this
        // conversion is sound.
        let mut state = unsafe { ThreadRandState::from_raw(check.clone().into_raw()) };
        assert_eq!(state.bits(32), check.bits(32));
        assert_eq!(
            unsafe { gmp::urandomb_ui(state.as_raw_mut(), 32) }.az::<u32>(),
            check.bits(32)
        );
        let mut raw = state.into_raw();
        assert_eq!(
            unsafe { gmp::urandomb_ui(&mut raw, 32) }.az::<u32>(),
            check.bits(32)
        );
        let mut state = unsafe { ThreadRandState::from_raw(raw) };
        assert_eq!(state.below(100), check.below(100));
    }

    #[test]
    fn congruential_size() {
        assert!(RandState::new_linear_congruential_size(128).is_some());
        assert!(RandState::new_linear_congruential_size(129).is_none());
    }
}
