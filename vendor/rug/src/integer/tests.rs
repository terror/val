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

use crate::integer::Order;
use crate::ops::{NegAssign, SubFrom};
use crate::{Assign, Complete, Integer};
use az::{Az, WrappingAs};

#[test]
fn check_int_conversions() {
    let mut i = Integer::from(-1);
    assert_eq!(i.to_u32_wrapping(), u32::MAX);
    assert_eq!(i.to_i32_wrapping(), -1);
    i.assign(0xff00_0000u32);
    i <<= 4;
    assert_eq!(i.to_u32_wrapping(), 0xf000_0000u32);
    assert_eq!(i.to_i32_wrapping(), 0xf000_0000u32.wrapping_as::<i32>());
    i = (i.clone() << 32) | i;
    assert_eq!(i.to_u32_wrapping(), 0xf000_0000u32);
    assert_eq!(i.to_i32_wrapping(), 0xf000_0000u32.wrapping_as::<i32>());
    i.neg_assign();
    assert_eq!(i.to_u32_wrapping(), 0x1000_0000u32);
    assert_eq!(i.to_i32_wrapping(), 0x1000_0000i32);
}

#[test]
fn check_option_conversion() {
    let mut i = Integer::new();
    assert_eq!(i.to_u32(), Some(0));
    assert_eq!(i.to_i32(), Some(0));
    assert_eq!(i.to_u64(), Some(0));
    assert_eq!(i.to_i64(), Some(0));
    i -= 1;
    assert_eq!(i.to_u32(), None);
    assert_eq!(i.to_i32(), Some(-1));
    assert_eq!(i.to_u64(), None);
    assert_eq!(i.to_i64(), Some(-1));

    i.assign(i32::MIN);
    assert_eq!(i.to_u32(), None);
    assert_eq!(i.to_i32(), Some(i32::MIN));
    assert_eq!(i.to_u64(), None);
    assert_eq!(i.to_i64(), Some(i64::from(i32::MIN)));
    i -= 1;
    assert_eq!(i.to_u32(), None);
    assert_eq!(i.to_i32(), None);
    assert_eq!(i.to_u64(), None);
    assert_eq!(i.to_i64(), Some(i64::from(i32::MIN) - 1));
    i.assign(i32::MAX);
    assert_eq!(i.to_u32(), Some(i32::MAX.wrapping_as::<u32>()));
    assert_eq!(i.to_i32(), Some(i32::MAX));
    assert_eq!(i.to_u64(), Some(i32::MAX.wrapping_as::<u64>()));
    assert_eq!(i.to_i64(), Some(i64::from(i32::MAX)));
    i += 1;
    assert_eq!(i.to_u32(), Some(i32::MAX.wrapping_as::<u32>() + 1));
    assert_eq!(i.to_i32(), None);
    assert_eq!(i.to_u64(), Some(i32::MAX.wrapping_as::<u64>() + 1));
    assert_eq!(i.to_i64(), Some(i64::from(i32::MAX) + 1));
    i.assign(u32::MAX);
    assert_eq!(i.to_u32(), Some(u32::MAX));
    assert_eq!(i.to_i32(), None);
    assert_eq!(i.to_u64(), Some(u64::from(u32::MAX)));
    assert_eq!(i.to_i64(), Some(i64::from(u32::MAX)));
    i += 1;
    assert_eq!(i.to_u32(), None);
    assert_eq!(i.to_i32(), None);
    assert_eq!(i.to_u64(), Some(u64::from(u32::MAX) + 1));
    assert_eq!(i.to_i64(), Some(i64::from(u32::MAX) + 1));

    i.assign(i64::MIN);
    assert_eq!(i.to_u32(), None);
    assert_eq!(i.to_i32(), None);
    assert_eq!(i.to_u64(), None);
    assert_eq!(i.to_i64(), Some(i64::MIN));
    i -= 1;
    assert_eq!(i.to_u32(), None);
    assert_eq!(i.to_i32(), None);
    assert_eq!(i.to_u64(), None);
    assert_eq!(i.to_i64(), None);
    i.assign(i64::MAX);
    assert_eq!(i.to_u32(), None);
    assert_eq!(i.to_i32(), None);
    assert_eq!(i.to_u64(), Some(i64::MAX.wrapping_as::<u64>()));
    assert_eq!(i.to_i64(), Some(i64::MAX));
    i += 1;
    assert_eq!(i.to_u32(), None);
    assert_eq!(i.to_i32(), None);
    assert_eq!(i.to_u64(), Some(i64::MAX.wrapping_as::<u64>() + 1));
    assert_eq!(i.to_i64(), None);
    i.assign(u64::MAX);
    assert_eq!(i.to_u32(), None);
    assert_eq!(i.to_i32(), None);
    assert_eq!(i.to_u64(), Some(u64::MAX));
    assert_eq!(i.to_i64(), None);
    i += 1;
    assert_eq!(i.to_u32(), None);
    assert_eq!(i.to_i32(), None);
    assert_eq!(i.to_u64(), None);
    assert_eq!(i.to_i64(), None);

    i.assign(i128::MIN);
    assert_eq!(i.to_u64(), None);
    assert_eq!(i.to_i64(), None);
    assert_eq!(i.to_u128(), None);
    assert_eq!(i.to_i128(), Some(i128::MIN));
    i -= 1;
    assert_eq!(i.to_u64(), None);
    assert_eq!(i.to_i64(), None);
    assert_eq!(i.to_u128(), None);
    assert_eq!(i.to_i128(), None);
    i.assign(i128::MAX);
    assert_eq!(i.to_u64(), None);
    assert_eq!(i.to_i64(), None);
    assert_eq!(i.to_u128(), Some(i128::MAX.wrapping_as::<u128>()));
    assert_eq!(i.to_i128(), Some(i128::MAX));
    i += 1;
    assert_eq!(i.to_u64(), None);
    assert_eq!(i.to_i64(), None);
    assert_eq!(i.to_u128(), Some(i128::MAX.wrapping_as::<u128>() + 1));
    assert_eq!(i.to_i128(), None);
    i.assign(u128::MAX);
    assert_eq!(i.to_u64(), None);
    assert_eq!(i.to_i64(), None);
    assert_eq!(i.to_u128(), Some(u128::MAX));
    assert_eq!(i.to_i128(), None);
    i += 1;
    assert_eq!(i.to_u64(), None);
    assert_eq!(i.to_i64(), None);
    assert_eq!(i.to_u128(), None);
    assert_eq!(i.to_i128(), None);
}

