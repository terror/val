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

#![allow(clippy::float_cmp)]

use crate::float;
use crate::float::{FreeCache, Round, Special};
use crate::ops::{AddAssignRound, AssignRound, NegAssign, SubAssignRound, SubFrom, SubFromRound};
#[cfg(feature = "std")]
#[cfg(feature = "rand")]
use crate::rand::{RandGen, RandState};
use crate::{Assign, Float};
use az::Az;
use core::cmp::Ordering;
use core::fmt::{Debug, Error as FmtError, Formatter};
use gmp_mpfr_sys::{gmp, mpfr};

pub fn nanflag() -> bool {
    unsafe { mpfr::nanflag_p() != 0 }
}

pub fn clear_nanflag() {
    unsafe {
        mpfr::clear_nanflag();
    }
}

#[derive(Clone, Copy)]
pub enum Cmp {
    F64(f64),
    Nan(bool),
}

impl Cmp {
    pub fn inf(neg: bool) -> Cmp {
        Cmp::F64(if neg {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        })
    }
}

impl Debug for Cmp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match *self {
            Cmp::F64(ref val) => val.fmt(f),
            Cmp::Nan(negative) => {
                let s = if negative { "-NaN" } else { "NaN" };
                s.fmt(f)
            }
        }
    }
}

impl PartialEq<Cmp> for Float {
    fn eq(&self, other: &Cmp) -> bool {
        match *other {
            Cmp::F64(ref f) => self.eq(f),
            Cmp::Nan(negative) => self.is_nan() && self.is_sign_negative() == negative,
        }
    }
}

#[test]
fn check_from_str() {
    assert!(Float::with_val(53, Float::parse("-0").unwrap()).is_sign_negative());
    assert!(Float::with_val(53, Float::parse("+0").unwrap()).is_sign_positive());
    assert!(Float::with_val(53, Float::parse("1e1000").unwrap()).is_finite());
    let huge_hex = "1@99999999999999999999999999999999";
    assert!(Float::with_val(53, Float::parse_radix(huge_hex, 16).unwrap()).is_infinite());

    let bad_strings = [
        ("", 10, "string has no digits"),
        ("-", 10, "string has no digits"),
        ("+", 10, "string has no digits"),
        (".", 10, "string has no digits"),
        ("inf", 11, "invalid digit found in string"),
        ("@ nan @", 10, "string has no digits for significand"),
        ("inf", 16, "invalid digit found in string"),
        ("1.1.", 10, "more than one point found in string"),
        ("1e", 10, "string has no digits for exponent"),
        ("e10", 10, "string has no digits for significand"),
        (".e10", 10, "string has no digits for significand"),
        ("1e1.", 10, "string has point in exponent"),
        ("1e1e1", 10, "more than one exponent found in string"),
        ("1e+-1", 10, "invalid digit found in string"),
        ("1e-+1", 10, "invalid digit found in string"),
        ("+-1", 10, "invalid digit found in string"),
        ("-+1", 10, "invalid digit found in string"),
        ("infinit", 10, "invalid digit found in string"),
        ("1@1a", 16, "invalid digit found in string"),
        ("9", 9, "invalid digit found in string"),
        ("nan(20) x", 10, "invalid digit found in string"),
    ];
    for &(s, radix, msg) in &bad_strings {
        match Float::parse_radix(s, radix) {
            Ok(o) => panic!(
                "\"{}\" (radix {}) parsed correctly as {}, expected: {}",
                s,
                radix,
                Float::with_val(53, o),
                msg
            ),
            Err(e) => assert_eq!(e.to_string(), msg, "\"{s}\" (radix {radix})"),
        }
    }
    let good_strings = [
        ("INF", 10, Cmp::inf(false)),
        ("iNfIniTY", 10, Cmp::inf(false)),
        ("- @iNf@", 16, Cmp::inf(true)),
        ("+0e99", 2, Cmp::F64(0.0)),
        ("-9.9e1", 10, Cmp::F64(-99.0)),
        ("-.99e+2", 10, Cmp::F64(-99.0)),
        ("+99.e+0", 10, Cmp::F64(99.0)),
        ("-99@-1", 10, Cmp::F64(-9.9f64)),
        ("-a_b__.C_d_E_@3", 16, Cmp::F64(f64::from(-0xabcde))),
        ("1e1023", 2, Cmp::F64(2.0f64.powi(1023))),
        (" NaN() ", 10, Cmp::Nan(false)),
        (" + NaN (20 Number_Is) ", 10, Cmp::Nan(false)),
        (" - @nan@", 2, Cmp::Nan(true)),
    ];
    for &(s, radix, f) in &good_strings {
        match Float::parse_radix(s, radix) {
            Ok(ok) => assert_eq!(Float::with_val(53, ok), f),
            Err(err) => panic!("could not parse {s}: {err}"),
        }
    }

    float::free_cache(FreeCache::All);
}

