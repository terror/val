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
#[cfg(feature = "integer")]
use crate::Integer;
#[cfg(feature = "rational")]
use crate::Rational;
#[cfg(feature = "rational")]
use crate::float::BorrowFloat;
use crate::float::{MiniFloat, Round, Special};
use crate::misc;
use crate::misc::{NegAbs, VecLike};
use crate::ops::NegAssign;
#[cfg(feature = "rand")]
use crate::rand::MutRandState;
use az::{CheckedAs, StrictCast, WrappingAs};
use core::cmp::Ordering;
use core::ffi::{c_int, c_long, c_ulong};
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use gmp_mpfr_sys::gmp;
use gmp_mpfr_sys::gmp::limb_t;
use gmp_mpfr_sys::mpfr;
use gmp_mpfr_sys::mpfr::{exp_t, mpfr_t, prec_t, rnd_t};
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use libc::{intmax_t, uintmax_t};

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[allow(non_camel_case_types)]
type intmax_t = i64;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[allow(non_camel_case_types)]
type uintmax_t = u64;

pub trait OptFloat: Copy {
    const IS_SOME: bool;
    fn mpfr(self) -> *const mpfr_t;
    fn mpfr_or(self, default: *mut mpfr_t) -> *const mpfr_t;
    #[cfg(feature = "rational")]
    fn unwrap_or<'a>(self, default: &'a mut Float) -> &'a Float
    where
        Self: 'a;
}

impl OptFloat for () {
    const IS_SOME: bool = false;
    #[inline(always)]
    fn mpfr(self) -> *const mpfr_t {
        unreachable!("unwrapping ()");
    }
    #[inline(always)]
    fn mpfr_or(self, default: *mut mpfr_t) -> *const mpfr_t {
        default.cast_const()
    }
    #[cfg(feature = "rational")]
    #[inline(always)]
    fn unwrap_or<'a>(self, default: &'a mut Float) -> &'a Float
    where
        Self: 'a,
    {
        &*default
    }
}

impl OptFloat for &Float {
    const IS_SOME: bool = true;
    #[inline(always)]
    fn mpfr(self) -> *const mpfr_t {
        self.as_raw()
    }
    #[inline(always)]
    fn mpfr_or(self, _default: *mut mpfr_t) -> *const mpfr_t {
        self.as_raw()
    }
    #[cfg(feature = "rational")]
    #[inline(always)]
    fn unwrap_or<'b>(self, _default: &'b mut Float) -> &'b Float
    where
        Self: 'b,
    {
        self
    }
}

#[inline]
pub fn raw_round(round: Round) -> rnd_t {
    match round {
        Round::Nearest => rnd_t::RNDN,
        Round::Zero => rnd_t::RNDZ,
        Round::Up => rnd_t::RNDU,
        Round::Down => rnd_t::RNDD,
        Round::AwayZero => rnd_t::RNDA,
    }
}

#[inline]
pub fn ordering1(ord: c_int) -> Ordering {
    ord.cmp(&0)
}

#[inline]
pub fn get_emin() -> exp_t {
    unsafe { mpfr::get_emin() }
}

#[inline]
pub fn get_emax() -> exp_t {
    unsafe { mpfr::get_emax() }
}

#[inline]
fn ordering2(ord: c_int) -> (Ordering, Ordering) {
    // ord == first + 4 * second
    let first = match ord & 3 {
        2 => -1,
        0 => 0,
        1 => 1,
        _ => unreachable!(),
    };
    let second = match ord >> 2 {
        2 => -1,
        0 => 0,
        1 => 1,
        _ => unreachable!(),
    };
    (ordering1(first), ordering1(second))
}

macro_rules! unsafe_wrap {
    (fn $fn:ident($($op:ident: $O:ident),* $(; $param:ident: $T:ty)*) -> $deleg:path) => {
        #[inline]
        pub fn $fn<$($O: OptFloat),*>(
            rop: &mut Float,
            $($op: $O,)*
            $($param: $T,)*
            rnd: Round,
        ) -> Ordering {
            let rop = rop.as_raw_mut();
            $(let $op = $op.mpfr_or(rop);)*
            ordering1(unsafe { $deleg(rop, $($op,)* $($param.into(),)* raw_round(rnd)) })
        }
    };
}

macro_rules! unsafe_wrap0 {
    (fn $fn:ident($($param:ident: $T:ty),*) -> $deleg:path) => {
        #[inline]
        pub fn $fn(rop: &mut Float, $($param: $T,)* rnd: Round) -> Ordering {
            ordering1(unsafe { $deleg(rop.as_raw_mut(), $($param.into(),)* raw_round(rnd)) })
        }
    };
}

#[inline]
pub fn si_pow_ui(rop: &mut Float, base: i32, exponent: u32, rnd: Round) -> Ordering {
    let (base_neg, base_abs) = base.neg_abs();
    if !base_neg || (exponent & 1) == 0 {
        ordering1(unsafe {
            mpfr::ui_pow_ui(
                rop.as_raw_mut(),
                base_abs.into(),
                exponent.into(),
                raw_round(rnd),
            )
        })
    } else {
        let reverse_rnd = raw_round(rnd.reverse());
        let reverse_ord = ordering1(unsafe {
            mpfr::ui_pow_ui(
                rop.as_raw_mut(),
                base_abs.into(),
                exponent.into(),
                reverse_rnd,
            )
        });
        neg(rop, (), Round::Nearest);
        reverse_ord.reverse()
    }
}

#[inline]
pub fn write_new_nan(x: &mut MaybeUninit<Float>, prec: prec_t) {
    // Safety: we can cast pointers to/from Float/mpfr_t as they are repr(transparent).
    unsafe {
        let inner_ptr = misc::cast_ptr_mut(x.as_mut_ptr());
        mpfr::init2(inner_ptr, prec);
    }
}

// do not use mpfr::neg for op is () to avoid function call
#[inline]
pub fn neg<O: OptFloat>(rop: &mut Float, op: O, rnd: Round) -> Ordering {
    if O::IS_SOME {
        ordering1(unsafe { mpfr::neg(rop.as_raw_mut(), op.mpfr(), raw_round(rnd)) })
    } else {
        // Safety: negating the sign of mpfr_t is always sound.
        unsafe {
            rop.inner_mut().sign.neg_assign();
        }
        if rop.is_nan() {
            set_nanflag();
        }
        Ordering::Equal
    }
}

#[inline]
pub fn signum<O: OptFloat>(rop: &mut Float, op: O, _rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op = op.mpfr_or(rop);
    unsafe {
        if mpfr::nan_p(op) != 0 {
            mpfr::set(rop, op, rnd_t::RNDZ);
        } else if mpfr::signbit(op) != 0 {
            mpfr::set_si(rop, -1, rnd_t::RNDZ);
        } else {
            mpfr::set_si(rop, 1, rnd_t::RNDZ);
        }
    }
    // mpfr_t has a minimum precision of 1, so -1 and 1 are exactly representable
    Ordering::Equal
}

