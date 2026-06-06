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

use crate::float;
use crate::float::tests::{Cmp, clear_nanflag, nanflag};
use crate::float::{FreeCache, Round, Special};
use crate::ops::{AddAssignRound, AssignRound, NegAssign, SubAssignRound, SubFrom, SubFromRound};
use crate::{Assign, Complex, Float};
use core::cmp::Ordering;
use core::panic::AssertUnwindSafe;
use core::ptr::NonNull;
use std::panic;

#[test]
fn check_from_str() {
    let mut c = Complex::new(53);
    c.assign(Complex::parse("(+0 -0)").unwrap());
    assert_eq!(c, (0, 0));
    assert!(c.real().is_sign_positive());
    assert!(c.imag().is_sign_negative());
    c.assign(Complex::parse("(5 6)").unwrap());
    assert_eq!(c, (5, 6));
    c.assign(Complex::parse_radix("(50 60)", 8).unwrap());
    assert_eq!(c, (0o50, 0o60));
    c.assign(Complex::parse_radix("33", 16).unwrap());
    assert_eq!(c, (0x33, 0));

    let bad_strings = [
        ("", 10, "string has no digits"),
        (
            "(0 0) 0",
            10,
            "string has more characters after closing bracket",
        ),
        (
            "(0 0 0)",
            10,
            "string has more than one separator inside brackets",
        ),
        ("(0) ", 10, "string has no separator inside brackets"),
        ("(, 0)", 10, "string has no real digits"),
        ("(0 )", 10, "string has no separator inside brackets"),
        ("(0, )", 10, "string has no imaginary digits"),
        (
            "(0,,0 )",
            10,
            "string has more than one separator inside brackets",
        ),
        (" ( 2)", 10, "string has no separator inside brackets"),
        (
            "+(1 1)",
            10,
            "string is not a valid float: invalid digit found in string",
        ),
        (
            "-(1. 1.)",
            10,
            "string is not a valid float: invalid digit found in string",
        ),
        (
            "(f 1)",
            10,
            "real part of string is not a valid float: invalid digit found in string",
        ),
        (
            "(1 1@1a)",
            16,
            "imaginary part of string is not a valid float: invalid digit found in string",
        ),
        ("(8 )", 9, "string has no separator inside brackets"),
    ];
    for &(s, radix, msg) in &bad_strings {
        match Complex::parse_radix(s, radix) {
            Ok(o) => panic!(
                "\"{}\" (radix {}) parsed correctly as {}, expected: {}",
                s,
                radix,
                Complex::with_val(53, o),
                msg
            ),
            Err(e) => assert_eq!(e.to_string(), msg, "\"{s}\" (radix {radix})"),
        }
    }
    let good_strings = [
        ("(inf -@inf@)", 10, Cmp::inf(false), Cmp::inf(true)),
        ("(+0e99 1.)", 2, Cmp::F64(0.0), Cmp::F64(1.0)),
        ("(+ 0 e 99, .1)", 2, Cmp::F64(0.0), Cmp::F64(0.5)),
        ("-9.9e1", 10, Cmp::F64(-99.0), Cmp::F64(0.0)),
        (
            " ( -@nan@( _ ) nan( 0 n ) ) ",
            10,
            Cmp::Nan(true),
            Cmp::Nan(false),
        ),
    ];
    for &(s, radix, r, i) in &good_strings {
        match Complex::parse_radix(s, radix) {
            Ok(ok) => {
                let c = Complex::with_val(53, ok);
                assert_eq!(*c.real(), r, "real mismatch for {s}");
                assert_eq!(*c.imag(), i, "imaginary mismatch for {s}");
            }
            Err(e) => panic!("could not parse {s} because {e}"),
        }
    }

    float::free_cache(FreeCache::All);
}