#[test]
fn check_float_conversions() {
    let mut i = Integer::from(0);
    assert_eq!(i.to_f32(), 0.0);
    assert_eq!(i.to_f64(), 0.0);
    i.assign(0xff);
    assert_eq!(i.to_f32(), 255.0);
    assert_eq!(i.to_f64(), 255.0);
    i <<= 80;
    assert_eq!(i.to_f32(), 255.0 * 2f32.powi(80));
    assert_eq!(i.to_f64(), 255.0 * 2f64.powi(80));
    i = (i.clone() << 30) | i;
    assert_eq!(i.to_f32(), 255.0 * 2f32.powi(110));
    assert_eq!(i.to_f64(), 255.0 * (2f64.powi(80) + 2f64.powi(110)));
    i <<= 100;
    assert_eq!(i.to_f32(), f32::INFINITY);
    assert_eq!(i.to_f64(), 255.0 * (2f64.powi(180) + 2f64.powi(210)));
    i <<= 1000;
    assert_eq!(i.to_f32(), f32::INFINITY);
    assert_eq!(i.to_f64(), f64::INFINITY);
    i.assign(-0xff_ffff);
    assert_eq!(i.to_f32(), (-0xff_ffff).az::<f32>());
    assert_eq!(i.to_f64(), f64::from(-0xff_ffff));
    i.assign(-0xfff_ffff);
    assert_eq!(i.to_f32(), (-0xfff_fff0).az::<f32>());
    assert_eq!(i.to_f64(), f64::from(-0xfff_ffff));
}