#[test]
fn check_clamping() {
    let mut f = Float::new(4);

    // Both 1.00002 and 1.00001 are rounded to 1.0 with the same
    // rounding direction, so these work even though min > max.

    f.assign(-1);
    let dir = f.clamp_round(&1.00002, &1.00001, Round::Down);
    assert_eq!(f, 1.0);
    assert_eq!(dir, Ordering::Less);

    f.assign(-1);
    let dir = f.clamp_round(&1.00002, &1.00001, Round::Up);
    assert_eq!(f, 1.125);
    assert_eq!(dir, Ordering::Greater);

    f.assign(2);
    let dir = f.clamp_round(&1.00002, &1.00001, Round::Down);
    assert_eq!(f, 1.0);
    assert_eq!(dir, Ordering::Less);

    f.assign(2);
    let dir = f.clamp_round(&1.00002, &1.00001, Round::Up);
    assert_eq!(f, 1.125);
    assert_eq!(dir, Ordering::Greater);

    float::free_cache(FreeCache::All);
}

#[test]
#[should_panic(expected = "minimum larger than maximum")]
fn check_clamping_panic() {
    let mut f = Float::new(4);
    f.assign(-1);
    // Both 1.00001 and 0.99999 would be rounded to 1.0, but one
    // would be larger and the other would be smaller.
    let _panic = f.clamp(&1.00001, &0.99999);
}