#[test]
fn check_formatting() {
    let mut c = Complex::new((53, 53));
    c.assign((Special::Zero, Special::NegZero));
    assert_eq!(format!("{c}"), "(0 -0)");
    assert_eq!(format!("{c:?}"), "(0 -0)");
    assert_eq!(format!("{c:+}"), "(+0 -0)");
    assert_eq!(format!("{:+}", *c.as_neg()), "(-0 +0)");
    assert_eq!(format!("{c:<15}"), "(0 -0)         ");
    assert_eq!(format!("{c:>15}"), "         (0 -0)");
    assert_eq!(format!("{c:15}"), "         (0 -0)");
    assert_eq!(format!("{c:^15}"), "    (0 -0)     ");
    assert_eq!(format!("{c:^16}"), "     (0 -0)     ");
    c.assign((2.7, f64::NEG_INFINITY));
    assert_eq!(format!("{c:.2}"), "(2.7 -inf)");
    assert_eq!(format!("{c:+.8}"), "(+2.7000000 -inf)");
    assert_eq!(format!("{c:.4e}"), "(2.700e0 -inf)");
    assert_eq!(format!("{c:.4E}"), "(2.700E0 -inf)");
    assert_eq!(format!("{c:16.2}"), "      (2.7 -inf)");
    c.assign((-3.5, 11));
    assert_eq!(format!("{c:.4b}"), "(-11.10 1011)");
    assert_eq!(format!("{c:#.4b}"), "(-0b11.10 0b1011)");
    assert_eq!(format!("{c:.4o}"), "(-3.400 13.00)");
    assert_eq!(format!("{c:#.4o}"), "(-0o3.400 0o13.00)");
    c.assign((3.5, -11));
    assert_eq!(format!("{c:.2x}"), "(3.8 -b.0)");
    assert_eq!(format!("{c:#.2x}"), "(0x3.8 -0xb.0)");
    assert_eq!(format!("{c:.2X}"), "(3.8 -B.0)");
    assert_eq!(format!("{c:#.2X}"), "(0x3.8 -0xB.0)");

    float::free_cache(FreeCache::All);
}

#[test]
fn check_nanflag() {
    clear_nanflag();
    let re_nan = Complex::with_val(53, (Special::Nan, Special::Zero));
    let im_nan = Complex::with_val(53, (Special::Zero, Special::Nan));
    assert!(!nanflag());

    clear_nanflag();
    let c = re_nan.clone();
    assert!(c.real().is_nan() && !c.imag().is_nan());
    assert!(!nanflag());
    clear_nanflag();
    let c = im_nan.clone();
    assert!(!c.real().is_nan() && c.imag().is_nan());
    assert!(!nanflag());

    clear_nanflag();
    let mut m = Complex::new(53);
    assert!(!m.real().is_nan() && !m.imag().is_nan());
    assert!(!nanflag());
    m.clone_from(&re_nan);
    assert!(m.real().is_nan() && !m.imag().is_nan());
    assert!(!nanflag());
    m.assign(&re_nan);
    assert!(m.real().is_nan() && !m.imag().is_nan());
    assert!(nanflag());
    clear_nanflag();
    m.assign(re_nan.clone());
    assert!(m.real().is_nan() && !m.imag().is_nan());
    assert!(nanflag());
    clear_nanflag();
    let mut m = Complex::new(53);
    assert!(!m.real().is_nan() && !m.imag().is_nan());
    assert!(!nanflag());
    m.clone_from(&im_nan);
    assert!(!m.real().is_nan() && m.imag().is_nan());
    assert!(!nanflag());
    m.assign(&im_nan);
    assert!(!m.real().is_nan() && m.imag().is_nan());
    assert!(nanflag());
    clear_nanflag();
    m.assign(im_nan.clone());
    assert!(!m.real().is_nan() && m.imag().is_nan());
    assert!(nanflag());

    clear_nanflag();
    let c = Complex::with_val(53, -&re_nan);
    assert!(c.real().is_nan() && !c.imag().is_nan());
    assert!(nanflag());
    clear_nanflag();
    let c = Complex::with_val(53, -&im_nan);
    assert!(!c.real().is_nan() && c.imag().is_nan());
    assert!(nanflag());

    clear_nanflag();
    let mut m = re_nan.clone();
    m.neg_assign();
    assert!(m.real().is_nan() && !m.imag().is_nan());
    assert!(nanflag());
    clear_nanflag();
    let mut m = im_nan.clone();
    m.neg_assign();
    assert!(!m.real().is_nan() && m.imag().is_nan());
    assert!(nanflag());

    // as_neg, as_conj, as_mul_i shouldn't set nan flag from version 1.24.0 onwards
    clear_nanflag();

    let a = re_nan.as_neg();
    assert!(a.real().is_nan() && !a.imag().is_nan());
    assert!(!nanflag());
    let a = im_nan.as_neg();
    assert!(!a.real().is_nan() && a.imag().is_nan());
    assert!(!nanflag());

    let a = re_nan.as_conj();
    assert!(a.real().is_nan() && !a.imag().is_nan());
    assert!(!nanflag());
    let a = im_nan.as_conj();
    assert!(!a.real().is_nan() && a.imag().is_nan());
    assert!(!nanflag());

    let a = re_nan.as_mul_i(false);
    assert!(!a.real().is_nan() && a.imag().is_nan());
    assert!(!nanflag());
    let a = im_nan.as_mul_i(true);
    assert!(a.real().is_nan() && !a.imag().is_nan());
    assert!(!nanflag());
}