#[test]
fn check_from_str() {
    let mut i: Integer = "+134".parse().unwrap();
    assert_eq!(i, 134);
    i.assign(Integer::parse_radix("-ffFFffffFfFfffffffffffffffffffff", 16).unwrap());
    assert_eq!(i.significant_bits(), 128);
    i -= 1;
    assert_eq!(i.significant_bits(), 129);

    let bad_strings = [
        ("_1", 10, "invalid digit found in string"),
        ("+_1", 10, "invalid digit found in string"),
        ("-_1", 10, "invalid digit found in string"),
        ("+-3", 10, "invalid digit found in string"),
        ("-+3", 10, "invalid digit found in string"),
        ("++3", 10, "invalid digit found in string"),
        ("--3", 10, "invalid digit found in string"),
        ("0+3", 10, "invalid digit found in string"),
        ("", 10, "string has no digits"),
        (" ", 10, "string has no digits"),
        ("9\09", 10, "invalid digit found in string"),
        ("80", 8, "invalid digit found in string"),
        ("0xf", 16, "invalid digit found in string"),
        ("9", 9, "invalid digit found in string"),
        ("/0", 36, "invalid digit found in string"),
        (":0", 36, "invalid digit found in string"),
        ("@0", 36, "invalid digit found in string"),
        ("[0", 36, "invalid digit found in string"),
        ("`0", 36, "invalid digit found in string"),
        ("{0", 36, "invalid digit found in string"),
        ("Z0", 35, "invalid digit found in string"),
        ("z0", 35, "invalid digit found in string"),
    ];
    for &(s, radix, msg) in &bad_strings {
        match Integer::parse_radix(s, radix) {
            Ok(o) => panic!(
                "\"{s}\" (radix {radix}) parsed correctly as {}, expected: {msg}",
                Integer::from(o),
            ),
            Err(e) => assert_eq!(e.to_string(), msg, "\"{s}\" (radix {radix})"),
        }
    }
    let good_strings = [
        ("0", 10, 0),
        ("+0", 16, 0),
        ("  + 1_2", 10, 12),
        ("  - 1_2", 10, -12),
        ("-0", 2, 0),
        ("99", 10, 99),
        ("+Cc", 16, 0xcc),
        ("-77", 8, -0o77),
        (" 1 2\n 3 4 ", 10, 1234),
        ("1_2__", 10, 12),
        ("z0", 36, 35 * 36),
        ("Z0", 36, 35 * 36),
    ];
    for &(s, radix, i) in &good_strings {
        match Integer::parse_radix(s, radix) {
            Ok(ok) => assert_eq!(Integer::from(ok), i),
            Err(err) => panic!("could not parse {s}: {err}"),
        }
    }
}

#[test]
fn check_formatting() {
    let mut i = Integer::from(11);
    assert_eq!(format!("{i}"), "11");
    assert_eq!(format!("{i:?}"), "11");
    assert_eq!(format!("{i:<10}"), "11        ");
    assert_eq!(format!("{i:>10}"), "        11");
    assert_eq!(format!("{i:10}"), "        11");
    assert_eq!(format!("{i:^10}"), "    11    ");
    assert_eq!(format!("{i:^11}"), "    11     ");
    assert_eq!(format!("{i:+}"), "+11");
    assert_eq!(format!("{i:b}"), "1011");
    assert_eq!(format!("{i:#b}"), "0b1011");
    assert_eq!(format!("{i:o}"), "13");
    assert_eq!(format!("{i:#o}"), "0o13");
    assert_eq!(format!("{i:x}"), "b");
    assert_eq!(format!("{i:X}"), "B");
    assert_eq!(format!("{i:8x}"), "       b");
    assert_eq!(format!("{i:08X}"), "0000000B");
    assert_eq!(format!("{i:#08x}"), "0x00000b");
    assert_eq!(format!("{i:#8X}"), "     0xB");
    i.assign(-11);
    assert_eq!(format!("{i}"), "-11");
    assert_eq!(format!("{i:?}"), "-11");
    assert_eq!(format!("{i:+}"), "-11");
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

    #[cfg(feature = "float")]
    {
        // one "1" followed by 1000 "0"s
        i.assign(Integer::u_pow_u(10, 1000));
        assert_eq!(format!("{i:.1e}"), "1e1000");
        assert_eq!(format!("{i:.2E}"), "1.0E1000");
        i -= Integer::from(Integer::u_pow_u(10, 990));
        // ten "9"s followed by 990 "0"s
        assert_eq!(format!("{i:.1e}"), "1e1000");
        assert_eq!(format!("{i:+.2E}"), "+1.0E1000");
        assert_eq!(format!("{i:.9e}"), "1.00000000e1000");
        assert_eq!(format!("{i:.10E}"), "9.999999999E999");
        i.neg_assign();
        assert_eq!(format!("{i:.9E}"), "-1.00000000E1000");
        assert_eq!(format!("{i:.10e}"), "-9.999999999e999");

        i.assign(0);
        assert_eq!(format!("{i:.1e}"), "0");
    }
}

