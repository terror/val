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
Function and type bindings for the [MPFR] library.

# Examples

```rust
use core::mem::MaybeUninit;
use gmp_mpfr_sys::mpfr;
let one_third = 1.0_f64 / 3.0;
unsafe {
    let mut f = {
        let mut f = MaybeUninit::uninit();
        mpfr::init2(f.as_mut_ptr(), 53);
        f.assume_init()
    };
    let dir = mpfr::set_d(&mut f, one_third, mpfr::rnd_t::RNDN);
    assert_eq!(dir, 0);
    let d = mpfr::get_d(&f, mpfr::rnd_t::RNDN);
    assert_eq!(d, one_third);
    mpfr::clear(&mut f);
}
```

The following example is a translation of the [MPFR sample] found on
the [MPFR] website. The program computes a lower bound on
1 + 1/1! + 1/2! + … + 1/100!
using 200-bit precision. The program outputs:

`Sum is 2.7182818284590452353602874713526624977572470936999595749669131e0`

```rust
use core::ffi::c_int;
use core::mem::MaybeUninit;
use gmp_mpfr_sys::mpfr;
use gmp_mpfr_sys::mpfr::rnd_t;
use libc::STDOUT_FILENO;

fn main() {
    unsafe {
        let mut t = {
            let mut t = MaybeUninit::uninit();
            mpfr::init2(t.as_mut_ptr(), 200);
            t.assume_init()
        };
        mpfr::set_d(&mut t, 1.0, rnd_t::RNDD);

        let mut s = {
            let mut s = MaybeUninit::uninit();
            mpfr::init2(s.as_mut_ptr(), 200);
            s.assume_init()
        };
        mpfr::set_d(&mut s, 1.0, rnd_t::RNDD);

        let mut u = {
            let mut u = MaybeUninit::uninit();
            mpfr::init2(u.as_mut_ptr(), 200);
            u.assume_init()
        };

        for i in 1..=100 {
            mpfr::mul_ui(&mut t, &t, i, rnd_t::RNDU);
            mpfr::set_d(&mut u, 1.0, rnd_t::RNDD);
            mpfr::div(&mut u, &u, &t, rnd_t::RNDD);
            mpfr::add(&mut s, &s, &u, rnd_t::RNDD);
        }

        let stdout = libc::fdopen(STDOUT_FILENO, b"w\0".as_ptr().cast());
#       let real_stdout = stdout;
#       let tmp_file = libc::tmpfile();
#       let stdout = if tmp_file.is_null() { real_stdout } else { tmp_file };
        libc::fputs(b"Sum is \0".as_ptr().cast(), stdout);
        mpfr::out_str(stdout, 10, 0, &s, rnd_t::RNDD);
        libc::fputc(b'\n' as c_int, stdout);
#       libc::rewind(stdout);
#       let mut buf = [0u8; 73];
#       libc::fread(buf.as_mut_ptr().cast(), 1, 73, stdout);
#       let stdout = real_stdout;
        libc::fclose(stdout);
#       if !tmp_file.is_null() {
#           libc::fclose(tmp_file);
#           assert_eq!(
#               &buf[..],
#               &b"Sum is 2.7182818284590452353602874713526624977572470936999595749669131e0\n"[..]
#           );
#       }

        mpfr::clear(&mut s);
        mpfr::clear(&mut t);
        mpfr::clear(&mut u);
        mpfr::free_cache();
    }
}
```

[MPFR sample]: https://www.mpfr.org/sample.html
[MPFR]: https://www.mpfr.org/
*/
#![allow(non_camel_case_types, non_snake_case)]
#![allow(clippy::needless_doctest_main)]

use crate::gmp::{limb_t, mpf_t, mpq_t, mpz_t, randstate_t, NUMB_BITS};
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr::NonNull;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use libc::{intmax_t, uintmax_t, FILE};

include!(concat!(env!("OUT_DIR"), "/mpfr_h.rs"));

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
type FILE = c_void;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
type intmax_t = i64;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
type uintmax_t = u64;

/// See: [`mpfr_prec_t`](../C/MPFR/constant.MPFR_Basics.html#index-mpfr_005fprec_005ft)
pub type prec_t = c_long;

/// See: [`mpfr_rnd_t`](../C/MPFR/constant.MPFR_Basics.html#index-mpfr_005frnd_005ft)
///
/// # Planned change
///
/// In the next major version of the crate (version 2), this enum will
/// be replaced by a type alias to [`c_int`]. The variants will be
/// replaced by constants, for example `rnd_t::RNDN` will be replaced
/// by a constant `RNDN` of type [`c_int`].
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(deprecated)]
pub enum rnd_t {
    /// See: [Rounding](../C/MPFR/constant.MPFR_Basics.html#Rounding)
    ///
    /// # Planned change
    ///
    /// In the next major version of the crate (version 2), the enum
    /// will be removed. This variant will be replaced by a constant
    /// `RNDN` of type [`c_int`].
    RNDN = 0,
    /// See: [Rounding Modes](../C/MPFR/constant.MPFR_Basics.html#Rounding)
    ///
    /// # Planned change
    ///
    /// In the next major version of the crate (version 2), the enum
    /// will be removed. This variant will be replaced by a constant
    /// `RNDZ` of type [`c_int`].
    RNDZ = 1,
    /// See: [Rounding Modes](../C/MPFR/constant.MPFR_Basics.html#Rounding)
    ///
    /// # Planned change
    ///
    /// In the next major version of the crate (version 2), the enum
    /// will be removed. This variant will be replaced by a constant
    /// `RNDU` of type [`c_int`].
    RNDU = 2,
    /// See: [Rounding Modes](../C/MPFR/constant.MPFR_Basics.html#Rounding)
    ///
    /// # Planned change
    ///
    /// In the next major version of the crate (version 2), the enum
    /// will be removed. This variant will be replaced by a constant
    /// `RNDD` of type [`c_int`].
    RNDD = 3,
    /// See: [Rounding Modes](../C/MPFR/constant.MPFR_Basics.html#Rounding)
    ///
    /// # Planned change
    ///
    /// In the next major version of the crate (version 2), the enum
    /// will be removed. This variant will be replaced by a constant
    /// `RNDA` of type [`c_int`].
    RNDA = 4,
    /// See: [Rounding Modes](../C/MPFR/constant.MPFR_Basics.html#Rounding)
    ///
    /// # Planned change
    ///
    /// In the next major version of the crate (version 2), the enum
    /// will be removed. This variant will be replaced by a constant
    /// `RNDF` of type [`c_int`].
    RNDF = 5,
    #[doc(hidden)]
    #[deprecated(since = "1.1.0", note = "do not use!")]
    RNDNA = -1,
}

/// See: [`mpfr_flags_t`](../C/MPFR/constant.MPFR_Basics.html#index-mpfr_005fflags_005ft)
pub type flags_t = c_uint;

/// See: [Exception Related Functions](../C/MPFR/constant.MPFR_Interface.html#Exception-Related-Functions)
pub type exp_t = c_long;

/// See: [Nomenclature and Types](../C/MPFR/constant.MPFR_Basics.html#Nomenclature-and-Types)
pub const PREC_MIN: prec_t = 1;
/// See: [Nomenclature and Types](../C/MPFR/constant.MPFR_Basics.html#Nomenclature-and-Types)
pub const PREC_MAX: prec_t = prec_t::MAX - 256;

/// See: [`mpfr_t`](../C/MPFR/constant.MPFR_Basics.html#index-mpfr_005ft)
/// and [Internals](../C/MPFR/constant.MPFR_Interface.html#Internals)
///
#[doc = include_str!("internal_fields.md")]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct mpfr_t {
    /// See: [Internals](../C/MPFR/constant.MPFR_Interface.html#Internals)
    pub prec: prec_t,
    /// See: [Internals](../C/MPFR/constant.MPFR_Interface.html#Internals)
    pub sign: c_int,
    /// See: [Internals](../C/MPFR/constant.MPFR_Interface.html#Internals)
    pub exp: exp_t,
    /// See: [Internals](../C/MPFR/constant.MPFR_Interface.html#Internals)
    pub d: NonNull<limb_t>,
}

/// See: [`mpfr_custom_init_set`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcustom_005finit_005fset)
pub const NAN_KIND: c_int = 0;
/// See: [`mpfr_custom_init_set`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcustom_005finit_005fset)
pub const INF_KIND: c_int = 1;
/// See: [`mpfr_custom_init_set`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcustom_005finit_005fset)
pub const ZERO_KIND: c_int = 2;
/// See: [`mpfr_custom_init_set`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcustom_005finit_005fset)
pub const REGULAR_KIND: c_int = 3;

/// See: [`mpfr_free_cache2`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffree_005fcache2)
pub const FREE_LOCAL_CACHE: c_int = 1;
/// See: [`mpfr_free_cache2`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffree_005fcache2)
pub const FREE_GLOBAL_CACHE: c_int = 2;

// Types for function declarations in this file.

type mpz_srcptr = *const mpz_t;
type mpz_ptr = *mut mpz_t;
type mpq_srcptr = *const mpq_t;
type mpq_ptr = *mut mpq_t;
type mpf_srcptr = *const mpf_t;
type mpf_ptr = *mut mpf_t;
type randstate_ptr = *mut randstate_t;
type mpfr_ptr = *mut mpfr_t;
type mpfr_srcptr = *const mpfr_t;

// Private constants.

