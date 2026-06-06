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

use crate::ops::SubFrom;
use crate::rational::MiniRational;
use crate::{Assign, Complete, Integer, Rational};
use core::panic::AssertUnwindSafe;
use std::panic;

#[test]
fn check_fract_trunc() {
    let ndwf = [
        (23, 10, 2, 3),
        (-23, 10, -2, -3),
        (20, 10, 2, 0),
        (-20, 10, -2, 0),
        (3, 10, 0, 3),
        (-3, 10, 0, -3),
        (0, 10, 0, 0),
    ];
    for &(n, d, whole, fract_n) in &ndwf {
        let r = Rational::from((n, d));

        let (fract, trunc) = r.clone().fract_trunc(Integer::new());
        assert_eq!(fract, MiniRational::from((fract_n, d)));
        assert_eq!(trunc, whole);

        let (fract, trunc) = <(Rational, Integer)>::from(r.fract_trunc_ref());
        assert_eq!(fract, MiniRational::from((fract_n, d)));
        assert_eq!(trunc, whole);

        let sep_fract = Rational::from(r.rem_trunc_ref());
        assert_eq!(sep_fract, MiniRational::from((fract_n, d)));
        let sep_trunc = Integer::from(r.trunc_ref());
        assert_eq!(sep_trunc, whole);

        let mut r = r;
        let mut trunc = Integer::new();
        r.fract_trunc_mut(&mut trunc);
        assert_eq!(r, MiniRational::from((fract_n, d)));
        assert_eq!(trunc, whole);
    }
}

#[test]
fn check_fract_ceil() {
    let ndwf = [
        (23, 10, 3, -7),
        (-23, 10, -2, -3),
        (20, 10, 2, 0),
        (-20, 10, -2, 0),
        (3, 10, 1, -7),
        (-3, 10, 0, -3),
        (0, 10, 0, 0),
    ];
    for &(n, d, whole, fract_n) in &ndwf {
        let r = Rational::from((n, d));

        let (fract, ceil) = r.clone().fract_ceil(Integer::new());
        assert_eq!(fract, MiniRational::from((fract_n, d)));
        assert_eq!(ceil, whole);

        let (fract, ceil) = <(Rational, Integer)>::from(r.fract_ceil_ref());
        assert_eq!(fract, MiniRational::from((fract_n, d)));
        assert_eq!(ceil, whole);

        let sep_fract = Rational::from(r.rem_ceil_ref());
        assert_eq!(sep_fract, MiniRational::from((fract_n, d)));
        let sep_ceil = Integer::from(r.ceil_ref());
        assert_eq!(sep_ceil, whole);

        let mut r = r;
        let mut ceil = Integer::new();
        r.fract_ceil_mut(&mut ceil);
        assert_eq!(r, MiniRational::from((fract_n, d)));
        assert_eq!(ceil, whole);
    }
}

#[test]
fn check_fract_floor() {
    let ndwf = [
        (23, 10, 2, 3),
        (-23, 10, -3, 7),
        (20, 10, 2, 0),
        (-20, 10, -2, 0),
        (3, 10, 0, 3),
        (-3, 10, -1, 7),
        (0, 10, 0, 0),
    ];
    for &(n, d, whole, fract_n) in &ndwf {
        let r = Rational::from((n, d));

        let (fract, floor) = r.clone().fract_floor(Integer::new());
        assert_eq!(fract, MiniRational::from((fract_n, d)));
        assert_eq!(floor, whole);

        let (fract, floor) = <(Rational, Integer)>::from(r.fract_floor_ref());
        assert_eq!(fract, MiniRational::from((fract_n, d)));
        assert_eq!(floor, whole);

        let sep_fract = Rational::from(r.rem_floor_ref());
        assert_eq!(sep_fract, MiniRational::from((fract_n, d)));
        let sep_floor = Integer::from(r.floor_ref());
        assert_eq!(sep_floor, whole);

        let mut r = r;
        let mut floor = Integer::new();
        r.fract_floor_mut(&mut floor);
        assert_eq!(r, MiniRational::from((fract_n, d)));
        assert_eq!(floor, whole);
    }
}