#[inline]
pub fn recip<O: OptFloat>(rop: &mut Float, op: O, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op = op.mpfr_or(rop);
    ordering1(unsafe { mpfr::ui_div(rop, 1, op, raw_round(rnd)) })
}

#[inline]
pub fn jn<O: OptFloat>(rop: &mut Float, op: O, n: i32, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op = op.mpfr_or(rop);
    ordering1(unsafe { mpfr::jn(rop, n.into(), op, raw_round(rnd)) })
}

#[inline]
pub fn yn<O: OptFloat>(rop: &mut Float, op: O, n: i32, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op = op.mpfr_or(rop);
    ordering1(unsafe { mpfr::yn(rop, n.into(), op, raw_round(rnd)) })
}

#[inline]
pub fn sin_cos<O: OptFloat>(
    rop_sin: &mut Float,
    rop_cos: &mut Float,
    op: O,
    rnd: Round,
) -> (Ordering, Ordering) {
    let rop_sin = rop_sin.as_raw_mut();
    let rop_cos = rop_cos.as_raw_mut();
    let op = op.mpfr_or(rop_sin);
    ordering2(unsafe { mpfr::sin_cos(rop_sin, rop_cos, op, raw_round(rnd)) })
}

#[inline]
pub fn sinh_cosh<O: OptFloat>(
    rop_sin: &mut Float,
    rop_cos: &mut Float,
    op: O,
    rnd: Round,
) -> (Ordering, Ordering) {
    let rop_sin = rop_sin.as_raw_mut();
    let rop_cos = rop_cos.as_raw_mut();
    let op = op.mpfr_or(rop_sin);
    ordering2(unsafe { mpfr::sinh_cosh(rop_sin, rop_cos, op, raw_round(rnd)) })
}

#[inline]
pub fn modf<O: OptFloat>(
    rop_i: &mut Float,
    rop_f: &mut Float,
    op: O,
    rnd: Round,
) -> (Ordering, Ordering) {
    let rop_i = rop_i.as_raw_mut();
    let rop_f = rop_f.as_raw_mut();
    let op = op.mpfr_or(rop_i);
    ordering2(unsafe { mpfr::modf(rop_i, rop_f, op, raw_round(rnd)) })
}

#[inline]
pub fn lgamma<O: OptFloat>(rop: &mut Float, op: O, rnd: Round) -> (Ordering, Ordering) {
    let mut sign = MaybeUninit::uninit();
    let rop = rop.as_raw_mut();
    let op = op.mpfr_or(rop);
    let ord = ordering1(unsafe { mpfr::lgamma(rop, sign.as_mut_ptr(), op, raw_round(rnd)) });
    let sign_ord = if (unsafe { sign.assume_init() }) < 0 {
        Ordering::Less
    } else {
        Ordering::Greater
    };
    (sign_ord, ord)
}

pub fn sum<'a, I>(rop: &mut Float, values: I, rnd: Round) -> Ordering
where
    I: Iterator<Item = &'a Float>,
{
    let pointers = values.map(Float::as_raw).collect::<VecLike<_>>();
    unsafe { sum_raw(rop.as_raw_mut(), pointers.as_slice(), rnd) }
}

// add original value of rop to sum
pub fn sum_including_old<'a, I>(rop: &mut Float, values: I, rnd: Round) -> Ordering
where
    I: Iterator<Item = &'a Float>,
{
    let rop = rop.as_raw_mut();
    let capacity = values.size_hint().0.checked_add(1).expect("overflow");
    let mut pointers = VecLike::new();
    pointers.reserve(capacity);
    pointers.push(rop.cast_const());
    pointers.extend(values.map(Float::as_raw));
    unsafe { sum_raw(rop, pointers.as_slice(), rnd) }
}

pub unsafe fn sum_raw(rop: *mut mpfr_t, pointers: &[*const mpfr_t], rnd: Round) -> Ordering {
    let n = pointers.len().strict_cast();
    let tab = misc::cast_ptr(pointers.as_ptr());
    let rnd = raw_round(rnd);
    ordering1(unsafe { mpfr::sum(rop, tab, n, rnd) })
}

pub fn dot<'a, I>(rop: &mut Float, values: I, rnd: Round) -> Ordering
where
    I: Iterator<Item = (&'a Float, &'a Float)>,
{
    let (pointers_a, pointers_b): (VecLike<_>, VecLike<_>) =
        values.map(|(a, b)| (a.as_raw(), b.as_raw())).unzip();
    unsafe {
        dot_raw(
            rop.as_raw_mut(),
            pointers_a.as_slice(),
            pointers_b.as_slice(),
            rnd,
        )
    }
}

// add original value of rop to dot
pub fn dot_including_old<'a, I>(rop: &mut Float, values: I, rnd: Round) -> Ordering
where
    I: Iterator<Item = (&'a Float, &'a Float)>,
{
    const LIMB_ONE: limb_t = 1;
    const LIMB_MSB: limb_t = LIMB_ONE << (gmp::LIMB_BITS - 1);
    const ONE: mpfr_t = mpfr_t {
        prec: 1,
        sign: 1,
        exp: 1,
        d: unsafe { NonNull::new_unchecked((&LIMB_MSB as *const limb_t).cast_mut()) },
    };

    let rop = rop.as_raw_mut();
    let capacity = values.size_hint().0.checked_add(1).expect("overflow");
    let mut pointers_a = VecLike::new();
    let mut pointers_b = VecLike::new();
    pointers_a.reserve(capacity);
    pointers_b.reserve(capacity);
    pointers_a.push(rop.cast_const());
    pointers_b.push(&ONE as *const mpfr_t);
    for a_b in values {
        pointers_a.push(a_b.0.as_raw());
        pointers_b.push(a_b.1.as_raw());
    }
    unsafe { dot_raw(rop, pointers_a.as_slice(), pointers_b.as_slice(), rnd) }
}

// pointers_a and pointers_b must have same length
unsafe fn dot_raw(
    rop: *mut mpfr_t,
    pointers_a: &[*const mpfr_t],
    pointers_b: &[*const mpfr_t],
    rnd: Round,
) -> Ordering {
    debug_assert_eq!(pointers_a.len(), pointers_b.len());
    let n = pointers_a.len().strict_cast();
    let a = misc::cast_ptr(pointers_a.as_ptr());
    let b = misc::cast_ptr(pointers_b.as_ptr());
    let rnd = raw_round(rnd);
    ordering1(unsafe { mpfr::dot(rop, a, b, n, rnd) })
}

