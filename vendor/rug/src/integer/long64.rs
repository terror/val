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

use crate::ext::xmpz;
use crate::misc;
use crate::misc::NegAbs;
#[cfg(feature = "rand")]
use crate::rand::MutRandState;
use crate::{Assign, Complete, Integer};
use az::{CheckedCast, StrictCast};
use core::ffi::c_ulong;
use gmp_mpfr_sys::gmp::bitcnt_t;

pub trait Sealed: Sized {}
impl Sealed for Integer {}

/// [`Integer`] extension trait with 64-bit alternatives of some methods.
///
/// Various [`Integer`] methods use 32-bit values for things like bit count or
/// exponents. On 64-bit platforms except Windows, alternatives of these methods
/// using 64-bit values are supported. This trait is only implemented on 64-bit
/// platforms except Windows.
///
/// This trait is sealed and is only implemented for [`Integer`].
pub trait IntegerExt64: Sealed {
    /// Converts to an [`f32`] and an exponent, rounding towards zero.
    ///
    /// The returned [`f32`] is in the range
    /// 0.5&nbsp;≤&nbsp;<i>x</i>&nbsp;<&nbsp;1. If the value is zero, `(0.0, 0)`
    /// is returned.
    ///
    /// This method is similar to [`to_f32_exp`][Integer::to_f32_exp] but
    /// returns the exponent as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let zero = Integer::new();
    /// let (d0, exp0) = zero.to_f32_exp64();
    /// assert_eq!((d0, exp0), (0.0, 0));
    /// let fifteen = Integer::from(15);
    /// let (d15, exp15) = fifteen.to_f32_exp64();
    /// assert_eq!((d15, exp15), (15.0 / 16.0, 4));
    /// ```
    fn to_f32_exp64(&self) -> (f32, u64);

    /// Converts to an [`f64`] and an exponent, rounding towards zero.
    ///
    /// The returned [`f64`] is in the range
    /// 0.5&nbsp;≤&nbsp;<i>x</i>&nbsp;<&nbsp;1. If the value is zero, `(0.0, 0)`
    /// is returned.
    ///
    /// This method is similar to [`to_f64_exp`][Integer::to_f64_exp] but
    /// returns the exponent as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let zero = Integer::new();
    /// let (d0, exp0) = zero.to_f64_exp64();
    /// assert_eq!((d0, exp0), (0.0, 0));
    /// let fifteen = Integer::from(15);
    /// let (d15, exp15) = fifteen.to_f64_exp64();
    /// assert_eq!((d15, exp15), (15.0 / 16.0, 4));
    /// ```
    fn to_f64_exp64(&self) -> (f64, u64);

    /// Returns [`true`] if the number is divisible by `divisor`. Unlike other
    /// division functions, `divisor` can be zero.
    ///
    /// This method is similar to [`is_divisible_u`][Integer::is_divisible_u]
    /// but takes the divisor as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let i = Integer::from(230);
    /// assert!(i.is_divisible_u64(23));
    /// assert!(!i.is_divisible_u64(100));
    /// assert!(!i.is_divisible_u64(0));
    /// ```
    fn is_divisible_u64(&self, divisor: u64) -> bool;

    /// Returns [`true`] if the number is divisible by 2<sup><i>b</i></sup>.
    ///
    /// This method is similar to
    /// [`is_divisible_2pow`][Integer::is_divisible_2pow] but takes `b` as
    /// [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let i = Integer::from(15 << 17);
    /// assert!(i.is_divisible_2pow_64(16));
    /// assert!(i.is_divisible_2pow_64(17));
    /// assert!(!i.is_divisible_2pow_64(18));
    /// ```
    fn is_divisible_2pow_64(&self, b: u64) -> bool;

    /// Returns [`true`] if the number is congruent to <i>c</i> mod
    /// <i>divisor</i>, that is, if there exists a <i>q</i> such that `self` =
    /// <i>c</i> + <i>q</i> × <i>divisor</i>. Unlike other division functions,
    /// `divisor` can be zero.
    ///
    /// This method is similar to [`is_congruent_u`][Integer::is_congruent_u]
    /// but takes `c` and the divisor as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let n = Integer::from(105);
    /// assert!(n.is_congruent_u64(3335, 10));
    /// assert!(!n.is_congruent_u64(107, 10));
    /// // n is congruent to itself if divisor is 0
    /// assert!(n.is_congruent_u64(105, 0));
    /// ```
    fn is_congruent_u64(&self, c: u64, divisor: u64) -> bool;

    /// Returns [`true`] if the number is congruent to <i>c</i> mod
    /// 2<sup><i>b</i></sup>, that is, if there exists a <i>q</i> such that
    /// `self` = <i>c</i> + <i>q</i> × 2<sup><i>b</i></sup>.
    ///
    /// This method is similar to
    /// [`is_congruent_2pow`][Integer::is_congruent_2pow] but takes `b` as
    /// [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let n = Integer::from(13 << 17 | 21);
    /// assert!(n.is_congruent_2pow_64(&Integer::from(7 << 17 | 21), 17));
    /// assert!(!n.is_congruent_2pow_64(&Integer::from(13 << 17 | 22), 17));
    /// ```
    fn is_congruent_2pow_64(&self, c: &Self, b: u64) -> bool;

    /// Returns the number of bits required to represent the absolute value.
    ///
    /// This method is similar to
    /// [`significant_bits`][Integer::significant_bits] but returns a [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    ///
    /// assert_eq!(Integer::from(0).significant_bits_64(), 0);  //    “”
    /// assert_eq!(Integer::from(1).significant_bits_64(), 1);  //   “1”
    /// assert_eq!(Integer::from(4).significant_bits_64(), 3);  // “100”
    /// assert_eq!(Integer::from(7).significant_bits_64(), 3);  // “111”
    /// assert_eq!(Integer::from(-1).significant_bits_64(), 1); //   “1”
    /// assert_eq!(Integer::from(-4).significant_bits_64(), 3); // “100”
    /// assert_eq!(Integer::from(-7).significant_bits_64(), 3); // “111”
    /// ```
    fn significant_bits_64(&self) -> u64;

