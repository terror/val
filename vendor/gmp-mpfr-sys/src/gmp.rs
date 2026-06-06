// Copyright © 2017–2026 Trevor Spiteri

// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License and a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.

/*!
Function and type bindings for the [GMP] library.

# Examples

```rust
use core::mem::MaybeUninit;
use gmp_mpfr_sys::gmp;
unsafe {
    let mut z = {
        let mut z = MaybeUninit::uninit();
        gmp::mpz_init(z.as_mut_ptr());
        z.assume_init()
    };
    gmp::mpz_set_ui(&mut z, 15);
    let u = gmp::mpz_get_ui(&z);
    assert_eq!(u, 15);
    gmp::mpz_clear(&mut z);
}
```

[GMP]: https://gmplib.org/
*/
#![allow(non_camel_case_types, non_snake_case)]

use core::cmp::Ordering;
use core::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_void};
use core::fmt::{Debug, Formatter, Result as FmtResult};
use core::mem::MaybeUninit;
use core::ptr;
use core::ptr::NonNull;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use libc::FILE;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
type FILE = c_void;

include!(concat!(env!("OUT_DIR"), "/gmp_h.rs"));

extern "C" {
    /// See: [`mp_bits_per_limb`](../C/GMP/constant.GMP_Basics.html#index-mp_005fbits_005fper_005flimb)
    #[link_name = "__gmp_bits_per_limb"]
    pub static bits_per_limb: c_int;
}
/// See: [`__GNU_MP_VERSION`](../C/GMP/constant.GMP_Basics.html#index-_005f_005fGNU_005fMP_005fVERSION)
pub const VERSION: c_int = GMP_VERSION;
/// See: [`__GNU_MP_VERSION_MINOR`](../C/GMP/constant.GMP_Basics.html#index-_005f_005fGNU_005fMP_005fVERSION_005fMINOR)
pub const VERSION_MINOR: c_int = GMP_VERSION_MINOR;
/// See: [`__GNU_MP_VERSION_PATCHLEVEL`](../C/GMP/constant.GMP_Basics.html#index-_005f_005fGNU_005fMP_005fVERSION_005fPATCHLEVEL)
pub const VERSION_PATCHLEVEL: c_int = GMP_VERSION_PATCHLEVEL;
extern "C" {
    /// See: [`gmp_version`](../C/GMP/constant.GMP_Basics.html#index-gmp_005fversion)
    #[link_name = "__gmp_version"]
    pub static version: *const c_char;
}
/// See: [`__GMP_CC`](../C/GMP/constant.GMP_Basics.html#index-_005f_005fGMP_005fCC)
pub const CC: *const c_char = GMP_CC;
/// See: [`__GMP_CFLAGS`](../C/GMP/constant.GMP_Basics.html#index-_005f_005fGMP_005fCFLAGS)
pub const CFLAGS: *const c_char = GMP_CFLAGS;

/// See: [`GMP_NAIL_BITS`](../C/GMP/constant.Low_level_Functions.html#index-GMP_005fNAIL_005fBITS)
pub const NAIL_BITS: c_int = GMP_NAIL_BITS;
/// See: [`GMP_NUMB_BITS`](../C/GMP/constant.Low_level_Functions.html#index-GMP_005fNUMB_005fBITS)
pub const NUMB_BITS: c_int = LIMB_BITS - NAIL_BITS;
/// See: [`GMP_LIMB_BITS`](../C/GMP/constant.Low_level_Functions.html#index-GMP_005fLIMB_005fBITS)
pub const LIMB_BITS: c_int = GMP_LIMB_BITS;
/// See: [`GMP_NAIL_MASK`](../C/GMP/constant.Low_level_Functions.html#index-GMP_005fNAIL_005fMASK)
pub const NAIL_MASK: limb_t = !NUMB_MASK;
/// See: [`GMP_NUMB_MASK`](../C/GMP/constant.Low_level_Functions.html#index-GMP_005fNUMB_005fMASK)
pub const NUMB_MASK: limb_t = (!(0 as limb_t)) >> NAIL_BITS;
/// See: [`GMP_NUMB_MAX`](../C/GMP/constant.Low_level_Functions.html#index-GMP_005fNUMB_005fMAX)
pub const NUMB_MAX: limb_t = NUMB_MASK;

/// See: [`mp_exp_t`](../C/GMP/constant.GMP_Basics.html#index-mp_005fexp_005ft)
pub type exp_t = c_long;
/// See: [`mp_limb_t`](../C/GMP/constant.GMP_Basics.html#index-mp_005flimb_005ft)
pub type limb_t = GMP_LIMB_T;
/// See: [`mp_size_t`](../C/GMP/constant.GMP_Basics.html#index-mp_005fsize_005ft)
pub type size_t = c_long;
/// See: [`mp_bitcnt_t`](../C/GMP/constant.GMP_Basics.html#index-mp_005fbitcnt_005ft)
pub type bitcnt_t = c_ulong;

/// See: [`mpz_t`](../C/GMP/constant.GMP_Basics.html#index-mpz_005ft)
/// and [Integer Internals](../C/GMP/constant.Internals.html#Integer-Internals)
///
#[doc = include_str!("internal_fields.md")]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct mpz_t {
    /// See: [Integer Internals](../C/GMP/constant.Internals.html#Integer-Internals)
    pub alloc: c_int,
    /// See: [Integer Internals](../C/GMP/constant.Internals.html#Integer-Internals)
    pub size: c_int,
    /// See: [Integer Internals](../C/GMP/constant.Internals.html#Integer-Internals)
    pub d: NonNull<limb_t>,
}

/// See: [`mpq_t`](../C/GMP/constant.GMP_Basics.html#index-mpq_005ft)
/// and [Rational Internals](../C/GMP/constant.Internals.html#Rational-Internals)
///
#[doc = include_str!("internal_fields.md")]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct mpq_t {
    /// Internal implementation detail: numerator.
    pub num: mpz_t,
    /// Internal implementation detail: denominator.
    pub den: mpz_t,
}

/// See: [`mpf_t`](../C/GMP/constant.GMP_Basics.html#index-mpf_005ft)
/// and [Float Internals](../C/GMP/constant.Internals.html#Float-Internals)
///
#[doc = include_str!("internal_fields.md")]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct mpf_t {
    /// See: [Float Internals](../C/GMP/constant.Internals.html#Float-Internals)
    pub prec: c_int,
    /// See: [Float Internals](../C/GMP/constant.Internals.html#Float-Internals)
    pub size: c_int,
    /// See: [Float Internals](../C/GMP/constant.Internals.html#Float-Internals)
    pub exp: exp_t,
    /// See: [Float Internals](../C/GMP/constant.Internals.html#Float-Internals)
    pub d: NonNull<limb_t>,
}

/// See: [`gmp_randstate_t`](../C/GMP/constant.GMP_Basics.html#index-gmp_005frandstate_005ft)
///
#[doc = include_str!("internal_fields.md")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct randstate_t {
    /// Internal implementation detail: state of the generator.
    pub seed: randseed_t,
    /// Internal implementation detail: unused.
    pub alg: MaybeUninit<c_int>,
    /// Internal implementation detail: pointer to function pointers
    /// structure.
    pub algdata: *const randfnptr_t,
}

impl Debug for randstate_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("randstate_t")
            .field("seed", &self.seed)
            .field("alg", &"unused")
            .field("algdata", &self.algdata)
            .finish()
    }
}

/// The type for the [`seed`] field in the [`randstate_t`] struct.
///
/// [`seed`]: `randstate_t::seed`
///
#[doc = include_str!("internal_fields.md")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct randseed_t {
    /// Internal implementation detail: unused.
    pub alloc: MaybeUninit<c_int>,
    /// Internal implementation detail: unused.
    pub size: MaybeUninit<c_int>,
    /// Internal implementation detail: state of the generator.
    pub d: NonNull<c_void>,
}

impl Debug for randseed_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("randseed_t")
            .field("alloc", &"unused")
            .field("size", &"unused")
            .field("d", &self.d)
            .finish()
    }
}

/// The type for the [`algdata`] field in the [`randstate_t`] struct.
///
/// [`algdata`]: `randstate_t::algdata`
///
#[doc = include_str!("internal_fields.md")]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct randfnptr_t {
    /// Internal implementation detail: pointer to function.
    pub seed: unsafe extern "C" fn(rstate: *mut randstate_t, seed: *const mpz_t),
    /// Internal implementation detail: pointer to function.
    pub get: unsafe extern "C" fn(rstate: *mut randstate_t, dest: *mut limb_t, nbits: c_ulong),
    /// Internal implementation detail: pointer to function.
    pub clear: unsafe extern "C" fn(rstate: *mut randstate_t),
    /// Internal implementation detail: pointer to function.
    pub iset: unsafe extern "C" fn(dst: *mut randstate_t, src: *const randstate_t),
}

// Types for function declarations in this file.

/// See: [`mpz_ptr`](../C/GMP/constant.GMP_Basics.html#index-mpz_005fptr)
pub type mpz_ptr = *mut mpz_t;
/// See: [`mpz_srcptr`](../C/GMP/constant.GMP_Basics.html#index-mpz_005fsrcptr)
pub type mpz_srcptr = *const mpz_t;
/// See: [`mpq_ptr`](../C/GMP/constant.GMP_Basics.html#index-mpq_005fptr)
pub type mpq_ptr = *mut mpq_t;
/// See: [`mpq_srcptr`](../C/GMP/constant.GMP_Basics.html#index-mpq_005fsrcptr)
pub type mpq_srcptr = *const mpq_t;
/// See: [`mpf_ptr`](../C/GMP/constant.GMP_Basics.html#index-mpf_005fptr)
pub type mpf_ptr = *mut mpf_t;
/// See: [`mpf_srcptr`](../C/GMP/constant.GMP_Basics.html#index-mpf_005fsrcptr)
pub type mpf_srcptr = *const mpf_t;
/// See: [`gmp_randstate_ptr`](../C/GMP/constant.GMP_Basics.html#index-gmp_005frandstate_005fptr)
pub type randstate_ptr = *mut randstate_t;
/// See: [`gmp_randstate_srcptr`](../C/GMP/constant.GMP_Basics.html#index-gmp_005frandstate_005fsrcptr)
pub type randstate_srcptr = *const randstate_t;

type mp_ptr = *mut limb_t;
type mp_srcptr = *const limb_t;

// Integers

// Initialization Functions

extern "C" {
    /// See: [`mpz_init`](../C/GMP/constant.Integer_Functions.html#index-mpz_005finit)
    #[link_name = "__gmpz_init"]
    pub fn mpz_init(x: mpz_ptr);
    #[link_name = "__gmpz_inits"]
    /// See: [`mpz_inits`](../C/GMP/constant.Integer_Functions.html#index-mpz_005finits)
    pub fn mpz_inits(x: mpz_ptr, ...);
    #[link_name = "__gmpz_init2"]
    /// See: [`mpz_init2`](../C/GMP/constant.Integer_Functions.html#index-mpz_005finit2)
    pub fn mpz_init2(x: mpz_ptr, n: bitcnt_t);
    #[link_name = "__gmpz_clear"]
    /// See: [`mpz_clear`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fclear)
    pub fn mpz_clear(x: mpz_ptr);
    #[link_name = "__gmpz_clears"]
    /// See: [`mpz_clears`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fclears)
    pub fn mpz_clears(x: mpz_ptr, ...);
    #[link_name = "__gmpz_realloc2"]
    /// See: [`mpz_realloc2`](../C/GMP/constant.Integer_Functions.html#index-mpz_005frealloc2)
    pub fn mpz_realloc2(x: mpz_ptr, n: bitcnt_t);

    // Assignment Functions

    /// See: [`mpz_set`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fset)
    #[link_name = "__gmpz_set"]
    pub fn mpz_set(rop: mpz_ptr, op: mpz_srcptr);
    /// See: [`mpz_set_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fset_005fui)
    #[link_name = "__gmpz_set_ui"]
    pub fn mpz_set_ui(rop: mpz_ptr, op: c_ulong);
    /// See: [`mpz_set_si`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fset_005fsi)
    #[link_name = "__gmpz_set_si"]
    pub fn mpz_set_si(rop: mpz_ptr, op: c_long);
    /// See: [`mpz_set_d`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fset_005fd)
    #[link_name = "__gmpz_set_d"]
    pub fn mpz_set_d(rop: mpz_ptr, op: f64);
}
/// See: [`mpz_set_q`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fset_005fq)
#[inline]
pub unsafe extern "C" fn mpz_set_q(rop: mpz_ptr, op: mpq_srcptr) {
    unsafe { mpz_tdiv_q(rop, mpq_numref_const(op), mpq_denref_const(op)) }
}
extern "C" {
    /// See: [`mpz_set_f`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fset_005ff)
    #[link_name = "__gmpz_set_f"]
    pub fn mpz_set_f(rop: mpz_ptr, op: mpf_srcptr);
    /// See: [`mpz_set_str`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fset_005fstr)
    #[link_name = "__gmpz_set_str"]
    pub fn mpz_set_str(rop: mpz_ptr, str: *const c_char, base: c_int) -> c_int;
    /// See: [`mpz_swap`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fswap)
    #[link_name = "__gmpz_swap"]
    pub fn mpz_swap(rop1: mpz_ptr, rop2: mpz_ptr);

    // Combined Initialization and Assignment Functions

    /// See: [`mpz_init_set`](../C/GMP/constant.Integer_Functions.html#index-mpz_005finit_005fset)
    #[link_name = "__gmpz_init_set"]
    pub fn mpz_init_set(rop: mpz_ptr, op: mpz_srcptr);
    /// See: [`mpz_init_set_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005finit_005fset_005fui)
    #[link_name = "__gmpz_init_set_ui"]
    pub fn mpz_init_set_ui(rop: mpz_ptr, op: c_ulong);
    /// See: [`mpz_init_set_si`](../C/GMP/constant.Integer_Functions.html#index-mpz_005finit_005fset_005fsi)
    #[link_name = "__gmpz_init_set_si"]
    pub fn mpz_init_set_si(rop: mpz_ptr, op: c_long);
    /// See: [`mpz_init_set_d`](../C/GMP/constant.Integer_Functions.html#index-mpz_005finit_005fset_005fd)
    #[link_name = "__gmpz_init_set_d"]
    pub fn mpz_init_set_d(rop: mpz_ptr, op: f64);
    /// See: [`mpz_init_set_str`](../C/GMP/constant.Integer_Functions.html#index-mpz_005finit_005fset_005fstr)
    #[link_name = "__gmpz_init_set_str"]
    pub fn mpz_init_set_str(rop: mpz_ptr, str: *const c_char, base: c_int) -> c_int;
}