#[test]
fn check_to_digits_bool() {
    const T: bool = true;
    const F: bool = false;

    let i = Integer::from(0b111_0010);
    assert_eq!(i.significant_digits::<bool>(), 7);
    let mut buf: [bool; 10] = [true; 10];

    i.write_digits(&mut buf, Order::Lsf);
    assert_eq!(buf, [F, T, F, F, T, T, T, F, F, F]);
    i.write_digits(&mut buf, Order::LsfLe);
    assert_eq!(buf, [F, T, F, F, T, T, T, F, F, F]);
    i.write_digits(&mut buf, Order::LsfBe);
    assert_eq!(buf, [F, T, F, F, T, T, T, F, F, F]);
    i.write_digits(&mut buf, Order::Msf);
    assert_eq!(buf, [F, F, F, T, T, T, F, F, T, F]);
    i.write_digits(&mut buf, Order::MsfLe);
    assert_eq!(buf, [F, F, F, T, T, T, F, F, T, F]);
    i.write_digits(&mut buf, Order::MsfBe);
    assert_eq!(buf, [F, F, F, T, T, T, F, F, T, F]);

    #[cfg(feature = "std")]
    {
        let vec: Vec<bool> = i.to_digits(Order::MsfBe);
        assert_eq!(vec, [T, T, T, F, F, T, F]);
    }
}

#[test]
fn check_from_digits_bool() {
    const T: bool = true;
    const F: bool = false;

    let mut i = Integer::from_digits(&[T, T, T, F, F, T, F], Order::MsfBe);
    assert_eq!(i, 0b111_0010);

    i.assign_digits(&[T, F, F, F, T, T, T, F, F, F], Order::Lsf);
    assert_eq!(i, 0b111_0001);
    i.assign_digits(&[F, F, F, T, F, F, F, T, T, T], Order::Msf);
    assert_eq!(i, 0b100_0111);
    i.assign_digits(&[T, F, F, F, T, T, T, F, F, F], Order::LsfLe);
    assert_eq!(i, 0b111_0001);
    i.assign_digits(&[F, F, F, T, F, F, F, T, T, T], Order::MsfLe);
    assert_eq!(i, 0b100_0111);
    i.assign_digits(&[T, F, F, F, T, T, T, F, F, F], Order::LsfBe);
    assert_eq!(i, 0b111_0001);
    i.assign_digits(&[F, F, F, T, F, F, F, T, T, T], Order::MsfBe);
    assert_eq!(i, 0b100_0111);
}

#[test]
fn check_to_digits_u8() {
    let i = Integer::from(0x01_02_03_04_05_06_07_08u64);
    assert_eq!(i.significant_digits::<u8>(), 8);
    let mut buf: [u8; 10] = [0xff; 10];

    i.write_digits(&mut buf, Order::Lsf);
    assert_eq!(buf, [8, 7, 6, 5, 4, 3, 2, 1, 0, 0]);
    i.write_digits(&mut buf, Order::LsfLe);
    assert_eq!(buf, [8, 7, 6, 5, 4, 3, 2, 1, 0, 0]);
    i.write_digits(&mut buf, Order::LsfBe);
    assert_eq!(buf, [8, 7, 6, 5, 4, 3, 2, 1, 0, 0]);
    i.write_digits(&mut buf, Order::Msf);
    assert_eq!(buf, [0, 0, 1, 2, 3, 4, 5, 6, 7, 8]);
    i.write_digits(&mut buf, Order::MsfLe);
    assert_eq!(buf, [0, 0, 1, 2, 3, 4, 5, 6, 7, 8]);
    i.write_digits(&mut buf, Order::MsfBe);
    assert_eq!(buf, [0, 0, 1, 2, 3, 4, 5, 6, 7, 8]);

    #[cfg(feature = "std")]
    {
        let vec: Vec<u8> = i.to_digits(Order::MsfBe);
        assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7, 8]);
    }
}