    /// Returns the number of bits required to represent the value using a
    /// two’s-complement representation.
    ///
    /// For non-negative numbers, this method returns one more than
    /// the [`significant_bits_64`] method, since an extra zero is needed
    /// before the most significant bit.
    ///
    /// This method is similar to [`signed_bits`][Integer::signed_bits] but
    /// returns a [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    ///
    /// assert_eq!(Integer::from(-5).signed_bits_64(), 4); // “1011”
    /// assert_eq!(Integer::from(-4).signed_bits_64(), 3); //  “100”
    /// assert_eq!(Integer::from(-3).signed_bits_64(), 3); //  “101”
    /// assert_eq!(Integer::from(-2).signed_bits_64(), 2); //   “10”
    /// assert_eq!(Integer::from(-1).signed_bits_64(), 1); //    “1”
    /// assert_eq!(Integer::from(0).signed_bits_64(), 1);  //    “0”
    /// assert_eq!(Integer::from(1).signed_bits_64(), 2);  //   “01”
    /// assert_eq!(Integer::from(2).signed_bits_64(), 3);  //  “010”
    /// assert_eq!(Integer::from(3).signed_bits_64(), 3);  //  “011”
    /// assert_eq!(Integer::from(4).signed_bits_64(), 4);  // “0100”
    /// ```
    ///
    /// [`significant_bits_64`]: Integer::significant_bits_64
    fn signed_bits_64(&self) -> u64;

    /// Returns the number of one bits if the value ≥ 0.
    ///
    /// This method is similar to [`count_ones`][Integer::count_ones] but
    /// returns a [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// assert_eq!(Integer::from(0).count_ones_64(), Some(0));
    /// assert_eq!(Integer::from(15).count_ones_64(), Some(4));
    /// assert_eq!(Integer::from(-1).count_ones_64(), None);
    /// ```
    fn count_ones_64(&self) -> Option<u64>;

    /// Returns the number of zero bits if the value < 0.
    ///
    /// This method is similar to [`count_zeros`][Integer::count_zeros] but
    /// returns a [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// assert_eq!(Integer::from(0).count_zeros_64(), None);
    /// assert_eq!(Integer::from(1).count_zeros_64(), None);
    /// assert_eq!(Integer::from(-1).count_zeros_64(), Some(0));
    /// assert_eq!(Integer::from(-2).count_zeros_64(), Some(1));
    /// assert_eq!(Integer::from(-7).count_zeros_64(), Some(2));
    /// assert_eq!(Integer::from(-8).count_zeros_64(), Some(3));
    /// ```
    fn count_zeros_64(&self) -> Option<u64>;

    /// Returns the location of the first zero, starting at `start`. If the bit
    /// at location `start` is zero, returns `start`.
    ///
    /// This method is similar to [`find_zero`][Integer::find_zero] but takes
    /// `start` as [`u64`] and returns a [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// // -2 is ...11111110
    /// assert_eq!(Integer::from(-2).find_zero_64(0), Some(0));
    /// assert_eq!(Integer::from(-2).find_zero_64(1), None);
    /// // 15 is ...00001111
    /// assert_eq!(Integer::from(15).find_zero_64(0), Some(4));
    /// assert_eq!(Integer::from(15).find_zero_64(20), Some(20));
    /// ```
    fn find_zero_64(&self, start: u64) -> Option<u64>;

    /// Returns the location of the first one, starting at `start`. If the bit
    /// at location `start` is one, returns `start`.
    ///
    /// This method is similar to [`find_one`][Integer::find_one] but takes
    /// `start` as [`u64`] and returns a [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// // 1 is ...00000001
    /// assert_eq!(Integer::from(1).find_one_64(0), Some(0));
    /// assert_eq!(Integer::from(1).find_one_64(1), None);
    /// // -16 is ...11110000
    /// assert_eq!(Integer::from(-16).find_one_64(0), Some(4));
    /// assert_eq!(Integer::from(-16).find_one_64(20), Some(20));
    /// ```
    fn find_one_64(&self, start: u64) -> Option<u64>;

    /// Sets the bit at location `index` to 1 if `val` is [`true`] or 0 if `val`
    /// is [`false`].
    ///
    /// This method is similar to [`set_bit`][Integer::set_bit] but takes
    /// `index` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::{Assign, Integer};
    /// let mut i = Integer::from(-1);
    /// assert_eq!(*i.set_bit_64(0, false), -2);
    /// i.assign(0xff);
    /// assert_eq!(*i.set_bit_64(11, true), 0x8ff);
    /// ```
    fn set_bit_64(&mut self, index: u64, val: bool) -> &mut Self;

    /// Returns [`true`] if the bit at location `index` is 1 or [`false`] if the
    /// bit is 0.
    ///
    /// This method is similar to [`get_bit`][Integer::get_bit] but takes
    /// `index` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let i = Integer::from(0b100101);
    /// assert!(i.get_bit_64(0));
    /// assert!(!i.get_bit_64(1));
    /// assert!(i.get_bit_64(5));
    /// let neg = Integer::from(-1);
    /// assert!(neg.get_bit_64(1000));
    /// ```
    fn get_bit_64(&self, index: u64) -> bool;

    /// Toggles the bit at location `index`.
    ///
    /// This method is similar to [`toggle_bit`][Integer::toggle_bit] but takes
    /// `index` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let mut i = Integer::from(0b100101);
    /// i.toggle_bit_64(5);
    /// assert_eq!(i, 0b101);
    /// ```
    fn toggle_bit_64(&mut self, index: u64) -> &mut Self;

    /// Retuns the Hamming distance if the two numbers have the same sign.
    ///
    /// The Hamming distance is the number of different bits.
    ///
    /// This method is similar to [`hamming_dist`][Integer::hamming_dist] but
    /// returns a [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let i = Integer::from(-1);
    /// assert_eq!(Integer::from(0).hamming_dist_64(&i), None);
    /// assert_eq!(Integer::from(-1).hamming_dist_64(&i), Some(0));
    /// // -1 is ...11111111 and -13 is ...11110011
    /// assert_eq!(Integer::from(-13).hamming_dist_64(&i), Some(2));
    /// ```
    fn hamming_dist_64(&self, other: &Self) -> Option<u64>;

    /// Keeps the <i>n</i> least significant bits only, producing a result that
    /// is greater or equal to 0.
    ///
    /// This method is similar to [`keep_bits`][Integer::keep_bits] but takes
    /// `n` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let i = Integer::from(-1);
    /// let keep_8 = i.keep_bits_64(8);
    /// assert_eq!(keep_8, 0xff);
    /// ```
    #[must_use]
    fn keep_bits_64(self, n: u64) -> Self;

    /// Keeps the <i>n</i> least significant bits only, producing a result that
    /// is greater or equal to 0.
    ///
    /// This method is similar to [`keep_bits_mut`][Integer::keep_bits_mut] but
    /// takes `n` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let mut i = Integer::from(-1);
    /// i.keep_bits_64_mut(8);
    /// assert_eq!(i, 0xff);
    /// ```
    fn keep_bits_64_mut(&mut self, n: u64);

    /// Keeps the <i>n</i> least significant bits only, producing a result that
    /// is greater or equal to 0.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [Integer]</code>
    ///   * <code>[From]\<Src> for [Integer]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [Integer]> for Src</code>
    ///
    /// This method is similar to [`keep_bits_ref`][Integer::keep_bits_ref] but
    /// takes `n` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let i = Integer::from(-1);
    /// let r = i.keep_bits_64_ref(8);
    /// let eight_bits = Integer::from(r);
    /// assert_eq!(eight_bits, 0xff);
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn keep_bits_64_ref(&self, n: u64) -> KeepBitsIncomplete<'_>;