const EXP_MAX: exp_t = exp_t::MAX;
const EXP_NAN: exp_t = 1 - EXP_MAX;
const EXP_ZERO: exp_t = 0 - EXP_MAX;
const EXP_INF: exp_t = 2 - EXP_MAX;

// Initialization Functions

extern "C" {
    /// See: [`mpfr_init2`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005finit2)
    #[link_name = "mpfr_init2"]
    pub fn init2(x: mpfr_ptr, prec: prec_t);
    /// See: [`mpfr_inits2`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005finits2)
    #[link_name = "mpfr_inits2"]
    pub fn inits2(prec: prec_t, x: mpfr_ptr, ...);
    /// See: [`mpfr_clear`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fclear)
    #[link_name = "mpfr_clear"]
    pub fn clear(x: mpfr_ptr);
    /// See: [`mpfr_clears`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fclears)
    #[link_name = "mpfr_clears"]
    pub fn clears(x: mpfr_ptr, ...);
    /// See: [`mpfr_init`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005finit)
    #[link_name = "mpfr_init"]
    pub fn init(x: mpfr_ptr);
    /// See: [`mpfr_inits`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005finits)
    #[link_name = "mpfr_inits"]
    pub fn inits(x: mpfr_ptr, ...);
}
// macro will be exported at top level, so link to C/MPFR... not to ../C/MPFR...
/// See: [`MPFR_DECL_INIT`](C/MPFR/constant.MPFR_Interface.html#index-MPFR_005fDECL_005fINIT)
#[macro_export]
macro_rules! MPFR_DECL_INIT {
    ($name:ident, $prec:expr) => {
        // limbs is visible only in one macro instance thanks to macro hygiene
        let mut limbs = [core::mem::MaybeUninit::<$crate::gmp::limb_t>::uninit();
            ($prec as usize - 1) / $crate::gmp::NUMB_BITS as usize + 1];
        let mut $name = $crate::mpfr::mpfr_t {
            prec: $prec as $crate::mpfr::prec_t,
            sign: 1,
            exp: 1 - $crate::mpfr::exp_t::MAX,
            d: unsafe { core::ptr::NonNull::new_unchecked(limbs[..].as_mut_ptr()).cast() },
        };
    };
}
extern "C" {
    /// See: [`mpfr_set_default_prec`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fdefault_005fprec)
    #[link_name = "mpfr_set_default_prec"]
    pub fn set_default_prec(prec: prec_t);
    /// See: [`mpfr_get_default_prec`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005fdefault_005fprec)
    #[link_name = "mpfr_get_default_prec"]
    pub fn get_default_prec() -> prec_t;
    /// See: [`mpfr_set_prec`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fprec)
    #[link_name = "mpfr_set_prec"]
    pub fn set_prec(x: mpfr_ptr, prec: prec_t);
}
/// See: [`mpfr_get_prec`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005fprec)
#[inline]
pub const unsafe extern "C" fn get_prec(x: mpfr_srcptr) -> prec_t {
    unsafe { (*x).prec }
}

// Assignment Functions