#[test]
fn check_from_digits_u8() {
    let mut i = Integer::from_digits(&[0u8, 0, 1, 2, 3, 4, 5, 6, 7, 8], Order::MsfBe);
    assert_eq!(i, 0x01_02_03_04_05_06_07_08u64);

    i.assign_digits(&[1, 2, 3, 4, 5, 6, 7, 8, 0, 0u8], Order::Lsf);
    assert_eq!(i, 0x08_07_06_05_04_03_02_01u64);
    i.assign_digits(&[0u8, 0, 1, 2, 3, 4, 5, 6, 7, 8], Order::Msf);
    assert_eq!(i, 0x01_02_03_04_05_06_07_08u64);
    i.assign_digits(&[1, 2, 3, 4, 5, 6, 7, 8, 0, 0u8], Order::LsfLe);
    assert_eq!(i, 0x08_07_06_05_04_03_02_01u64);
    i.assign_digits(&[0u8, 0, 1, 2, 3, 4, 5, 6, 7, 8], Order::MsfLe);
    assert_eq!(i, 0x01_02_03_04_05_06_07_08u64);
    i.assign_digits(&[1, 2, 3, 4, 5, 6, 7, 8, 0, 0u8], Order::LsfBe);
    assert_eq!(i, 0x08_07_06_05_04_03_02_01u64);
    i.assign_digits(&[0u8, 0, 1, 2, 3, 4, 5, 6, 7, 8], Order::MsfBe);
    assert_eq!(i, 0x01_02_03_04_05_06_07_08u64);
}

#[test]
fn check_from_digits_resize() {
    // Ensure that we correctly handle increasing capacity
    let mut i = Integer::new();
    i.assign_digits(
        &[0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8],
        Order::Lsf,
    );
}

#[test]
fn check_to_digits_u16() {
    let le_0708 = 0x0708u16.to_le();
    let le_0506 = 0x0506u16.to_le();
    let le_0304 = 0x0304u16.to_le();
    let le_0102 = 0x0102u16.to_le();
    let be_0708 = 0x0708u16.to_be();
    let be_0506 = 0x0506u16.to_be();
    let be_0304 = 0x0304u16.to_be();
    let be_0102 = 0x0102u16.to_be();

    let i = Integer::from(0x0102_0304_0506_0708u64);
    assert_eq!(i.significant_digits::<u16>(), 4);
    let mut buf: [u16; 5] = [0xffff; 5];

    i.write_digits(&mut buf, Order::Lsf);
    assert_eq!(buf, [0x0708, 0x0506, 0x0304, 0x0102, 0]);
    i.write_digits(&mut buf, Order::LsfLe);
    assert_eq!(buf, [le_0708, le_0506, le_0304, le_0102, 0]);
    i.write_digits(&mut buf, Order::LsfBe);
    assert_eq!(buf, [be_0708, be_0506, be_0304, be_0102, 0]);
    i.write_digits(&mut buf, Order::Msf);
    assert_eq!(buf, [0, 0x0102, 0x0304, 0x0506, 0x0708]);
    i.write_digits(&mut buf, Order::MsfLe);
    assert_eq!(buf, [0, le_0102, le_0304, le_0506, le_0708]);
    i.write_digits(&mut buf, Order::MsfBe);
    assert_eq!(buf, [0, be_0102, be_0304, be_0506, be_0708]);

    #[cfg(feature = "std")]
    {
        let vec: Vec<u16> = i.to_digits(Order::MsfBe);
        assert_eq!(*vec, [be_0102, be_0304, be_0506, be_0708]);
    }
}