    /// Keeps the <i>n</i> least significant bits only, producing a negative
    /// result if the <i>n</i>th least significant bit is one.
    ///
    /// This method is similar to
    /// [`keep_signed_bits`][Integer::keep_signed_bits] but takes `n` as
    /// [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let i = Integer::from(-1);
    /// let i_keep_8 = i.keep_signed_bits_64(8);
    /// assert_eq!(i_keep_8, -1);
    /// let j = Integer::from(15 << 8 | 15);
    /// let j_keep_8 = j.keep_signed_bits_64(8);
    /// assert_eq!(j_keep_8, 15);
    /// ```
    #[must_use]
    fn keep_signed_bits_64(self, n: u64) -> Self;

    /// Keeps the <i>n</i> least significant bits only, producing a negative
    /// result if the <i>n</i>th least significant bit is one.
    ///
    /// This method is similar to
    /// [`keep_signed_bits_mut`][Integer::keep_signed_bits_mut] but takes `n` as
    /// [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let mut i = Integer::from(-1);
    /// i.keep_signed_bits_64_mut(8);
    /// assert_eq!(i, -1);
    /// let mut j = Integer::from(15 << 8 | 15);
    /// j.keep_signed_bits_64_mut(8);
    /// assert_eq!(j, 15);
    /// ```
    fn keep_signed_bits_64_mut(&mut self, n: u64);

    /// Keeps the <i>n</i> least significant bits only, producing a negative
    /// result if the <i>n</i>th least significant bit is one.
    ///
    /// The following are implemented with the returned
    /// [incomplete-computation value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [Integer]</code>
    ///   * <code>[From]\<Src> for [Integer]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [Integer]> for Src</code>
    ///
    /// This method is similar to
    /// [`keep_signed_bits_ref`][Integer::keep_signed_bits_ref] but takes `n` as
    /// [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let i = Integer::from(-1);
    /// let r = i.keep_signed_bits_64_ref(8);
    /// let eight_bits = Integer::from(r);
    /// assert_eq!(eight_bits, -1);
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn keep_signed_bits_64_ref(&self, n: u64) -> KeepSignedBitsIncomplete<'_>;

    /// Returns the modulo, or the remainder of Euclidean division by a [`u64`].
    ///
    /// The result is always zero or positive.
    ///
    /// This method is similar to [`mod_u`][Integer::mod_u] but takes `modulo`
    /// as [`u64`] and returns a [`u64`].
    ///
    /// # Panics
    ///
    /// Panics if `modulo` is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let pos = Integer::from(23);
    /// assert_eq!(pos.mod_u64(1), 0);
    /// assert_eq!(pos.mod_u64(10), 3);
    /// assert_eq!(pos.mod_u64(100), 23);
    /// let neg = Integer::from(-23);
    /// assert_eq!(neg.mod_u64(1), 0);
    /// assert_eq!(neg.mod_u64(10), 7);
    /// assert_eq!(neg.mod_u64(100), 77);
    /// ```
    fn mod_u64(&self, modulo: u64) -> u64;

    /// Performs an exact division.
    ///
    /// This is much faster than normal division, but produces correct results
    /// only when the division is exact.
    ///
    /// This method is similar to [`div_exact_u`][Integer::div_exact_u] but
    /// takes the divisor as [`u64`].
    ///
    /// # Panics
    ///
    /// Panics if `divisor` is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let i = Integer::from(12345 * 54321);
    /// let q = i.div_exact_u64(12345);
    /// assert_eq!(q, 54321);
    /// ```
    #[must_use]
    fn div_exact_u64(self, divisor: u64) -> Self;

    /// Performs an exact division.
    ///
    /// This is much faster than normal division, but produces correct results
    /// only when the division is exact.
    ///
    /// This method is similar to [`div_exact_u_mut`][Integer::div_exact_u_mut]
    /// but takes the divisor as [`u64`].
    ///
    /// # Panics
    ///
    /// Panics if `divisor` is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let mut i = Integer::from(12345 * 54321);
    /// i.div_exact_u64_mut(12345);
    /// assert_eq!(i, 54321);
    /// ```
    fn div_exact_u64_mut(&mut self, divisor: u64);

    /// Performs an exact division.
    ///
    /// This is much faster than normal division, but produces correct results
    /// only when the division is exact.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [Integer]</code>
    ///   * <code>[From]\<Src> for [Integer]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [Integer]> for Src</code>
    ///
    /// This method is similar to [`div_exact_u_ref`][Integer::div_exact_u_ref]
    /// but takes the divisor as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let i = Integer::from(12345 * 54321);
    /// let r = i.div_exact_u64_ref(12345);
    /// assert_eq!(Integer::from(r), 54321);
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn div_exact_u64_ref(&self, divisor: u64) -> DivExactUIncomplete<'_>;

    /// Raises `base` to the power of `exponent`.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [Integer]</code>
    ///   * <code>[From]\<Src> for [Integer]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [Integer]> for Src</code>
    ///
    /// This method is similar to [`u_pow_u`][Integer::u_pow_u] but takes `base`
    /// and `exponent` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::{Complete, Integer};
    /// assert_eq!(Integer::u64_pow_u64(13, 12).complete(), 13_u64.pow(12));
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn u64_pow_u64(base: u64, exponent: u64) -> UPowUIncomplete;

    /// Raises `base` to the power of `exponent`.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [Integer]</code>
    ///   * <code>[From]\<Src> for [Integer]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [Integer]> for Src</code>
    ///
    /// This method is similar to [`i_pow_u`][Integer::i_pow_u] but takes `base`
    /// as [`i64`] and `exponent` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::{Assign, Integer};
    /// let mut ans = Integer::new();
    /// ans.assign(Integer::i64_pow_u64(-13, 13));
    /// assert_eq!(ans, (-13_i64).pow(13));
    /// ans.assign(Integer::i64_pow_u64(13, 13));
    /// assert_eq!(ans, (13_i64).pow(13));
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn i64_pow_u64(base: i64, exponent: u64) -> IPowUIncomplete;

    /// Computes the <i>n</i>th root and truncates the result.
    ///
    /// This method is similar to [`root`][Integer::root] but takes `n` as
    /// [`u64`].
    ///
    /// # Panics
    ///
    /// Panics if <i>n</i> is zero or if <i>n</i> is even and the value is
    /// negative.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let i = Integer::from(1004);
    /// let root = i.root_64(3);
    /// assert_eq!(root, 10);
    /// ```
    #[must_use]
    fn root_64(self, n: u64) -> Self;