#[test]
fn check_formatting() {
    let mut f = Float::with_val(53, Special::Zero);
    assert_eq!(format!("{f}"), "0");
    assert_eq!(format!("{f:e}"), "0");
    assert_eq!(format!("{f:?}"), "0");
    assert_eq!(format!("{f:+?}"), "+0");
    assert_eq!(format!("{f:<10}"), "0         ");
    assert_eq!(format!("{f:>10}"), "         0");
    assert_eq!(format!("{f:10}"), "         0");
    assert_eq!(format!("{f:^10}"), "    0     ");
    assert_eq!(format!("{f:^11}"), "     0     ");
    f.assign(Special::NegZero);
    assert_eq!(format!("{f}"), "-0");
    assert_eq!(format!("{f:?}"), "-0");
    assert_eq!(format!("{f:+?}"), "-0");
    f.assign(Special::Infinity);
    assert_eq!(format!("{f}"), "inf");
    assert_eq!(format!("{f:+}"), "+inf");
    assert_eq!(format!("{f:x}"), "@inf@");
    f.assign(Special::NegInfinity);
    assert_eq!(format!("{f}"), "-inf");
    assert_eq!(format!("{f:x}"), "-@inf@");
    f.assign(Special::Nan);
    assert_eq!(format!("{f}"), "NaN");
    assert_eq!(format!("{f:+}"), "+NaN");
    assert_eq!(format!("{f:x}"), "@NaN@");
    f = -f;
    assert_eq!(format!("{f}"), "-NaN");
    assert_eq!(format!("{f:x}"), "-@NaN@");
    f.assign(-2.75);
    assert_eq!(format!("{f:.1}"), "-3");
    assert_eq!(format!("{f:.2}"), "-2.8");
    assert_eq!(format!("{f:.4?}"), "-2.750");
    assert_eq!(format!("{f:.1e}"), "-3e0");
    assert_eq!(format!("{f:.2e}"), "-2.8e0");
    assert_eq!(format!("{f:.4e}"), "-2.750e0");
    assert_eq!(format!("{f:.4E}"), "-2.750E0");
    assert_eq!(format!("{f:.8b}"), "-10.110000");
    assert_eq!(format!("{f:.3b}"), "-11.0");
    assert_eq!(format!("{f:#.8b}"), "-0b10.110000");
    assert_eq!(format!("{f:.2o}"), "-2.6");
    assert_eq!(format!("{f:#.2o}"), "-0o2.6");
    assert_eq!(format!("{f:.2x}"), "-2.c");
    assert_eq!(format!("{f:.2X}"), "-2.C");
    assert_eq!(format!("{f:12.1x}"), "          -3");
    assert_eq!(format!("{f:12.2x}"), "        -2.c");
    assert_eq!(format!("{f:012.3X}"), "-00000002.C0");
    assert_eq!(format!("{f:#012.2x}"), "-0x0000002.c");
    assert_eq!(format!("{f:#12.2X}"), "      -0x2.C");
    f.assign(-27);
    assert_eq!(format!("{f:.1}"), "-3e1");
    assert_eq!(format!("{f:.2}"), "-27");
    assert_eq!(format!("{f:.4?}"), "-27.00");
    assert_eq!(format!("{f:.1e}"), "-3e1");
    assert_eq!(format!("{f:.2e}"), "-2.7e1");
    assert_eq!(format!("{f:.4e}"), "-2.700e1");
    assert_eq!(format!("{f:.4E}"), "-2.700E1");
    assert_eq!(format!("{f:.8b}"), "-11011.000");
    assert_eq!(format!("{f:.3b}"), "-1.11e4");
    assert_eq!(format!("{f:#.8b}"), "-0b11011.000");
    assert_eq!(format!("{f:.2o}"), "-33");
    assert_eq!(format!("{f:#.2o}"), "-0o33");
    assert_eq!(format!("{f:.2x}"), "-1b");
    assert_eq!(format!("{f:.2X}"), "-1B");
    assert_eq!(format!("{f:12.1x}"), "        -2@1");
    assert_eq!(format!("{f:12.2x}"), "         -1b");
    assert_eq!(format!("{f:012.3X}"), "-00000001B.0");
    assert_eq!(format!("{f:#012.2x}"), "-0x00000001b");
    assert_eq!(format!("{f:#12.2X}"), "       -0x1B");
    f <<= 144;
    assert_eq!(format!("{f:.8b}"), "-1.1011000e148");
    assert_eq!(format!("{f:.3b}"), "-1.11e148");
    assert_eq!(format!("{f:#.8b}"), "-0b1.1011000e148");
    assert_eq!(format!("{f:.2o}"), "-3.3e49");
    assert_eq!(format!("{f:#.2o}"), "-0o3.3e49");
    assert_eq!(format!("{f:.1x}"), "-2@37");
    assert_eq!(format!("{f:.2x}"), "-1.b@37");
    assert_eq!(format!("{f:.2X}"), "-1.B@37");
    assert_eq!(format!("{f:12.1x}"), "       -2@37");
    assert_eq!(format!("{f:12.2x}"), "     -1.b@37");
    assert_eq!(format!("{f:012.3X}"), "-00001.B0@37");
    assert_eq!(format!("{f:#012.2x}"), "-0x0001.b@37");
    assert_eq!(format!("{f:#12.2X}"), "   -0x1.B@37");

    float::free_cache(FreeCache::All);
}

#[test]
fn check_assumptions() {
    assert_eq!(unsafe { mpfr::custom_get_size(64) }, 8);
    assert!(unsafe { mpfr::custom_get_size(32) } <= gmp::NUMB_BITS.az::<usize>());

    float::free_cache(FreeCache::All);
}

#[test]
fn check_i_pow_u() {
    for &(i, u) in &[(13, 4), (13, 5), (-13, 4), (-13, 5)] {
        let p = Float::i_pow_u(i, u);
        let f = Float::with_val(53, p);
        assert_eq!(f, i.pow(u));
    }
}