#[test]
fn check_from_digits_u16() {
    let le_0708 = 0x0708u16.to_le();
    let le_0506 = 0x0506u16.to_le();
    let le_0304 = 0x0304u16.to_le();
    let le_0102 = 0x0102u16.to_le();
    let be_0708 = 0x0708u16.to_be();
    let be_0506 = 0x0506u16.to_be();
    let be_0304 = 0x0304u16.to_be();
    let be_0102 = 0x0102u16.to_be();

    let mut i = Integer::from_digits(&[0u16, be_0102, be_0304, be_0506, be_0708], Order::MsfBe);
    assert_eq!(i, 0x0102_0304_0506_0708u64);

    i.assign_digits(&[0x0102, 0x0304, 0x0506, 0x0708, 0u16], Order::Lsf);
    assert_eq!(i, 0x0708_0506_0304_0102u64);
    i.assign_digits(&[0u16, 0x0102, 0x0304, 0x0506, 0x0708], Order::Msf);
    assert_eq!(i, 0x0102_0304_0506_0708u64);
    i.assign_digits(&[le_0102, le_0304, le_0506, le_0708, 0u16], Order::LsfLe);
    assert_eq!(i, 0x0708_0506_0304_0102u64);
    i.assign_digits(&[0u16, le_0102, le_0304, le_0506, le_0708], Order::MsfLe);
    assert_eq!(i, 0x0102_0304_0506_0708u64);
    i.assign_digits(&[be_0102, be_0304, be_0506, be_0708, 0u16], Order::LsfBe);
    assert_eq!(i, 0x0708_0506_0304_0102u64);
    i.assign_digits(&[0u16, be_0102, be_0304, be_0506, be_0708], Order::MsfBe);
    assert_eq!(i, 0x0102_0304_0506_0708u64);
}

#[test]
fn check_to_digits_u128() {
    let le_2222 = 0x2222u128.to_le();
    let le_1111 = 0x1111u128.to_le();
    let be_2222 = 0x2222u128.to_be();
    let be_1111 = 0x1111u128.to_be();

    let i: Integer = (Integer::from(0x1111) << 256) | 0x2222;
    assert_eq!(i.significant_digits::<u128>(), 3);
    let mut buf: [u128; 5] = [0xffff; 5];

    i.write_digits(&mut buf, Order::Lsf);
    assert_eq!(buf, [0x2222, 0, 0x1111, 0, 0]);
    i.write_digits(&mut buf, Order::LsfLe);
    assert_eq!(buf, [le_2222, 0, le_1111, 0, 0]);
    i.write_digits(&mut buf, Order::LsfBe);
    assert_eq!(buf, [be_2222, 0, be_1111, 0, 0]);
    i.write_digits(&mut buf, Order::Msf);
    assert_eq!(buf, [0, 0, 0x1111, 0, 0x2222]);
    i.write_digits(&mut buf, Order::MsfLe);
    assert_eq!(buf, [0, 0, le_1111, 0, le_2222]);
    i.write_digits(&mut buf, Order::MsfBe);
    assert_eq!(buf, [0, 0, be_1111, 0, be_2222]);

    #[cfg(feature = "std")]
    {
        let vec: Vec<u128> = i.to_digits(Order::MsfBe);
        assert_eq!(*vec, [be_1111, 0, be_2222]);
    }
}

#[test]
fn check_from_digits_u128() {
    let le_2222 = 0x2222u128.to_le();
    let le_1111 = 0x1111u128.to_le();
    let be_2222 = 0x2222u128.to_be();
    let be_1111 = 0x1111u128.to_be();

    let i102: Integer = (Integer::from(0x1111) << 256) | 0x2222;
    let i201: Integer = (Integer::from(0x2222) << 256) | 0x1111;

    let mut i = Integer::from_digits(&[0u128, 0, be_1111, 0, be_2222], Order::MsfBe);
    assert_eq!(i, i102);

    i.assign_digits(&[0x1111, 0, 0x2222, 0, 0u128], Order::Lsf);
    assert_eq!(i, i201);
    i.assign_digits(&[0u128, 0, 0x1111, 0, 0x2222], Order::Msf);
    assert_eq!(i, i102);
    i.assign_digits(&[le_1111, 0, le_2222, 0, 0u128], Order::LsfLe);
    assert_eq!(i, i201);
    i.assign_digits(&[0u128, 0, le_1111, 0, le_2222], Order::MsfLe);
    assert_eq!(i, i102);
    i.assign_digits(&[be_1111, 0, be_2222, 0, 0u128], Order::LsfBe);
    assert_eq!(i, i201);
    i.assign_digits(&[0u128, 0, be_1111, 0, be_2222], Order::MsfBe);
    assert_eq!(i, i102);
}