    /// Computes the <i>n</i>th root and truncates the result.
    ///
    /// This method is similar to [`root_mut`][Integer::root_mut] but takes `n`
    /// as [`u64`].
    ///
    /// # Panics
    ///
    /// Panics if <i>n</i> is zero or if <i>n</i> is even and the value is
    /// negative.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let mut i = Integer::from(1004);
    /// i.root_64_mut(3);
    /// assert_eq!(i, 10);
    /// ```
    fn root_64_mut(&mut self, n: u64);

    /// Computes the <i>n</i>th root and truncates the result.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [Integer]</code>
    ///   * <code>[From]\<Src> for [Integer]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [Integer]> for Src</code>
    ///
    /// This method is similar to [`root_ref`][Integer::root_ref] but takes `n`
    /// as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let i = Integer::from(1004);
    /// assert_eq!(Integer::from(i.root_64_ref(3)), 10);
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn root_64_ref(&self, n: u64) -> RootIncomplete<'_>;

    /// Computes the <i>n</i>th root and returns the truncated root and the
    /// remainder.
    ///
    /// The remainder is the original number minus the truncated root raised to
    /// the power of <i>n</i>.
    ///
    /// The initial value of `remainder` is ignored.
    ///
    /// This method is similar to [`root_rem`][Integer::root_rem] but takes `n`
    /// as [`u64`].
    ///
    /// # Panics
    ///
    /// Panics if <i>n</i> is zero or if <i>n</i> is even and the value is
    /// negative.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let i = Integer::from(1004);
    /// let (root, rem) = i.root_rem_64(Integer::new(), 3);
    /// assert_eq!(root, 10);
    /// assert_eq!(rem, 4);
    /// ```
    fn root_rem_64(self, remainder: Self, n: u64) -> (Self, Self);

    /// Computes the <i>n</i>th root and returns the truncated root and the
    /// remainder.
    ///
    /// The remainder is the original number minus the truncated root raised to
    /// the power of <i>n</i>.
    ///
    /// The initial value of `remainder` is ignored.
    ///
    /// This method is similar to [`root_rem_mut`][Integer::root_rem_mut] but
    /// takes `n` as [`u64`].
    ///
    /// # Panics
    ///
    /// Panics if <i>n</i> is zero or if <i>n</i> is even and the value is
    /// negative.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let mut i = Integer::from(1004);
    /// let mut rem = Integer::new();
    /// i.root_rem_64_mut(&mut rem, 3);
    /// assert_eq!(i, 10);
    /// assert_eq!(rem, 4);
    /// ```
    fn root_rem_64_mut(&mut self, remainder: &mut Self, n: u64);

    /// Computes the <i>n</i>th root and returns the truncated root and the
    /// remainder.
    ///
    /// The remainder is the original number minus the truncated root raised to
    /// the power of <i>n</i>.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [(][tuple][Integer][], [Integer][][)][tuple]</code>
    ///   * <code>[Assign]\<Src> for [(][tuple]\&mut [Integer], \&mut [Integer][][)][tuple]</code>
    ///   * <code>[From]\<Src> for [(][tuple][Integer][], [Integer][][)][tuple]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [(][tuple][Integer][], [Integer][][)][tuple]> for Src</code>
    ///
    /// This method is similar to [`root_rem_ref`][Integer::root_rem_ref] but
    /// takes `n` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::{Assign, Complete, Integer};
    /// let i = Integer::from(1004);
    /// let mut root = Integer::new();
    /// let mut rem = Integer::new();
    /// // 1004 = 10^3 + 5
    /// (&mut root, &mut rem).assign(i.root_rem_64_ref(3));
    /// assert_eq!(root, 10);
    /// assert_eq!(rem, 4);
    /// // 1004 = 3^6 + 275
    /// let (other_root, other_rem) = i.root_rem_64_ref(6).complete();
    /// assert_eq!(other_root, 3);
    /// assert_eq!(other_rem, 275);
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn root_rem_64_ref(&self, n: u64) -> RootRemIncomplete<'_>;

    /// Finds the greatest common divisor.
    ///
    /// The result is always positive except when both inputs are zero.
    ///
    /// This method is similar to [`gcd_u`][Integer::gcd_u] but takes `other` as
    /// [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let i = Integer::new();
    /// // gcd of 0, 0 is 0
    /// let gcd1 = i.gcd_u64(0);
    /// assert_eq!(gcd1, 0);
    /// // gcd of 0, 10 is 10
    /// let gcd2 = gcd1.gcd_u64(10);
    /// assert_eq!(gcd2, 10);
    /// // gcd of 10, 25 is 5
    /// let gcd3 = gcd2.gcd_u64(25);
    /// assert_eq!(gcd3, 5);
    /// ```
    #[must_use]
    fn gcd_u64(self, other: u64) -> Self;

    /// Finds the greatest common divisor.
    ///
    /// The result is always positive except when both inputs are zero.
    ///
    /// This method is similar to [`gcd_u_mut`][Integer::gcd_u_mut] but takes
    /// `other` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let mut i = Integer::new();
    /// // gcd of 0, 0 is 0
    /// i.gcd_u64_mut(0);
    /// assert_eq!(i, 0);
    /// // gcd of 0, 10 is 10
    /// i.gcd_u64_mut(10);
    /// assert_eq!(i, 10);
    /// // gcd of 10, 25 is 5
    /// i.gcd_u64_mut(25);
    /// assert_eq!(i, 5);
    /// ```
    fn gcd_u64_mut(&mut self, other: u64);

    /// Finds the greatest common divisor.
    ///
    /// The result is always positive except when both inputs are zero.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [Integer]</code>
    ///   * <code>[From]\<Src> for [Integer]</code>
    ///   * <code>[From]\<Src> for [Option]\<[u64]></code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [Integer]> for Src</code>
    ///
    /// The implementation of <code>[From]\<Src> for [Option]\<[u64]></code> is
    /// useful to obtain the result as a [`u64`] if it fits. If
    /// `other`&nbsp;>&nbsp;0 , the result always fits. If the result does not
    /// fit, it is equal to the absolute value of `self`.
    ///
    /// This method is similar to [`gcd_u_ref`][Integer::gcd_u_ref] but takes
    /// `other` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let i = Integer::from(100);
    /// let r = i.gcd_u64_ref(125);
    /// // gcd of 100, 125 is 25
    /// assert_eq!(Integer::from(r), 25);
    /// let r = i.gcd_u64_ref(125);
    /// assert_eq!(Option::<u64>::from(r), Some(25));
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn gcd_u64_ref(&self, other: u64) -> GcdUIncomplete<'_>;