extern "C" {
    #[link_name = "mpfr_set4"]
    fn set4(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t, i: c_int) -> c_int;
}
/// See: [`mpfr_set`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset)
#[inline]
pub unsafe extern "C" fn set(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int {
    unsafe { set4(rop, op, rnd, (*op).sign) }
}
extern "C" {
    /// See: [`mpfr_set_ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fui)
    #[link_name = "mpfr_set_ui"]
    pub fn set_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_si`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fsi)
    #[link_name = "mpfr_set_si"]
    pub fn set_si(rop: mpfr_ptr, op: c_long, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_uj`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fuj)
    #[link_name = "__gmpfr_set_uj"]
    pub fn set_uj(rop: mpfr_ptr, op: uintmax_t, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_sj`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fsj)
    #[link_name = "__gmpfr_set_sj"]
    pub fn set_sj(rop: mpfr_ptr, op: intmax_t, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_flt`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fflt)
    #[link_name = "mpfr_set_flt"]
    pub fn set_flt(rop: mpfr_ptr, op: f32, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_d`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fd)
    #[link_name = "mpfr_set_d"]
    pub fn set_d(rop: mpfr_ptr, op: f64, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_z`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fz)
    #[link_name = "mpfr_set_z"]
    pub fn set_z(rop: mpfr_ptr, op: mpz_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_q`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fq)
    #[link_name = "mpfr_set_q"]
    pub fn set_q(rop: mpfr_ptr, op: mpq_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_f`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005ff)
    #[link_name = "mpfr_set_f"]
    pub fn set_f(rop: mpfr_ptr, op: mpf_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_ui_2exp`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fui_005f2exp)
    #[link_name = "mpfr_set_ui_2exp"]
    pub fn set_ui_2exp(rop: mpfr_ptr, op: c_ulong, e: exp_t, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_si_2exp`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fsi_005f2exp)
    #[link_name = "mpfr_set_si_2exp"]
    pub fn set_si_2exp(rop: mpfr_ptr, op: c_long, e: exp_t, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_uj_2exp`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fuj_005f2exp)
    #[link_name = "__gmpfr_set_uj_2exp"]
    pub fn set_uj_2exp(rop: mpfr_ptr, op: uintmax_t, e: intmax_t, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_sj_2exp`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fsj_005f2exp)
    #[link_name = "__gmpfr_set_sj_2exp"]
    pub fn set_sj_2exp(rop: mpfr_ptr, op: intmax_t, e: intmax_t, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_z_2exp`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fz_005f2exp)
    #[link_name = "mpfr_set_z_2exp"]
    pub fn set_z_2exp(rop: mpfr_ptr, op: mpz_srcptr, e: exp_t, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_str`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fstr)
    #[link_name = "mpfr_set_str"]
    pub fn set_str(rop: mpfr_ptr, s: *const c_char, base: c_int, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_strtofr`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fstrtofr)
    #[link_name = "mpfr_strtofr"]
    pub fn strtofr(
        rop: mpfr_ptr,
        nptr: *const c_char,
        endptr: *mut *mut c_char,
        base: c_int,
        rnd: rnd_t,
    ) -> c_int;
    /// See: [`mpfr_set_nan`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fnan)
    #[link_name = "mpfr_set_nan"]
    pub fn set_nan(x: mpfr_ptr);
    /// See: [`mpfr_set_inf`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005finf)
    #[link_name = "mpfr_set_inf"]
    pub fn set_inf(x: mpfr_ptr, sign: c_int);
    /// See: [`mpfr_set_zero`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fzero)
    #[link_name = "mpfr_set_zero"]
    pub fn set_zero(x: mpfr_ptr, sign: c_int);
    /// See: [`mpfr_swap`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fswap)
    #[link_name = "mpfr_swap"]
    pub fn swap(x: mpfr_ptr, y: mpfr_ptr);
}

// Combined Initialization and Assignment Functions

/// See: [`mpfr_init_set`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005finit_005fset)
#[inline]
pub unsafe extern "C" fn init_set(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int {
    unsafe {
        init(rop);
        set(rop, op, rnd)
    }
}
/// See: [`mpfr_init_set_ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005finit_005fset_005fui)
#[inline]
pub unsafe extern "C" fn init_set_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int {
    unsafe {
        init(rop);
        set_ui(rop, op, rnd)
    }
}
/// See: [`mpfr_init_set_si`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005finit_005fset_005fsi)
#[inline]
pub unsafe extern "C" fn init_set_si(rop: mpfr_ptr, op: c_long, rnd: rnd_t) -> c_int {
    unsafe {
        init(rop);
        set_si(rop, op, rnd)
    }
}
/// See: [`mpfr_init_set_d`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005finit_005fset_005fd)
#[inline]
pub unsafe extern "C" fn init_set_d(rop: mpfr_ptr, op: f64, rnd: rnd_t) -> c_int {
    unsafe {
        init(rop);
        set_d(rop, op, rnd)
    }
}
/// See: [`mpfr_init_set_z`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005finit_005fset_005fz)
#[inline]
pub unsafe extern "C" fn init_set_z(rop: mpfr_ptr, op: mpz_srcptr, rnd: rnd_t) -> c_int {
    unsafe {
        init(rop);
        set_z(rop, op, rnd)
    }
}
/// See: [`mpfr_init_set_q`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005finit_005fset_005fq)
#[inline]
pub unsafe extern "C" fn init_set_q(rop: mpfr_ptr, op: mpq_srcptr, rnd: rnd_t) -> c_int {
    unsafe {
        init(rop);
        set_q(rop, op, rnd)
    }
}
/// See: [`mpfr_init_set_f`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005finit_005fset_005ff)
#[inline]
pub unsafe extern "C" fn init_set_f(rop: mpfr_ptr, op: mpf_srcptr, rnd: rnd_t) -> c_int {
    unsafe {
        init(rop);
        set_f(rop, op, rnd)
    }
}
extern "C" {
    /// See: [`mpfr_init_set_str`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005finit_005fset_005fstr)
    #[link_name = "mpfr_init_set_str"]
    pub fn init_set_str(x: mpfr_ptr, s: *const c_char, base: c_int, rnd: rnd_t) -> c_int;

    // Conversion Functions

    /// See: [`mpfr_get_flt`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005fflt)
    #[link_name = "mpfr_get_flt"]
    pub fn get_flt(op: mpfr_srcptr, rnd: rnd_t) -> f32;
    /// See: [`mpfr_get_d`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005fd)
    #[link_name = "mpfr_get_d"]
    pub fn get_d(op: mpfr_srcptr, rnd: rnd_t) -> f64;
    /// See: [`mpfr_get_si`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005fsi)
    #[link_name = "mpfr_get_si"]
    pub fn get_si(op: mpfr_srcptr, rnd: rnd_t) -> c_long;
    /// See: [`mpfr_get_ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005fui)
    #[link_name = "mpfr_get_ui"]
    pub fn get_ui(op: mpfr_srcptr, rnd: rnd_t) -> c_ulong;
    /// See: [`mpfr_get_sj`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005fsj)
    #[link_name = "__gmpfr_mpfr_get_sj"]
    pub fn get_sj(op: mpfr_srcptr, rnd: rnd_t) -> intmax_t;
    /// See: [`mpfr_get_uj`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005fuj)
    #[link_name = "__gmpfr_mpfr_get_uj"]
    pub fn get_uj(op: mpfr_srcptr, rnd: rnd_t) -> uintmax_t;
    /// See: [`mpfr_get_d_2exp`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005fd_005f2exp)
    #[link_name = "mpfr_get_d_2exp"]
    pub fn get_d_2exp(exp: *mut c_long, op: mpfr_srcptr, rnd: rnd_t) -> f64;
    /// See: [`mpfr_frexp`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffrexp)
    #[link_name = "mpfr_frexp"]
    pub fn frexp(exp: *mut exp_t, y: mpfr_ptr, x: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_get_z_2exp`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005fz_005f2exp)
    #[link_name = "mpfr_get_z_2exp"]
    pub fn get_z_2exp(rop: mpz_ptr, op: mpfr_srcptr) -> exp_t;
    /// See: [`mpfr_get_z`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005fz)
    #[link_name = "mpfr_get_z"]
    pub fn get_z(z: mpz_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_get_q`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005fq)
    #[link_name = "mpfr_get_q"]
    pub fn get_q(rop: mpq_ptr, op: mpfr_srcptr);
    /// See: [`mpfr_get_f`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005ff)
    #[link_name = "mpfr_get_f"]
    pub fn get_f(rop: mpf_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_get_str_ndigits`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005fstr_005fndigits)
    #[link_name = "mpfr_get_str_ndigits"]
    pub fn get_str_ndigits(b: c_int, p: prec_t) -> usize;
    /// See: [`mpfr_get_str`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005fstr)
    #[link_name = "mpfr_get_str"]
    pub fn get_str(
        str: *mut c_char,
        expptr: *mut exp_t,
        base: c_int,
        n: usize,
        op: mpfr_srcptr,
        rnd: rnd_t,
    ) -> *mut c_char;
    /// See: [`mpfr_free_str`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffree_005fstr)
    #[link_name = "mpfr_free_str"]
    pub fn free_str(str: *mut c_char);
    /// See: [`mpfr_fits_ulong_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffits_005fulong_005fp)
    #[link_name = "mpfr_fits_ulong_p"]
    pub fn fits_ulong_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fits_slong_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffits_005fslong_005fp)
    #[link_name = "mpfr_fits_slong_p"]
    pub fn fits_slong_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fits_uint_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffits_005fuint_005fp)
    #[link_name = "mpfr_fits_uint_p"]
    pub fn fits_uint_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fits_sint_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffits_005fsint_005fp)
    #[link_name = "mpfr_fits_sint_p"]
    pub fn fits_sint_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fits_ushort_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffits_005fushort_005fp)
    #[link_name = "mpfr_fits_ushort_p"]
    pub fn fits_ushort_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fits_sshort_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffits_005fsshort_005fp)
    #[link_name = "mpfr_fits_sshort_p"]
    pub fn fits_sshort_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fits_uintmax_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffits_005fuintmax_005fp)
    #[link_name = "mpfr_fits_uintmax_p"]
    pub fn fits_uintmax_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fits_intmax_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffits_005fintmax_005fp)
    #[link_name = "mpfr_fits_intmax_p"]
    pub fn fits_intmax_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;

    // Arithmetic Functions

    /// See: [`mpfr_add`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fadd)
    #[link_name = "mpfr_add"]
    pub fn add(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_add_ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fadd_005fui)
    #[link_name = "mpfr_add_ui"]
    pub fn add_ui(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_add_si`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fadd_005fsi)
    #[link_name = "mpfr_add_si"]
    pub fn add_si(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_long, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_add_d`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fadd_005fd)
    #[link_name = "mpfr_add_d"]
    pub fn add_d(rop: mpfr_ptr, op1: mpfr_srcptr, op2: f64, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_add_z`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fadd_005fz)
    #[link_name = "mpfr_add_z"]
    pub fn add_z(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpz_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_add_q`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fadd_005fq)
    #[link_name = "mpfr_add_q"]
    pub fn add_q(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpq_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sub`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsub)
    #[link_name = "mpfr_sub"]
    pub fn sub(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_ui_sub`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fui_005fsub)
    #[link_name = "mpfr_ui_sub"]
    pub fn ui_sub(rop: mpfr_ptr, op1: c_ulong, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sub_ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsub_005fui)
    #[link_name = "mpfr_sub_ui"]
    pub fn sub_ui(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_si_sub`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsi_005fsub)
    #[link_name = "mpfr_si_sub"]
    pub fn si_sub(rop: mpfr_ptr, op1: c_long, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sub_si`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsub_005fsi)
    #[link_name = "mpfr_sub_si"]
    pub fn sub_si(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_long, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_d_sub`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fd_005fsub)
    #[link_name = "mpfr_d_sub"]
    pub fn d_sub(rop: mpfr_ptr, op1: f64, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sub_d`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsub_005fd)
    #[link_name = "mpfr_sub_d"]
    pub fn sub_d(rop: mpfr_ptr, op1: mpfr_srcptr, op2: f64, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_z_sub`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fz_005fsub)
    #[link_name = "mpfr_z_sub"]
    pub fn z_sub(rop: mpfr_ptr, op1: mpz_srcptr, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sub_z`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsub_005fz)
    #[link_name = "mpfr_sub_z"]
    pub fn sub_z(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpz_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sub_q`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsub_005fq)
    #[link_name = "mpfr_sub_q"]
    pub fn sub_q(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpq_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_mul`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fmul)
    #[link_name = "mpfr_mul"]
    pub fn mul(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_mul_ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fmul_005fui)
    #[link_name = "mpfr_mul_ui"]
    pub fn mul_ui(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_mul_si`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fmul_005fsi)
    #[link_name = "mpfr_mul_si"]
    pub fn mul_si(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_long, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_mul_d`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fmul_005fd)
    #[link_name = "mpfr_mul_d"]
    pub fn mul_d(rop: mpfr_ptr, op1: mpfr_srcptr, op2: f64, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_mul_z`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fmul_005fz)
    #[link_name = "mpfr_mul_z"]
    pub fn mul_z(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpz_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_mul_q`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fmul_005fq)
    #[link_name = "mpfr_mul_q"]
    pub fn mul_q(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpq_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sqr`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsqr)
    #[link_name = "mpfr_sqr"]
    pub fn sqr(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_div`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fdiv)
    #[link_name = "mpfr_div"]
    pub fn div(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_ui_div`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fui_005fdiv)
    #[link_name = "mpfr_ui_div"]
    pub fn ui_div(rop: mpfr_ptr, op1: c_ulong, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_div_ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fdiv_005fui)
    #[link_name = "mpfr_div_ui"]
    pub fn div_ui(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_si_div`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsi_005fdiv)
    #[link_name = "mpfr_si_div"]
    pub fn si_div(rop: mpfr_ptr, op1: c_long, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_div_si`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fdiv_005fsi)
    #[link_name = "mpfr_div_si"]
    pub fn div_si(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_long, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_d_div`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fd_005fdiv)
    #[link_name = "mpfr_d_div"]
    pub fn d_div(rop: mpfr_ptr, op1: f64, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_div_d`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fdiv_005fd)
    #[link_name = "mpfr_div_d"]
    pub fn div_d(rop: mpfr_ptr, op1: mpfr_srcptr, op2: f64, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_div_z`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fdiv_005fz)
    #[link_name = "mpfr_div_z"]
    pub fn div_z(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpz_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_div_q`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fdiv_005fq)
    #[link_name = "mpfr_div_q"]
    pub fn div_q(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpq_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sqrt`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsqrt)
    #[link_name = "mpfr_sqrt"]
    pub fn sqrt(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sqrt_ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsqrt_005fui)
    #[link_name = "mpfr_sqrt_ui"]
    pub fn sqrt_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_rec_sqrt`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005frec_005fsqrt)
    #[link_name = "mpfr_rec_sqrt"]
    pub fn rec_sqrt(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_cbrt`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcbrt)
    #[link_name = "mpfr_cbrt"]
    pub fn cbrt(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_rootn_ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005frootn_005fui)
    #[link_name = "mpfr_rootn_ui"]
    pub fn rootn_ui(rop: mpfr_ptr, op: mpfr_srcptr, n: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_rootn_si`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005frootn_005fsi)
    #[link_name = "mpfr_rootn_si"]
    pub fn rootn_si(rop: mpfr_ptr, op: mpfr_srcptr, n: c_long, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_root`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005froot)
    #[link_name = "mpfr_root"]
    #[deprecated(
        since = "1.1.0",
        note = "replaced by the slightly different `rootn_ui`"
    )]
    pub fn root(rop: mpfr_ptr, op: mpfr_srcptr, n: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_neg`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fneg)
    #[link_name = "mpfr_neg"]
    pub fn neg(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
}
/// See: [`mpfr_abs`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fabs)
#[inline]
pub unsafe extern "C" fn abs(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int {
    unsafe { set4(rop, op, rnd, 1) }
}
extern "C" {
    /// See: [`mpfr_dim`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fdim)
    #[link_name = "mpfr_dim"]
    pub fn dim(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_mul_2ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fmul_005f2ui)
    #[link_name = "mpfr_mul_2ui"]
    pub fn mul_2ui(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_mul_2si`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fmul_005f2si)
    #[link_name = "mpfr_mul_2si"]
    pub fn mul_2si(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_long, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_div_2ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fdiv_005f2ui)
    #[link_name = "mpfr_div_2ui"]
    pub fn div_2ui(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_div_2si`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fdiv_005f2si)
    #[link_name = "mpfr_div_2si"]
    pub fn div_2si(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_long, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fac_ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffac_005fui)
    #[link_name = "mpfr_fac_ui"]
    pub fn fac_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fma`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffma)
    #[link_name = "mpfr_fma"]
    pub fn fma(
        rop: mpfr_ptr,
        op1: mpfr_srcptr,
        op2: mpfr_srcptr,
        op3: mpfr_srcptr,
        rnd: rnd_t,
    ) -> c_int;
    /// See: [`mpfr_fms`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffms)
    #[link_name = "mpfr_fms"]
    pub fn fms(
        rop: mpfr_ptr,
        op1: mpfr_srcptr,
        op2: mpfr_srcptr,
        op3: mpfr_srcptr,
        rnd: rnd_t,
    ) -> c_int;
    /// See: [`mpfr_fmma`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffmma)
    #[link_name = "mpfr_fmma"]
    pub fn fmma(
        rop: mpfr_ptr,
        op1: mpfr_srcptr,
        op2: mpfr_srcptr,
        op3: mpfr_srcptr,
        op4: mpfr_srcptr,
        rnd: rnd_t,
    ) -> c_int;
    /// See: [`mpfr_fmms`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffmms)
    #[link_name = "mpfr_fmms"]
    pub fn fmms(
        rop: mpfr_ptr,
        op1: mpfr_srcptr,
        op2: mpfr_srcptr,
        op3: mpfr_srcptr,
        op4: mpfr_srcptr,
        rnd: rnd_t,
    ) -> c_int;
    /// See: [`mpfr_hypot`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fhypot)
    #[link_name = "mpfr_hypot"]
    pub fn hypot(rop: mpfr_ptr, x: mpfr_srcptr, y: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sum`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsum)
    #[link_name = "mpfr_sum"]
    pub fn sum(rop: mpfr_ptr, tab: *const mpfr_ptr, n: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_dot`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fdot)
    #[link_name = "mpfr_dot"]
    pub fn dot(
        rop: mpfr_ptr,
        a: *const mpfr_ptr,
        b: *const mpfr_ptr,
        n: c_ulong,
        rnd: rnd_t,
    ) -> c_int;
}