#[test]
fn check_fract_round() {
    let ndwf = [
        (27, 10, 3, -3),
        (-27, 10, -3, 3),
        (25, 10, 3, -5),
        (-25, 10, -3, 5),
        (23, 10, 2, 3),
        (-23, 10, -2, -3),
        (20, 10, 2, 0),
        (-20, 10, -2, 0),
        (3, 10, 0, 3),
        (-3, 10, 0, -3),
        (0, 10, 0, 0),
    ];
    for &(n, d, whole, fract_n) in &ndwf {
        let r = Rational::from((n, d));

        let (fract, round) = r.clone().fract_round(Integer::new());
        assert_eq!(fract, MiniRational::from((fract_n, d)));
        assert_eq!(round, whole);

        let (fract, round) = <(Rational, Integer)>::from(r.fract_round_ref());
        assert_eq!(fract, MiniRational::from((fract_n, d)));
        assert_eq!(round, whole);

        let sep_fract = Rational::from(r.rem_round_ref());
        assert_eq!(sep_fract, MiniRational::from((fract_n, d)));
        let sep_round = Integer::from(r.round_ref());
        assert_eq!(sep_round, whole);

        let mut r = r;
        let mut round = Integer::new();
        r.fract_round_mut(&mut round);
        assert_eq!(r, MiniRational::from((fract_n, d)));
        assert_eq!(round, whole);
    }
}

#[test]
fn check_from_str() {
    assert_eq!(
        "-13/7".parse::<Rational>().unwrap(),
        MiniRational::from((-13, 7))
    );

    let bad_strings = [
        ("_1", 10, "invalid digit found in string"),
        ("+_1", 10, "invalid digit found in string"),
        ("-_1", 10, "invalid digit found in string"),
        ("1/_1", 10, "invalid digit found in string"),
        ("+-3", 10, "invalid digit found in string"),
        ("-+3", 10, "invalid digit found in string"),
        ("++3", 10, "invalid digit found in string"),
        ("--3", 10, "invalid digit found in string"),
        ("0+3", 10, "invalid digit found in string"),
        ("", 10, "string has no digits"),
        (" ", 10, "string has no digits"),
        ("1/-1", 10, "invalid digit found in string"),
        ("1/+3", 10, "invalid digit found in string"),
        ("1/0", 10, "string has zero denominator"),
        ("/2", 10, "string has no digits for numerator"),
        ("2/", 10, "string has no digits for denominator"),
        ("2/2/", 10, "more than one / found in string"),
        ("1/80", 8, "invalid digit found in string"),
        ("0xf", 16, "invalid digit found in string"),
        ("9", 9, "invalid digit found in string"),
        (":0", 36, "invalid digit found in string"),
        ("/0", 36, "string has no digits for numerator"),
        (":0", 36, "invalid digit found in string"),
        ("@0", 36, "invalid digit found in string"),
        ("[0", 36, "invalid digit found in string"),
        ("`0", 36, "invalid digit found in string"),
        ("{0", 36, "invalid digit found in string"),
        ("Z0", 35, "invalid digit found in string"),
        ("z0", 35, "invalid digit found in string"),
    ];
    for &(s, radix, msg) in &bad_strings {
        match Rational::parse_radix(s, radix) {
            Ok(o) => panic!(
                "\"{}\" (radix {}) parsed correctly as {}, expected: {}",
                s,
                radix,
                Rational::from(o),
                msg
            ),
            Err(e) => assert_eq!(e.to_string(), msg, "\"{s}\" (radix {radix})"),
        }
    }
    let good_strings = [
        ("0", 10, 0, 1),
        ("+0/fC", 16, 0, 1),
        (" + 1 _ / 2 _ ", 10, 1, 2),
        (" - 1 _ / 2 _ ", 10, -1, 2),
        ("-0/10", 2, 0, 1),
        ("-99/3", 10, -33, 1),
        ("+Ce/fF", 16, 0xce, 0xff),
        ("-77/2", 8, -0o77, 2),
        ("Z/z0", 36, 1, 36),
    ];
    for &(s, radix, n, d) in &good_strings {
        match Rational::parse_radix(s, radix) {
            Ok(ok) => {
                let r = Rational::from(ok);
                assert_eq!(*r.numer(), n, "numerator mismatch for {s}");
                assert_eq!(*r.denom(), d, "denominator mismatch for {s}");
            }
            Err(err) => panic!("could not parse {s}: {err}"),
        }
    }
}