    /// Finds the least common multiple.
    ///
    /// The result is always positive except when one or both inputs are zero.
    ///
    /// This method is similar to [`lcm_u`][Integer::lcm_u] but takes `other` as
    /// [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let i = Integer::from(10);
    /// // lcm of 10, 25 is 50
    /// let lcm1 = i.lcm_u64(25);
    /// assert_eq!(lcm1, 50);
    /// // lcm of 50, 0 is 0
    /// let lcm2 = lcm1.lcm_u64(0);
    /// assert_eq!(lcm2, 0);
    /// ```
    #[must_use]
    fn lcm_u64(self, other: u64) -> Self;

    /// Finds the least common multiple.
    ///
    /// The result is always positive except when one or both inputs are zero.
    ///
    /// This method is similar to [`lcm_u_mut`][Integer::lcm_u_mut] but takes
    /// `other` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let mut i = Integer::from(10);
    /// // lcm of 10, 25 is 50
    /// i.lcm_u64_mut(25);
    /// assert_eq!(i, 50);
    /// // lcm of 50, 0 is 0
    /// i.lcm_u64_mut(0);
    /// assert_eq!(i, 0);
    /// ```
    fn lcm_u64_mut(&mut self, other: u64);

    /// Finds the least common multiple.
    ///
    /// The result is always positive except when one or both inputs are zero.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [Integer]</code>
    ///   * <code>[From]\<Src> for [Integer]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [Integer]> for Src</code>
    ///
    /// This method is similar to [`lcm_u_ref`][Integer::lcm_u_ref] but takes
    /// `other` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let i = Integer::from(100);
    /// let r = i.lcm_u64_ref(125);
    /// // lcm of 100, 125 is 500
    /// assert_eq!(Integer::from(r), 500);
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn lcm_u64_ref(&self, other: u64) -> LcmUIncomplete<'_>;

    /// Removes all occurrences of `factor`, and returns the number of
    /// occurrences removed.
    ///
    /// This method is similar to [`remove_factor`][Integer::remove_factor] but
    /// returns the number of occurrences removed as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let mut i = Integer::from(Integer::u_pow_u(13, 50));
    /// i *= 1000;
    /// let (remove, count) = i.remove_factor_64(&Integer::from(13));
    /// assert_eq!(remove, 1000);
    /// assert_eq!(count, 50);
    /// ```
    fn remove_factor_64(self, factor: &Self) -> (Self, u64);

    /// Removes all occurrences of `factor`, and returns the number of
    /// occurrences removed.
    ///
    /// This method is similar to
    /// [`remove_factor_mut`][Integer::remove_factor_mut] but returns the number
    /// of occurrences removed as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// let mut i = Integer::from(Integer::u_pow_u(13, 50));
    /// i *= 1000;
    /// let count = i.remove_factor_64_mut(&Integer::from(13));
    /// assert_eq!(i, 1000);
    /// assert_eq!(count, 50);
    /// ```
    fn remove_factor_64_mut(&mut self, factor: &Self) -> u64;

    /// Removes all occurrences of `factor`, and counts the number of
    /// occurrences removed.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [(][tuple][Integer][], [u64][][)][tuple]</code>
    ///   * <code>[Assign]\<Src> for [(][tuple]\&mut [Integer], \&mut [u64][][)][tuple]</code>
    ///   * <code>[From]\<Src> for [(][tuple][Integer][], [u64][][)][tuple]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [(][tuple][Integer][], [u64][][)][tuple]> for Src</code>
    ///
    /// This method is similar to
    /// [`remove_factor_ref`][Integer::remove_factor_ref] but returns the number
    /// of occurrences removed as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::{Assign, Integer};
    /// let mut i = Integer::from(Integer::u_pow_u(13, 50));
    /// i *= 1000;
    /// let factor = Integer::from(13);
    /// let r = i.remove_factor_64_ref(&factor);
    /// let (mut j, mut count) = (Integer::new(), 0);
    /// (&mut j, &mut count).assign(r);
    /// assert_eq!(count, 50);
    /// assert_eq!(j, 1000);
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn remove_factor_64_ref<'a>(&'a self, factor: &'a Self) -> RemoveFactorIncomplete<'a>;

    /// Computes the factorial of <i>n</i>.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [Integer]</code>
    ///   * <code>[From]\<Src> for [Integer]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [Integer]> for Src</code>
    ///
    /// This method is similar to [`factorial`][Integer::factorial] but takes
    /// `n` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::{Complete, Integer};
    /// // 10 × 9 × 8 × 7 × 6 × 5 × 4 × 3 × 2 × 1
    /// assert_eq!(Integer::factorial_64(10).complete(), 3628800);
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn factorial_64(n: u64) -> FactorialIncomplete;

    /// Computes the double factorial of <i>n</i>.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [Integer]</code>
    ///   * <code>[From]\<Src> for [Integer]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [Integer]> for Src</code>
    ///
    /// This method is similar to [`factorial_2`][Integer::factorial_2] but
    /// takes `n` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::{Complete, Integer};
    /// // 10 × 8 × 6 × 4 × 2
    /// assert_eq!(Integer::factorial_2_64(10).complete(), 3840);
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn factorial_2_64(n: u64) -> Factorial2Incomplete;

    /// Computes the <i>m</i>-multi factorial of <i>n</i>.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [Integer]</code>
    ///   * <code>[From]\<Src> for [Integer]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [Integer]> for Src</code>
    ///
    /// This method is similar to [`factorial_m`][Integer::factorial_m] but
    /// takes `n` and `m` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::{Complete, Integer};
    /// // 10 × 7 × 4 × 1
    /// assert_eq!(Integer::factorial_m_64(10, 3).complete(), 280);
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn factorial_m_64(n: u64, m: u64) -> FactorialMIncomplete;

    /// Computes the primorial of <i>n</i>.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [Integer]</code>
    ///   * <code>[From]\<Src> for [Integer]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [Integer]> for Src</code>
    ///
    /// This method is similar to [`primorial`][Integer::primorial] but takes
    /// `n` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::{Complete, Integer};
    /// // 7 × 5 × 3 × 2
    /// assert_eq!(Integer::primorial_64(10).complete(), 210);
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn primorial_64(n: u64) -> PrimorialIncomplete;

    /// Computes the binomial coefficient over <i>k</i>.
    ///
    /// This method is similar to [`binomial`][Integer::binomial] but takes `k`
    /// as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// // 7 choose 2 is 21
    /// let i = Integer::from(7);
    /// let bin = i.binomial_64(2);
    /// assert_eq!(bin, 21);
    /// ```
    #[must_use]
    fn binomial_64(self, k: u64) -> Self;

    /// Computes the binomial coefficient over <i>k</i>.
    ///
    /// This method is similar to [`binomial_mut`][Integer::binomial_mut] but
    /// takes `k` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// // 7 choose 2 is 21
    /// let mut i = Integer::from(7);
    /// i.binomial_64_mut(2);
    /// assert_eq!(i, 21);
    /// ```
    fn binomial_64_mut(&mut self, k: u64);