#[test]
fn check_sum_dot_product() {
    // sum = 5 + 10 + 11 = 26
    // dot = 25 + 100 + 121 = 246
    // product = 5 * 10 * 11 = 550
    let numbers = &[Integer::from(5), Integer::from(10), Integer::from(11)];
    let sum = || Integer::sum(numbers.iter());
    let dot = || Integer::dot(numbers.iter().zip(numbers.iter()));
    let product = || Integer::product(numbers.iter());
    let mut n = Integer::new();

    n.assign(sum());
    assert_eq!(n, 26);
    assert_eq!(Integer::from(sum()), 26);
    assert_eq!(sum().complete(), 26);
    n.assign(100);
    n += sum();
    assert_eq!(n, 126);
    assert_eq!(n.clone() + sum(), 152);
    assert_eq!(sum() + n.clone(), 152);
    n -= sum();
    assert_eq!(n, 100);
    n.sub_from(sum());
    assert_eq!(n, -74);
    assert_eq!(n.clone() - sum(), -100);
    assert_eq!(sum() - n.clone(), 100);

    n.assign(dot());
    assert_eq!(n, 246);
    assert_eq!(Integer::from(dot()), 246);
    assert_eq!(dot().complete(), 246);
    n.assign(100);
    n += dot();
    assert_eq!(n, 346);
    assert_eq!(n.clone() + dot(), 592);
    assert_eq!(dot() + n.clone(), 592);
    n -= dot();
    assert_eq!(n, 100);
    n.sub_from(dot());
    assert_eq!(n, 146);
    assert_eq!(n.clone() - dot(), -100);
    assert_eq!(dot() - n.clone(), 100);

    n.assign(product());
    assert_eq!(n, 550);
    assert_eq!(Integer::from(product()), 550);
    assert_eq!(product().complete(), 550);
    n.assign(100);
    n *= product();
    assert_eq!(n, 55_000);
    assert_eq!(n.clone() * product(), 30_250_000);
    assert_eq!(product() * n, 30_250_000);
}

#[test]
fn check_shrink() {
    let mut i = Integer::from(1) << 999u32;
    let cap_new = i.capacity();
    assert!(cap_new >= 1000);
    i.shrink_to(10_000);
    assert_eq!(i.capacity(), cap_new);
    i >>= 800;
    i.shrink_to(1000);
    let cap_1000 = i.capacity();
    assert!(1000 <= cap_1000 && cap_1000 <= cap_new);
    i.shrink_to(500);
    let cap_500 = i.capacity();
    assert!(500 <= cap_500 && cap_500 < cap_1000);
    i.shrink_to_fit();
    let cap_200 = i.capacity();
    assert!(200 <= cap_200 && cap_200 < cap_500);
    i *= 0;
    assert_eq!(i.capacity(), cap_200);
    i.shrink_to_fit();
    assert_eq!(i.capacity(), 0);
}

#[test]
fn incomplete_issue_69() {
    let two = Integer::from(2);

    assert_eq!(two.clamp_ref(Integer::NEG_ONE, Integer::ONE).complete(), 1);
    assert_eq!(Integer::ONE.invert_ref(&two).unwrap().complete(), 1);
    assert_eq!(two.pow_mod_ref(Integer::ONE, &two).unwrap().complete(), 0);
}

#[cfg(feature = "rand")]
#[test]
fn incomplete_issue_69_rand() {
    use crate::rand::RandState;

    let mut rng = RandState::new();
    assert_eq!(Integer::ONE.random_below_ref(&mut rng).complete(), 0);
    assert_eq!(Integer::random_bits(0, &mut rng).complete(), 0);
}