#[test]
fn check_nanflag() {
    clear_nanflag();
    let nan = Float::with_val(53, Special::Nan);
    assert!(!nanflag());

    clear_nanflag();
    let c = nan.clone();
    assert!(c.is_nan());
    assert!(!nanflag());

    clear_nanflag();
    let mut m = Float::new(53);
    assert!(!m.is_nan());
    assert!(!nanflag());
    m.clone_from(&nan);
    assert!(m.is_nan());
    assert!(!nanflag());
    m.assign(&nan);
    assert!(m.is_nan());
    assert!(nanflag());
    clear_nanflag();
    m.assign(nan.clone());
    assert!(m.is_nan());
    assert!(nanflag());

    clear_nanflag();
    let c = Float::with_val(53, -&nan);
    assert!(c.is_nan());
    assert!(nanflag());

    clear_nanflag();
    let mut m = nan.clone();
    m.neg_assign();
    assert!(m.is_nan());
    assert!(nanflag());

    clear_nanflag();
    let c = Float::with_val(53, nan.clamp_ref(&0, &0));
    assert!(c.is_nan());
    assert!(nanflag());

    clear_nanflag();
    let mut m = nan.clone();
    m.clamp_mut(&0, &0);
    assert!(m.is_nan());
    assert!(nanflag());

    // as_neg, as_abs shouldn't set nan flag from version 1.24.0 onwards
    clear_nanflag();

    let a = nan.as_neg();
    assert!(a.is_nan());
    assert!(!nanflag());

    let a = nan.as_abs();
    assert!(a.is_nan());
    assert!(!nanflag());
}

#[cfg(feature = "std")]
#[cfg(feature = "rand")]
struct OnesZerosRand {
    one_words: u32,
}

#[cfg(feature = "std")]
#[cfg(feature = "rand")]
impl RandGen for OnesZerosRand {
    fn r#gen(&mut self) -> u32 {
        if self.one_words > 0 {
            self.one_words -= 1;
            !0
        } else {
            0
        }
    }
}

#[cfg(feature = "std")]
#[cfg(feature = "rand")]
#[test]
fn check_nan_random_bits() {
    use crate::ext::xmpfr;

    // Least significant 64 bits (two 32-bit words) of mantissa
    // will be ones, all others will be zeros. With 256 bits of
    // precision, the "random" number will be 0.0{192}1{64}. This
    // will be normalized to 0.1{64} * 2^-192.
    for i in 0..2 {
        let mut zeros_ones = OnesZerosRand { one_words: 2 };
        let mut rand = RandState::new_custom(&mut zeros_ones);
        let save_emin = xmpfr::get_emin();
        unsafe {
            mpfr::set_emin(-192 + i);
        }
        let f = Float::with_val(256, Float::random_bits(&mut rand));
        if i == 0 {
            assert_eq!(f, Float::with_val(64, !0u64) >> 256);
        } else {
            assert!(f.is_nan());
        }
        unsafe {
            mpfr::set_emin(save_emin);
        }
    }

    float::free_cache(FreeCache::All);
}