    /// Computes the binomial coefficient over <i>k</i>.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [Integer]</code>
    ///   * <code>[From]\<Src> for [Integer]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [Integer]> for Src</code>
    ///
    /// This method is similar to [`binomial_ref`][Integer::binomial_ref] but
    /// takes `k` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::{Complete, Integer};
    /// // 7 choose 2 is 21
    /// let i = Integer::from(7);
    /// assert_eq!(i.binomial_64_ref(2).complete(), 21);
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn binomial_64_ref(&self, k: u64) -> BinomialIncomplete<'_>;

    /// Computes the binomial coefficient <i>n</i> over <i>k</i>.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [Integer]</code>
    ///   * <code>[From]\<Src> for [Integer]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [Integer]> for Src</code>
    ///
    /// This method is similar to [`binomial_u`][Integer::binomial_u] but takes
    /// `n` and `k` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::Integer;
    /// // 7 choose 2 is 21
    /// let b = Integer::binomial_u64(7, 2);
    /// let i = Integer::from(b);
    /// assert_eq!(i, 21);
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn binomial_u64(n: u64, k: u64) -> BinomialUIncomplete;

    /// Computes the Fibonacci number.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [Integer]</code>
    ///   * <code>[From]\<Src> for [Integer]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [Integer]> for Src</code>
    ///
    /// This function is meant for an isolated number. If a sequence of
    /// Fibonacci numbers is required, the first two values of the sequence
    /// should be computed with the [`fibonacci_2_64`][Integer::fibonacci_2_64]
    /// method, then iterations should be used.
    ///
    /// This method is similar to [`fibonacci`][Integer::fibonacci] but takes
    /// `n` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::{Complete, Integer};
    /// assert_eq!(Integer::fibonacci_64(12).complete(), 144);
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn fibonacci_64(n: u64) -> FibonacciIncomplete;

    /// Computes a Fibonacci number, and the previous Fibonacci number.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [(][tuple][Integer][], [Integer][][)][tuple]</code>
    ///   * <code>[Assign]\<Src> for [(][tuple]\&mut [Integer], \&mut [Integer][][)][tuple]</code>
    ///   * <code>[From]\<Src> for [(][tuple][Integer][], [Integer][][)][tuple]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [(][tuple][Integer][], [Integer][][)][tuple]> for Src</code>
    ///
    /// This function is meant to calculate isolated numbers. If a sequence of
    /// Fibonacci numbers is required, the first two values of the sequence
    /// should be computed with this function, then iterations should be used.
    ///
    /// This method is similar to [`fibonacci_2`][Integer::fibonacci_2] but
    /// takes `n` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::{Assign, Integer};
    /// let f = Integer::fibonacci_2_64(12);
    /// let mut pair = <(Integer, Integer)>::from(f);
    /// assert_eq!(pair.0, 144);
    /// assert_eq!(pair.1, 89);
    /// // Fibonacci number F[-1] is 1
    /// pair.assign(Integer::fibonacci_2_64(0));
    /// assert_eq!(pair.0, 0);
    /// assert_eq!(pair.1, 1);
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn fibonacci_2_64(n: u64) -> Fibonacci2Incomplete;

    /// Computes the Lucas number.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [Integer]</code>
    ///   * <code>[From]\<Src> for [Integer]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [Integer]> for Src</code>
    ///
    /// This function is meant for an isolated number. If a sequence of Lucas
    /// numbers is required, the first two values of the sequence should be
    /// computed with the [`lucas_2_64`][Integer::lucas_2_64] method, then
    /// iterations should be used.
    ///
    /// This method is similar to [`lucas`][Integer::lucas] but takes `n` as
    /// [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::{Complete, Integer};
    /// assert_eq!(Integer::lucas_64(12).complete(), 322);
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn lucas_64(n: u64) -> LucasIncomplete;

    /// Computes a Lucas number, and the previous Lucas number.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [(][tuple][Integer][], [Integer][][)][tuple]</code>
    ///   * <code>[Assign]\<Src> for [(][tuple]\&mut [Integer], \&mut [Integer][][)][tuple]</code>
    ///   * <code>[From]\<Src> for [(][tuple][Integer][], [Integer][][)][tuple]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [(][tuple][Integer][], [Integer][][)][tuple]> for Src</code>
    ///
    /// This function is meant to calculate isolated numbers. If a sequence of
    /// Lucas numbers is required, the first two values of the sequence should
    /// be computed with this function, then iterations should be used.
    ///
    /// This method is similar to [`lucas_2`][Integer::lucas_2] but takes `n` as
    /// [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::{Assign, Integer};
    /// let l = Integer::lucas_2_64(12);
    /// let mut pair = <(Integer, Integer)>::from(l);
    /// assert_eq!(pair.0, 322);
    /// assert_eq!(pair.1, 199);
    /// pair.assign(Integer::lucas_2_64(0));
    /// assert_eq!(pair.0, 2);
    /// assert_eq!(pair.1, -1);
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn lucas_2_64(n: u64) -> Lucas2Incomplete;

    #[cfg(feature = "rand")]
    /// Generates a random number with a specified maximum number of bits.
    ///
    /// The following are implemented with the returned [incomplete-computation
    /// value][icv] as `Src`:
    ///   * <code>[Assign]\<Src> for [Integer]</code>
    ///   * <code>[From]\<Src> for [Integer]</code>
    ///   * <code>[Complete]\<[Completed][Complete::Completed] = [Integer]> for Src</code>
    ///
    /// This method is similar to [`random_bits`][Integer::random_bits] but
    /// takes `bits` as [`u64`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::IntegerExt64;
    /// use rug::rand::RandState;
    /// use rug::{Assign, Integer};
    /// let mut rand = RandState::new();
    /// let mut i = Integer::from(Integer::random_bits(0, &mut rand));
    /// assert_eq!(i, 0);
    /// i.assign(Integer::random_bits_64(80, &mut rand));
    /// assert!(i.significant_bits() <= 80);
    /// ```
    ///
    /// [icv]: crate#incomplete-computation-values
    fn random_bits_64(bits: u64, rng: &mut dyn MutRandState) -> RandomBitsIncomplete<'_>;
}

impl IntegerExt64 for Integer {
    #[inline]
    fn to_f32_exp64(&self) -> (f32, u64) {
        let (f, exp) = self.to_f64_exp64();
        let trunc_f = misc::trunc_f64_to_f32(f);
        (trunc_f, exp)
    }

    #[inline]
    fn to_f64_exp64(&self) -> (f64, u64) {
        let (f, exp) = xmpz::get_f64_2exp(self);
        (f, exp.strict_cast())
    }

    #[inline]
    fn is_divisible_u64(&self, divisor: u64) -> bool {
        xmpz::divisible_ui_p(self, divisor.strict_cast())
    }