unsafe_wrap! { fn set(src: O) -> mpfr::set }
unsafe_wrap0! { fn set_si(src: c_long) -> mpfr::set_si }
unsafe_wrap0! { fn set_sj(src: intmax_t) -> mpfr::set_sj }
unsafe_wrap0! { fn set_ui(src: c_ulong) -> mpfr::set_ui }
unsafe_wrap0! { fn set_uj(src: uintmax_t) -> mpfr::set_uj }
unsafe_wrap0! { fn set_f64(src: f64) -> mpfr::set_d }
unsafe_wrap0! { fn set_f32(src: f32) -> mpfr::set_flt }
unsafe_wrap0! { fn prec_round(prec: prec_t) -> mpfr::prec_round }
unsafe_wrap! { fn remainder(x: O, y: P) -> mpfr::remainder }
unsafe_wrap0! { fn ui_pow_ui(base: u32, exponent: u32) -> mpfr::ui_pow_ui }
unsafe_wrap0! { fn ui_2exp(ui: u32, exponent: i32) -> mpfr::set_ui_2exp }
unsafe_wrap0! { fn si_2exp(si: i32, exponent: i32) -> mpfr::set_si_2exp }
unsafe_wrap! { fn copysign(op1: O, op2: P) -> mpfr::copysign }
unsafe_wrap! { fn sqr(op: O) -> mpfr::sqr }
unsafe_wrap! { fn sqrt(op: O) -> mpfr::sqrt }
unsafe_wrap0! { fn sqrt_ui(u: u32) -> mpfr::sqrt_ui }
unsafe_wrap! { fn rec_sqrt(op: O) -> mpfr::rec_sqrt }
unsafe_wrap! { fn cbrt(op: O) -> mpfr::cbrt }
unsafe_wrap! { fn rootn_ui(op: O; k: u32) -> mpfr::rootn_ui }
unsafe_wrap! { fn rootn_si(op: O; k: i32) -> mpfr::rootn_si }
unsafe_wrap! { fn abs(op: O) -> mpfr::abs }
unsafe_wrap! { fn min(op1: O, op2: P) -> mpfr::min }
unsafe_wrap! { fn max(op1: O, op2: P) -> mpfr::max }
unsafe_wrap! { fn dim(op1: O, op2: P) -> mpfr::dim }
unsafe_wrap! { fn log(op: O) -> mpfr::log }
unsafe_wrap0! { fn log_ui(u: u32) -> mpfr::log_ui }
unsafe_wrap! { fn log2(op: O) -> mpfr::log2 }
unsafe_wrap! { fn log10(op: O) -> mpfr::log10 }
unsafe_wrap! { fn exp(op: O) -> mpfr::exp }
unsafe_wrap! { fn exp2(op: O) -> mpfr::exp2 }
unsafe_wrap! { fn exp10(op: O) -> mpfr::exp10 }
unsafe_wrap! { fn sin(op: O) -> mpfr::sin }
unsafe_wrap! { fn cos(op: O) -> mpfr::cos }
unsafe_wrap! { fn tan(op: O) -> mpfr::tan }
unsafe_wrap! { fn sinu(op: O; u: u32) -> mpfr::sinu }
unsafe_wrap! { fn cosu(op: O; u: u32) -> mpfr::cosu }
unsafe_wrap! { fn tanu(op: O; u: u32) -> mpfr::tanu }
unsafe_wrap! { fn sinpi(op: O) -> mpfr::sinpi }
unsafe_wrap! { fn cospi(op: O) -> mpfr::cospi }
unsafe_wrap! { fn tanpi(op: O) -> mpfr::tanpi }
unsafe_wrap! { fn sec(op: O) -> mpfr::sec }
unsafe_wrap! { fn csc(op: O) -> mpfr::csc }
unsafe_wrap! { fn cot(op: O) -> mpfr::cot }
unsafe_wrap! { fn acos(op: O) -> mpfr::acos }
unsafe_wrap! { fn asin(op: O) -> mpfr::asin }
unsafe_wrap! { fn atan(op: O) -> mpfr::atan }
unsafe_wrap! { fn atan2(y: O, x: P) -> mpfr::atan2 }
unsafe_wrap! { fn acosu(op: O; u: u32) -> mpfr::acosu }
unsafe_wrap! { fn asinu(op: O; u: u32) -> mpfr::asinu }
unsafe_wrap! { fn atanu(op: O; u: u32) -> mpfr::atanu }
unsafe_wrap! { fn atan2u(y: O, x: P; u: u32) -> mpfr::atan2u }
unsafe_wrap! { fn acospi(op: O) -> mpfr::acospi }
unsafe_wrap! { fn asinpi(op: O) -> mpfr::asinpi }
unsafe_wrap! { fn atanpi(op: O) -> mpfr::atanpi }
unsafe_wrap! { fn atan2pi(y: O, x: P) -> mpfr::atan2pi }
unsafe_wrap! { fn cosh(op: O) -> mpfr::cosh }
unsafe_wrap! { fn sinh(op: O) -> mpfr::sinh }
unsafe_wrap! { fn tanh(op: O) -> mpfr::tanh }
unsafe_wrap! { fn sech(op: O) -> mpfr::sech }
unsafe_wrap! { fn csch(op: O) -> mpfr::csch }
unsafe_wrap! { fn coth(op: O) -> mpfr::coth }
unsafe_wrap! { fn acosh(op: O) -> mpfr::acosh }
unsafe_wrap! { fn asinh(op: O) -> mpfr::asinh }
unsafe_wrap! { fn atanh(op: O) -> mpfr::atanh }
unsafe_wrap0! { fn fac_ui(n: u32) -> mpfr::fac_ui }
unsafe_wrap! { fn log1p(op: O) -> mpfr::log1p }
unsafe_wrap! { fn log2p1(op: O) -> mpfr::log2p1 }
unsafe_wrap! { fn log10p1(op: O) -> mpfr::log10p1 }
unsafe_wrap! { fn expm1(op: O) -> mpfr::expm1 }
unsafe_wrap! { fn exp2m1(op: O) -> mpfr::exp2m1 }
unsafe_wrap! { fn exp10m1(op: O) -> mpfr::exp10m1 }
unsafe_wrap! { fn compound_si(op: O; n: i32) -> mpfr::compound_si }
unsafe_wrap! { fn eint(op: O) -> mpfr::eint }
unsafe_wrap! { fn li2(op: O) -> mpfr::li2 }
unsafe_wrap! { fn gamma(op: O) -> mpfr::gamma }
unsafe_wrap! { fn gamma_inc(op1: O, op2: P) -> mpfr::gamma_inc }
unsafe_wrap! { fn lngamma(op: O) -> mpfr::lngamma }
unsafe_wrap! { fn digamma(op: O) -> mpfr::digamma }
unsafe_wrap! { fn zeta(op: O) -> mpfr::zeta }
unsafe_wrap0! { fn zeta_ui(u: u32) -> mpfr::zeta_ui }
unsafe_wrap! { fn erf(op: O) -> mpfr::erf }
unsafe_wrap! { fn erfc(op: O) -> mpfr::erfc }
unsafe_wrap! { fn j0(op: O) -> mpfr::j0 }
unsafe_wrap! { fn j1(op: O) -> mpfr::j1 }
unsafe_wrap! { fn y0(op: O) -> mpfr::y0 }
unsafe_wrap! { fn y1(op: O) -> mpfr::y1 }
unsafe_wrap! { fn agm(op1: O, op2: P) -> mpfr::agm }
unsafe_wrap! { fn hypot(x: O, y: P) -> mpfr::hypot }
unsafe_wrap! { fn ai(op: O) -> mpfr::ai }
unsafe_wrap! { fn rint_ceil(op: O) -> mpfr::rint_ceil }
unsafe_wrap! { fn rint_floor(op: O) -> mpfr::rint_floor }
unsafe_wrap! { fn rint_round(op: O) -> mpfr::rint_round }
unsafe_wrap! { fn rint_roundeven(op: O) -> mpfr::rint_roundeven }
unsafe_wrap! { fn rint_trunc(op: O) -> mpfr::rint_trunc }
unsafe_wrap! { fn frac(op: O) -> mpfr::frac }
unsafe_wrap0! { fn const_log2() -> mpfr::const_log2 }
unsafe_wrap0! { fn const_pi() -> mpfr::const_pi }
unsafe_wrap0! { fn const_euler() -> mpfr::const_euler }
unsafe_wrap0! { fn const_catalan() -> mpfr::const_catalan }
unsafe_wrap! { fn fma(op1: O, op2: P, op3: Q) -> mpfr::fma }
unsafe_wrap! { fn fms(op1: O, op2: P, op3: Q) -> mpfr::fms }
unsafe_wrap! { fn fmma(op1: O, op2: P, op3: Q, op4: R) -> mpfr::fmma }
unsafe_wrap! { fn fmms(op1: O, op2: P, op3: Q, op4: R) -> mpfr::fmms }
unsafe_wrap! { fn add(op1: O, op2: P) -> mpfr::add }
unsafe_wrap! { fn sub(op1: O, op2: P) -> mpfr::sub }
unsafe_wrap! { fn mul(op1: O, op2: P) -> mpfr::mul }
unsafe_wrap! { fn div(op1: O, op2: P) -> mpfr::div }
unsafe_wrap! { fn fmod(op1: O, op2: P) -> mpfr::fmod }
unsafe_wrap! { fn pow(op1: O, op2: P) -> mpfr::pow }
unsafe_wrap! { fn add_si(op1: O; op2: c_long) -> mpfr::add_si }
unsafe_wrap! { fn sub_si(op1: O; op2: c_long) -> mpfr::sub_si }
unsafe_wrap! { fn mul_si(op1: O; op2: c_long) -> mpfr::mul_si }
unsafe_wrap! { fn div_si(op1: O; op2: c_long) -> mpfr::div_si }
unsafe_wrap! { fn pow_si(op1: O; op2: c_long) -> mpfr::pow_si }
unsafe_wrap! { fn add_ui(op1: O; op2: c_ulong) -> mpfr::add_ui }
unsafe_wrap! { fn sub_ui(op1: O; op2: c_ulong) -> mpfr::sub_ui }
unsafe_wrap! { fn mul_ui(op1: O; op2: c_ulong) -> mpfr::mul_ui }
unsafe_wrap! { fn div_ui(op1: O; op2: c_ulong) -> mpfr::div_ui }
unsafe_wrap! { fn fmod_ui(op1: O; op2: c_ulong) -> mpfr::fmod_ui }
unsafe_wrap! { fn pow_ui(op1: O; op2: c_ulong) -> mpfr::pow_ui }
unsafe_wrap! { fn add_d(op1: O; op2: f64) -> mpfr::add_d }
unsafe_wrap! { fn sub_d(op1: O; op2: f64) -> mpfr::sub_d }
unsafe_wrap! { fn mul_d(op1: O; op2: f64) -> mpfr::mul_d }
unsafe_wrap! { fn div_d(op1: O; op2: f64) -> mpfr::div_d }
unsafe_wrap! { fn shl_i32(op1: O; op2: i32) -> mpfr::mul_2si }
unsafe_wrap! { fn shr_i32(op1: O; op2: i32) -> mpfr::div_2si }
unsafe_wrap! { fn shl_u32(op1: O; op2: u32) -> mpfr::mul_2ui }
unsafe_wrap! { fn shr_u32(op1: O; op2: u32) -> mpfr::div_2ui }
unsafe_wrap! { fn shl_isize(op1: O; op2: isize) -> mul_2isize }
unsafe_wrap! { fn shr_isize(op1: O; op2: isize) -> div_2isize }
unsafe_wrap! { fn shl_usize(op1: O; op2: usize) -> mul_2usize }
unsafe_wrap! { fn shr_usize(op1: O; op2: usize) -> div_2usize }