// Conversion Functions

/// See: [`mpz_get_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fget_005fui)
#[inline]
#[cfg(any(not(nails), long_long_limb))]
pub unsafe extern "C" fn mpz_get_ui(op: mpz_srcptr) -> c_ulong {
    if unsafe { (*op).size } != 0 {
        unsafe {
            let p = (*op).d.as_ptr();
            (*p) as c_ulong
        }
    } else {
        0
    }
}
/// See: [`mpz_get_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fget_005fui)
#[inline]
#[cfg(all(nails, not(long_long_limb)))]
pub unsafe extern "C" fn mpz_get_ui(op: mpz_srcptr) -> c_ulong {
    let p = unsafe { (*op).d }.as_ptr();
    let n = unsafe { (*op).size }.unsigned_abs();
    if n == 0 {
        0
    } else if n == 1 {
        unsafe { *p }
    } else {
        unsafe { *p + ((*(p.offset(1))) << NUMB_BITS) }
    }
}
extern "C" {
    /// See: [`mpz_get_si`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fget_005fsi)
    #[link_name = "__gmpz_get_si"]
    pub fn mpz_get_si(op: mpz_srcptr) -> c_long;
    /// See: [`mpz_get_d`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fget_005fd)
    #[link_name = "__gmpz_get_d"]
    pub fn mpz_get_d(op: mpz_srcptr) -> f64;
    /// See: [`mpz_get_d_2exp`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fget_005fd_005f2exp)
    #[link_name = "__gmpz_get_d_2exp"]
    pub fn mpz_get_d_2exp(exp: *mut c_long, op: mpz_srcptr) -> f64;
    /// See: [`mpz_get_str`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fget_005fstr)
    #[link_name = "__gmpz_get_str"]
    pub fn mpz_get_str(str: *mut c_char, base: c_int, op: mpz_srcptr) -> *mut c_char;

    // Arithmetic Functions

    /// See: [`mpz_add`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fadd)
    #[link_name = "__gmpz_add"]
    pub fn mpz_add(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    /// See: [`mpz_add_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fadd_005fui)
    #[link_name = "__gmpz_add_ui"]
    pub fn mpz_add_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    /// See: [`mpz_sub`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fsub)
    #[link_name = "__gmpz_sub"]
    pub fn mpz_sub(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    /// See: [`mpz_sub_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fsub_005fui)
    #[link_name = "__gmpz_sub_ui"]
    pub fn mpz_sub_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    /// See: [`mpz_ui_sub`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fui_005fsub)
    #[link_name = "__gmpz_ui_sub"]
    pub fn mpz_ui_sub(rop: mpz_ptr, op1: c_ulong, op2: mpz_srcptr);
    /// See: [`mpz_mul`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fmul)
    #[link_name = "__gmpz_mul"]
    pub fn mpz_mul(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    /// See: [`mpz_mul_si`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fmul_005fsi)
    #[link_name = "__gmpz_mul_si"]
    pub fn mpz_mul_si(rop: mpz_ptr, op1: mpz_srcptr, op2: c_long);
    /// See: [`mpz_mul_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fmul_005fui)
    #[link_name = "__gmpz_mul_ui"]
    pub fn mpz_mul_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    /// See: [`mpz_addmul`](../C/GMP/constant.Integer_Functions.html#index-mpz_005faddmul)
    #[link_name = "__gmpz_addmul"]
    pub fn mpz_addmul(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    /// See: [`mpz_addmul_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005faddmul_005fui)
    #[link_name = "__gmpz_addmul_ui"]
    pub fn mpz_addmul_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    /// See: [`mpz_submul`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fsubmul)
    #[link_name = "__gmpz_submul"]
    pub fn mpz_submul(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    /// See: [`mpz_submul_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fsubmul_005fui)
    #[link_name = "__gmpz_submul_ui"]
    pub fn mpz_submul_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    /// See: [`mpz_mul_2exp`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fmul_005f2exp)
    #[link_name = "__gmpz_mul_2exp"]
    pub fn mpz_mul_2exp(rop: mpz_ptr, op1: mpz_srcptr, op2: bitcnt_t);
}
/// See: [`mpz_neg`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fneg)
#[inline]
pub unsafe extern "C" fn mpz_neg(rop: mpz_ptr, op: mpz_srcptr) {
    if !ptr::eq(rop, op) {
        unsafe {
            mpz_set(rop, op);
        }
    }
    unsafe {
        (*rop).size = -(*rop).size;
    }
}
/// See: [`mpz_abs`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fabs)
#[inline]
pub unsafe extern "C" fn mpz_abs(rop: mpz_ptr, op: mpz_srcptr) {
    unsafe {
        if !ptr::eq(rop, op) {
            mpz_set(rop, op);
        }
        (*rop).size = (*rop).size.abs();
    }
}

// Division Functions

extern "C" {
    /// See: [`mpz_cdiv_q`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcdiv_005fq)
    #[link_name = "__gmpz_cdiv_q"]
    pub fn mpz_cdiv_q(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    /// See: [`mpz_cdiv_r`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcdiv_005fr)
    #[link_name = "__gmpz_cdiv_r"]
    pub fn mpz_cdiv_r(r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    /// See: [`mpz_cdiv_qr`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcdiv_005fqr)
    #[link_name = "__gmpz_cdiv_qr"]
    pub fn mpz_cdiv_qr(q: mpz_ptr, r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    /// See: [`mpz_cdiv_q_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcdiv_005fq_005fui)
    #[link_name = "__gmpz_cdiv_q_ui"]
    pub fn mpz_cdiv_q_ui(q: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;
    /// See: [`mpz_cdiv_r_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcdiv_005fr_005fui)
    #[link_name = "__gmpz_cdiv_r_ui"]
    pub fn mpz_cdiv_r_ui(r: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;
    /// See: [`mpz_cdiv_qr_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcdiv_005fqr_005fui)
    #[link_name = "__gmpz_cdiv_qr_ui"]
    pub fn mpz_cdiv_qr_ui(q: mpz_ptr, r: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;
    /// See: [`mpz_cdiv_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcdiv_005fui)
    #[link_name = "__gmpz_cdiv_ui"]
    pub fn mpz_cdiv_ui(n: mpz_srcptr, d: c_ulong) -> c_ulong;
    /// See: [`mpz_cdiv_q_2exp`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcdiv_005fq_005f2exp)
    #[link_name = "__gmpz_cdiv_q_2exp"]
    pub fn mpz_cdiv_q_2exp(q: mpz_ptr, n: mpz_srcptr, b: bitcnt_t);
    /// See: [`mpz_cdiv_r_2exp`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcdiv_005fr_005f2exp)
    #[link_name = "__gmpz_cdiv_r_2exp"]
    pub fn mpz_cdiv_r_2exp(r: mpz_ptr, n: mpz_srcptr, b: bitcnt_t);
    /// See: [`mpz_fdiv_q`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ffdiv_005fq)
    #[link_name = "__gmpz_fdiv_q"]
    pub fn mpz_fdiv_q(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    /// See: [`mpz_fdiv_r`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ffdiv_005fr)
    #[link_name = "__gmpz_fdiv_r"]
    pub fn mpz_fdiv_r(r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    /// See: [`mpz_fdiv_qr`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ffdiv_005fqr)
    #[link_name = "__gmpz_fdiv_qr"]
    pub fn mpz_fdiv_qr(q: mpz_ptr, r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    /// See: [`mpz_fdiv_q_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ffdiv_005fq_005fui)
    #[link_name = "__gmpz_fdiv_q_ui"]
    pub fn mpz_fdiv_q_ui(q: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;
    /// See: [`mpz_fdiv_r_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ffdiv_005fr_005fui)
    #[link_name = "__gmpz_fdiv_r_ui"]
    pub fn mpz_fdiv_r_ui(r: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;
    /// See: [`mpz_fdiv_qr_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ffdiv_005fqr_005fui)
    #[link_name = "__gmpz_fdiv_qr_ui"]
    pub fn mpz_fdiv_qr_ui(q: mpz_ptr, r: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;
    /// See: [`mpz_fdiv_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ffdiv_005fui)
    #[link_name = "__gmpz_fdiv_ui"]
    pub fn mpz_fdiv_ui(n: mpz_srcptr, d: c_ulong) -> c_ulong;
    /// See: [`mpz_fdiv_q_2exp`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ffdiv_005fq_005f2exp)
    #[link_name = "__gmpz_fdiv_q_2exp"]
    pub fn mpz_fdiv_q_2exp(q: mpz_ptr, n: mpz_srcptr, b: bitcnt_t);
    /// See: [`mpz_fdiv_r_2exp`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ffdiv_005fr_005f2exp)
    #[link_name = "__gmpz_fdiv_r_2exp"]
    pub fn mpz_fdiv_r_2exp(r: mpz_ptr, n: mpz_srcptr, b: bitcnt_t);
    /// See: [`mpz_tdiv_q`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ftdiv_005fq)
    #[link_name = "__gmpz_tdiv_q"]
    pub fn mpz_tdiv_q(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    /// See: [`mpz_tdiv_r`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ftdiv_005fr)
    #[link_name = "__gmpz_tdiv_r"]
    pub fn mpz_tdiv_r(r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    /// See: [`mpz_tdiv_qr`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ftdiv_005fqr)
    #[link_name = "__gmpz_tdiv_qr"]
    pub fn mpz_tdiv_qr(q: mpz_ptr, r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    /// See: [`mpz_tdiv_q_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ftdiv_005fq_005fui)
    #[link_name = "__gmpz_tdiv_q_ui"]
    pub fn mpz_tdiv_q_ui(q: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;
    /// See: [`mpz_tdiv_r_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ftdiv_005fr_005fui)
    #[link_name = "__gmpz_tdiv_r_ui"]
    pub fn mpz_tdiv_r_ui(r: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;
    /// See: [`mpz_tdiv_qr_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ftdiv_005fqr_005fui)
    #[link_name = "__gmpz_tdiv_qr_ui"]
    pub fn mpz_tdiv_qr_ui(q: mpz_ptr, r: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;
    /// See: [`mpz_tdiv_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ftdiv_005fui)
    #[link_name = "__gmpz_tdiv_ui"]
    pub fn mpz_tdiv_ui(n: mpz_srcptr, d: c_ulong) -> c_ulong;
    /// See: [`mpz_tdiv_q_2exp`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ftdiv_005fq_005f2exp)
    #[link_name = "__gmpz_tdiv_q_2exp"]
    pub fn mpz_tdiv_q_2exp(q: mpz_ptr, n: mpz_srcptr, b: bitcnt_t);
    /// See: [`mpz_tdiv_r_2exp`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ftdiv_005fr_005f2exp)
    #[link_name = "__gmpz_tdiv_r_2exp"]
    pub fn mpz_tdiv_r_2exp(r: mpz_ptr, n: mpz_srcptr, b: bitcnt_t);
    /// See: [`mpz_mod`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fmod)
    #[link_name = "__gmpz_mod"]
    pub fn mpz_mod(r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
}
/// See: [`mpz_mod_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fmod_005fui)
#[inline]
pub unsafe extern "C" fn mpz_mod_ui(r: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong {
    unsafe { mpz_fdiv_r_ui(r, n, d) }
}
extern "C" {
    /// See: [`mpz_divexact`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fdivexact)
    #[link_name = "__gmpz_divexact"]
    pub fn mpz_divexact(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    /// See: [`mpz_divexact_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fdivexact_005fui)
    #[link_name = "__gmpz_divexact_ui"]
    pub fn mpz_divexact_ui(q: mpz_ptr, n: mpz_srcptr, d: c_ulong);
    /// See: [`mpz_divisible_p`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fdivisible_005fp)
    #[link_name = "__gmpz_divisible_p"]
    pub fn mpz_divisible_p(n: mpz_srcptr, d: mpz_srcptr) -> c_int;
    /// See: [`mpz_divisible_ui_p`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fdivisible_005fui_005fp)
    #[link_name = "__gmpz_divisible_ui_p"]
    pub fn mpz_divisible_ui_p(n: mpz_srcptr, d: c_ulong) -> c_int;
    /// See: [`mpz_divisible_2exp_p`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fdivisible_005f2exp_005fp)
    #[link_name = "__gmpz_divisible_2exp_p"]
    pub fn mpz_divisible_2exp_p(n: mpz_srcptr, b: bitcnt_t) -> c_int;
    /// See: [`mpz_congruent_p`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcongruent_005fp)
    #[link_name = "__gmpz_congruent_p"]
    pub fn mpz_congruent_p(n: mpz_srcptr, c: mpz_srcptr, d: mpz_srcptr) -> c_int;
    /// See: [`mpz_congruent_ui_p`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcongruent_005fui_005fp)
    #[link_name = "__gmpz_congruent_ui_p"]
    pub fn mpz_congruent_ui_p(n: mpz_srcptr, c: c_ulong, d: c_ulong) -> c_int;
    /// See: [`mpz_congruent_2exp_p`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcongruent_005f2exp_005fp)
    #[link_name = "__gmpz_congruent_2exp_p"]
    pub fn mpz_congruent_2exp_p(n: mpz_srcptr, c: mpz_srcptr, b: bitcnt_t) -> c_int;

    // Exponentiation Functions

    /// See: [`mpz_powm`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fpowm)
    #[link_name = "__gmpz_powm"]
    pub fn mpz_powm(rop: mpz_ptr, base: mpz_srcptr, exp: mpz_srcptr, modu: mpz_srcptr);
    /// See: [`mpz_powm_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fpowm_005fui)
    #[link_name = "__gmpz_powm_ui"]
    pub fn mpz_powm_ui(rop: mpz_ptr, base: mpz_srcptr, exp: c_ulong, modu: mpz_srcptr);
    /// See: [`mpz_powm_sec`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fpowm_005fsec)
    #[link_name = "__gmpz_powm_sec"]
    pub fn mpz_powm_sec(rop: mpz_ptr, base: mpz_srcptr, exp: mpz_srcptr, modu: mpz_srcptr);
    /// See: [`mpz_pow_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fpow_005fui)
    #[link_name = "__gmpz_pow_ui"]
    pub fn mpz_pow_ui(rop: mpz_ptr, base: mpz_srcptr, exp: c_ulong);
    /// See: [`mpz_ui_pow_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fui_005fpow_005fui)
    #[link_name = "__gmpz_ui_pow_ui"]
    pub fn mpz_ui_pow_ui(rop: mpz_ptr, base: c_ulong, exp: c_ulong);

    // Root Extraction Functions

    /// See: [`mpz_root`](../C/GMP/constant.Integer_Functions.html#index-mpz_005froot)
    #[link_name = "__gmpz_root"]
    pub fn mpz_root(rop: mpz_ptr, op: mpz_srcptr, n: c_ulong) -> c_int;
    /// See: [`mpz_rootrem`](../C/GMP/constant.Integer_Functions.html#index-mpz_005frootrem)
    #[link_name = "__gmpz_rootrem"]
    pub fn mpz_rootrem(root: mpz_ptr, rem: mpz_ptr, op: mpz_srcptr, n: c_ulong);
    /// See: [`mpz_sqrt`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fsqrt)
    #[link_name = "__gmpz_sqrt"]
    pub fn mpz_sqrt(rop: mpz_ptr, op: mpz_srcptr);
    /// See: [`mpz_sqrtrem`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fsqrtrem)
    #[link_name = "__gmpz_sqrtrem"]
    pub fn mpz_sqrtrem(rop1: mpz_ptr, rop2: mpz_ptr, op: mpz_srcptr);
    /// See: [`mpz_perfect_power_p`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fperfect_005fpower_005fp)
    #[link_name = "__gmpz_perfect_power_p"]
    pub fn mpz_perfect_power_p(op: mpz_srcptr) -> c_int;
}
/// See: [`mpz_perfect_square_p`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fperfect_005fsquare_005fp)
#[inline]
pub unsafe extern "C" fn mpz_perfect_square_p(op: mpz_srcptr) -> c_int {
    let op_size = unsafe { (*op).size };
    if op_size > 0 {
        unsafe { mpn_perfect_square_p((*op).d.as_ptr(), op_size.into()) }
    } else {
        (op_size >= 0) as c_int
    }
}

// Number Theoretic Functions

extern "C" {
    /// See: [`mpz_probab_prime_p`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fprobab_005fprime_005fp)
    #[link_name = "__gmpz_probab_prime_p"]
    pub fn mpz_probab_prime_p(n: mpz_srcptr, reps: c_int) -> c_int;
    /// See: [`mpz_nextprime`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fnextprime)
    #[link_name = "__gmpz_nextprime"]
    pub fn mpz_nextprime(rop: mpz_ptr, op: mpz_srcptr);
    /// See: [`mpz_prevprime`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fprevprime)
    #[link_name = "__gmpz_prevprime"]
    pub fn mpz_prevprime(rop: mpz_ptr, op: mpz_srcptr);
    /// See: [`mpz_gcd`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fgcd)
    #[link_name = "__gmpz_gcd"]
    pub fn mpz_gcd(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    /// See: [`mpz_gcd_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fgcd_005fui)
    #[link_name = "__gmpz_gcd_ui"]
    pub fn mpz_gcd_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong) -> c_ulong;
    /// See: [`mpz_gcdext`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fgcdext)
    #[link_name = "__gmpz_gcdext"]
    pub fn mpz_gcdext(g: mpz_ptr, s: mpz_ptr, t: mpz_ptr, a: mpz_srcptr, b: mpz_srcptr);
    /// See: [`mpz_lcm`](../C/GMP/constant.Integer_Functions.html#index-mpz_005flcm)
    #[link_name = "__gmpz_lcm"]
    pub fn mpz_lcm(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    /// See: [`mpz_lcm_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005flcm_005fui)
    #[link_name = "__gmpz_lcm_ui"]
    pub fn mpz_lcm_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    /// See: [`mpz_invert`](../C/GMP/constant.Integer_Functions.html#index-mpz_005finvert)
    #[link_name = "__gmpz_invert"]
    pub fn mpz_invert(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr) -> c_int;
    /// See: [`mpz_jacobi`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fjacobi)
    #[link_name = "__gmpz_jacobi"]
    pub fn mpz_jacobi(a: mpz_srcptr, b: mpz_srcptr) -> c_int;
}
/// See: [`mpz_legendre`](../C/GMP/constant.Integer_Functions.html#index-mpz_005flegendre)
#[inline]
pub unsafe extern "C" fn mpz_legendre(a: mpz_srcptr, p: mpz_srcptr) -> c_int {
    unsafe { mpz_jacobi(a, p) }
}
/// See: [`mpz_kronecker`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fkronecker)
#[inline]
pub unsafe extern "C" fn mpz_kronecker(a: mpz_srcptr, b: mpz_srcptr) -> c_int {
    unsafe { mpz_jacobi(a, b) }
}
extern "C" {
    /// See: [`mpz_kronecker_si`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fkronecker_005fsi)
    #[link_name = "__gmpz_kronecker_si"]
    pub fn mpz_kronecker_si(a: mpz_srcptr, b: c_long) -> c_int;
    /// See: [`mpz_kronecker_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fkronecker_005fui)
    #[link_name = "__gmpz_kronecker_ui"]
    pub fn mpz_kronecker_ui(a: mpz_srcptr, b: c_ulong) -> c_int;
    /// See: [`mpz_si_kronecker`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fsi_005fkronecker)
    #[link_name = "__gmpz_si_kronecker"]
    pub fn mpz_si_kronecker(a: c_long, b: mpz_srcptr) -> c_int;
    /// See: [`mpz_ui_kronecker`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fui_005fkronecker)
    #[link_name = "__gmpz_ui_kronecker"]
    pub fn mpz_ui_kronecker(a: c_ulong, b: mpz_srcptr) -> c_int;
    /// See: [`mpz_remove`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fremove)
    #[link_name = "__gmpz_remove"]
    pub fn mpz_remove(rop: mpz_ptr, op: mpz_srcptr, f: mpz_srcptr) -> bitcnt_t;
    /// See: [`mpz_fac_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ffac_005fui)
    #[link_name = "__gmpz_fac_ui"]
    pub fn mpz_fac_ui(rop: mpz_ptr, n: c_ulong);
    /// See: [`mpz_2fac_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005f2fac_005fui)
    #[link_name = "__gmpz_2fac_ui"]
    pub fn mpz_2fac_ui(rop: mpz_ptr, n: c_ulong);
    /// See: [`mpz_mfac_uiui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fmfac_005fuiui)
    #[link_name = "__gmpz_mfac_uiui"]
    pub fn mpz_mfac_uiui(rop: mpz_ptr, n: c_ulong, m: c_ulong);
    /// See: [`mpz_primorial_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fprimorial_005fui)
    #[link_name = "__gmpz_primorial_ui"]
    pub fn mpz_primorial_ui(r: mpz_ptr, n: c_ulong);
    /// See: [`mpz_bin_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fbin_005fui)
    #[link_name = "__gmpz_bin_ui"]
    pub fn mpz_bin_ui(rop: mpz_ptr, n: mpz_srcptr, k: c_ulong);
    /// See: [`mpz_bin_uiui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fbin_005fuiui)
    #[link_name = "__gmpz_bin_uiui"]
    pub fn mpz_bin_uiui(rop: mpz_ptr, n: c_ulong, k: c_ulong);
    /// See: [`mpz_fib_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ffib_005fui)
    #[link_name = "__gmpz_fib_ui"]
    pub fn mpz_fib_ui(f_n: mpz_ptr, n: c_ulong);
    /// See: [`mpz_fib2_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ffib2_005fui)
    #[link_name = "__gmpz_fib2_ui"]
    pub fn mpz_fib2_ui(f_n: mpz_ptr, fnsub1: mpz_ptr, n: c_ulong);
    /// See: [`mpz_lucnum_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005flucnum_005fui)
    #[link_name = "__gmpz_lucnum_ui"]
    pub fn mpz_lucnum_ui(ln: mpz_ptr, n: c_ulong);
    /// See: [`mpz_lucnum2_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005flucnum2_005fui)
    #[link_name = "__gmpz_lucnum2_ui"]
    pub fn mpz_lucnum2_ui(ln: mpz_ptr, lnsub1: mpz_ptr, n: c_ulong);

    // Comparison Functions

    /// See: [`mpz_cmp`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcmp)
    #[link_name = "__gmpz_cmp"]
    pub fn mpz_cmp(op1: mpz_srcptr, op2: mpz_srcptr) -> c_int;
    /// See: [`mpz_cmp_d`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcmp_005fd)
    #[link_name = "__gmpz_cmp_d"]
    pub fn mpz_cmp_d(op1: mpz_srcptr, op2: f64) -> c_int;
    /// See: [`mpz_cmp_si`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcmp_005fsi)
    #[link_name = "__gmpz_cmp_si"]
    pub fn mpz_cmp_si(op1: mpz_srcptr, op2: c_long) -> c_int;
    /// See: [`mpz_cmp_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcmp_005fui)
    #[link_name = "__gmpz_cmp_ui"]
    pub fn mpz_cmp_ui(op1: mpz_srcptr, op2: c_ulong) -> c_int;
    /// See: [`mpz_cmpabs`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcmpabs)
    #[link_name = "__gmpz_cmpabs"]
    pub fn mpz_cmpabs(op1: mpz_srcptr, op2: mpz_srcptr) -> c_int;
    /// See: [`mpz_cmpabs_d`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcmpabs_005fd)
    #[link_name = "__gmpz_cmpabs_d"]
    pub fn mpz_cmpabs_d(op1: mpz_srcptr, op2: f64) -> c_int;
    /// See: [`mpz_cmpabs_ui`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcmpabs_005fui)
    #[link_name = "__gmpz_cmpabs_ui"]
    pub fn mpz_cmpabs_ui(op1: mpz_srcptr, op2: c_ulong) -> c_int;
}
/// See: [`mpz_sgn`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fsgn)
#[inline]
pub const unsafe extern "C" fn mpz_sgn(op: mpz_srcptr) -> c_int {
    let size = unsafe { (*op).size };
    if size < 0 {
        -1
    } else {
        (size > 0) as c_int
    }
}
extern "C" {
    /// See: [`mpz_and`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fand)
    #[link_name = "__gmpz_and"]
    pub fn mpz_and(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    /// See: [`mpz_ior`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fior)
    #[link_name = "__gmpz_ior"]
    pub fn mpz_ior(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    /// See: [`mpz_xor`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fxor)
    #[link_name = "__gmpz_xor"]
    pub fn mpz_xor(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    /// See: [`mpz_com`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcom)
    #[link_name = "__gmpz_com"]
    pub fn mpz_com(rop: mpz_ptr, op: mpz_srcptr);
}
/// See: [`mpz_popcount`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fpopcount)
#[inline]
pub unsafe extern "C" fn mpz_popcount(op: mpz_srcptr) -> bitcnt_t {
    let size = unsafe { (*op).size };
    match size.cmp(&0) {
        Ordering::Less => !0,
        Ordering::Equal => 0,
        Ordering::Greater => unsafe { mpn_popcount((*op).d.as_ptr(), size.into()) },
    }
}
extern "C" {
    /// See: [`mpz_hamdist`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fhamdist)
    #[link_name = "__gmpz_hamdist"]
    pub fn mpz_hamdist(op1: mpz_srcptr, op2: mpz_srcptr) -> bitcnt_t;
    /// See: [`mpz_scan0`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fscan0)
    #[link_name = "__gmpz_scan0"]
    pub fn mpz_scan0(op: mpz_srcptr, starting_bit: bitcnt_t) -> bitcnt_t;
    /// See: [`mpz_scan1`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fscan1)
    #[link_name = "__gmpz_scan1"]
    pub fn mpz_scan1(op: mpz_srcptr, starting_bit: bitcnt_t) -> bitcnt_t;
    /// See: [`mpz_setbit`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fsetbit)
    #[link_name = "__gmpz_setbit"]
    pub fn mpz_setbit(rop: mpz_ptr, bit_index: bitcnt_t);
    /// See: [`mpz_clrbit`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fclrbit)
    #[link_name = "__gmpz_clrbit"]
    pub fn mpz_clrbit(rop: mpz_ptr, bit_index: bitcnt_t);
    /// See: [`mpz_combit`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fcombit)
    #[link_name = "__gmpz_combit"]
    pub fn mpz_combit(rop: mpz_ptr, bit_index: bitcnt_t);
    /// See: [`mpz_tstbit`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ftstbit)
    #[link_name = "__gmpz_tstbit"]
    pub fn mpz_tstbit(op: mpz_srcptr, bit_index: bitcnt_t) -> c_int;

    // Input and Ouput Functions

    /// See: [`mpz_out_str`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fout_005fstr)
    #[link_name = "__gmpz_out_str"]
    pub fn mpz_out_str(stream: *mut FILE, base: c_int, op: mpz_srcptr) -> usize;
    /// See: [`mpz_inp_str`](../C/GMP/constant.Integer_Functions.html#index-mpz_005finp_005fstr)
    #[link_name = "__gmpz_inp_str"]
    pub fn mpz_inp_str(rop: mpz_ptr, stream: *mut FILE, base: c_int) -> usize;
    /// See: [`mpz_out_raw`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fout_005fraw)
    #[link_name = "__gmpz_out_raw"]
    pub fn mpz_out_raw(stream: *mut FILE, op: mpz_srcptr) -> usize;
    /// See: [`mpz_inp_raw`](../C/GMP/constant.Integer_Functions.html#index-mpz_005finp_005fraw)
    #[link_name = "__gmpz_inp_raw"]
    pub fn mpz_inp_raw(rop: mpz_ptr, stream: *mut FILE) -> usize;

    // Random Number Functions

    /// See: [`mpz_urandomb`](../C/GMP/constant.Integer_Functions.html#index-mpz_005furandomb)
    #[link_name = "__gmpz_urandomb"]
    pub fn mpz_urandomb(rop: mpz_ptr, state: randstate_ptr, n: bitcnt_t);
    /// See: [`mpz_urandomm`](../C/GMP/constant.Integer_Functions.html#index-mpz_005furandomm)
    #[link_name = "__gmpz_urandomm"]
    pub fn mpz_urandomm(rop: mpz_ptr, state: randstate_ptr, n: mpz_srcptr);
    /// See: [`mpz_rrandomb`](../C/GMP/constant.Integer_Functions.html#index-mpz_005frrandomb)
    #[link_name = "__gmpz_rrandomb"]
    pub fn mpz_rrandomb(rop: mpz_ptr, state: randstate_ptr, n: bitcnt_t);
    /// See: [`mpz_random2`](../C/GMP/constant.Integer_Functions.html#index-mpz_005frandom2)
    #[link_name = "__gmpz_random2"]
    pub fn mpz_random2(rop: mpz_ptr, max_size: size_t);

    // Integer Import and Export

    /// See: [`mpz_import`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fimport)
    #[link_name = "__gmpz_import"]
    pub fn mpz_import(
        rop: mpz_ptr,
        count: usize,
        order: c_int,
        size: usize,
        endian: c_int,
        nails: usize,
        op: *const c_void,
    );
    /// See: [`mpz_export`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fexport)
    #[link_name = "__gmpz_export"]
    pub fn mpz_export(
        rop: *mut c_void,
        countp: *mut usize,
        order: c_int,
        size: usize,
        endian: c_int,
        nails: usize,
        op: mpz_srcptr,
    ) -> *mut c_void;
}

// Miscellaneous Functions

macro_rules! mpz_fits {
    { $(#[$attr:meta])* fn $name:ident($max:expr); } => {
        #[cfg(not(nails))]
        $(#[$attr])*
        #[inline]
        pub const unsafe extern "C" fn $name(op: mpz_srcptr) -> c_int {
            let n = unsafe { (*op).size };
            let p = unsafe { (*op).d }.as_ptr().cast_const();
            let fits = n == 0 || (n == 1 && unsafe { *p } <= ($max as limb_t));
            fits as c_int
        }
        #[cfg(nails)]
        $(#[$attr])*
        #[inline]
        pub const unsafe extern "C" fn $name(op: mpz_srcptr) -> c_int {
            let n = unsafe { (*op).size };
            let p = unsafe { (*op).d }.as_ptr().cast_const();
            let fits = n == 0 || (n == 1 && unsafe { *p } <= ($max as limb_t))
                || (n == 2
                    && unsafe { *(p.offset(1)) } <= ($max as limb_t) >> NUMB_BITS);
            fits as c_int
        }
    }
}
mpz_fits! {
    /// See: [`mpz_fits_ulong_p`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ffits_005fulong_005fp)
    fn mpz_fits_ulong_p(c_ulong::MAX);
}
extern "C" {
    /// See: [`mpz_fits_slong_p`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ffits_005fslong_005fp)
    #[link_name = "__gmpz_fits_slong_p"]
    pub fn mpz_fits_slong_p(op: mpz_srcptr) -> c_int;
}
mpz_fits! {
    /// See: [`mpz_fits_uint_p`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ffits_005fuint_005fp)
    fn mpz_fits_uint_p(c_uint::MAX);
}
extern "C" {
    /// See: [`mpz_fits_sint_p`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ffits_005fsint_005fp)
    #[link_name = "__gmpz_fits_sint_p"]
    pub fn mpz_fits_sint_p(op: mpz_srcptr) -> c_int;
}
mpz_fits! {
    /// See: [`mpz_fits_ushort_p`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ffits_005fushort_005fp)
    fn mpz_fits_ushort_p(c_ushort::MAX);
}
extern "C" {
    /// See: [`mpz_fits_sshort_p`](../C/GMP/constant.Integer_Functions.html#index-mpz_005ffits_005fsshort_005fp)
    #[link_name = "__gmpz_fits_sshort_p"]
    pub fn mpz_fits_sshort_p(op: mpz_srcptr) -> c_int;
}
/// See: [`mpz_odd_p`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fodd_005fp)
#[inline]
pub const unsafe extern "C" fn mpz_odd_p(op: mpz_srcptr) -> c_int {
    if unsafe { (*op).size } == 0 {
        0
    } else {
        1 & unsafe { *((*op).d.as_ptr().cast_const()) } as c_int
    }
}
/// See: [`mpz_even_p`](../C/GMP/constant.Integer_Functions.html#index-mpz_005feven_005fp)
#[inline]
pub const unsafe extern "C" fn mpz_even_p(op: mpz_srcptr) -> c_int {
    (unsafe { mpz_odd_p(op) } == 0) as c_int
}
extern "C" {
    /// See: [`mpz_sizeinbase`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fsizeinbase)
    #[link_name = "__gmpz_sizeinbase"]
    pub fn mpz_sizeinbase(arg1: mpz_srcptr, arg2: c_int) -> usize;

    // Special Functions

    /// See: [`_mpz_realloc`](../C/GMP/constant.Integer_Functions.html#index-_005fmpz_005frealloc)
    #[link_name = "__gmpz_realloc"]
    pub fn _mpz_realloc(integer: mpz_ptr, new_alloc: size_t) -> *mut c_void;
}
/// See: [`mpz_getlimbn`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fgetlimbn)
#[inline]
pub const unsafe extern "C" fn mpz_getlimbn(op: mpz_srcptr, n: size_t) -> limb_t {
    if n >= 0 && (n as usize) < (unsafe { (*op).size }.unsigned_abs() as usize) {
        unsafe { *(*op).d.as_ptr().cast_const().offset(n as isize) }
    } else {
        0
    }
}
/// See: [`mpz_size`](../C/GMP/constant.Integer_Functions.html#index-mpz_005fsize)
#[inline]
pub const unsafe extern "C" fn mpz_size(op: mpz_srcptr) -> usize {
    unsafe { (*op).size }.unsigned_abs() as usize
}
extern "C" {
    /// See: [`mpz_limbs_read`](../C/GMP/constant.Integer_Functions.html#index-mpz_005flimbs_005fread)
    #[link_name = "__gmpz_limbs_read"]
    pub fn mpz_limbs_read(x: mpz_srcptr) -> mp_srcptr;
    /// See: [`mpz_limbs_write`](../C/GMP/constant.Integer_Functions.html#index-mpz_005flimbs_005fwrite)
    #[link_name = "__gmpz_limbs_write"]
    pub fn mpz_limbs_write(x: mpz_ptr, n: size_t) -> mp_ptr;
    /// See: [`mpz_limbs_modify`](../C/GMP/constant.Integer_Functions.html#index-mpz_005flimbs_005fmodify)
    #[link_name = "__gmpz_limbs_modify"]
    pub fn mpz_limbs_modify(x: mpz_ptr, n: size_t) -> mp_ptr;
    /// See: [`mpz_limbs_finish`](../C/GMP/constant.Integer_Functions.html#index-mpz_005flimbs_005ffinish)
    #[link_name = "__gmpz_limbs_finish"]
    pub fn mpz_limbs_finish(x: mpz_ptr, s: size_t);
    /// See: [`mpz_roinit_n`](../C/GMP/constant.Integer_Functions.html#index-mpz_005froinit_005fn)
    #[link_name = "__gmpz_roinit_n"]
    pub fn mpz_roinit_n(x: mpz_ptr, xp: mp_srcptr, xs: size_t) -> mpz_srcptr;
}
/// See: [`MPZ_ROINIT_N`](../C/GMP/constant.Integer_Functions.html#index-MPZ_005fROINIT_005fN)
#[inline]
pub const unsafe extern "C" fn MPZ_ROINIT_N(xp: mp_ptr, xs: size_t) -> mpz_t {
    mpz_t {
        alloc: 0,
        size: xs as c_int,
        d: unsafe { NonNull::new_unchecked(xp) },
    }
}

// Rational numbers

extern "C" {
    /// See: [`mpq_canonicalize`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fcanonicalize)
    #[link_name = "__gmpq_canonicalize"]
    pub fn mpq_canonicalize(op: mpq_ptr);

    // Initialization and Assignment Functions

    /// See: [`mpq_init`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005finit)
    #[link_name = "__gmpq_init"]
    pub fn mpq_init(x: mpq_ptr);
    /// See: [`mpq_inits`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005finits)
    #[link_name = "__gmpq_inits"]
    pub fn mpq_inits(x: mpq_ptr, ...);
    /// See: [`mpq_clear`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fclear)
    #[link_name = "__gmpq_clear"]
    pub fn mpq_clear(x: mpq_ptr);
    /// See: [`mpq_clears`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fclears)
    #[link_name = "__gmpq_clears"]
    pub fn mpq_clears(x: mpq_ptr, ...);
    /// See: [`mpq_set`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fset)
    #[link_name = "__gmpq_set"]
    pub fn mpq_set(rop: mpq_ptr, op: mpq_srcptr);
    /// See: [`mpq_set_z`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fset_005fz)
    #[link_name = "__gmpq_set_z"]
    pub fn mpq_set_z(rop: mpq_ptr, op: mpz_srcptr);
    /// See: [`mpq_set_ui`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fset_005fui)
    #[link_name = "__gmpq_set_ui"]
    pub fn mpq_set_ui(rop: mpq_ptr, op1: c_ulong, op2: c_ulong);
    /// See: [`mpq_set_si`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fset_005fsi)
    #[link_name = "__gmpq_set_si"]
    pub fn mpq_set_si(rop: mpq_ptr, op1: c_long, op2: c_ulong);
    /// See: [`mpq_set_str`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fset_005fstr)
    #[link_name = "__gmpq_set_str"]
    pub fn mpq_set_str(rop: mpq_ptr, str: *const c_char, base: c_int) -> c_int;
    /// See: [`mpq_swap`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fswap)
    #[link_name = "__gmpq_swap"]
    pub fn mpq_swap(rop1: mpq_ptr, rop2: mpq_ptr);

    // Conversion Functions

    /// See: [`mpq_get_d`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fget_005fd)
    #[link_name = "__gmpq_get_d"]
    pub fn mpq_get_d(op: mpq_srcptr) -> f64;
    /// See: [`mpq_set_d`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fset_005fd)
    #[link_name = "__gmpq_set_d"]
    pub fn mpq_set_d(rop: mpq_ptr, op: f64);
    /// See: [`mpq_set_f`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fset_005ff)
    #[link_name = "__gmpq_set_f"]
    pub fn mpq_set_f(rop: mpq_ptr, op: mpf_srcptr);
    /// See: [`mpq_get_str`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fget_005fstr)
    #[link_name = "__gmpq_get_str"]
    pub fn mpq_get_str(str: *mut c_char, base: c_int, op: mpq_srcptr) -> *mut c_char;

    // Arithmetic Functions

    /// See: [`mpq_add`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fadd)
    #[link_name = "__gmpq_add"]
    pub fn mpq_add(sum: mpq_ptr, addend1: mpq_srcptr, addend2: mpq_srcptr);
    /// See: [`mpq_sub`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fsub)
    #[link_name = "__gmpq_sub"]
    pub fn mpq_sub(difference: mpq_ptr, minuend: mpq_srcptr, subtrahend: mpq_srcptr);
    /// See: [`mpq_mul`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fmul)
    #[link_name = "__gmpq_mul"]
    pub fn mpq_mul(product: mpq_ptr, multiplier: mpq_srcptr, multiplicand: mpq_srcptr);
    /// See: [`mpq_mul_2exp`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fmul_005f2exp)
    #[link_name = "__gmpq_mul_2exp"]
    pub fn mpq_mul_2exp(rop: mpq_ptr, op1: mpq_srcptr, op2: bitcnt_t);
    /// See: [`mpq_div`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fdiv)
    #[link_name = "__gmpq_div"]
    pub fn mpq_div(quotient: mpq_ptr, dividend: mpq_srcptr, divisor: mpq_srcptr);
    /// See: [`mpq_div_2exp`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fdiv_005f2exp)
    #[link_name = "__gmpq_div_2exp"]
    pub fn mpq_div_2exp(rop: mpq_ptr, op1: mpq_srcptr, op2: bitcnt_t);
}
/// See: [`mpq_neg`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fneg)
#[inline]
pub unsafe extern "C" fn mpq_neg(negated_operand: mpq_ptr, operand: mpq_srcptr) {
    if !ptr::eq(negated_operand, operand) {
        unsafe { mpq_set(negated_operand, operand) };
    }
    unsafe {
        (*negated_operand).num.size = -(*negated_operand).num.size;
    }
}
/// See: [`mpq_abs`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fabs)
#[inline]
pub unsafe extern "C" fn mpq_abs(rop: mpq_ptr, op: mpq_srcptr) {
    if !ptr::eq(rop, op) {
        unsafe {
            mpq_set(rop, op);
        }
    }
    unsafe {
        (*rop).num.size = (*rop).num.size.abs();
    }
}
extern "C" {
    /// See: [`mpq_inv`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005finv)
    #[link_name = "__gmpq_inv"]
    pub fn mpq_inv(inverted_number: mpq_ptr, number: mpq_srcptr);

    // Comparison Functions

    /// See: [`mpq_cmp`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fcmp)
    #[link_name = "__gmpq_cmp"]
    pub fn mpq_cmp(op1: mpq_srcptr, op2: mpq_srcptr) -> c_int;
    /// See: [`mpq_cmp_z`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fcmp_005fz)
    #[link_name = "__gmpq_cmp_z"]
    pub fn mpq_cmp_z(op1: mpq_srcptr, op2: mpz_srcptr) -> c_int;
    /// See: [`mpq_cmp_ui`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fcmp_005fui)
    #[link_name = "__gmpq_cmp_ui"]
    pub fn mpq_cmp_ui(op1: mpq_srcptr, num2: c_ulong, den2: c_ulong) -> c_int;
    /// See: [`mpq_cmp_si`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fcmp_005fsi)
    #[link_name = "__gmpq_cmp_si"]
    pub fn mpq_cmp_si(op1: mpq_srcptr, num2: c_long, den2: c_ulong) -> c_int;
}
/// See: [`mpq_sgn`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fsgn)
#[inline]
pub const unsafe extern "C" fn mpq_sgn(op: mpq_srcptr) -> c_int {
    let size = unsafe { (*op).num.size };
    if size < 0 {
        -1
    } else {
        (size > 0) as c_int
    }
}
extern "C" {
    /// See: [`mpq_equal`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fequal)
    #[link_name = "__gmpq_equal"]
    pub fn mpq_equal(op1: mpq_srcptr, op2: mpq_srcptr) -> c_int;
}

// Applying Integer Functions to Rationals

/// See: [`mpq_numref`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fnumref)
#[inline]
pub const unsafe extern "C" fn mpq_numref(op: mpq_ptr) -> mpz_ptr {
    unsafe { mpq_numref_const(op) as mpz_ptr }
}
/// Constant version of [`mpq_numref`](fn.mpq_numref.html).
#[inline]
pub const unsafe extern "C" fn mpq_numref_const(op: mpq_srcptr) -> mpz_srcptr {
    unsafe { ptr::addr_of!((*op).num) }
}
/// See: [`mpq_denref`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fdenref)
#[inline]
pub const unsafe extern "C" fn mpq_denref(op: mpq_ptr) -> mpz_ptr {
    unsafe { mpq_denref_const(op) as mpz_ptr }
}
/// Constant version of [`mpq_denref`](fn.mpq_denref.html).
#[inline]
pub const unsafe extern "C" fn mpq_denref_const(op: mpq_srcptr) -> mpz_srcptr {
    unsafe { ptr::addr_of!((*op).den) }
}
extern "C" {
    /// See: [`mpq_get_num`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fget_005fnum)
    #[link_name = "__gmpq_get_num"]
    pub fn mpq_get_num(numerator: mpz_ptr, rational: mpq_srcptr);
    /// See: [`mpq_get_den`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fget_005fden)
    #[link_name = "__gmpq_get_den"]
    pub fn mpq_get_den(denominator: mpz_ptr, rational: mpq_srcptr);
    /// See: [`mpq_set_num`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fset_005fnum)
    #[link_name = "__gmpq_set_num"]
    pub fn mpq_set_num(rational: mpq_ptr, denominator: mpz_srcptr);
    /// See: [`mpq_set_den`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fset_005fden)
    #[link_name = "__gmpq_set_den"]
    pub fn mpq_set_den(rational: mpq_ptr, numerator: mpz_srcptr);

    // Input and Output Functions

    /// See: [`mpq_out_str`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fout_005fstr)
    #[link_name = "__gmpq_out_str"]
    pub fn mpq_out_str(stream: *mut FILE, base: c_int, op: mpq_srcptr) -> usize;
    /// See: [`mpq_inp_str`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005finp_005fstr)
    #[link_name = "__gmpq_inp_str"]
    pub fn mpq_inp_str(rop: mpq_ptr, stream: *mut FILE, base: c_int) -> usize;
}

// Floating-point numbers

// Initialization Functions

extern "C" {
    /// See: [`mpf_set_default_prec`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fset_005fdefault_005fprec)
    #[link_name = "__gmpf_set_default_prec"]
    pub fn mpf_set_default_prec(prec: bitcnt_t);
    /// See: [`mpf_get_default_prec`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fget_005fdefault_005fprec)
    #[link_name = "__gmpf_get_default_prec"]
    pub fn mpf_get_default_prec() -> bitcnt_t;
    /// See: [`mpf_init`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005finit)
    #[link_name = "__gmpf_init"]
    pub fn mpf_init(x: mpf_ptr);
    /// See: [`mpf_init2`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005finit2)
    #[link_name = "__gmpf_init2"]
    pub fn mpf_init2(x: mpf_ptr, prec: bitcnt_t);
    /// See: [`mpf_inits`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005finits)
    #[link_name = "__gmpf_inits"]
    pub fn mpf_inits(x: mpf_ptr, ...);
    /// See: [`mpf_clear`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fclear)
    #[link_name = "__gmpf_clear"]
    pub fn mpf_clear(x: mpf_ptr);
    /// See: [`mpf_clears`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fclears)
    #[link_name = "__gmpf_clears"]
    pub fn mpf_clears(x: mpf_ptr, ...);
    /// See: [`mpf_get_prec`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fget_005fprec)
    #[link_name = "__gmpf_get_prec"]
    pub fn mpf_get_prec(op: mpf_srcptr) -> bitcnt_t;
    /// See: [`mpf_set_prec`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fset_005fprec)
    #[link_name = "__gmpf_set_prec"]
    pub fn mpf_set_prec(rop: mpf_ptr, prec: bitcnt_t);
    /// See: [`mpf_set_prec_raw`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fset_005fprec_005fraw)
    #[link_name = "__gmpf_set_prec_raw"]
    pub fn mpf_set_prec_raw(rop: mpf_ptr, prec: bitcnt_t);

    // Assignment Functions

    /// See: [`mpf_set`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fset)
    #[link_name = "__gmpf_set"]
    pub fn mpf_set(rop: mpf_ptr, op: mpf_srcptr);
    /// See: [`mpf_set_ui`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fset_005fui)
    #[link_name = "__gmpf_set_ui"]
    pub fn mpf_set_ui(rop: mpf_ptr, op: c_ulong);
    /// See: [`mpf_set_si`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fset_005fsi)
    #[link_name = "__gmpf_set_si"]
    pub fn mpf_set_si(rop: mpf_ptr, op: c_long);
    /// See: [`mpf_set_default_prec`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fset_005fdefault_005fprec)
    #[link_name = "__gmpf_set_d"]
    pub fn mpf_set_d(rop: mpf_ptr, op: f64);
    /// See: [`mpf_set_z`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fset_005fz)
    #[link_name = "__gmpf_set_z"]
    pub fn mpf_set_z(rop: mpf_ptr, op: mpz_srcptr);
    /// See: [`mpf_set_q`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fset_005fq)
    #[link_name = "__gmpf_set_q"]
    pub fn mpf_set_q(rop: mpf_ptr, op: mpq_srcptr);
    /// See: [`mpf_set_str`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fset_005fstr)
    #[link_name = "__gmpf_set_str"]
    pub fn mpf_set_str(rop: mpf_ptr, str: *const c_char, base: c_int) -> c_int;
    /// See: [`mpf_swap`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fswap)
    #[link_name = "__gmpf_swap"]
    pub fn mpf_swap(rop1: mpf_ptr, rop2: mpf_ptr);

    // Combined Initialization and Assignment Functions

    /// See: [`mpf_init_set`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005finit_005fset)
    #[link_name = "__gmpf_init_set"]
    pub fn mpf_init_set(rop: mpf_ptr, op: mpf_srcptr);
    /// See: [`mpf_init_set_ui`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005finit_005fset_005fui)
    #[link_name = "__gmpf_init_set_ui"]
    pub fn mpf_init_set_ui(rop: mpf_ptr, op: c_ulong);
    /// See: [`mpf_init_set_si`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005finit_005fset_005fsi)
    #[link_name = "__gmpf_init_set_si"]
    pub fn mpf_init_set_si(rop: mpf_ptr, op: c_long);
    /// See: [`mpf_init_set_d`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005finit_005fset_005fd)
    #[link_name = "__gmpf_init_set_d"]
    pub fn mpf_init_set_d(rop: mpf_ptr, op: f64);
    /// See: [`mpf_init_set_str`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005finit_005fset_005fstr)
    #[link_name = "__gmpf_init_set_str"]
    pub fn mpf_init_set_str(rop: mpf_ptr, str: *const c_char, base: c_int) -> c_int;

    // Conversion Functions

    /// See: [`mpf_get_d`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fget_005fd)
    #[link_name = "__gmpf_get_d"]
    pub fn mpf_get_d(op: mpf_srcptr) -> f64;
    /// See: [`mpf_get_d_2exp`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fget_005fd_005f2exp)
    #[link_name = "__gmpf_get_d_2exp"]
    pub fn mpf_get_d_2exp(exp: *mut c_long, op: mpf_srcptr) -> f64;
    /// See: [`mpf_get_si`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fget_005fsi)
    #[link_name = "__gmpf_get_si"]
    pub fn mpf_get_si(op: mpf_srcptr) -> c_long;
    /// See: [`mpf_get_ui`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fget_005fui)
    #[link_name = "__gmpf_get_ui"]
    pub fn mpf_get_ui(op: mpf_srcptr) -> c_ulong;
    /// See: [`mpf_get_str`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fget_005fstr)
    #[link_name = "__gmpf_get_str"]
    pub fn mpf_get_str(
        str: *mut c_char,
        expptr: *mut exp_t,
        base: c_int,
        n_digits: usize,
        op: mpf_srcptr,
    ) -> *mut c_char;

    // Arithmetic Functions

    /// See: [`mpf_add`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fadd)
    #[link_name = "__gmpf_add"]
    pub fn mpf_add(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    /// See: [`mpf_add_ui`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fadd_005fui)
    #[link_name = "__gmpf_add_ui"]
    pub fn mpf_add_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    /// See: [`mpf_sub`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fsub)
    #[link_name = "__gmpf_sub"]
    pub fn mpf_sub(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    /// See: [`mpf_ui_sub`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fui_005fsub)
    #[link_name = "__gmpf_ui_sub"]
    pub fn mpf_ui_sub(rop: mpf_ptr, op1: c_ulong, op2: mpf_srcptr);
    /// See: [`mpf_sub_ui`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fsub_005fui)
    #[link_name = "__gmpf_sub_ui"]
    pub fn mpf_sub_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    /// See: [`mpf_mul`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fmul)
    #[link_name = "__gmpf_mul"]
    pub fn mpf_mul(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    /// See: [`mpf_mul_ui`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fmul_005fui)
    #[link_name = "__gmpf_mul_ui"]
    pub fn mpf_mul_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    /// See: [`mpf_div`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fdiv)
    #[link_name = "__gmpf_div"]
    pub fn mpf_div(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    /// See: [`mpf_ui_div`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fui_005fdiv)
    #[link_name = "__gmpf_ui_div"]
    pub fn mpf_ui_div(rop: mpf_ptr, op1: c_ulong, op2: mpf_srcptr);
    /// See: [`mpf_div_ui`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fdiv_005fui)
    #[link_name = "__gmpf_div_ui"]
    pub fn mpf_div_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    /// See: [`mpf_sqrt`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fsqrt)
    #[link_name = "__gmpf_sqrt"]
    pub fn mpf_sqrt(rop: mpf_ptr, op: mpf_srcptr);
    /// See: [`mpf_sqrt_ui`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fsqrt_005fui)
    #[link_name = "__gmpf_sqrt_ui"]
    pub fn mpf_sqrt_ui(rop: mpf_ptr, op: c_ulong);
    /// See: [`mpf_pow_ui`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fpow_005fui)
    #[link_name = "__gmpf_pow_ui"]
    pub fn mpf_pow_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    /// See: [`mpf_neg`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fneg)
    #[link_name = "__gmpf_neg"]
    pub fn mpf_neg(rop: mpf_ptr, op: mpf_srcptr);
    /// See: [`mpf_abs`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fabs)
    #[link_name = "__gmpf_abs"]
    pub fn mpf_abs(rop: mpf_ptr, op: mpf_srcptr);
    /// See: [`mpf_mul_2exp`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fmul_005f2exp)
    #[link_name = "__gmpf_mul_2exp"]
    pub fn mpf_mul_2exp(rop: mpf_ptr, op1: mpf_srcptr, op2: bitcnt_t);
    /// See: [`mpf_div_2exp`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fdiv_005f2exp)
    #[link_name = "__gmpf_div_2exp"]
    pub fn mpf_div_2exp(rop: mpf_ptr, op1: mpf_srcptr, op2: bitcnt_t);

    // Comparison Functions

    /// See: [`mpn_cmp`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fcmp)
    #[link_name = "__gmpf_cmp"]
    pub fn mpf_cmp(op1: mpf_srcptr, op2: mpf_srcptr) -> c_int;
    /// See: [`mpq_cmp_z`](../C/GMP/constant.Rational_Number_Functions.html#index-mpq_005fcmp_005fz)
    #[link_name = "__gmpf_cmp_z"]
    pub fn mpf_cmp_z(op1: mpf_srcptr, op2: mpz_srcptr) -> c_int;
    /// See: [`mpf_cmp_d`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fcmp_005fd)
    #[link_name = "__gmpf_cmp_d"]
    pub fn mpf_cmp_d(op1: mpf_srcptr, op2: f64) -> c_int;
    /// See: [`mpf_cmp_ui`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fcmp_005fui)
    #[link_name = "__gmpf_cmp_ui"]
    pub fn mpf_cmp_ui(op1: mpf_srcptr, op2: c_ulong) -> c_int;
    /// See: [`mpf_cmp_si`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fcmp_005fsi)
    #[link_name = "__gmpf_cmp_si"]
    pub fn mpf_cmp_si(op1: mpf_srcptr, op2: c_long) -> c_int;
    /// See: [`mpf_eq`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005feq)
    #[link_name = "__gmpf_eq"]
    pub fn mpf_eq(op1: mpf_srcptr, op2: mpf_srcptr, op3: bitcnt_t) -> c_int;
    /// See: [`mpf_reldiff`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005freldiff)
    #[link_name = "__gmpf_reldiff"]
    pub fn mpf_reldiff(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
}
/// See: [`mpf_sgn`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fsgn)
#[inline]
pub const unsafe extern "C" fn mpf_sgn(op: mpf_srcptr) -> c_int {
    let size = unsafe { (*op).size };
    if size < 0 {
        -1
    } else {
        (size > 0) as c_int
    }
}

// Input and Output Functions

extern "C" {
    /// See: [`mpf_out_str`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fout_005fstr)
    #[link_name = "__gmpf_out_str"]
    pub fn mpf_out_str(stream: *mut FILE, base: c_int, n_digits: usize, op: mpf_srcptr) -> usize;
    /// See: [`mpf_inp_str`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005finp_005fstr)
    #[link_name = "__gmpf_inp_str"]
    pub fn mpf_inp_str(rop: mpf_ptr, stream: *mut FILE, base: c_int) -> usize;

    // Miscellaneous Functions

    /// See: [`mpf_ceil`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005fceil)
    #[link_name = "__gmpf_ceil"]
    pub fn mpf_ceil(rop: mpf_ptr, op: mpf_srcptr);
    /// See: [`mpf_floor`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005ffloor)
    #[link_name = "__gmpf_floor"]
    pub fn mpf_floor(rop: mpf_ptr, op: mpf_srcptr);
    /// See: [`mpf_trunc`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005ftrunc)
    #[link_name = "__gmpf_trunc"]
    pub fn mpf_trunc(rop: mpf_ptr, op: mpf_srcptr);
    /// See: [`mpf_integer_p`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005finteger_005fp)
    #[link_name = "__gmpf_integer_p"]
    pub fn mpf_integer_p(op: mpf_srcptr) -> c_int;
    /// See: [`mpf_fits_ulong_p`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005ffits_005fulong_005fp)
    #[link_name = "__gmpf_fits_ulong_p"]
    pub fn mpf_fits_ulong_p(op: mpf_srcptr) -> c_int;
    /// See: [`mpf_fits_slong_p`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005ffits_005fslong_005fp)
    #[link_name = "__gmpf_fits_slong_p"]
    pub fn mpf_fits_slong_p(op: mpf_srcptr) -> c_int;
    /// See: [`mpf_fits_uint_p`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005ffits_005fuint_005fp)
    #[link_name = "__gmpf_fits_uint_p"]
    pub fn mpf_fits_uint_p(op: mpf_srcptr) -> c_int;
    /// See: [`mpf_fits_sint_p`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005ffits_005fsint_005fp)
    #[link_name = "__gmpf_fits_sint_p"]
    pub fn mpf_fits_sint_p(op: mpf_srcptr) -> c_int;
    /// See: [`mpf_fits_ushort_p`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005ffits_005fushort_005fp)
    #[link_name = "__gmpf_fits_ushort_p"]
    pub fn mpf_fits_ushort_p(op: mpf_srcptr) -> c_int;
    /// See: [`mpf_fits_sshort_p`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005ffits_005fsshort_005fp)
    #[link_name = "__gmpf_fits_sshort_p"]
    pub fn mpf_fits_sshort_p(op: mpf_srcptr) -> c_int;
    /// See: [`mpf_urandomb`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005furandomb)
    #[link_name = "__gmpf_urandomb"]
    pub fn mpf_urandomb(rop: mpf_t, state: randstate_ptr, nbits: bitcnt_t);
    /// See: [`mpf_random2`](../C/GMP/constant.Floating_point_Functions.html#index-mpf_005frandom2)
    #[link_name = "__gmpf_random2"]
    pub fn mpf_random2(rop: mpf_ptr, max_size: size_t, exp: exp_t);
}

// Low-Level Functions

extern "C" {
    /// See: [`mpn_add_n`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fadd_005fn)
    #[link_name = "__gmpn_add_n"]
    pub fn mpn_add_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t) -> limb_t;
    /// See: [`mpn_add_1`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fadd_005f1)
    #[link_name = "__gmpn_add_1"]
    pub fn mpn_add_1(rp: mp_ptr, s1p: mp_srcptr, n: size_t, s2limb: limb_t) -> limb_t;
    /// See: [`mpn_add`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fadd)
    #[link_name = "__gmpn_add"]
    pub fn mpn_add(rp: mp_ptr, s1p: mp_srcptr, s1n: size_t, s2p: mp_srcptr, s2n: size_t) -> limb_t;
    /// See: [`mpn_cnd_sub_n`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fcnd_005fsub_005fn)
    #[link_name = "__gmpn_sub_n"]
    pub fn mpn_sub_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t) -> limb_t;
    /// See: [`mpn_sub_1`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsub_005f1)
    #[link_name = "__gmpn_sub_1"]
    pub fn mpn_sub_1(rp: mp_ptr, s1p: mp_srcptr, n: size_t, s2limb: limb_t) -> limb_t;
    /// See: [`mpn_sub`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsub)
    #[link_name = "__gmpn_sub"]
    pub fn mpn_sub(rp: mp_ptr, s1p: mp_srcptr, s1n: size_t, s2p: mp_srcptr, s2n: size_t) -> limb_t;
    /// See: [`mpn_neg`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fneg)
    #[link_name = "__gmpn_neg"]
    pub fn mpn_neg(rp: mp_ptr, sp: mp_srcptr, n: size_t) -> limb_t;
    /// See: [`mpn_mul_n`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fmul_005fn)
    #[link_name = "__gmpn_mul_n"]
    pub fn mpn_mul_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t);
    /// See: [`mpn_mul`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fmul)
    #[link_name = "__gmpn_mul"]
    pub fn mpn_mul(rp: mp_ptr, s1p: mp_srcptr, s1n: size_t, s2p: mp_srcptr, s2n: size_t) -> limb_t;
    /// See: [`mpn_sqr`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsqr)
    #[link_name = "__gmpn_sqr"]
    pub fn mpn_sqr(rp: mp_ptr, s1p: mp_srcptr, n: size_t);
    /// See: [`mpn_mul_1`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fmul_005f1)
    #[link_name = "__gmpn_mul_1"]
    pub fn mpn_mul_1(rp: mp_ptr, s1p: mp_srcptr, n: size_t, s2limb: limb_t) -> limb_t;
    /// See: [`mpn_addmul_1`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005faddmul_005f1)
    #[link_name = "__gmpn_addmul_1"]
    pub fn mpn_addmul_1(rp: mp_ptr, s1p: mp_srcptr, n: size_t, s2limb: limb_t) -> limb_t;
    /// See: [`mpn_submul_1`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsubmul_005f1)
    #[link_name = "__gmpn_submul_1"]
    pub fn mpn_submul_1(rp: mp_ptr, s1p: mp_srcptr, n: size_t, s2limb: limb_t) -> limb_t;
    /// See: [`mpn_tdiv_qr`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005ftdiv_005fqr)
    #[link_name = "__gmpn_tdiv_qr"]
    pub fn mpn_tdiv_qr(
        qp: mp_ptr,
        rp: mp_ptr,
        qxn: size_t,
        np: mp_srcptr,
        nn: size_t,
        dp: mp_srcptr,
        dn: size_t,
    );
    /// See: [`mpn_divrem_1`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fdivrem_005f1)
    #[link_name = "__gmpn_divrem_1"]
    pub fn mpn_divrem_1(
        r1p: mp_ptr,
        qxn: size_t,
        s2p: mp_srcptr,
        s2n: size_t,
        s3limb: limb_t,
    ) -> limb_t;
}
/// See: [`mpn_divmod_1`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fdivmod_005f1)
#[inline]
pub unsafe extern "C" fn mpn_divmod_1(
    r1p: mp_ptr,
    s2p: mp_srcptr,
    s2n: size_t,
    s3limb: limb_t,
) -> limb_t {
    unsafe { mpn_divrem_1(r1p, 0, s2p, s2n, s3limb) }
}
extern "C" {
    /// See: [`mpn_divexact_1`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fdivexact_005f1)
    #[link_name = "__gmpn_divexact_1"]
    pub fn mpn_divexact_1(rp: mp_ptr, sp: mp_srcptr, n: size_t, d: limb_t);
}
/// See: [`mpn_divexact_by3`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fdivexact_005fby3)
#[inline]
pub unsafe extern "C" fn mpn_divexact_by3(rp: mp_ptr, sp: mp_srcptr, n: size_t) -> limb_t {
    unsafe { mpn_divexact_by3c(rp, sp, n, 0) }
}
extern "C" {
    /// See: [`mpn_divexact_by3c`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fdivexact_005fby3c)
    #[link_name = "__gmpn_divexact_by3c"]
    pub fn mpn_divexact_by3c(rp: mp_ptr, sp: mp_srcptr, n: size_t, carry: limb_t) -> limb_t;
    /// See: [`mpn_divmod_1`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fdivmod_005f1)
    #[link_name = "__gmpn_mod_1"]
    pub fn mpn_mod_1(s1p: mp_srcptr, s1n: size_t, s2limb: limb_t) -> limb_t;
    /// See: [`mpn_lshift`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005flshift)
    #[link_name = "__gmpn_lshift"]
    pub fn mpn_lshift(rp: mp_ptr, sp: mp_srcptr, n: size_t, count: c_uint) -> limb_t;
    /// See: [`mpn_rshift`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005frshift)
    #[link_name = "__gmpn_rshift"]
    pub fn mpn_rshift(rp: mp_ptr, sp: mp_srcptr, n: size_t, count: c_uint) -> limb_t;
    /// See: [`mpn_cmp`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fcmp)
    #[link_name = "__gmpn_cmp"]
    pub fn mpn_cmp(s1p: mp_srcptr, s2p: mp_srcptr, n: size_t) -> c_int;
    /// See: [`mpn_zero_p`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fzero_005fp)
    #[link_name = "__gmpn_zero_p"]
    pub fn mpn_zero_p(sp: mp_srcptr, n: size_t) -> c_int;
    /// See: [`mpn_gcd`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fgcd)
    #[link_name = "__gmpn_gcd"]
    pub fn mpn_gcd(rp: mp_ptr, xp: mp_ptr, xn: size_t, yp: mp_ptr, yn: size_t) -> size_t;
    /// See: [`mpn_gcd_1`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fgcd_005f1)
    #[link_name = "__gmpn_gcd_1"]
    pub fn mpn_gcd_1(xp: mp_srcptr, xn: size_t, yimb: limb_t) -> limb_t;
    /// See: [`mpn_gcdext`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fgcdext)
    #[link_name = "__gmpn_gcdext"]
    pub fn mpn_gcdext(
        gp: mp_ptr,
        sp: mp_ptr,
        sn: *mut size_t,
        up: mp_ptr,
        un: size_t,
        vp: mp_ptr,
        vn: size_t,
    ) -> size_t;
    /// See: [`mpn_sqrtrem`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsqrtrem)
    #[link_name = "__gmpn_sqrtrem"]
    pub fn mpn_sqrtrem(r1p: mp_ptr, r2p: mp_ptr, sp: mp_srcptr, n: size_t) -> size_t;
    /// See: [`mpn_sizeinbase`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsizeinbase)
    #[link_name = "__gmpn_sizeinbase"]
    pub fn mpn_sizeinbase(xp: mp_srcptr, n: size_t, base: c_int) -> usize;
    /// See: [`mpn_get_str`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fget_005fstr)
    #[link_name = "__gmpn_get_str"]
    pub fn mpn_get_str(str: *mut c_uchar, base: c_int, s1p: mp_ptr, s1n: size_t) -> usize;
    /// See: [`mpn_set_str`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fset_005fstr)
    #[link_name = "__gmpn_set_str"]
    pub fn mpn_set_str(rp: mp_ptr, str: *const c_uchar, strsize: usize, base: c_int) -> size_t;
    /// See: [`mpn_scan0`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fscan0)
    #[link_name = "__gmpn_scan0"]
    pub fn mpn_scan0(s1p: mp_srcptr, bit: bitcnt_t) -> bitcnt_t;
    /// See: [`mpn_scan1`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fscan1)
    #[link_name = "__gmpn_scan1"]
    pub fn mpn_scan1(s1p: mp_srcptr, bit: bitcnt_t) -> bitcnt_t;
    /// See: [`mpn_random`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005frandom)
    #[link_name = "__gmpn_random"]
    pub fn mpn_random(r1p: mp_ptr, r1n: size_t);
    /// See: [`mpn_random2`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005frandom2)
    #[link_name = "__gmpn_random2"]
    pub fn mpn_random2(r1p: mp_ptr, r1n: size_t);
    /// See: [`mpn_popcount`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fpopcount)
    #[link_name = "__gmpn_popcount"]
    pub fn mpn_popcount(s1p: mp_srcptr, n: size_t) -> bitcnt_t;
    /// See: [`mpn_hamdist`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fhamdist)
    #[link_name = "__gmpn_hamdist"]
    pub fn mpn_hamdist(s1p: mp_srcptr, s2p: mp_srcptr, n: size_t) -> bitcnt_t;
    /// See: [`mpn_perfect_square_p`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fperfect_005fsquare_005fp)
    #[link_name = "__gmpn_perfect_square_p"]
    pub fn mpn_perfect_square_p(s1p: mp_srcptr, n: size_t) -> c_int;
    /// See: [`mpn_and_n`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fand_005fn)
    #[link_name = "__gmpn_and_n"]
    pub fn mpn_and_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t);
    /// See: [`mpn_ior_n`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fior_005fn)
    #[link_name = "__gmpn_ior_n"]
    pub fn mpn_ior_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t);
    /// See: [`mpn_xor_n`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fxor_005fn)
    #[link_name = "__gmpn_xor_n"]
    pub fn mpn_xor_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t);
    /// See: [`mpn_andn_n`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fandn_005fn)
    #[link_name = "__gmpn_andn_n"]
    pub fn mpn_andn_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t);
    /// See: [`mpn_iorn_n`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fiorn_005fn)
    #[link_name = "__gmpn_iorn_n"]
    pub fn mpn_iorn_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t);
    /// See: [`mpn_nand_n`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fnand_005fn)
    #[link_name = "__gmpn_nand_n"]
    pub fn mpn_nand_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t);
    /// See: [`mpn_nior_n`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fnior_005fn)
    #[link_name = "__gmpn_nior_n"]
    pub fn mpn_nior_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t);
    /// See: [`mpn_xnor_n`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fxnor_005fn)
    #[link_name = "__gmpn_xnor_n"]
    pub fn mpn_xnor_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t);
    /// See: [`mpn_com`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fcom)
    #[link_name = "__gmpn_com"]
    pub fn mpn_com(rp: mp_ptr, sp: mp_srcptr, n: size_t);
    /// See: [`mpn_copyi`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fcopyi)
    #[link_name = "__gmpn_copyi"]
    pub fn mpn_copyi(rp: mp_ptr, s1p: mp_srcptr, n: size_t);
    /// See: [`mpn_copyd`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fcopyd)
    #[link_name = "__gmpn_copyd"]
    pub fn mpn_copyd(rp: mp_ptr, s1p: mp_srcptr, n: size_t);
    /// See: [`mpn_zero`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fzero)
    #[link_name = "__gmpn_zero"]
    pub fn mpn_zero(rp: mp_ptr, n: size_t);

    // Low-level functions for cryptography

    /// See: [`mpn_cnd_add_n`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fcnd_005fadd_005fn)
    #[link_name = "__gmpn_cnd_add_n"]
    pub fn mpn_cnd_add_n(
        cnd: limb_t,
        rp: mp_ptr,
        s1p: mp_srcptr,
        s2p: mp_srcptr,
        n: size_t,
    ) -> limb_t;
    /// See: [`mpn_cnd_sub_n`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fcnd_005fsub_005fn)
    #[link_name = "__gmpn_cnd_sub_n"]
    pub fn mpn_cnd_sub_n(
        cnd: limb_t,
        rp: mp_ptr,
        s1p: mp_srcptr,
        s2p: mp_srcptr,
        n: size_t,
    ) -> limb_t;
    /// See: [`mpn_sec_add_1`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsec_005fadd_005f1)
    #[link_name = "__gmpn_sec_add_1"]
    pub fn mpn_sec_add_1(rp: mp_ptr, ap: mp_srcptr, n: size_t, b: limb_t, tp: mp_ptr) -> limb_t;
    /// See: [`mpn_sec_add_1`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsec_005fadd_005f1)
    #[link_name = "__gmpn_sec_add_1_itch"]
    pub fn mpn_sec_add_1_itch(n: size_t) -> size_t;
    /// See: [`mpn_sec_sub_1`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsec_005fsub_005f1)
    #[link_name = "__gmpn_sec_sub_1"]
    pub fn mpn_sec_sub_1(rp: mp_ptr, ap: mp_srcptr, n: size_t, b: limb_t, tp: mp_ptr) -> limb_t;
    /// See: [`mpn_sec_sub_1`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsec_005fsub_005f1)
    #[link_name = "__gmpn_sec_sub_1_itch"]
    pub fn mpn_sec_sub_1_itch(n: size_t) -> size_t;
    /// See: [`mpn_cnd_swap`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fcnd_005fswap)
    #[link_name = "__gmpn_cnd_swap"]
    pub fn mpn_cnd_swap(cnd: limb_t, ap: *mut limb_t, bp: *mut limb_t, n: size_t);
    /// See: [`mpn_sec_mul`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsec_005fmul)
    #[link_name = "__gmpn_sec_mul"]
    pub fn mpn_sec_mul(
        rp: mp_ptr,
        ap: mp_srcptr,
        an: size_t,
        bp: mp_srcptr,
        bn: size_t,
        tp: mp_ptr,
    );
    /// See: [`mpn_sec_mul_itch`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsec_005fmul_005fitch)
    #[link_name = "__gmpn_sec_mul_itch"]
    pub fn mpn_sec_mul_itch(an: size_t, bn: size_t) -> size_t;
    /// See: [`mpn_sec_sqr`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsec_005fsqr)
    #[link_name = "__gmpn_sec_sqr"]
    pub fn mpn_sec_sqr(rp: mp_ptr, ap: mp_srcptr, an: size_t, tp: mp_ptr);
    /// See: [`mpn_sec_sqr_itch`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsec_005fsqr_005fitch)
    #[link_name = "__gmpn_sec_sqr_itch"]
    pub fn mpn_sec_sqr_itch(an: size_t) -> size_t;
    /// See: [`mpn_sec_powm`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsec_005fpowm)
    #[link_name = "__gmpn_sec_powm"]
    pub fn mpn_sec_powm(
        rp: mp_ptr,
        bp: mp_srcptr,
        bn: size_t,
        ep: mp_srcptr,
        enb: bitcnt_t,
        mp: mp_srcptr,
        n: size_t,
        tp: mp_ptr,
    );
    /// See: [`mpn_sec_powm_itch`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsec_005fpowm_005fitch)
    #[link_name = "__gmpn_sec_powm_itch"]
    pub fn mpn_sec_powm_itch(bn: size_t, enb: bitcnt_t, n: size_t) -> size_t;
    /// See: [`mpn_sec_tabselect`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsec_005ftabselect)
    #[link_name = "__gmpn_sec_tabselect"]
    pub fn mpn_sec_tabselect(
        rp: *mut limb_t,
        tab: *const limb_t,
        n: size_t,
        nents: size_t,
        which: size_t,
    );
    /// See: [`mpn_sec_div_qr`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsec_005fdiv_005fqr)
    #[link_name = "__gmpn_sec_div_qr"]
    pub fn mpn_sec_div_qr(
        qp: mp_ptr,
        np: mp_ptr,
        nn: size_t,
        dp: mp_srcptr,
        dn: size_t,
        tp: mp_ptr,
    ) -> limb_t;
    /// See: [`mpn_sec_div_qr_itch`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsec_005fdiv_005fqr_005fitch)
    #[link_name = "__gmpn_sec_div_qr_itch"]
    pub fn mpn_sec_div_qr_itch(nn: size_t, dn: size_t) -> size_t;
    /// See: [`mpn_sec_div_r`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsec_005fdiv_005fr)
    #[link_name = "__gmpn_sec_div_r"]
    pub fn mpn_sec_div_r(np: mp_ptr, nn: size_t, dp: mp_srcptr, dn: size_t, tp: mp_ptr);
    /// See: [`mpn_sec_div_r_itch`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsec_005fdiv_005fr_005fitch)
    #[link_name = "__gmpn_sec_div_r_itch"]
    pub fn mpn_sec_div_r_itch(nn: size_t, dn: size_t) -> size_t;
    /// See: [`mpn_sec_invert`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsec_005finvert)
    #[link_name = "__gmpn_sec_invert"]
    pub fn mpn_sec_invert(
        rp: mp_ptr,
        ap: mp_ptr,
        mp: mp_srcptr,
        n: size_t,
        nbcnt: bitcnt_t,
        tp: mp_ptr,
    ) -> c_int;
    /// See: [`mpn_sec_invert_itch`](../C/GMP/constant.Low_level_Functions.html#index-mpn_005fsec_005finvert_005fitch)
    #[link_name = "__gmpn_sec_invert_itch"]
    pub fn mpn_sec_invert_itch(n: size_t) -> size_t;
}

// Random Numbers

// Random State Initialization

extern "C" {
    /// See: [`gmp_randinit_default`](../C/GMP/constant.Random_Number_Functions.html#index-gmp_005frandinit_005fdefault)
    #[link_name = "__gmp_randinit_default"]
    pub fn randinit_default(state: randstate_ptr);
    /// See: [`gmp_randinit_mt`](../C/GMP/constant.Random_Number_Functions.html#index-gmp_005frandinit_005fmt)
    #[link_name = "__gmp_randinit_mt"]
    pub fn randinit_mt(state: randstate_ptr);
    /// See: [`gmp_randinit_lc_2exp`](../C/GMP/constant.Random_Number_Functions.html#index-gmp_005frandinit_005flc_005f2exp)
    #[link_name = "__gmp_randinit_lc_2exp"]
    pub fn randinit_lc_2exp(state: randstate_ptr, a: mpz_srcptr, c: c_ulong, m2exp: bitcnt_t);
    /// See: [`gmp_randinit_lc_2exp_size`](../C/GMP/constant.Random_Number_Functions.html#index-gmp_005frandinit_005flc_005f2exp_005fsize)
    #[link_name = "__gmp_randinit_lc_2exp_size"]
    pub fn randinit_lc_2exp_size(state: randstate_ptr, size: bitcnt_t) -> c_int;
    /// See: [`gmp_randinit_set`](../C/GMP/constant.Random_Number_Functions.html#index-gmp_005frandinit_005fset)
    #[link_name = "__gmp_randinit_set"]
    pub fn randinit_set(rop: randstate_ptr, op: randstate_srcptr);
    /// See: [`gmp_randclear`](../C/GMP/constant.Random_Number_Functions.html#index-gmp_005frandclear)
    #[link_name = "__gmp_randclear"]
    pub fn randclear(state: randstate_ptr);

    // Random State Seeding

    /// See: [`gmp_randseed`](../C/GMP/constant.Random_Number_Functions.html#index-gmp_005frandseed)
    #[link_name = "__gmp_randseed"]
    pub fn randseed(state: randstate_ptr, seed: mpz_srcptr);
    /// See: [`gmp_randseed_ui`](../C/GMP/constant.Random_Number_Functions.html#index-gmp_005frandseed_005fui)
    #[link_name = "__gmp_randseed_ui"]
    pub fn randseed_ui(state: randstate_ptr, seed: c_ulong);

    // Random State Miscellaneous

    /// See: [`gmp_urandomb_ui`](../C/GMP/constant.Random_Number_Functions.html#index-gmp_005furandomb_005fui)
    #[link_name = "__gmp_urandomb_ui"]
    pub fn urandomb_ui(state: randstate_ptr, n: c_ulong) -> c_ulong;
    /// See: [`gmp_urandomm_ui`](../C/GMP/constant.Random_Number_Functions.html#index-gmp_005furandomm_005fui)
    #[link_name = "__gmp_urandomm_ui"]
    pub fn urandomm_ui(state: randstate_ptr, n: c_ulong) -> c_ulong;
}

// Formatted Output

extern "C" {
    /// See: [`gmp_printf`](../C/GMP/constant.Formatted_Output.html#index-gmp_005fprintf)
    #[link_name = "__gmp_printf"]
    pub fn printf(fmt: *const c_char, ...) -> c_int;
    /// See: [`gmp_fprintf`](../C/GMP/constant.Formatted_Output.html#index-gmp_005ffprintf)
    #[link_name = "__gmp_fprintf"]
    pub fn fprintf(fp: *mut FILE, fmt: *const c_char, ...) -> c_int;
    /// See: [`gmp_sprintf`](../C/GMP/constant.Formatted_Output.html#index-gmp_005fsprintf)
    #[link_name = "__gmp_sprintf"]
    pub fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    /// See: [`gmp_snprintf`](../C/GMP/constant.Formatted_Output.html#index-gmp_005fsnprintf)
    #[link_name = "__gmp_snprintf"]
    pub fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    /// See: [`gmp_asprintf`](../C/GMP/constant.Formatted_Output.html#index-gmp_005fasprintf)
    #[link_name = "__gmp_asprintf"]
    pub fn asprintf(pp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
}

// Formatted Input

extern "C" {
    /// See: [`gmp_scanf`](../C/GMP/constant.Formatted_Input.html#index-gmp_005fscanf)
    #[link_name = "__gmp_scanf"]
    pub fn scanf(fmt: *const c_char, ...) -> c_int;
    /// See: [`gmp_fscanf`](../C/GMP/constant.Formatted_Input.html#index-gmp_005ffscanf)
    #[link_name = "__gmp_fscanf"]
    pub fn fscanf(fp: *mut FILE, fmt: *const c_char, ...) -> c_int;
    /// See: [`gmp_sscanf`](../C/GMP/constant.Formatted_Input.html#index-gmp_005fsscanf)
    #[link_name = "__gmp_sscanf"]
    pub fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
}

// Custom Allocation

/// See: [`allocate_function`](../C/GMP/constant.Custom_Allocation.html#index-allocate_005ffunction)
///
/// # Planned change
///
/// In the next major version of the crate (version 2), this will be
/// changed to
/// `unsafe extern "C" fn(alloc_size: usize) -> *mut c_void`, that is
/// it will no longer be an [`Option`], and the function can also be
/// unsafe.
pub type allocate_function = Option<extern "C" fn(alloc_size: usize) -> *mut c_void>;
/// See: [`reallocate_function`](../C/GMP/constant.Custom_Allocation.html#index-reallocate_005ffunction)
///
/// # Planned change
///
/// In the next major version of the crate (version 2), this will be
/// changed to
/// `unsafe extern "C" fn(ptr: *mut c_void, old_size: usize, new_size: usize) -> *mut c_void`,
/// that is it will no longer be an [`Option`].
pub type reallocate_function =
    Option<unsafe extern "C" fn(ptr: *mut c_void, old_size: usize, new_size: usize) -> *mut c_void>;
/// See: [`free_function`](../C/GMP/constant.Custom_Allocation.html#index-free_005ffunction)
///
/// # Planned change
///
/// In the next major version of the crate (version 2), this will be
/// changed to `unsafe extern "C" fn(ptr: *mut c_void, size: usize)`,
/// that is it will no longer be an [`Option`].
pub type free_function = Option<unsafe extern "C" fn(ptr: *mut c_void, size: usize)>;
extern "C" {
    /// See: [`mp_set_memory_functions`](../C/GMP/constant.Custom_Allocation.html#index-mp_005fset_005fmemory_005ffunctions)
    ///
    /// # Planned change
    ///
    /// In the next major version of the crate (version 2), the arguments will be of types
    /// <code>[Option][`Option`]&lt;[allocate\_function][`allocate_function`]&gt;</code>,
    /// <code>[Option][`Option`]&lt;[reallocate\_function][`reallocate_function`]&gt;</code>
    /// and
    /// <code>[Option][`Option`]&lt;[free\_function][`free_function`]&gt;</code>,
    /// since the function types themselves will no longer be [`Option`].
    #[link_name = "__gmp_set_memory_functions"]
    pub fn set_memory_functions(
        alloc_func_ptr: allocate_function,
        realloc_func_ptr: reallocate_function,
        free_func_ptr: free_function,
    );
    /// See: [`mp_get_memory_functions`](../C/GMP/constant.Custom_Allocation.html#index-mp_005fget_005fmemory_005ffunctions)
    #[link_name = "__gmp_get_memory_functions"]
    pub fn get_memory_functions(
        alloc_func_ptr: *mut allocate_function,
        realloc_func_ptr: *mut reallocate_function,
        free_func_ptr: *mut free_function,
    );
}

#[cfg(test)]
mod tests {
    use crate::gmp;
    use core::mem;
    use core::ptr::NonNull;

    #[test]
    fn check_mpq_num_den_offsets() {
        let mut limbs: [gmp::limb_t; 2] = [1, 1];
        let mut q = gmp::mpq_t {
            num: gmp::mpz_t {
                alloc: 1,
                size: 1,
                d: NonNull::from(&mut limbs[0]),
            },
            den: gmp::mpz_t {
                alloc: 1,
                size: 1,
                d: NonNull::from(&mut limbs[1]),
            },
        };
        unsafe {
            assert_eq!(&q.num as *const gmp::mpz_t, gmp::mpq_numref_const(&q));
            assert_eq!(&q.den as *const gmp::mpz_t, gmp::mpq_denref_const(&q));
            assert_eq!(&mut q.num as *mut gmp::mpz_t, gmp::mpq_numref(&mut q));
            assert_eq!(&mut q.den as *mut gmp::mpz_t, gmp::mpq_denref(&mut q));
        }
    }

    #[test]
    fn check_limb_size() {
        let from_static = unsafe { gmp::bits_per_limb };
        let from_type = mem::size_of::<gmp::limb_t>() * 8;
        let from_constant = gmp::LIMB_BITS;
        assert_eq!(from_static as usize, from_type);
        assert_eq!(from_static, from_constant);
    }

    #[test]
    fn check_version() {
        use crate::tests;

        let (major, minor, patchlevel) = (6, 3, 0);
        let version = "6.3.0";

        assert_eq!(gmp::VERSION, major);
        assert!(gmp::VERSION_MINOR >= minor);
        if cfg!(not(feature = "use-system-libs")) {
            assert!(gmp::VERSION_MINOR > minor || gmp::VERSION_PATCHLEVEL >= patchlevel);
            if gmp::VERSION_MINOR == minor && gmp::VERSION_PATCHLEVEL == patchlevel {
                assert_eq!(unsafe { tests::str_from_cstr(gmp::version) }, version);
            }
        }
    }
}