    #[inline]
    fn is_divisible_2pow_64(&self, b: u64) -> bool {
        xmpz::divisible_2exp_p(self, b.strict_cast())
    }

    #[inline]
    fn is_congruent_u64(&self, c: u64, divisor: u64) -> bool {
        xmpz::congruent_ui_p(self, c.strict_cast(), divisor.strict_cast())
    }

    #[inline]
    fn is_congruent_2pow_64(&self, c: &Self, b: u64) -> bool {
        xmpz::congruent_2exp_p(self, c, b.strict_cast())
    }

    #[inline]
    fn significant_bits_64(&self) -> u64 {
        xmpz::significant_bits(self).strict_cast()
    }

    #[inline]
    fn signed_bits_64(&self) -> u64 {
        xmpz::signed_bits(self).strict_cast()
    }

    #[inline]
    fn count_ones_64(&self) -> Option<u64> {
        xmpz::popcount(self).map(From::from)
    }

    #[inline]
    fn count_zeros_64(&self) -> Option<u64> {
        xmpz::zerocount(self).map(From::from)
    }

    #[inline]
    fn find_zero_64(&self, start: u64) -> Option<u64> {
        xmpz::scan0(self, start.strict_cast()).map(From::from)
    }

    #[inline]
    fn find_one_64(&self, start: u64) -> Option<u64> {
        xmpz::scan1(self, start.strict_cast()).map(From::from)
    }

    #[inline]
    fn set_bit_64(&mut self, index: u64, val: bool) -> &mut Self {
        if val {
            xmpz::setbit(self, index.strict_cast());
        } else {
            xmpz::clrbit(self, index.strict_cast());
        }
        self
    }

    #[inline]
    fn get_bit_64(&self, index: u64) -> bool {
        xmpz::tstbit(self, index.strict_cast())
    }

    #[inline]
    fn toggle_bit_64(&mut self, index: u64) -> &mut Self {
        xmpz::combit(self, index.strict_cast());
        self
    }

    #[inline]
    fn hamming_dist_64(&self, other: &Self) -> Option<u64> {
        xmpz::hamdist(self, other).map(From::from)
    }

    #[inline]
    fn keep_bits_64(mut self, n: u64) -> Self {
        self.keep_bits_64_mut(n);
        self
    }

    #[inline]
    fn keep_bits_64_mut(&mut self, n: u64) {
        xmpz::fdiv_r_2exp(self, (), n.strict_cast());
    }

    #[inline]
    fn keep_bits_64_ref(&self, n: u64) -> KeepBitsIncomplete<'_> {
        let n = n.strict_cast();
        KeepBitsIncomplete { ref_self: self, n }
    }

    #[inline]
    fn keep_signed_bits_64(mut self, n: u64) -> Self {
        self.keep_signed_bits_64_mut(n);
        self
    }

    #[inline]
    fn keep_signed_bits_64_mut(&mut self, n: u64) {
        xmpz::keep_signed_bits(self, (), n.strict_cast());
    }

    #[inline]
    fn keep_signed_bits_64_ref(&self, n: u64) -> KeepSignedBitsIncomplete<'_> {
        let n = n.strict_cast();
        KeepSignedBitsIncomplete { ref_self: self, n }
    }

    #[inline]
    fn mod_u64(&self, modulo: u64) -> u64 {
        xmpz::fdiv_ui(self, modulo.strict_cast()).into()
    }

    #[inline]
    fn div_exact_u64(mut self, divisor: u64) -> Self {
        self.div_exact_u64_mut(divisor);
        self
    }

    #[inline]
    fn div_exact_u64_mut(&mut self, divisor: u64) {
        xmpz::divexact_ui(self, (), divisor);
    }

    #[inline]
    fn div_exact_u64_ref(&self, divisor: u64) -> DivExactUIncomplete<'_> {
        DivExactUIncomplete {
            ref_self: self,
            divisor,
        }
    }

    #[inline]
    fn u64_pow_u64(base: u64, exponent: u64) -> UPowUIncomplete {
        UPowUIncomplete { base, exponent }
    }

    #[inline]
    fn i64_pow_u64(base: i64, exponent: u64) -> IPowUIncomplete {
        IPowUIncomplete { base, exponent }
    }

    #[inline]
    fn root_64(mut self, n: u64) -> Self {
        self.root_64_mut(n);
        self
    }

    #[inline]
    fn root_64_mut(&mut self, n: u64) {
        xmpz::root(self, (), n.strict_cast());
    }

    #[inline]
    fn root_64_ref(&self, n: u64) -> RootIncomplete<'_> {
        let n = n.strict_cast();
        RootIncomplete { ref_self: self, n }
    }

    #[inline]
    fn root_rem_64(mut self, mut remainder: Self, n: u64) -> (Self, Self) {
        self.root_rem_64_mut(&mut remainder, n);
        (self, remainder)
    }

    #[inline]
    fn root_rem_64_mut(&mut self, remainder: &mut Self, n: u64) {
        xmpz::rootrem(self, remainder, (), n.strict_cast());
    }

    #[inline]
    fn root_rem_64_ref(&self, n: u64) -> RootRemIncomplete<'_> {
        let n = n.strict_cast();
        RootRemIncomplete { ref_self: self, n }
    }

    #[inline]
    fn gcd_u64(mut self, other: u64) -> Self {
        self.gcd_u64_mut(other);
        self
    }

    #[inline]
    fn gcd_u64_mut(&mut self, other: u64) {
        xmpz::gcd_ui(self, (), other);
    }

    #[inline]
    fn gcd_u64_ref(&self, other: u64) -> GcdUIncomplete<'_> {
        GcdUIncomplete {
            ref_self: self,
            other,
        }
    }

    #[inline]
    fn lcm_u64(mut self, other: u64) -> Self {
        self.lcm_u64_mut(other);
        self
    }

    #[inline]
    fn lcm_u64_mut(&mut self, other: u64) {
        xmpz::lcm_ui(self, (), other);
    }

    #[inline]
    fn lcm_u64_ref(&self, other: u64) -> LcmUIncomplete<'_> {
        LcmUIncomplete {
            ref_self: self,
            other,
        }
    }

    #[inline]
    fn remove_factor_64(mut self, factor: &Self) -> (Self, u64) {
        let count = self.remove_factor_64_mut(factor);
        (self, count)
    }

    #[inline]
    fn remove_factor_64_mut(&mut self, factor: &Self) -> u64 {
        xmpz::remove(self, (), factor).into()
    }

    #[inline]
    fn remove_factor_64_ref<'a>(&'a self, factor: &'a Self) -> RemoveFactorIncomplete<'a> {
        RemoveFactorIncomplete {
            ref_self: self,
            factor,
        }
    }

    #[inline]
    fn factorial_64(n: u64) -> FactorialIncomplete {
        let n = n.strict_cast();
        FactorialIncomplete { n }
    }

    #[inline]
    fn factorial_2_64(n: u64) -> Factorial2Incomplete {
        let n = n.strict_cast();
        Factorial2Incomplete { n }
    }

    #[inline]
    fn factorial_m_64(n: u64, m: u64) -> FactorialMIncomplete {
        let n = n.strict_cast();
        let m = m.strict_cast();
        FactorialMIncomplete { n, m }
    }

    #[inline]
    fn primorial_64(n: u64) -> PrimorialIncomplete {
        let n = n.strict_cast();
        PrimorialIncomplete { n }
    }

    #[inline]
    fn binomial_64(mut self, k: u64) -> Self {
        self.binomial_64_mut(k);
        self
    }

    #[inline]
    fn binomial_64_mut(&mut self, k: u64) {
        xmpz::bin_ui(self, (), k.strict_cast());
    }

    #[inline]
    fn binomial_64_ref(&self, k: u64) -> BinomialIncomplete<'_> {
        let k = k.strict_cast();
        BinomialIncomplete { ref_self: self, k }
    }

    #[inline]
    fn binomial_u64(n: u64, k: u64) -> BinomialUIncomplete {
        let n = n.strict_cast();
        let k = k.strict_cast();
        BinomialUIncomplete { n, k }
    }

    #[inline]
    fn fibonacci_64(n: u64) -> FibonacciIncomplete {
        let n = n.strict_cast();
        FibonacciIncomplete { n }
    }

    #[inline]
    fn fibonacci_2_64(n: u64) -> Fibonacci2Incomplete {
        let n = n.strict_cast();
        Fibonacci2Incomplete { n }
    }

    #[inline]
    fn lucas_64(n: u64) -> LucasIncomplete {
        let n = n.strict_cast();
        LucasIncomplete { n }
    }

    #[inline]
    fn lucas_2_64(n: u64) -> Lucas2Incomplete {
        let n = n.strict_cast();
        Lucas2Incomplete { n }
    }

    #[cfg(feature = "rand")]
    #[inline]
    fn random_bits_64(bits: u64, rng: &mut dyn MutRandState) -> RandomBitsIncomplete<'_> {
        let bits = bits.strict_cast();
        RandomBitsIncomplete { bits, rng }
    }
}