#[inline]
pub fn remainder_quo31<O: OptFloat, P: OptFloat>(
    rop: &mut Float,
    op1: O,
    op2: P,
    rnd: Round,
) -> (Ordering, i32) {
    let rop = rop.as_raw_mut();
    let op1 = op1.mpfr_or(rop);
    let op2 = op2.mpfr_or(rop);
    let ord;
    let quo31;
    unsafe {
        let mut quo_long = MaybeUninit::<c_long>::uninit();
        ord = mpfr::remquo(rop, quo_long.as_mut_ptr(), op1, op2, raw_round(rnd));
        quo31 = if mpfr::nan_p(rop) == 0 {
            let quo_long = quo_long.assume_init();
            let lower31 = quo_long.wrapping_as::<i32>() & i32::MAX;
            // Do not add sign if lower31 is 0, otherwise we end up with
            // i32::MIN which has one extra significant bit.
            let sign = (quo_long.is_negative() && lower31 != 0).wrapping_as::<i32>() << 31;
            sign | lower31
        } else {
            0
        };
    }
    (ordering1(ord), quo31)
}

#[inline]
pub fn frexp<O: OptFloat>(rop: &mut Float, op: O, rnd: Round) -> (Ordering, i32) {
    let rop = rop.as_raw_mut();
    let op = op.mpfr_or(rop);
    let ord;
    let exp32;
    unsafe {
        let mut exp = MaybeUninit::<exp_t>::uninit();
        ord = mpfr::frexp(exp.as_mut_ptr(), rop, op, raw_round(rnd));
        exp32 = if mpfr::number_p(rop) != 0 {
            let exp = exp.assume_init();
            exp.strict_cast()
        } else {
            0
        };
    }
    (ordering1(ord), exp32)
}

#[cfg(feature = "nightly-float")]
#[inline]
pub fn set_f16(rop: &mut Float, src: f16, rnd: Round) -> Ordering {
    set_f64(rop, src.into(), rnd)
}

#[cfg(feature = "nightly-float")]
#[inline]
pub fn set_f128(rop: &mut Float, src: f128, rnd: Round) -> Ordering {
    let mut small = MiniFloat::from(src);
    set(rop, small.borrow_excl(), rnd)
}

#[inline]
unsafe fn mul_2isize(rop: *mut mpfr_t, op1: *const mpfr_t, op2: isize, rnd: rnd_t) -> c_int {
    let op2 = op2.strict_cast();
    unsafe { mpfr::mul_2si(rop, op1, op2, rnd) }
}