#[test]
fn check_sum_dot() {
    // sum = (2, 1.25) + (-0.125, 2) = (1.875, 3.25)
    // dot = (2, 1.25) * (-0.125, 2) * 2 = (-5.5, 7.6875)
    let numbers = &[
        Complex::with_val(53, (2, 1.25)),
        Complex::with_val(53, (-0.125, 2)),
    ];
    let sum = || Complex::sum(numbers.iter());
    let dot = || Complex::dot(numbers.iter().zip(numbers.iter().rev()));
    let mut n = Complex::new(53);
    let mut n3 = Complex::new(3);
    let down = (Round::Down, Round::Down);
    let up = (Round::Up, Round::Up);
    let less = (Ordering::Less, Ordering::Less);
    let greater = (Ordering::Greater, Ordering::Greater);

    n.assign(sum());
    assert_eq!(n, (1.875, 3.25));
    n3.assign(sum());
    assert_eq!(n3, (2, 3));
    // (1.875, 3.25) down to (1.75, 3)
    assert_eq!(n3.assign_round(sum(), down), less);
    assert_eq!(n3, (1.75, 3));
    // (1.875, 3.25) up to (2, 3.5)
    assert_eq!(n3.assign_round(sum(), up), greater);
    assert_eq!(n3, (2, 3.5));
    // (1.875, 3.25) + (1.875, 3.25) = (3.75, 6.5)
    n += sum();
    assert_eq!(n, (3.75, 6.5));
    // (2, 3.5) + (1.875, 3.25) = (3.875, 6.75) down to (3.5, 6)
    assert_eq!(n3.add_assign_round(sum(), down), less);
    assert_eq!(n3, (3.5, 6));
    // (3.5, 6) + (1.875, 3.25) = (5.375, 9.25) up to (6, 10)
    assert_eq!(n3.add_assign_round(sum(), up), greater);
    assert_eq!(n3, (6, 10));
    // (3.75, 6.5) + (1.875, 3.25) = (5.625, 9.75)
    assert_eq!(n.clone() + sum(), (5.625, 9.75));
    assert_eq!(sum() + n.clone(), (5.625, 9.75));
    // (3.75, 6.5) - (1.875, 3.25) = (1.875, 3.25)
    n -= sum();
    assert_eq!(n, (1.875, 3.25));
    n.assign(10);
    // (1.875, 3.25) - 10 = (-8.125, 3.25)
    n.sub_from(sum());
    assert_eq!(n, (-8.125, 3.25));
    // (6, 10) - (1.875, 3.25) = (4.125, 6.75) down to (4, 6)
    assert_eq!(n3.sub_assign_round(sum(), down), less);
    assert_eq!(n3, (4, 6));
    // (4, 6) - (1.875, 3.25) = (2.125, 2.75) up to (2.5, 3)
    assert_eq!(n3.sub_assign_round(sum(), up), greater);
    assert_eq!(n3, (2.5, 3));
    n3.assign(10);
    // (1.875, 3.25) - 10 = (-8.125, 3.25) down to (-10, 3)
    assert_eq!(n3.sub_from_round(sum(), down), less);
    assert_eq!(n3, (-10, 3));
    n3.assign(10);
    // (1.875, 3.25) - 10 = (-8.125, 3.25) UP to (-8, 3.5)
    assert_eq!(n3.sub_from_round(sum(), up), greater);
    assert_eq!(n3, (-8, 3.5));
    // (-8.125, 3.25) - (1.875, 3.25) = -10
    assert_eq!(n.clone() - sum(), -10);
    // (1.875, 3.25) - (-8.125, 3.25) = 10
    assert_eq!(sum() - n.clone(), 10);

    n.assign(dot());
    assert_eq!(n, (-5.5, 7.6875));
    n3.assign(dot());
    assert_eq!(n3, (-6, 8));
    // (-5.5, 7.6875) down to (-6, 7)
    assert_eq!(n3.assign_round(dot(), down), less);
    assert_eq!(n3, (-6, 7));
    // (-5.5, 7.6875) up to (-5, 8)
    assert_eq!(n3.assign_round(dot(), up), greater);
    assert_eq!(n3, (-5, 8));
    // (-5.5, 7.6875) + (-5.5, 7.6875) = (-11, 15.375)
    n += dot();
    assert_eq!(n, (-11, 15.375));
    // (-5, 8) + (-5.5, 7.6875) = (-10.5, 15.6875) down to (-12, 14)
    assert_eq!(n3.add_assign_round(dot(), down), less);
    assert_eq!(n3, (-12, 14));
    // (-12, 14) + (-5.5, 7.6875) = (-17.5, 21.6875) up to (-16, 24)
    assert_eq!(n3.add_assign_round(dot(), up), greater);
    assert_eq!(n3, (-16, 24));
    // (-11, 15.375) + (-5.5, 7.6875) = (-16.5, 23.0625)
    assert_eq!(n.clone() + dot(), (-16.5, 23.0625));
    assert_eq!(dot() + n.clone(), (-16.5, 23.0625));
    // (-11, 15.375) - (-5.5, 7.6875) = (-5.5, 7.6875)
    n -= dot();
    assert_eq!(n, (-5.5, 7.6875));
    n.assign(10);
    // (-5.5, 7.6875) - 10 = (-15.5, 7.6875)
    n.sub_from(dot());
    assert_eq!(n, (-15.5, 7.6875));
    // (-16, 24) - (-5.5, 7.6875) = (-10.5, 16.3125) down to (-12, 16)
    assert_eq!(n3.sub_assign_round(dot(), down), less);
    assert_eq!(n3, (-12, 16));
    // (-12, 16) - (-5.5, 7.6875) = (-6.5, 8.3125) up to (-6, 10)
    assert_eq!(n3.sub_assign_round(dot(), up), greater);
    assert_eq!(n3, (-6, 10));
    n3.assign(20);
    // (-5.5, 7.6875) - 20 = (-25.5, 7.6875) down to (-28, 7)
    assert_eq!(n3.sub_from_round(dot(), down), less);
    assert_eq!(n3, (-28, 7));
    // (-5.5, 7.6875) - (-28, 7) = (23.5, 0.6875) up to (24, 0.75)
    assert_eq!(n3.sub_from_round(dot(), up), greater);
    assert_eq!(n3, (24, 0.75));
    // (-15.5, 7.6875) - (-5.5, 7.6875) = -10
    assert_eq!(n.clone() - dot(), -10);
    // (-5.5, 7.6875) - (-15.5, 7.6875) = 10
    assert_eq!(dot() - n, 10);
}

#[test]
fn check_unwind_safety() {
    let mut real = Float::with_val(53, 42);
    let old_data = real.inner().d;
    let mut new_data = NonNull::dangling();
    panic::catch_unwind(AssertUnwindSafe(|| {
        Complex::mutate_real_imag(&mut real, &mut Float::new(53), |c| {
            let new_val = Float::new(53);
            #[allow(unused_assignments)]
            {
                new_data = new_val.inner().d;
            }
            // drop old value
            *c.mut_real() = new_val;
            panic!();
        });
    }))
    .unwrap_err();
    let data_at_end = real.inner().d;
    assert_ne!(data_at_end, old_data);
    assert_eq!(data_at_end, new_data);
}