#[test]
fn check_sum_dot() {
    // sum = 2.5 - 10 = -7.5
    // dot = 2.5 * -10 + 2.5 * -10 = -50
    let numbers = &[Float::with_val(53, 2.5), Float::with_val(53, -10)];
    let sum = || Float::sum(numbers.iter());
    let dot = || Float::dot(numbers.iter().zip(numbers.iter().rev()));
    let mut n = Float::new(53);
    let mut n3 = Float::new(3);

    n.assign(sum());
    assert_eq!(n, -7.5);
    n3.assign(sum());
    assert_eq!(n3, -8);
    // -7.5 down to -8
    assert_eq!(n3.assign_round(sum(), Round::Down), Ordering::Less);
    assert_eq!(n3, -8);
    // -7.5 up to -7
    assert_eq!(n3.assign_round(sum(), Round::Up), Ordering::Greater);
    assert_eq!(n3, -7);
    // -7.5 - 7.5 = -15
    n += sum();
    assert_eq!(n, -15);
    // -7 - 7.5 = -14.5 down to -16
    assert_eq!(n3.add_assign_round(sum(), Round::Down), Ordering::Less);
    assert_eq!(n3, -16);
    // -16 - 7.5 = -23.5 up to -20
    assert_eq!(n3.add_assign_round(sum(), Round::Up), Ordering::Greater);
    assert_eq!(n3, -20);
    // -15 - 7.5 = -22.5
    assert_eq!(n.clone() + sum(), -22.5);
    assert_eq!(sum() + n.clone(), -22.5);
    // -15 - -7.5 = -7.5
    n -= sum();
    assert_eq!(n, -7.5);
    n.assign(10);
    // -7.5 - 10 = -17.5
    n.sub_from(sum());
    assert_eq!(n, -17.5);
    // -20 - -7.5 = -12.5 down to -14
    assert_eq!(n3.sub_assign_round(sum(), Round::Down), Ordering::Less);
    assert_eq!(n3, -14);
    // -14 - -7.5 = -6.5 up to -6
    assert_eq!(n3.sub_assign_round(sum(), Round::Up), Ordering::Greater);
    assert_eq!(n3, -6);
    n3.assign(10);
    // -7.5 - 10 = -17.5 down to -20
    assert_eq!(n3.sub_from_round(sum(), Round::Down), Ordering::Less);
    assert_eq!(n3, -20);
    // -7.5 - -20 = 13.5 up to 14
    assert_eq!(n3.sub_from_round(sum(), Round::Up), Ordering::Greater);
    assert_eq!(n3, 14);
    // -17.5 - -7.5 = -10
    assert_eq!(n.clone() - sum(), -10);
    // -7.5 - -17.5 = 10
    assert_eq!(sum() - n.clone(), 10);

    n.assign(dot());
    assert_eq!(n, -50);
    n3.assign(dot());
    assert_eq!(n3, -48);
    // -50 down to -56
    assert_eq!(n3.assign_round(dot(), Round::Down), Ordering::Less);
    assert_eq!(n3, -56);
    // -50 up to -48
    assert_eq!(n3.assign_round(dot(), Round::Up), Ordering::Greater);
    assert_eq!(n3, -48);
    // -50 - 50 = -100
    n += dot();
    assert_eq!(n, -100);
    // -48 - 50 = -98 down to -112
    assert_eq!(n3.add_assign_round(dot(), Round::Down), Ordering::Less);
    assert_eq!(n3, -112);
    // -112 - 50 = -162 up to -160
    assert_eq!(n3.add_assign_round(dot(), Round::Up), Ordering::Greater);
    assert_eq!(n3, -160);
    // -100 - 50 = -150
    assert_eq!(n.clone() + dot(), -150);
    assert_eq!(dot() + n.clone(), -150);
    // -100 - -50 = -50
    n -= dot();
    assert_eq!(n, -50);
    n.assign(10);
    // -50 - 10 = -60
    n.sub_from(dot());
    assert_eq!(n, -60);
    // -160 - -50 = -110 down to -112
    assert_eq!(n3.sub_assign_round(dot(), Round::Down), Ordering::Less);
    assert_eq!(n3, -112);
    // -112 - -50 = -62 up to -56
    assert_eq!(n3.sub_assign_round(dot(), Round::Up), Ordering::Greater);
    assert_eq!(n3, -56);
    n3.assign(20);
    // -50 - 20 = -70 down to -80
    assert_eq!(n3.sub_from_round(dot(), Round::Down), Ordering::Less);
    assert_eq!(n3, -80);
    // -50 - -80 = 30 up to 32
    assert_eq!(n3.sub_from_round(dot(), Round::Up), Ordering::Greater);
    assert_eq!(n3, 32);
    // -60 - -50 = -10
    assert_eq!(n.clone() - dot(), -10);
    // -50 - -60 = 10
    assert_eq!(dot() - n, 10);
}

#[test]
fn check_issue_56() {
    let x = Float::with_val(21, 1_111_111) >> 154u32;
    let rounded = x.to_f32_round(Round::Up);
    assert!(rounded >= x);
}