#[inline]
unsafe fn div_2isize(rop: *mut mpfr_t, op1: *const mpfr_t, op2: isize, rnd: rnd_t) -> c_int {
    let op2 = op2.strict_cast();
    unsafe { mpfr::div_2si(rop, op1, op2, rnd) }
}

#[inline]
unsafe fn mul_2usize(rop: *mut mpfr_t, op1: *const mpfr_t, op2: usize, rnd: rnd_t) -> c_int {
    let op2 = op2.strict_cast();
    unsafe { mpfr::mul_2ui(rop, op1, op2, rnd) }
}

#[inline]
unsafe fn div_2usize(rop: *mut mpfr_t, op1: *const mpfr_t, op2: usize, rnd: rnd_t) -> c_int {
    let op2 = op2.strict_cast();
    unsafe { mpfr::div_2ui(rop, op1, op2, rnd) }
}

#[inline]
pub fn check_range(x: &mut Float, dir: Ordering, rnd: Round) -> Ordering {
    let dir = match dir {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    };
    ordering1(unsafe { mpfr::check_range(x.as_raw_mut(), dir, raw_round(rnd)) })
}

#[inline]
pub fn set_nanflag() {
    unsafe {
        mpfr::set_nanflag();
    }
}

#[inline]
pub fn set_prec_nan(x: &mut Float, prec: prec_t) {
    unsafe {
        mpfr::set_prec(x.as_raw_mut(), prec);
    }
}

#[inline]
pub fn set_special(x: &mut Float, src: Special) {
    // Safety: We do not change inner.d, and we set inner.exp to
    // indicate a singular number so even if the data at inner.d is
    // uninitialized, we still won't use it.
    unsafe {
        let inner = x.inner_mut();
        match src {
            Special::Zero => {
                inner.sign = 1;
                inner.exp = EXP_ZERO;
            }
            Special::NegZero => {
                inner.sign = -1;
                inner.exp = EXP_ZERO;
            }
            Special::Infinity => {
                inner.sign = 1;
                inner.exp = EXP_INF;
            }
            Special::NegInfinity => {
                inner.sign = -1;
                inner.exp = EXP_INF;
            }
            Special::Nan => {
                inner.sign = 1;
                inner.exp = EXP_NAN;
            }
        }
    }
}

#[cfg(feature = "integer")]
#[inline]
pub fn set_z(dst: &mut Float, src: &Integer, rnd: Round) -> Ordering {
    ordering1(unsafe { mpfr::set_z(dst.as_raw_mut(), src.as_raw(), raw_round(rnd)) })
}

#[cfg(feature = "rational")]
#[inline]
pub fn set_q(dst: &mut Float, src: &Rational, rnd: Round) -> Ordering {
    ordering1(unsafe { mpfr::set_q(dst.as_raw_mut(), src.as_raw(), raw_round(rnd)) })
}

#[inline]
pub fn set_i128(rop: &mut Float, val: i128, rnd: Round) -> Ordering {
    let mut small = MiniFloat::from(val);
    set(rop, small.borrow_excl(), rnd)
}

#[inline]
pub fn set_u128(rop: &mut Float, val: u128, rnd: Round) -> Ordering {
    let mut small = MiniFloat::from(val);
    set(rop, small.borrow_excl(), rnd)
}

#[inline]
pub fn si_2exp_t(rop: &mut Float, si: i32, exponent: exp_t, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    ordering1(unsafe { mpfr::set_si_2exp(rop, si.into(), exponent, raw_round(rnd)) })
}

#[inline]
pub fn nexttoward(rop: &mut Float, op: &Float) {
    unsafe { mpfr::nexttoward(rop.as_raw_mut(), op.as_raw()) }
}

#[inline]
pub fn nextabove(rop: &mut Float) {
    unsafe { mpfr::nextabove(rop.as_raw_mut()) }
}

#[inline]
pub fn nextbelow(rop: &mut Float) {
    unsafe { mpfr::nextbelow(rop.as_raw_mut()) }
}