// Comparison Functions

extern "C" {
    #[link_name = "mpfr_cmp3"]
    fn cmp3(op1: mpfr_srcptr, op2: mpfr_srcptr, i: c_int) -> c_int;
}
/// See: [`mpfr_cmp`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcmp)
#[inline]
pub unsafe extern "C" fn cmp(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int {
    unsafe { cmp3(op1, op2, 1) }
}
/// See: [`mpfr_cmp_ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcmp_005fui)
#[inline]
pub unsafe extern "C" fn cmp_ui(op1: mpfr_srcptr, op2: c_ulong) -> c_int {
    unsafe { cmp_ui_2exp(op1, op2, 0) }
}
/// See: [`mpfr_cmp_si`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcmp_005fsi)
#[inline]
pub unsafe extern "C" fn cmp_si(op1: mpfr_srcptr, op2: c_long) -> c_int {
    unsafe { cmp_si_2exp(op1, op2, 0) }
}
extern "C" {
    /// See: [`mpfr_cmp_d`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcmp_005fd)
    #[link_name = "mpfr_cmp_d"]
    pub fn cmp_d(op1: mpfr_srcptr, op2: f64) -> c_int;
    /// See: [`mpfr_cmp_z`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcmp_005fz)
    #[link_name = "mpfr_cmp_z"]
    pub fn cmp_z(op1: mpfr_srcptr, op2: mpz_srcptr) -> c_int;
    /// See: [`mpfr_cmp_q`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcmp_005fq)
    #[link_name = "mpfr_cmp_q"]
    pub fn cmp_q(op1: mpfr_srcptr, op2: mpq_srcptr) -> c_int;
    /// See: [`mpfr_cmp_f`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcmp_005ff)
    #[link_name = "mpfr_cmp_f"]
    pub fn cmp_f(op1: mpfr_srcptr, op2: mpf_srcptr) -> c_int;
    /// See: [`mpfr_cmp_ui_2exp`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcmp_005fui_005f2exp)
    #[link_name = "mpfr_cmp_ui_2exp"]
    pub fn cmp_ui_2exp(op1: mpfr_srcptr, op2: c_ulong, e: exp_t) -> c_int;
    /// See: [`mpfr_cmp_si_2exp`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcmp_005fsi_005f2exp)
    #[link_name = "mpfr_cmp_si_2exp"]
    pub fn cmp_si_2exp(op1: mpfr_srcptr, op2: c_long, e: exp_t) -> c_int;
    /// See: [`mpfr_cmpabs`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcmpabs)
    #[link_name = "mpfr_cmpabs"]
    pub fn cmpabs(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_cmpabs_ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcmpabs_005fui)
    #[link_name = "mpfr_cmpabs_ui"]
    pub fn cmpabs_ui(op1: mpfr_srcptr, op2: c_ulong) -> c_int;
}
/// See: [`mpfr_nan_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fnan_005fp)
#[inline]
pub const unsafe extern "C" fn nan_p(op: mpfr_srcptr) -> c_int {
    (unsafe { (*op).exp } == EXP_NAN) as c_int
}
/// See: [`mpfr_inf_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005finf_005fp)
#[inline]
pub const unsafe extern "C" fn inf_p(op: mpfr_srcptr) -> c_int {
    (unsafe { (*op).exp } == EXP_INF) as c_int
}
/// See: [`mpfr_number_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fnumber_005fp)
#[inline]
pub const unsafe extern "C" fn number_p(op: mpfr_srcptr) -> c_int {
    // Note: mpfr_number_p is not a macro in mpfr.h, but in isnum.c it simply returns
    // MPFR_IS_FP(x), which is a macro defined in mpfr-impl.h as not nan and not inf
    let exp = unsafe { (*op).exp };
    (exp != EXP_NAN && exp != EXP_INF) as c_int
}
/// See: [`mpfr_zero_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fzero_005fp)
#[inline]
pub const unsafe extern "C" fn zero_p(op: mpfr_srcptr) -> c_int {
    (unsafe { (*op).exp } == EXP_ZERO) as c_int
}
/// See: [`mpfr_regular_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fregular_005fp)
#[inline]
pub const unsafe extern "C" fn regular_p(op: mpfr_srcptr) -> c_int {
    (unsafe { (*op).exp } > EXP_INF) as c_int
}
/// See: [`mpfr_sgn`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsgn)
#[inline]
pub unsafe extern "C" fn sgn(op: mpfr_srcptr) -> c_int {
    if unsafe { (*op).exp } < EXP_INF {
        unsafe {
            if nan_p(op) != 0 {
                set_erangeflag();
            }
        }
        0
    } else {
        unsafe { (*op).sign }
    }
}
extern "C" {
    /// See: [`mpfr_greater_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fgreater_005fp)
    #[link_name = "mpfr_greater_p"]
    pub fn greater_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_greaterequal_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fgreaterequal_005fp)
    #[link_name = "mpfr_greaterequal_p"]
    pub fn greaterequal_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_less_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fless_005fp)
    #[link_name = "mpfr_less_p"]
    pub fn less_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_lessequal_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005flessequal_005fp)
    #[link_name = "mpfr_lessequal_p"]
    pub fn lessequal_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_equal_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fequal_005fp)
    #[link_name = "mpfr_equal_p"]
    pub fn equal_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_lessgreater_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005flessgreater_005fp)
    #[link_name = "mpfr_lessgreater_p"]
    pub fn lessgreater_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_unordered_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005funordered_005fp)
    #[link_name = "mpfr_unordered_p"]
    pub fn unordered_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_total_order_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ftotal_005forder_005fp)
    #[link_name = "mpfr_total_order_p"]
    pub fn total_order_p(x: mpfr_srcptr, y: mpfr_srcptr) -> c_int;

    // Transcendental Functions

    /// See: [`mpfr_log`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005flog)
    #[link_name = "mpfr_log"]
    pub fn log(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_log_ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005flog_005fui)
    #[link_name = "mpfr_log_ui"]
    pub fn log_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_log2`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005flog2)
    #[link_name = "mpfr_log2"]
    pub fn log2(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_log10`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005flog10)
    #[link_name = "mpfr_log10"]
    pub fn log10(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_log1p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005flog1p)
    #[link_name = "mpfr_log1p"]
    pub fn log1p(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_log2p1`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005flog2p1)
    #[link_name = "mpfr_log2p1"]
    pub fn log2p1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_log10p1`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005flog10p1)
    #[link_name = "mpfr_log10p1"]
    pub fn log10p1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_exp`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fexp)
    #[link_name = "mpfr_exp"]
    pub fn exp(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_exp2`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fexp2)
    #[link_name = "mpfr_exp2"]
    pub fn exp2(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_exp10`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fexp10)
    #[link_name = "mpfr_exp10"]
    pub fn exp10(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_expm1`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fexpm1)
    #[link_name = "mpfr_expm1"]
    pub fn expm1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_exp2m1`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fexp2m1)
    #[link_name = "mpfr_exp2m1"]
    pub fn exp2m1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_exp10m1`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fexp10m1)
    #[link_name = "mpfr_exp10m1"]
    pub fn exp10m1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_pow`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fpow)
    #[link_name = "mpfr_pow"]
    pub fn pow(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_powr`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fpowr)
    #[link_name = "mpfr_powr"]
    pub fn powr(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_pow_ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fpow_005fui)
    #[link_name = "mpfr_pow_ui"]
    pub fn pow_ui(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_pow_si`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fpow_005fsi)
    #[link_name = "mpfr_pow_si"]
    pub fn pow_si(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_long, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_pow_uj`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fpow_005fuj)
    #[link_name = "__gmpfr_mpfr_pow_uj"]
    pub fn pow_uj(rop: mpfr_ptr, op1: mpfr_srcptr, op2: uintmax_t, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_pow_sj`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fpow_005fsj)
    #[link_name = "__gmpfr_mpfr_pow_sj"]
    pub fn pow_sj(rop: mpfr_ptr, op1: mpfr_srcptr, op2: intmax_t, rnd: rnd_t) -> c_int;
}
/// See: [`mpfr_pown`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fpown)
#[inline]
pub unsafe extern "C" fn pown(rop: mpfr_ptr, op1: mpfr_srcptr, op2: intmax_t, rnd: rnd_t) -> c_int {
    unsafe { pow_sj(rop, op1, op2, rnd) }
}
extern "C" {
    /// See: [`mpfr_pow_z`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fpow_005fz)
    #[link_name = "mpfr_pow_z"]
    pub fn pow_z(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpz_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_ui_pow_ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fui_005fpow_005fui)
    #[link_name = "mpfr_ui_pow_ui"]
    pub fn ui_pow_ui(rop: mpfr_ptr, op1: c_ulong, op2: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_ui_pow`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fui_005fpow)
    #[link_name = "mpfr_ui_pow"]
    pub fn ui_pow(rop: mpfr_ptr, op1: c_ulong, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    //int mpfr_compound_si (mpfr_t rop, mpfr_t op, long int n, mpfr_rnd_t rnd)
    /// See: [`mpfr_compound_si`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcompound_005fsi)
    #[link_name = "mpfr_compound_si"]
    pub fn compound_si(rop: mpfr_ptr, op: mpfr_srcptr, n: c_long, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_cos`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcos)
    #[link_name = "mpfr_cos"]
    pub fn cos(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sin`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsin)
    #[link_name = "mpfr_sin"]
    pub fn sin(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_tan`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ftan)
    #[link_name = "mpfr_tan"]
    pub fn tan(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_cosu`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcosu)
    #[link_name = "mpfr_cosu"]
    pub fn cosu(rop: mpfr_ptr, op: mpfr_srcptr, u: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sinu`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsinu)
    #[link_name = "mpfr_sinu"]
    pub fn sinu(rop: mpfr_ptr, op: mpfr_srcptr, u: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_tanu`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ftanu)
    #[link_name = "mpfr_tanu"]
    pub fn tanu(rop: mpfr_ptr, op: mpfr_srcptr, u: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_cospi`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcospi)
    #[link_name = "mpfr_cospi"]
    pub fn cospi(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sinpi`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsinpi)
    #[link_name = "mpfr_sinpi"]
    pub fn sinpi(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_tanpi`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ftanpi)
    #[link_name = "mpfr_tanpi"]
    pub fn tanpi(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sin_cos`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsin_005fcos)
    #[link_name = "mpfr_sin_cos"]
    pub fn sin_cos(sop: mpfr_ptr, cop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sec`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsec)
    #[link_name = "mpfr_sec"]
    pub fn sec(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_csc`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcsc)
    #[link_name = "mpfr_csc"]
    pub fn csc(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_cot`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcot)
    #[link_name = "mpfr_cot"]
    pub fn cot(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_acos`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005facos)
    #[link_name = "mpfr_acos"]
    pub fn acos(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_asin`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fasin)
    #[link_name = "mpfr_asin"]
    pub fn asin(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_atan`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fatan)
    #[link_name = "mpfr_atan"]
    pub fn atan(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_acosu`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005facosu)
    #[link_name = "mpfr_acosu"]
    pub fn acosu(rop: mpfr_ptr, op: mpfr_srcptr, u: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_asinu`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fasinu)
    #[link_name = "mpfr_asinu"]
    pub fn asinu(rop: mpfr_ptr, op: mpfr_srcptr, u: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_atanu`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fatanu)
    #[link_name = "mpfr_atanu"]
    pub fn atanu(rop: mpfr_ptr, op: mpfr_srcptr, u: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_acospi`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005facospi)
    #[link_name = "mpfr_acospi"]
    pub fn acospi(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_asinpi`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fasinpi)
    #[link_name = "mpfr_asinpi"]
    pub fn asinpi(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_atanpi`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fatanpi)
    #[link_name = "mpfr_atanpi"]
    pub fn atanpi(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_atan2`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fatan2)
    #[link_name = "mpfr_atan2"]
    pub fn atan2(rop: mpfr_ptr, y: mpfr_srcptr, x: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_atan2u`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fatan2u)
    #[link_name = "mpfr_atan2u"]
    pub fn atan2u(rop: mpfr_ptr, y: mpfr_srcptr, x: mpfr_srcptr, u: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_atan2pi`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fatan2pi)
    #[link_name = "mpfr_atan2pi"]
    pub fn atan2pi(rop: mpfr_ptr, y: mpfr_srcptr, x: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_cosh`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcosh)
    #[link_name = "mpfr_cosh"]
    pub fn cosh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sinh`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsinh)
    #[link_name = "mpfr_sinh"]
    pub fn sinh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_tanh`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ftanh)
    #[link_name = "mpfr_tanh"]
    pub fn tanh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sinh_cosh`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsinh_005fcosh)
    #[link_name = "mpfr_sinh_cosh"]
    pub fn sinh_cosh(sop: mpfr_ptr, cop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sech`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsech)
    #[link_name = "mpfr_sech"]
    pub fn sech(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_csch`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcsch)
    #[link_name = "mpfr_csch"]
    pub fn csch(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_coth`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcoth)
    #[link_name = "mpfr_coth"]
    pub fn coth(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_acosh`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005facosh)
    #[link_name = "mpfr_acosh"]
    pub fn acosh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_asinh`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fasinh)
    #[link_name = "mpfr_asinh"]
    pub fn asinh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_atanh`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fatanh)
    #[link_name = "mpfr_atanh"]
    pub fn atanh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_eint`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005feint)
    #[link_name = "mpfr_eint"]
    pub fn eint(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_li2`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fli2)
    #[link_name = "mpfr_li2"]
    pub fn li2(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_gamma`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fgamma)
    #[link_name = "mpfr_gamma"]
    pub fn gamma(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_gamma_inc`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fgamma_005finc)
    #[link_name = "mpfr_gamma_inc"]
    pub fn gamma_inc(rop: mpfr_ptr, op: mpfr_srcptr, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_lngamma`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005flngamma)
    #[link_name = "mpfr_lngamma"]
    pub fn lngamma(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_lgamma`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005flgamma)
    #[link_name = "mpfr_lgamma"]
    pub fn lgamma(rop: mpfr_ptr, signp: *mut c_int, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_digamma`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fdigamma)
    #[link_name = "mpfr_digamma"]
    pub fn digamma(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_beta`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fbeta)
    #[link_name = "mpfr_beta"]
    pub fn beta(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_zeta`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fzeta)
    #[link_name = "mpfr_zeta"]
    pub fn zeta(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_zeta_ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fzeta_005fui)
    #[link_name = "mpfr_zeta_ui"]
    pub fn zeta_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_erf`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ferf)
    #[link_name = "mpfr_erf"]
    pub fn erf(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_erfc`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ferfc)
    #[link_name = "mpfr_erfc"]
    pub fn erfc(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_j0`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fj0)
    #[link_name = "mpfr_j0"]
    pub fn j0(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_j1`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fj1)
    #[link_name = "mpfr_j1"]
    pub fn j1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_jn`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fjn)
    #[link_name = "mpfr_jn"]
    pub fn jn(rop: mpfr_ptr, n: c_long, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_y0`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fy0)
    #[link_name = "mpfr_y0"]
    pub fn y0(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_y1`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fy1)
    #[link_name = "mpfr_y1"]
    pub fn y1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_yn`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fyn)
    #[link_name = "mpfr_yn"]
    pub fn yn(rop: mpfr_ptr, n: c_long, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_agm`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fagm)
    #[link_name = "mpfr_agm"]
    pub fn agm(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_ai`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fai)
    #[link_name = "mpfr_ai"]
    pub fn ai(rop: mpfr_ptr, x: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_const_log2`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fconst_005flog2)
    #[link_name = "mpfr_const_log2"]
    pub fn const_log2(rop: mpfr_ptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_const_pi`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fconst_005fpi)
    #[link_name = "mpfr_const_pi"]
    pub fn const_pi(rop: mpfr_ptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_const_euler`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fconst_005feuler)
    #[link_name = "mpfr_const_euler"]
    pub fn const_euler(rop: mpfr_ptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_const_catalan`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fconst_005fcatalan)
    #[link_name = "mpfr_const_catalan"]
    pub fn const_catalan(rop: mpfr_ptr, rnd: rnd_t) -> c_int;

    // Input and Output Functions

    /// See: [`mpfr_out_str`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fout_005fstr)
    #[link_name = "__gmpfr_out_str"]
    pub fn out_str(stream: *mut FILE, base: c_int, n: usize, op: mpfr_srcptr, rnd: rnd_t) -> usize;
    /// See: [`mpfr_inp_str`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005finp_005fstr)
    #[link_name = "__gmpfr_inp_str"]
    pub fn inp_str(rop: mpfr_ptr, stream: *mut FILE, base: c_int, rnd: rnd_t) -> usize;
    /// See: [`mpfr_fpif_export`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffpip_005fexport)
    #[link_name = "__gmpfr_fpif_export"]
    pub fn fpif_export(stream: *mut FILE, op: mpfr_ptr) -> c_int;
    /// See: [`mpfr_fpif_import`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffpip_005fimport)
    #[link_name = "__gmpfr_fpif_import"]
    pub fn fpif_import(op: mpfr_ptr, stream: *mut FILE) -> c_int;
    /// See: [`mpfr_dump`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fdump)
    #[link_name = "mpfr_dump"]
    pub fn dump(op: mpfr_srcptr);

    // Formatted Output Functions

    /// See: [`mpfr_fprintf`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffprintf)
    #[link_name = "__gmpfr_fprintf"]
    pub fn fprintf(stream: *mut FILE, template: *const c_char, ...) -> c_int;
    /// See: [`mpfr_printf`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fprintf)
    #[link_name = "mpfr_printf"]
    pub fn printf(template: *const c_char, ...) -> c_int;
    /// See: [`mpfr_sprintf`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsprintf)
    #[link_name = "mpfr_sprintf"]
    pub fn sprintf(buf: *mut c_char, template: *const c_char, ...) -> c_int;
    /// See: [`mpfr_snprintf`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsnprintf)
    #[link_name = "mpfr_snprintf"]
    pub fn snprintf(buf: *mut c_char, n: usize, template: *const c_char, ...) -> c_int;
    /// See: [`mpfr_asprintf`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fasprintf)
    #[link_name = "mpfr_asprintf"]
    pub fn asprintf(str: *mut *mut c_char, template: *const c_char, ...) -> c_int;

    // Integer and Remainder Related Functions

    /// See: [`mpfr_rint`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005frint)
    #[link_name = "mpfr_rint"]
    pub fn rint(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
}
/// See: [`mpfr_ceil`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fceil)
#[inline]
pub unsafe extern "C" fn ceil(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int {
    unsafe { rint(rop, op, rnd_t::RNDU) }
}
/// See: [`mpfr_floor`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffloor)
#[inline]
pub unsafe extern "C" fn floor(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int {
    unsafe { rint(rop, op, rnd_t::RNDD) }
}
/// See: [`mpfr_round`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fround)
#[inline]
pub unsafe extern "C" fn round(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int {
    #[allow(deprecated)]
    unsafe {
        rint(rop, op, rnd_t::RNDNA)
    }
}
extern "C" {
    /// See: [`mpfr_roundeven`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005froundeven)
    #[link_name = "mpfr_roundeven"]
    pub fn roundeven(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
}
/// See: [`mpfr_trunc`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ftrunc)
#[inline]
pub unsafe extern "C" fn trunc(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int {
    unsafe { rint(rop, op, rnd_t::RNDZ) }
}
extern "C" {
    /// See: [`mpfr_rint_ceil`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005frint_005fceil)
    #[link_name = "mpfr_rint_ceil"]
    pub fn rint_ceil(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_rint_floor`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005frint_005ffloor)
    #[link_name = "mpfr_rint_floor"]
    pub fn rint_floor(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_rint_round`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005frint_005fround)
    #[link_name = "mpfr_rint_round"]
    pub fn rint_round(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_rint_roundeven`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005frint_005froundeven)
    #[link_name = "mpfr_rint_roundeven"]
    pub fn rint_roundeven(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_rint_trunc`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005frint_005ftrunc)
    #[link_name = "mpfr_rint_trunc"]
    pub fn rint_trunc(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_frac`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffrac)
    #[link_name = "mpfr_frac"]
    pub fn frac(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_modf`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fmodf)
    #[link_name = "mpfr_modf"]
    pub fn modf(iop: mpfr_ptr, fop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fmod`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffmod)
    #[link_name = "mpfr_fmod"]
    pub fn fmod(r: mpfr_ptr, x: mpfr_srcptr, y: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fmod_ui`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffmod_005fui)
    #[link_name = "mpfr_fmod_ui"]
    pub fn fmod_ui(r: mpfr_ptr, x: mpfr_srcptr, y: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fmodquo`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffmodquo)
    #[link_name = "mpfr_fmodquo"]
    pub fn fmodquo(
        r: mpfr_ptr,
        q: *mut c_long,
        x: mpfr_srcptr,
        y: mpfr_srcptr,
        rnd: rnd_t,
    ) -> c_int;
    /// See: [`mpfr_remainder`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fremainder)
    #[link_name = "mpfr_remainder"]
    pub fn remainder(r: mpfr_ptr, x: mpfr_srcptr, y: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_remquo`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fremquo)
    #[link_name = "mpfr_remquo"]
    pub fn remquo(r: mpfr_ptr, q: *mut c_long, x: mpfr_srcptr, y: mpfr_srcptr, rnd: rnd_t)
        -> c_int;
    /// See: [`mpfr_integer_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005finteger_005fp)
    #[link_name = "mpfr_integer_p"]
    pub fn integer_p(op: mpfr_srcptr) -> c_int;

    // Rounding Related Functions

    /// See: [`mpfr_set_default_rounding_mode`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fdefault_005frounding_005fmode)
    #[link_name = "mpfr_set_default_rounding_mode"]
    pub fn set_default_rounding_mode(rnd: rnd_t);
    /// See: [`mpfr_get_default_rounding_mode`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005fdefault_005frounding_005fmode)
    #[link_name = "mpfr_get_default_rounding_mode"]
    pub fn get_default_rounding_mode() -> rnd_t;
    /// See: [`mpfr_prec_round`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fprec_005fround)
    #[link_name = "mpfr_prec_round"]
    pub fn prec_round(x: mpfr_ptr, prec: prec_t, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_can_round`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcan_005fround)
    #[link_name = "mpfr_can_round"]
    pub fn can_round(b: mpfr_srcptr, err: exp_t, rnd1: rnd_t, rnd2: rnd_t, prec: prec_t) -> c_int;
    /// See: [`mpfr_min_prec`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fmin_005fprec)
    #[link_name = "mpfr_min_prec"]
    pub fn min_prec(x: mpfr_srcptr) -> prec_t;
    /// See: [`mpfr_print_rnd_mode`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fprint_005frnd_005fmode)
    #[link_name = "mpfr_print_rnd_mode"]
    pub fn print_rnd_mode(rnd: rnd_t) -> *const c_char;
}
// macro will be exported at top level, so link to C/MPFR... not to ../C/MPFR...
/// See: [`mpfr_round_nearest_away`](C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fround_005fnearest_005faway)
#[macro_export]
macro_rules! mpfr_round_nearest_away {
    ($foo:expr, $rop:expr $(, $op:expr)*) => {{
        use core::ffi::c_int;
        type mpfr_ptr = *mut $crate::mpfr::mpfr_t;
        let rop: mpfr_ptr = $rop;
        extern "C" {
            fn mpfr_round_nearest_away_begin(rop: mpfr_ptr);
            fn mpfr_round_nearest_away_end(rop: mpfr_ptr, inex: c_int) -> c_int;
        }
        mpfr_round_nearest_away_begin(rop);
        mpfr_round_nearest_away_end(
            rop,
            $foo(rop $(, $op)*, $crate::mpfr::rnd_t::RNDN),
        )
    }};
}

extern "C" {
    // Miscellaneous Functions

    /// See: [`mpfr_nexttoward`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fnexttoward)
    #[link_name = "mpfr_nexttoward"]
    pub fn nexttoward(x: mpfr_ptr, y: mpfr_srcptr);
    /// See: [`mpfr_nextabove`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fnextabove)
    #[link_name = "mpfr_nextabove"]
    pub fn nextabove(x: mpfr_ptr);
    /// See: [`mpfr_nextbelow`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fnextbelow)
    #[link_name = "mpfr_nextbelow"]
    pub fn nextbelow(x: mpfr_ptr);
    /// See: [`mpfr_min`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fmin)
    #[link_name = "mpfr_min"]
    pub fn min(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_max`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fmax)
    #[link_name = "mpfr_max"]
    pub fn max(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_urandomb`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005furandomb)
    #[link_name = "mpfr_urandomb"]
    pub fn urandomb(rop: mpfr_ptr, state: randstate_ptr) -> c_int;
    /// See: [`mpfr_urandom`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005furandom)
    #[link_name = "mpfr_urandom"]
    pub fn urandom(rop: mpfr_ptr, state: randstate_ptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_nrandom`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fnrandom)
    #[link_name = "mpfr_nrandom"]
    pub fn nrandom(rop1: mpfr_ptr, state: randstate_ptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_grandom`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fgrandom)
    #[link_name = "mpfr_grandom"]
    #[deprecated(since = "1.1.0", note = "replaced by `nrandom`")]
    pub fn grandom(rop1: mpfr_ptr, rop2: mpfr_ptr, state: randstate_ptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_erandom`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ferandom)
    #[link_name = "mpfr_erandom"]
    pub fn erandom(rop1: mpfr_ptr, state: randstate_ptr, rnd: rnd_t) -> c_int;
}
/// See: [`mpfr_get_exp`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005fexp)
#[inline]
pub const unsafe extern "C" fn get_exp(x: mpfr_srcptr) -> exp_t {
    unsafe { (*x).exp }
}
extern "C" {
    /// See: [`mpfr_set_exp`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fexp)
    #[link_name = "mpfr_set_exp"]
    pub fn set_exp(x: mpfr_ptr, e: exp_t) -> c_int;
}
/// See: [`mpfr_signbit`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsignbit)
#[inline]
pub const unsafe extern "C" fn signbit(op: mpfr_srcptr) -> c_int {
    (unsafe { (*op).sign } < 0) as c_int
}
/// See: [`mpfr_setsign`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsetsign)
#[inline]
pub unsafe extern "C" fn setsign(rop: mpfr_ptr, op: mpfr_srcptr, s: c_int, rnd: rnd_t) -> c_int {
    unsafe { set4(rop, op, rnd, if s != 0 { -1 } else { 1 }) }
}
/// See: [`mpfr_copysign`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcopysign)
#[inline]
pub unsafe extern "C" fn copysign(
    rop: mpfr_ptr,
    op1: mpfr_srcptr,
    op2: mpfr_srcptr,
    rnd: rnd_t,
) -> c_int {
    unsafe { set4(rop, op1, rnd, (*op2).sign) }
}
extern "C" {
    /// See: [`mpfr_get_version`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005fversion)
    #[link_name = "mpfr_get_version"]
    pub fn get_version() -> *const c_char;
}
/// See: [`MPFR_VERSION`](../C/MPFR/constant.MPFR_Interface.html#index-MPFR_005fVERSION)
pub const VERSION: c_int = (VERSION_MAJOR << 16) | (VERSION_MINOR << 8) | VERSION_PATCHLEVEL;
/// See: [`MPFR_VERSION_MAJOR`](../C/MPFR/constant.MPFR_Interface.html#index-MPFR_005fVERSION_005fMAJOR)
pub const VERSION_MAJOR: c_int = MPFR_VERSION_MAJOR;
/// See: [`MPFR_VERSION_MINOR`](../C/MPFR/constant.MPFR_Interface.html#index-MPFR_005fVERSION_005fMINOR)
pub const VERSION_MINOR: c_int = MPFR_VERSION_MINOR;
/// See: [`MPFR_VERSION_PATCHLEVEL`](../C/MPFR/constant.MPFR_Interface.html#index-MPFR_005fVERSION_005fPATCHLEVEL)
pub const VERSION_PATCHLEVEL: c_int = MPFR_VERSION_PATCHLEVEL;
/// See: [`MPFR_VERSION_STRING`](../C/MPFR/constant.MPFR_Interface.html#index-MPFR_005fVERSION_005fSTRING)
pub const VERSION_STRING: *const c_char = MPFR_VERSION_STRING;
/// See: [`MPFR_VERSION_NUM`](../C/MPFR/constant.MPFR_Interface.html#index-MPFR_005fVERSION_005fNUM)
#[inline]
pub const extern "C" fn VERSION_NUM(major: c_int, minor: c_int, patchlevel: c_int) -> c_int {
    (major << 16) | (minor << 8) | patchlevel
}
extern "C" {
    /// See: [`mpfr_get_patches`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005fpatches)
    #[link_name = "mpfr_get_patches"]
    pub fn get_patches() -> *const c_char;
    /// See: [`mpfr_buildopt_tls_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fbuildopt_005ftls_005fp)
    #[link_name = "mpfr_buildopt_tls_p"]
    pub fn buildopt_tls_p() -> c_int;
    /// See: [`mpfr_buildopt_float128_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fbuildopt_005ffloat128_005fp)
    #[link_name = "mpfr_buildopt_float128_p"]
    pub fn buildopt_float128_p() -> c_int;
    /// See: [`mpfr_buildopt_decimal_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fbuildopt_005fdecimal_005fp)
    #[link_name = "mpfr_buildopt_decimal_p"]
    pub fn buildopt_decimal_p() -> c_int;
    /// See: [`mpfr_buildopt_gmpinternals_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fbuildopt_005fgmpinternals_005fp)
    #[link_name = "mpfr_buildopt_gmpinternals_p"]
    pub fn buildopt_gmpinternals_p() -> c_int;
    /// See: [`mpfr_buildopt_sharedcache_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fbuildopt_005fsharedcache_005fp)
    #[link_name = "mpfr_buildopt_sharedcache_p"]
    pub fn buildopt_sharedcache_p() -> c_int;
    /// See: [`mpfr_buildopt_tune_case`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fbuildopt_005ftune_005fcase)
    #[link_name = "mpfr_buildopt_tune_case"]
    pub fn buildopt_tune_case() -> *const c_char;

    // Exception Related Functions

    /// See: [`mpfr_get_emin`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005femin)
    #[link_name = "mpfr_get_emin"]
    pub fn get_emin() -> exp_t;
    /// See: [`mpfr_get_emax`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005femax)
    #[link_name = "mpfr_get_emax"]
    pub fn get_emax() -> exp_t;
    /// See: [`mpfr_set_emin`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005femin)
    #[link_name = "mpfr_set_emin"]
    pub fn set_emin(exp: exp_t) -> c_int;
    /// See: [`mpfr_set_emax`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005femax)
    #[link_name = "mpfr_set_emax"]
    pub fn set_emax(exp: exp_t) -> c_int;
    /// See: [`mpfr_get_emin_min`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005femin_005fmin)
    #[link_name = "mpfr_get_emin_min"]
    pub fn get_emin_min() -> exp_t;
    /// See: [`mpfr_get_emin_max`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005femin_005fmax)
    #[link_name = "mpfr_get_emin_max"]
    pub fn get_emin_max() -> exp_t;
    /// See: [`mpfr_get_emax_min`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005femax_005fmin)
    #[link_name = "mpfr_get_emax_min"]
    pub fn get_emax_min() -> exp_t;
    /// See: [`mpfr_get_emax_max`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fget_005femax_005fmax)
    #[link_name = "mpfr_get_emax_max"]
    pub fn get_emax_max() -> exp_t;
    /// See: [`mpfr_check_range`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcheck_005frange)
    #[link_name = "mpfr_check_range"]
    pub fn check_range(x: mpfr_ptr, t: c_int, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_subnormalize`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fsubnormalize)
    #[link_name = "mpfr_subnormalize"]
    pub fn subnormalize(x: mpfr_ptr, t: c_int, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_clear_underflow`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fclear_005funderflow)
    #[link_name = "mpfr_clear_underflow"]
    pub fn clear_underflow();
    /// See: [`mpfr_clear_overflow`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fclear_005foverflow)
    #[link_name = "mpfr_clear_overflow"]
    pub fn clear_overflow();
    /// See: [`mpfr_clear_divby0`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fclear_005fdivby0)
    #[link_name = "mpfr_clear_divby0"]
    pub fn clear_divby0();
    /// See: [`mpfr_clear_nanflag`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fclear_005fnanflag)
    #[link_name = "mpfr_clear_nanflag"]
    pub fn clear_nanflag();
    /// See: [`mpfr_clear_inexflag`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fclear_005finexflag)
    #[link_name = "mpfr_clear_inexflag"]
    pub fn clear_inexflag();
    /// See: [`mpfr_clear_erangeflag`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fclear_005ferangeflag)
    #[link_name = "mpfr_clear_erangeflag"]
    pub fn clear_erangeflag();
    /// See: [`mpfr_set_underflow`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005funderflow)
    #[link_name = "mpfr_set_underflow"]
    pub fn set_underflow();
    /// See: [`mpfr_set_overflow`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005foverflow)
    #[link_name = "mpfr_set_overflow"]
    pub fn set_overflow();
    /// See: [`mpfr_set_divby0`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fdivby0)
    #[link_name = "mpfr_set_divby0"]
    pub fn set_divby0();
    /// See: [`mpfr_set_nanflag`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fnanflag)
    #[link_name = "mpfr_set_nanflag"]
    pub fn set_nanflag();
    /// See: [`mpfr_set_inexflag`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005finexflag)
    #[link_name = "mpfr_set_inexflag"]
    pub fn set_inexflag();
    /// See: [`mpfr_set_erangeflag`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005ferangeflag)
    #[link_name = "mpfr_set_erangeflag"]
    pub fn set_erangeflag();
    /// See: [`mpfr_clear_flags`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fclear_005fflags)
    #[link_name = "mpfr_clear_flags"]
    pub fn clear_flags();
    /// See: [`mpfr_underflow_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005funderflow_005fp)
    #[link_name = "mpfr_underflow_p"]
    pub fn underflow_p() -> c_int;
    /// See: [`mpfr_overflow_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005foverflow_005fp)
    #[link_name = "mpfr_overflow_p"]
    pub fn overflow_p() -> c_int;
    /// See: [`mpfr_divby0_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fdivby0_005fp)
    #[link_name = "mpfr_divby0_p"]
    pub fn divby0_p() -> c_int;
    /// See: [`mpfr_nanflag_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fnanflag_005fp)
    #[link_name = "mpfr_nanflag_p"]
    pub fn nanflag_p() -> c_int;
    /// See: [`mpfr_inexflag_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005finexflag_005fp)
    #[link_name = "mpfr_inexflag_p"]
    pub fn inexflag_p() -> c_int;
    /// See: [`mpfr_erangeflag_p`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ferangeflag_005fp)
    #[link_name = "mpfr_erangeflag_p"]
    pub fn erangeflag_p() -> c_int;
    /// See: [`mpfr_flags_clear`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fflags_005fclear)
    #[link_name = "mpfr_flags_clear"]
    pub fn flags_clear(mask: flags_t);
    /// See: [`mpfr_flags_set`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fflags_005fset)
    #[link_name = "mpfr_flags_set"]
    pub fn flags_set(mask: flags_t);
    /// See: [`mpfr_flags_test `](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fflags_005ftest)
    #[link_name = "mpfr_flags_test"]
    pub fn flags_test(mask: flags_t) -> flags_t;
    /// See: [`mpfr_flags_save`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fflags_005fsave)
    #[link_name = "mpfr_flags_save"]
    pub fn flags_save() -> flags_t;
    /// See: [`mpfr_flags_restore`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fflags_005frestore)
    #[link_name = "mpfr_flags_restore"]
    pub fn flags_restore(flags: flags_t, mask: flags_t);

    // Memory Handling Functions

    /// See: [`mpfr_free_cache`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffree_005fcache)
    #[link_name = "mpfr_free_cache"]
    pub fn free_cache();
    /// See: [`mpfr_free_cache2`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffree_005fcache2)
    #[link_name = "mpfr_free_cache2"]
    pub fn free_cache2(way: c_int);
    /// See: [`mpfr_free_pool`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005ffree_005fpool)
    #[link_name = "mpfr_free_pool"]
    pub fn free_pool();
    /// See: [`mpfr_mp_memory_cleanup`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fmp_005fmemory_005fcleanup)
    #[link_name = "mpfr_mp_memory_cleanup"]
    pub fn mp_memory_cleanup();

    // Compatibility with MPF

    /// See: [`mpfr_set_prec_raw`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fset_005fprec_005fraw)
    #[link_name = "mpfr_set_prec_raw"]
    pub fn set_prec_raw(x: mpfr_ptr, prec: prec_t);
    /// See: [`mpfr_eq`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005feq)
    #[link_name = "mpfr_eq"]
    pub fn eq(op1: mpfr_srcptr, op2: mpfr_srcptr, op3: c_ulong) -> c_int;
    /// See: [`mpfr_reldiff`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005freldiff)
    #[link_name = "mpfr_reldiff"]
    pub fn reldiff(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpfr_srcptr, rnd: rnd_t);
}
/// See: [`mpfr_mul_2exp`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fmul_005f2exp)
#[inline]
pub unsafe extern "C" fn mul_2exp(
    rop: mpfr_ptr,
    op1: mpfr_srcptr,
    op2: c_ulong,
    rnd: rnd_t,
) -> c_int {
    unsafe { mul_2ui(rop, op1, op2, rnd) }
}
/// See: [`mpfr_div_2exp`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fdiv_005f2exp)
#[inline]
pub unsafe extern "C" fn div_2exp(
    rop: mpfr_ptr,
    op1: mpfr_srcptr,
    op2: c_ulong,
    rnd: rnd_t,
) -> c_int {
    unsafe { div_2ui(rop, op1, op2, rnd) }
}

// Custom Interface

/// See: [`mpfr_custom_get_size`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcustom_005fget_005fsize)
#[inline]
pub const unsafe extern "C" fn custom_get_size(prec: prec_t) -> usize {
    let bits = NUMB_BITS as prec_t;
    ((prec + bits - 1) / bits) as usize * mem::size_of::<limb_t>()
}
/// See: [`mpfr_custom_init`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcustom_005finit)
#[inline]
pub const unsafe extern "C" fn custom_init(significand: *mut c_void, prec: prec_t) {
    let _ = (significand, prec);
}
/// See: [`mpfr_custom_init_set`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcustom_005finit_005fset)
#[inline]
pub unsafe extern "C" fn custom_init_set(
    x: mpfr_ptr,
    kind: c_int,
    exp: exp_t,
    prec: prec_t,
    significand: *mut c_void,
) {
    let (t, s) = if kind >= 0 { (kind, 1) } else { (-kind, -1) };
    let e = match t {
        REGULAR_KIND => exp,
        NAN_KIND => EXP_NAN,
        INF_KIND => EXP_INF,
        _ => EXP_ZERO,
    };
    unsafe {
        (*x).prec = prec;
        (*x).sign = s;
        (*x).exp = e;
        (*x).d = NonNull::new_unchecked(significand.cast());
    }
}
/// See: [`mpfr_custom_get_kind`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcustom_005fget_005fkind)
#[inline]
pub const unsafe extern "C" fn custom_get_kind(x: mpfr_srcptr) -> c_int {
    unsafe {
        if (*x).exp > EXP_INF {
            REGULAR_KIND * (*x).sign
        } else if (*x).exp == EXP_INF {
            INF_KIND * (*x).sign
        } else if (*x).exp == EXP_NAN {
            NAN_KIND
        } else {
            ZERO_KIND * (*x).sign
        }
    }
}
/// See: [`mpfr_custom_get_significand`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcustom_005fget_005fsignificand)
#[inline]
pub const unsafe extern "C" fn custom_get_significand(x: mpfr_srcptr) -> *mut c_void {
    unsafe { (*x).d }.as_ptr().cast()
}
/// See: [`mpfr_custom_get_exp`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcustom_005fget_005fexp)
#[inline]
pub const unsafe extern "C" fn custom_get_exp(x: mpfr_srcptr) -> exp_t {
    unsafe { (*x).exp }
}
/// See: [`mpfr_custom_move`](../C/MPFR/constant.MPFR_Interface.html#index-mpfr_005fcustom_005fmove)
#[inline]
pub unsafe extern "C" fn custom_move(x: mpfr_ptr, new_position: *mut c_void) {
    unsafe { (*x).d = NonNull::new_unchecked(new_position.cast()) }
}

#[cfg(test)]
mod tests {
    use crate::mpfr;
    use core::mem::MaybeUninit;

    fn version_matches_with_optional_p_suffix(to_test: &str, check: &str) -> bool {
        if to_test == check {
            true
        } else if !to_test.starts_with(check) {
            false
        } else {
            to_test[check.len()..].starts_with("-p")
        }
    }

    #[test]
    fn check_version() {
        use crate::tests;

        let (major, minor, patchlevel) = (4, 2, 2);
        // do not include "-p*" suffix
        let version = "4.2.2";

        assert_eq!(mpfr::VERSION_MAJOR, major);
        assert!(mpfr::VERSION_MINOR >= minor);
        if cfg!(not(feature = "use-system-libs")) {
            assert!(mpfr::VERSION_MINOR > minor || mpfr::VERSION_PATCHLEVEL >= patchlevel);
            if mpfr::VERSION_MINOR == minor && mpfr::VERSION_PATCHLEVEL == patchlevel {
                // tested string can have "-p*" suffix
                assert!(version_matches_with_optional_p_suffix(
                    unsafe { tests::str_from_cstr(mpfr::get_version()) },
                    version
                ));
                assert!(version_matches_with_optional_p_suffix(
                    unsafe { tests::str_from_cstr(mpfr::VERSION_STRING) },
                    version
                ));
            }
        }
    }

    #[test]
    fn check_round_nearest_away() {
        unsafe {
            let mut f = MaybeUninit::uninit();
            mpfr::init2(f.as_mut_ptr(), 4);
            let mut f = f.assume_init();

            // mpfr_round_nearest_away needs emin > emin_min
            if mpfr::get_emin() == mpfr::get_emin_min() {
                mpfr::set_emin(mpfr::get_emin_min() + 1);
            }

            // tie to even: 10101 becomes 10100
            let dir_tie_even = mpfr::set_ui(&mut f, 21, mpfr::rnd_t::RNDN);
            assert!(dir_tie_even < 0);
            let tie_even = mpfr::get_ui(&f, mpfr::rnd_t::RNDN);
            assert_eq!(tie_even, 20);

            // tie away from zero, 10101 becomes 10110
            let dir_tie_away = mpfr_round_nearest_away!(mpfr::set_ui, &mut f, 21);
            assert!(dir_tie_away > 0);
            let tie_away = mpfr::get_ui(&f, mpfr::rnd_t::RNDN);
            assert_eq!(tie_away, 22);

            // tie away from zero, 101001 becomes 101000
            let dir_tie_away2 = mpfr_round_nearest_away!(mpfr::set_ui, &mut f, 41);
            assert!(dir_tie_away2 < 0);
            let tie_away2 = mpfr::get_ui(&f, mpfr::rnd_t::RNDN);
            assert_eq!(tie_away2, 40);

            mpfr::clear(&mut f);
        }
    }

    #[test]
    fn check_decl_init() {
        MPFR_DECL_INIT!(f, 5);
        unsafe {
            assert_eq!(mpfr::get_prec(&f), 5);
            assert_ne!(mpfr::nan_p(&f), 0);
            assert!(mpfr::set_ui(&mut f, 0xff, mpfr::rnd_t::RNDD) < 0);
            assert_eq!(mpfr::nan_p(&f), 0);
            assert_eq!(mpfr::get_ui(&f, mpfr::rnd_t::RNDN), 0xf8);
        }
    }
}