#[test]
fn check_remainder_quo31() {
    let num = Float::with_val(100, 1u32 << 31);
    let den = Float::with_val(100, -1);
    // remainder should be 0, quotient should be 0, not -1<<31, as it should be
    // 31 significant bits only
    let (remainder, quo31) = num.remainder_quo31(&den);
    assert_eq!(remainder, 0);
    assert_eq!(quo31, 0);

    let num = Float::with_val(100, 1);
    let den = Float::with_val(100, -1);
    let (remainder, quo31) = num.remainder_quo31(&den);
    assert_eq!(remainder, 0);
    assert_eq!(quo31, -1);
}

#[track_caller]
fn check_sub(factor: f64, prev_ord: Ordering, round: Round, expected: f64, ord: Ordering) {
    let mut f = Float::with_val(53, f64::from_bits(1)) * factor;
    assert_eq!(f.subnormalize_ieee_round(prev_ord, round), ord);
    assert_eq!(f, expected);
    assert_eq!(f.is_sign_negative(), expected.is_sign_negative());
}

#[track_caller]
fn check_sub_exact(factor: f64, expected: f64) {
    use Ordering::*;
    use Round::*;

    for prev_ord in [Less, Equal, Greater] {
        for round in [Nearest, Down, Up, Zero, AwayZero] {
            check_sub(factor, prev_ord, round, expected, prev_ord);
        }
    }
}

#[track_caller]
fn check_sub_up_down(factor: f64, expected_up: f64, expected_down: f64) {
    use Ordering::*;
    use Round::*;

    let (expected_zero, dir_zero, expected_away, dir_away) = if factor.is_sign_negative() {
        (expected_up, Greater, expected_down, Less)
    } else {
        (expected_down, Less, expected_up, Greater)
    };
    for prev_ord in [Less, Equal, Greater] {
        check_sub(factor, prev_ord, Down, expected_down, Less);
        check_sub(factor, prev_ord, Up, expected_up, Greater);
        check_sub(factor, prev_ord, Zero, expected_zero, dir_zero);
        check_sub(factor, prev_ord, AwayZero, expected_away, dir_away);
    }
}

#[test]
fn check_subnormalize_tiny() {
    use Ordering::*;
    use Round::*;

    let sub = f64::from_bits(1);

    check_sub_exact(1.0, sub);

    check_sub_up_down(0.75, sub, 0.0);
    check_sub(0.75, Less, Nearest, sub, Greater);
    check_sub(0.75, Equal, Nearest, sub, Greater);
    check_sub(0.75, Greater, Nearest, sub, Greater);

    check_sub_up_down(0.5, sub, 0.0);
    check_sub(0.5, Less, Nearest, sub, Greater);
    check_sub(0.5, Equal, Nearest, 0.0, Less);
    check_sub(0.5, Greater, Nearest, 0.0, Less);

    check_sub_up_down(0.25, sub, 0.0);
    check_sub(0.25, Less, Nearest, 0.0, Less);
    check_sub(0.25, Equal, Nearest, 0.0, Less);
    check_sub(0.25, Greater, Nearest, 0.0, Less);

    check_sub_exact(0.0, 0.0);

    check_sub_exact(-0.0, -0.0);

    check_sub_up_down(-0.25, -0.0, -sub);
    check_sub(-0.25, Less, Nearest, -0.0, Greater);
    check_sub(-0.25, Equal, Nearest, -0.0, Greater);
    check_sub(-0.25, Greater, Nearest, -0.0, Greater);

    check_sub_up_down(-0.5, -0.0, -sub);
    check_sub(-0.5, Less, Nearest, -0.0, Greater);
    check_sub(-0.5, Equal, Nearest, -0.0, Greater);
    check_sub(-0.5, Greater, Nearest, -sub, Less);

    check_sub_up_down(-0.75, -0.0, -sub);
    check_sub(-0.75, Less, Nearest, -sub, Less);
    check_sub(-0.75, Equal, Nearest, -sub, Less);
    check_sub(-0.75, Greater, Nearest, -sub, Less);

    check_sub_exact(-1.0, -sub);
}