#[inline]
fn i64_pow_u64(rop: &mut Integer, base: i64, exp: u64) {
    let (base_neg, base_abs) = base.neg_abs();
    xmpz::ui_pow_ui(rop, base_abs, exp);
    if base_neg && (exp & 1) == 1 {
        xmpz::neg(rop, ());
    }
}

ref_math_op1! { Integer; xmpz::fdiv_r_2exp; struct KeepBitsIncomplete { n: bitcnt_t } }
ref_math_op1! { Integer; xmpz::keep_signed_bits; struct KeepSignedBitsIncomplete { n: bitcnt_t } }
ref_math_op1! { Integer; xmpz::divexact_ui; struct DivExactUIncomplete { divisor: u64 } }
ref_math_op0! { Integer; xmpz::ui_pow_ui; struct UPowUIncomplete { base: u64, exponent: u64 } }
ref_math_op0! { Integer; i64_pow_u64; struct IPowUIncomplete { base: i64, exponent: u64 } }
ref_math_op1! { Integer; xmpz::root; struct RootIncomplete { n: c_ulong } }
ref_math_op1_2! { Integer; xmpz::rootrem; struct RootRemIncomplete { n: c_ulong } }
ref_math_op1! { Integer; xmpz::gcd_ui; struct GcdUIncomplete { other: u64 } }

impl From<GcdUIncomplete<'_>> for Option<u64> {
    #[inline]
    fn from(src: GcdUIncomplete) -> Self {
        let gcd = xmpz::gcd_opt_ui(None, src.ref_self, src.other.into());
        if gcd == 0 && !src.ref_self.is_zero() {
            None
        } else {
            gcd.checked_cast()
        }
    }
}

ref_math_op1! { Integer; xmpz::lcm_ui; struct LcmUIncomplete { other: u64 } }

#[derive(Debug)]
pub struct RemoveFactorIncomplete<'a> {
    ref_self: &'a Integer,
    factor: &'a Integer,
}

impl Assign<RemoveFactorIncomplete<'_>> for (&mut Integer, &mut u64) {
    #[inline]
    fn assign(&mut self, src: RemoveFactorIncomplete<'_>) {
        *self.1 = xmpz::remove(self.0, src.ref_self, src.factor).into();
    }
}

impl Assign<RemoveFactorIncomplete<'_>> for (Integer, u64) {
    #[inline]
    fn assign(&mut self, src: RemoveFactorIncomplete<'_>) {
        (&mut self.0, &mut self.1).assign(src);
    }
}

from_assign! { RemoveFactorIncomplete<'_> => Integer, u64 }
ref_math_op0! { Integer; xmpz::fac_ui; struct FactorialIncomplete { n: c_ulong } }
ref_math_op0! { Integer; xmpz::twofac_ui; struct Factorial2Incomplete { n: c_ulong } }
ref_math_op0! { Integer; xmpz::mfac_uiui; struct FactorialMIncomplete { n: c_ulong, m: c_ulong } }
ref_math_op0! { Integer; xmpz::primorial_ui; struct PrimorialIncomplete { n: c_ulong } }
ref_math_op1! { Integer; xmpz::bin_ui; struct BinomialIncomplete { k: c_ulong } }
ref_math_op0! { Integer; xmpz::bin_uiui; struct BinomialUIncomplete { n: c_ulong, k: c_ulong } }
ref_math_op0! { Integer; xmpz::fib_ui; struct FibonacciIncomplete { n: c_ulong } }
ref_math_op0_2! { Integer; xmpz::fib2_ui; struct Fibonacci2Incomplete { n: c_ulong } }
ref_math_op0! { Integer; xmpz::lucnum_ui; struct LucasIncomplete { n: c_ulong } }
ref_math_op0_2! { Integer; xmpz::lucnum2_ui; struct Lucas2Incomplete { n: c_ulong } }

#[cfg(feature = "rand")]
pub struct RandomBitsIncomplete<'a> {
    bits: bitcnt_t,
    rng: &'a mut dyn MutRandState,
}

#[cfg(feature = "rand")]
impl Assign<RandomBitsIncomplete<'_>> for Integer {
    #[inline]
    fn assign(&mut self, src: RandomBitsIncomplete) {
        xmpz::urandomb(self, src.rng, src.bits);
    }
}

#[cfg(feature = "rand")]
impl From<RandomBitsIncomplete<'_>> for Integer {
    #[inline]
    fn from(src: RandomBitsIncomplete) -> Self {
        let mut dst = Integer::new();
        dst.assign(src);
        dst
    }
}