#[inline]
pub const fn sgn_not_nan(x: &Float) -> Ordering {
    // do not use mpfr::sgn as it contains a check for NaN and is not const as
    // it can call set_erangeflag
    if zero_p(x) {
        Ordering::Equal
    } else if signbit(x) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

#[inline]
pub const fn get_exp(op: &Float) -> exp_t {
    unsafe { mpfr::get_exp(op.as_raw()) }
}

#[cfg(feature = "nightly-float")]
pub fn get_f16(op: &Float, rnd: Round) -> f16 {
    const ZERO: MiniFloat = MiniFloat::const_from_f16(0.0);
    let mut small = ZERO;
    // SAFETY: we are not going to change precision.
    let small_mut = unsafe { small.as_nonreallocating_float() };
    let ordering = set(small_mut, op, rnd);
    let negative = small_mut.is_sign_negative();
    if small_mut.is_nan() {
        let n = f16::NAN;
        return if negative == n.is_sign_negative() {
            n
        } else {
            -n
        };
    }
    if small_mut.is_infinite() {
        return if negative {
            f16::NEG_INFINITY
        } else {
            f16::INFINITY
        };
    }
    if small_mut.is_zero() {
        return if negative { -0.0 } else { 0.0 };
    }
    let exp = small_mut.get_exp().expect("expected exponent");
    if exp > f16::MAX_EXP {
        return if negative {
            f16::NEG_INFINITY
        } else {
            f16::INFINITY
        };
    }
    let sign = if negative { 1u16 << 15 } else { 0 };
    if exp < f16::MIN_EXP {
        // Subnormal.
        small_mut.subnormalize_round(f16::MIN_EXP, ordering, rnd);
    }
    // Subnormalization may change exp because of rounding, so update it.
    if small_mut.is_zero() {
        return if negative { -0.0 } else { 0.0 };
    }
    let exp = small_mut.get_exp().expect("expected exponent");
    if exp < f16::MIN_EXP {
        // Still subnormal.

        // The number of significant bits is MANTISSA_DIGITS - (MIN_EXP - exp).
        // For example, if we are just subnormal; then exp == MIN_EXP - 1, and
        // we should have MANTISSA_DIGITS - 1 significant bits.
        let diff = f16::MIN_EXP.abs_diff(exp);
        if diff >= f16::MANTISSA_DIGITS {
            return if negative { -0.0 } else { 0.0 };
        }
        let nsig = f16::MANTISSA_DIGITS - diff;
        let mant = (small_mut.inner_data()[0] >> (limb_t::BITS - nsig)) as u16;

        return f16::from_bits(sign | mant);
    }
    // Normal. MANTISSA_DIGITS significant digits.
    let mant = (small_mut.inner_data()[0] >> (limb_t::BITS - f16::MANTISSA_DIGITS)) as u16;
    // This includes the implicit one, which acts as 1 in the exponent.
    // So we need to add (bias - 1) instead of bias to the exponent.
    // For example, if exp == MIN_EXP, we need to add 0, since the biased
    // exponent should be 1 for the smallest normal number.
    let biased_minus_1 = (exp - f16::MIN_EXP) as u16;
    let exp_mant = (biased_minus_1 << (f16::MANTISSA_DIGITS - 1)) + mant;
    f16::from_bits(sign + exp_mant)
}

pub fn get_f32(op: &Float, rnd: Round) -> f32 {
    unsafe { mpfr::get_flt(op.as_raw(), raw_round(rnd)) }
}

#[inline]
pub fn get_f64(op: &Float, rnd: Round) -> f64 {
    unsafe { mpfr::get_d(op.as_raw(), raw_round(rnd)) }
}

#[cfg(feature = "nightly-float")]
pub fn get_f128(op: &Float, rnd: Round) -> f128 {
    const ZERO: MiniFloat = MiniFloat::const_from_f128(0.0);
    let mut small = ZERO;
    // SAFETY: we are not going to change precision.
    let small_mut = unsafe { small.as_nonreallocating_float() };
    let ordering = set(small_mut, op, rnd);
    let negative = small_mut.is_sign_negative();
    if small_mut.is_nan() {
        let n = f128::NAN;
        return if negative == n.is_sign_negative() {
            n
        } else {
            -n
        };
    }
    if small_mut.is_infinite() {
        return if negative {
            f128::NEG_INFINITY
        } else {
            f128::INFINITY
        };
    }
    if small_mut.is_zero() {
        return if negative { -0.0 } else { 0.0 };
    }
    let exp = small_mut.get_exp().expect("expected exponent");
    if exp > f128::MAX_EXP {
        return if negative {
            f128::NEG_INFINITY
        } else {
            f128::INFINITY
        };
    }
    let sign = if negative { 1u128 << 127 } else { 0 };
    if exp < f128::MIN_EXP {
        // Subnormal.
        small_mut.subnormalize_round(f128::MIN_EXP, ordering, rnd);
    }
    // Subnormalization may change exp because of rounding, so update it.
    if small_mut.is_zero() {
        return if negative { -0.0 } else { 0.0 };
    }
    let exp = small_mut.get_exp().expect("expected exponent");
    if exp < f128::MIN_EXP {
        // Still subnormal.

        // The number of significant bits is MANTISSA_DIGITS - (MIN_EXP - exp).
        // For example, if we are just subnormal; then exp == MIN_EXP - 1, and
        // we should have MANTISSA_DIGITS - 1 significant bits.
        let diff = f128::MIN_EXP.abs_diff(exp);
        if diff >= f128::MANTISSA_DIGITS {
            return if negative { -0.0 } else { 0.0 };
        }
        let nsig = f128::MANTISSA_DIGITS - diff;
        let mant = {
            let data = small_mut.inner_data();
            #[cfg(gmp_limb_bits_64)]
            {
                ((data[1] as u128) << 64) | (data[0] as u128)
            }
            #[cfg(gmp_limb_bits_32)]
            {
                ((data[3] as u128) << 96)
                    | ((data[2] as u128) << 64)
                    | ((data[1] as u128) << 32)
                    | (data[0] as u128)
            }
        } >> (128 - nsig);

        return f128::from_bits(sign | mant);
    }
    // Normal. MANTISSA_DIGITS significant digits.
    let mant = {
        let data = small_mut.inner_data();
        #[cfg(gmp_limb_bits_64)]
        {
            ((data[1] as u128) << 64) | (data[0] as u128)
        }
        #[cfg(gmp_limb_bits_32)]
        {
            ((data[3] as u128) << 96)
                | ((data[2] as u128) << 64)
                | ((data[1] as u128) << 32)
                | (data[0] as u128)
        }
    } >> (128 - f128::MANTISSA_DIGITS);
    // This includes the implicit one, which acts as 1 in the exponent.
    // So we need to add (bias - 1) instead of bias to the exponent.
    // For example, if exp == MIN_EXP, we need to add 0, since the biased
    // exponent should be 1 for the smallest normal number.
    let biased_minus_1 = (exp - f128::MIN_EXP) as u128;
    let exp_mant = (biased_minus_1 << (f128::MANTISSA_DIGITS - 1)) + mant;
    f128::from_bits(sign + exp_mant)
}

#[inline]
pub fn get_f64_2exp(op: &Float, rnd: Round) -> (f64, c_long) {
    unsafe {
        let mut exp = MaybeUninit::uninit();
        let f = mpfr::get_d_2exp(exp.as_mut_ptr(), op.as_raw(), raw_round(rnd));
        let exp = if f.is_normal() { exp.assume_init() } else { 0 };
        (f, exp)
    }
}

#[inline]
pub fn get_si(op: &Float, rnd: Round) -> c_long {
    unsafe { mpfr::get_si(op.as_raw(), raw_round(rnd)) }
}

#[inline]
pub fn get_ui(op: &Float, rnd: Round) -> c_ulong {
    unsafe { mpfr::get_ui(op.as_raw(), raw_round(rnd)) }
}

#[inline]
pub const fn get_prec(op: &Float) -> prec_t {
    unsafe { mpfr::get_prec(op.as_raw()) }
}

#[inline]
pub fn get_str_ndigits(b: i32, prec: prec_t) -> usize {
    assert!((2..=62).contains(&b), "radix out of range");
    unsafe { mpfr::get_str_ndigits(b.into(), prec) }
}

#[inline]
pub fn cmp(op1: &Float, op2: &Float) -> Ordering {
    ordering1(unsafe { mpfr::cmp(op1.as_raw(), op2.as_raw()) })
}

#[inline]
pub fn cmpabs(op1: &Float, op2: &Float) -> Ordering {
    ordering1(unsafe { mpfr::cmpabs(op1.as_raw(), op2.as_raw()) })
}

#[inline]
pub fn cmp_si(op1: &Float, op2: c_long) -> Ordering {
    ordering1(unsafe { mpfr::cmp_si(op1.as_raw(), op2) })
}

#[inline]
pub fn cmp_ui(op1: &Float, op2: c_ulong) -> Ordering {
    ordering1(unsafe { mpfr::cmp_ui(op1.as_raw(), op2) })
}

#[inline]
pub fn cmp_i64(op1: &Float, op2: i64) -> Ordering {
    if let Some(op2) = op2.checked_as() {
        ordering1(unsafe { mpfr::cmp_si(op1.as_raw(), op2) })
    } else {
        let mut small = MiniFloat::from(op2);
        ordering1(unsafe { mpfr::cmp(op1.as_raw(), small.borrow_excl().as_raw()) })
    }
}

#[inline]
pub fn cmp_u64(op1: &Float, op2: u64) -> Ordering {
    if let Some(op2) = op2.checked_as() {
        ordering1(unsafe { mpfr::cmp_ui(op1.as_raw(), op2) })
    } else {
        let mut small = MiniFloat::from(op2);
        ordering1(unsafe { mpfr::cmp(op1.as_raw(), small.borrow_excl().as_raw()) })
    }
}

#[inline]
pub fn cmp_i128(op1: &Float, op2: i128) -> Ordering {
    let mut small = MiniFloat::from(op2);
    ordering1(unsafe { mpfr::cmp(op1.as_raw(), small.borrow_excl().as_raw()) })
}

#[inline]
pub fn cmp_u128(op1: &Float, op2: u128) -> Ordering {
    let mut small = MiniFloat::from(op2);
    ordering1(unsafe { mpfr::cmp(op1.as_raw(), small.borrow_excl().as_raw()) })
}

#[inline]
pub fn cmp_f64(op1: &Float, op2: f64) -> Ordering {
    ordering1(unsafe { mpfr::cmp_d(op1.as_raw(), op2) })
}

#[cfg(feature = "integer")]
#[inline]
pub fn cmp_z(op1: &Float, op2: &Integer) -> Ordering {
    ordering1(unsafe { mpfr::cmp_z(op1.as_raw(), op2.as_raw()) })
}

#[cfg(feature = "rational")]
#[inline]
pub fn get_q(rop: &mut Rational, op: &Float) {
    unsafe {
        mpfr::get_q(rop.as_raw_mut(), op.as_raw());
    }
}

#[cfg(feature = "rational")]
#[inline]
pub fn cmp_q(op1: &Float, op2: &Rational) -> Ordering {
    ordering1(unsafe { mpfr::cmp_q(op1.as_raw(), op2.as_raw()) })
}

#[inline]
pub fn cmp_u32_2exp(op1: &Float, op2: u32, e: i32) -> Ordering {
    ordering1(unsafe { mpfr::cmp_ui_2exp(op1.as_raw(), op2.into(), e.into()) })
}

#[inline]
pub fn cmp_i32_2exp(op1: &Float, op2: i32, e: i32) -> Ordering {
    ordering1(unsafe { mpfr::cmp_si_2exp(op1.as_raw(), op2.into(), e.into()) })
}

#[inline]
pub const fn signbit(op: &Float) -> bool {
    (unsafe { mpfr::signbit(op.as_raw()) }) != 0
}

#[inline]
pub fn equal_p(op1: &Float, op2: &Float) -> bool {
    (unsafe { mpfr::equal_p(op1.as_raw(), op2.as_raw()) }) != 0
}

#[inline]
pub fn unordered_p(op1: &Float, op2: &Float) -> bool {
    (unsafe { mpfr::unordered_p(op1.as_raw(), op2.as_raw()) }) != 0
}

#[inline]
pub fn less_p(op1: &Float, op2: &Float) -> bool {
    (unsafe { mpfr::less_p(op1.as_raw(), op2.as_raw()) }) != 0
}

#[inline]
pub fn lessequal_p(op1: &Float, op2: &Float) -> bool {
    (unsafe { mpfr::lessequal_p(op1.as_raw(), op2.as_raw()) }) != 0
}

#[inline]
pub fn greater_p(op1: &Float, op2: &Float) -> bool {
    (unsafe { mpfr::greater_p(op1.as_raw(), op2.as_raw()) }) != 0
}

#[inline]
pub fn greaterequal_p(op1: &Float, op2: &Float) -> bool {
    (unsafe { mpfr::greaterequal_p(op1.as_raw(), op2.as_raw()) }) != 0
}

#[inline]
pub fn integer_p(op: &Float) -> bool {
    (unsafe { mpfr::integer_p(op.as_raw()) }) != 0
}

#[inline]
pub const fn nan_p(op: &Float) -> bool {
    (unsafe { mpfr::nan_p(op.as_raw()) }) != 0
}

#[inline]
pub const fn inf_p(op: &Float) -> bool {
    (unsafe { mpfr::inf_p(op.as_raw()) }) != 0
}

#[inline]
pub const fn number_p(op: &Float) -> bool {
    (unsafe { mpfr::number_p(op.as_raw()) }) != 0
}

#[inline]
pub const fn zero_p(op: &Float) -> bool {
    (unsafe { mpfr::zero_p(op.as_raw()) }) != 0
}

#[inline]
pub const fn regular_p(op: &Float) -> bool {
    (unsafe { mpfr::regular_p(op.as_raw()) }) != 0
}

#[inline]
pub fn submul<O: OptFloat>(
    rop: &mut Float,
    add: O,
    mul1: &Float,
    mul2: &Float,
    rnd: Round,
) -> Ordering {
    let reverse_ord = fms(rop, mul1, mul2, add, rnd.reverse());
    if !zero_p(rop) {
        // the negation here is exact
        neg(rop, (), Round::Zero);
    }
    reverse_ord.reverse()
}

pub const EXP_ZERO: exp_t = -exp_t::MAX;
pub const EXP_NAN: exp_t = 1 - exp_t::MAX;
pub const EXP_INF: exp_t = 2 - exp_t::MAX;

#[inline]
#[cfg(feature = "rand")]
pub fn urandomb(rop: &mut Float, rng: &mut dyn MutRandState) {
    unsafe {
        let err = mpfr::urandomb(rop.as_raw_mut(), rng.private().0);
        assert_eq!(rop.is_nan(), err != 0);
    }
}

#[inline]
#[cfg(feature = "rand")]
pub fn urandom(rop: &mut Float, rng: &mut dyn MutRandState, rnd: Round) -> Ordering {
    ordering1(unsafe { mpfr::urandom(rop.as_raw_mut(), rng.private().0, raw_round(rnd)) })
}

#[inline]
#[cfg(feature = "rand")]
pub fn nrandom(rop: &mut Float, rng: &mut dyn MutRandState, rnd: Round) -> Ordering {
    ordering1(unsafe { mpfr::nrandom(rop.as_raw_mut(), rng.private().0, raw_round(rnd)) })
}

#[inline]
#[cfg(feature = "rand")]
pub fn erandom(rop: &mut Float, rng: &mut dyn MutRandState, rnd: Round) -> Ordering {
    ordering1(unsafe { mpfr::erandom(rop.as_raw_mut(), rng.private().0, raw_round(rnd)) })
}

#[inline]
pub fn si_sub<O: OptFloat>(rop: &mut Float, op1: c_long, op2: O, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op2 = op2.mpfr_or(rop);
    ordering1(unsafe { mpfr::si_sub(rop, op1, op2, raw_round(rnd)) })
}

#[inline]
pub fn si_div<O: OptFloat>(rop: &mut Float, op1: c_long, op2: O, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op2 = op2.mpfr_or(rop);
    ordering1(unsafe { mpfr::si_div(rop, op1, op2, raw_round(rnd)) })
}

#[inline]
pub fn ui_sub<O: OptFloat>(rop: &mut Float, op1: c_ulong, op2: O, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op2 = op2.mpfr_or(rop);
    ordering1(unsafe { mpfr::ui_sub(rop, op1, op2, raw_round(rnd)) })
}

#[inline]
pub fn ui_div<O: OptFloat>(rop: &mut Float, op1: c_ulong, op2: O, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op2 = op2.mpfr_or(rop);
    ordering1(unsafe { mpfr::ui_div(rop, op1, op2, raw_round(rnd)) })
}

#[inline]
pub fn ui_pow<O: OptFloat>(rop: &mut Float, op1: c_ulong, op2: O, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op2 = op2.mpfr_or(rop);
    ordering1(unsafe { mpfr::ui_pow(rop, op1, op2, raw_round(rnd)) })
}

#[inline]
pub fn d_sub<O: OptFloat>(rop: &mut Float, op1: f64, op2: O, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op2 = op2.mpfr_or(rop);
    ordering1(unsafe { mpfr::d_sub(rop, op1, op2, raw_round(rnd)) })
}

#[inline]
pub fn d_div<O: OptFloat>(rop: &mut Float, op1: f64, op2: O, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op2 = op2.mpfr_or(rop);
    ordering1(unsafe { mpfr::d_div(rop, op1, op2, raw_round(rnd)) })
}

#[inline]
#[cfg(feature = "integer")]
pub fn add_z<O: OptFloat>(rop: &mut Float, op1: O, op2: &Integer, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op1 = op1.mpfr_or(rop);
    ordering1(unsafe { mpfr::add_z(rop, op1, op2.as_raw(), raw_round(rnd)) })
}

#[inline]
#[cfg(feature = "integer")]
pub fn sub_z<O: OptFloat>(rop: &mut Float, op1: O, op2: &Integer, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op1 = op1.mpfr_or(rop);
    ordering1(unsafe { mpfr::sub_z(rop, op1, op2.as_raw(), raw_round(rnd)) })
}

#[inline]
#[cfg(feature = "integer")]
pub fn z_sub<O: OptFloat>(rop: &mut Float, op1: &Integer, op2: O, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op2 = op2.mpfr_or(rop);
    ordering1(unsafe { mpfr::z_sub(rop, op1.as_raw(), op2, raw_round(rnd)) })
}

#[inline]
#[cfg(feature = "integer")]
pub fn mul_z<O: OptFloat>(rop: &mut Float, op1: O, op2: &Integer, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op1 = op1.mpfr_or(rop);
    ordering1(unsafe { mpfr::mul_z(rop, op1, op2.as_raw(), raw_round(rnd)) })
}

#[inline]
#[cfg(feature = "integer")]
pub fn div_z<O: OptFloat>(rop: &mut Float, op1: O, op2: &Integer, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op1 = op1.mpfr_or(rop);
    ordering1(unsafe { mpfr::div_z(rop, op1, op2.as_raw(), raw_round(rnd)) })
}

#[cfg(feature = "integer")]
pub fn z_div<O: OptFloat>(rop: &mut Float, op1: &Integer, op2: O, rnd: Round) -> Ordering {
    if let Some(op1) = op1.to_i32() {
        si_div(rop, op1.into(), op2, rnd)
    } else {
        let op1 = Float::with_val(op1.significant_bits(), op1);
        div(rop, &op1, op2, rnd)
    }
}

#[inline]
#[cfg(feature = "integer")]
pub fn pow_z<O: OptFloat>(rop: &mut Float, op1: O, op2: &Integer, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op1 = op1.mpfr_or(rop);
    ordering1(unsafe { mpfr::pow_z(rop, op1, op2.as_raw(), raw_round(rnd)) })
}

#[inline]
#[cfg(feature = "rational")]
pub fn add_q<O: OptFloat>(rop: &mut Float, op1: O, op2: &Rational, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op1 = op1.mpfr_or(rop);
    ordering1(unsafe { mpfr::add_q(rop, op1, op2.as_raw(), raw_round(rnd)) })
}

#[inline]
#[cfg(feature = "rational")]
pub fn sub_q<O: OptFloat>(rop: &mut Float, op1: O, op2: &Rational, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op1 = op1.mpfr_or(rop);
    ordering1(unsafe { mpfr::sub_q(rop, op1, op2.as_raw(), raw_round(rnd)) })
}

#[cfg(feature = "rational")]
pub fn q_sub<O: OptFloat>(rop: &mut Float, op1: &Rational, op2: O, rnd: Round) -> Ordering {
    let reverse_ord = sub_q(rop, op2, op1, rnd.reverse());
    if !zero_p(rop) {
        // the negation here is exact
        neg(rop, (), Round::Zero);
    }
    reverse_ord.reverse()
}

#[inline]
#[cfg(feature = "rational")]
pub fn mul_q<O: OptFloat>(rop: &mut Float, op1: O, op2: &Rational, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op1 = op1.mpfr_or(rop);
    ordering1(unsafe { mpfr::mul_q(rop, op1, op2.as_raw(), raw_round(rnd)) })
}

#[inline]
#[cfg(feature = "rational")]
pub fn div_q<O: OptFloat>(rop: &mut Float, op1: O, op2: &Rational, rnd: Round) -> Ordering {
    let rop = rop.as_raw_mut();
    let op1 = op1.mpfr_or(rop);
    ordering1(unsafe { mpfr::div_q(rop, op1, op2.as_raw(), raw_round(rnd)) })
}

#[cfg(feature = "rational")]
pub fn q_div<O: OptFloat>(rop: &mut Float, op1: &Rational, op2: O, rnd: Round) -> Ordering {
    if op1.is_zero() {
        return ui_div(rop, 0, op2, rnd);
    }

    let op1_num = op1.numer();
    let op1_den = op1.denom();
    let op2_u = op2.unwrap_or(rop);

    // Multiply op1_den to op2 to get our denominator. But we need to check that
    // this product does not overflow if op2 has a large exponent (issue 85).
    if op2_u.is_normal() {
        let denom_prec = op1_den
            .significant_bits()
            .checked_add(op2_u.prec())
            .expect("overflow");

        let mut op2_raw = *op2_u.inner();
        let mut deferred_exp = 0;
        if op2_raw.exp > 0 {
            deferred_exp = op2_raw.exp;
            op2_raw.exp = 0;
        }
        let normalized_denom = {
            // Safety: normalized_op2 lives long enough
            let normalized_op2 = unsafe { BorrowFloat::from_raw(op2_raw) };
            Float::with_val(denom_prec, op1_den * &*normalized_op2)
        };
        let ordering_div = z_div(rop, op1_num, &normalized_denom, rnd);
        if deferred_exp > 0 {
            debug_assert!(rop.is_normal());
            let ordering_shift = {
                let rop = rop.as_raw_mut();
                unsafe { mpfr::div_2si(rop, rop, deferred_exp, rnd_t::RNDZ) }
            };
            if rop.is_normal() {
                // shifting was exact
                ordering_div
            } else {
                // shifting caused underflow, which is more significant than the division
                ordering1(ordering_shift)
            }
        } else {
            ordering_div
        }
    } else {
        // Any non-normal number multiplied by a positive integer is itself,
        // so the denominator = positive integer * op2 = op2
        z_div(rop, op1_num, op2, rnd)
    }
}