#[test]
fn check_formatting() {
    let r = Rational::from((-11, 15));
    assert_eq!(format!("{r}"), "-11/15");
    assert_eq!(format!("{r:?}"), "-11/15");
    assert_eq!(format!("{r:<10}"), "-11/15    ");
    assert_eq!(format!("{r:>10}"), "    -11/15");
    assert_eq!(format!("{r:10}"), "    -11/15");
    assert_eq!(format!("{r:^10}"), "  -11/15  ");
    assert_eq!(format!("{r:^11}"), "  -11/15   ");
    assert_eq!(format!("{r:b}"), "-1011/1111");
    assert_eq!(format!("{r:#b}"), "-0b1011/1111");
    assert_eq!(format!("{r:o}"), "-13/17");
    assert_eq!(format!("{r:#o}"), "-0o13/17");
    assert_eq!(format!("{r:x}"), "-b/f");
    assert_eq!(format!("{r:X}"), "-B/F");
    assert_eq!(format!("{r:8x}"), "    -b/f");
    assert_eq!(format!("{r:08X}"), "-0000B/F");
    assert_eq!(format!("{r:#08x}"), "-0x00b/f");
    assert_eq!(format!("{r:#8X}"), "  -0xB/F");
    let i = r * MiniRational::from(15).borrow_excl();
    assert_eq!(format!("{i}"), "-11");
    assert_eq!(format!("{i:?}"), "-11");
    assert_eq!(format!("{i:b}"), "-1011");
    assert_eq!(format!("{i:#b}"), "-0b1011");
    assert_eq!(format!("{i:o}"), "-13");
    assert_eq!(format!("{i:#o}"), "-0o13");
    assert_eq!(format!("{i:x}"), "-b");
    assert_eq!(format!("{i:X}"), "-B");
    assert_eq!(format!("{i:8x}"), "      -b");
    assert_eq!(format!("{i:08X}"), "-000000B");
    assert_eq!(format!("{i:#08x}"), "-0x0000b");
    assert_eq!(format!("{i:#8X}"), "    -0xB");
}

#[test]
fn check_sum_dot_product() {
    // sum = 5/2 - 1/3 = 13/6
    // dot = 25/4 + 1/9 = 229/36
    // product = 5/2 * -1/3 = -5/6
    let numbers = &[Rational::from((5, 2)), Rational::from((-1, 3))];
    let sum = || Rational::sum(numbers.iter());
    let dot = || Rational::dot(numbers.iter().zip(numbers.iter()));
    let product = || Rational::product(numbers.iter());
    let mut n = Rational::new();

    n.assign(sum());
    assert_eq!(n, MiniRational::from((13, 6)));
    assert_eq!(Rational::from(sum()), MiniRational::from((13, 6)));
    assert_eq!(sum().complete(), MiniRational::from((13, 6)));
    n.assign(10);
    n += sum();
    assert_eq!(n, MiniRational::from((73, 6)));
    assert_eq!(n.clone() + sum(), MiniRational::from((43, 3)));
    assert_eq!(sum() + n.clone(), MiniRational::from((43, 3)));
    n -= sum();
    assert_eq!(n, 10);
    n.sub_from(sum());
    assert_eq!(n, MiniRational::from((-47, 6)));
    assert_eq!(n.clone() - sum(), -10);
    assert_eq!(sum() - n.clone(), 10);

    n.assign(dot());
    assert_eq!(n, MiniRational::from((229, 36)));
    assert_eq!(Rational::from(dot()), MiniRational::from((229, 36)));
    assert_eq!(dot().complete(), MiniRational::from((229, 36)));
    n.assign(10);
    n += dot();
    assert_eq!(n, MiniRational::from((589, 36)));
    assert_eq!(n.clone() + dot(), MiniRational::from((409, 18)));
    assert_eq!(dot() + n.clone(), MiniRational::from((409, 18)));
    n -= dot();
    assert_eq!(n, 10);
    n.sub_from(dot());
    assert_eq!(n, MiniRational::from((-131, 36)));
    assert_eq!(n.clone() - dot(), -10);
    assert_eq!(dot() - n.clone(), 10);

    n.assign(product());
    assert_eq!(n, MiniRational::from((-5, 6)));
    assert_eq!(Rational::from(product()), MiniRational::from((-5, 6)));
    assert_eq!(product().complete(), MiniRational::from((-5, 6)));
    n.assign(10);
    n *= product();
    assert_eq!(n, MiniRational::from((-25, 3)));
    assert_eq!(n.clone() * product(), MiniRational::from((125, 18)));
    assert_eq!(product() * n, MiniRational::from((125, 18)));
}

#[test]
fn check_issue_47_unwind_safety() {
    let mut r = Rational::new();
    panic::catch_unwind(AssertUnwindSafe(|| {
        r.mutate_numer_denom(|num, den| {
            num.assign(2);
            den.assign(4);
            panic!();
        });
    }))
    .unwrap_err();
    assert_eq!(*r.numer(), 1);
    assert_eq!(*r.denom(), 2);
}

#[test]
fn check_issue_49_unwind_safety() {
    let mut r = Rational::new();
    panic::catch_unwind(AssertUnwindSafe(|| {
        r.mutate_numer_denom(|_, den| {
            den.assign(0);
        });
    }))
    .unwrap_err();
    assert_ne!(*r.denom(), 0);
}
