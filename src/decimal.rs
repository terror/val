use super::*;

/// A finite decimal representation of a rational number.
///
/// A decimal is stored as a string of digits, a sign bit and the position of
/// the decimal point. The decimal point is represented as the number of digits
/// that appear before the point.
///
/// For example, the decimal `12.34` is represented with digits `1234` and a
/// point of `2`, while `0.001` is represented with digits `1` and a point of
/// `-2`.
///
/// This representation is intended to support formatting of rational numbers
/// that have a terminating decimal expansion.
#[derive(Clone, Debug)]
pub(crate) struct Decimal {
  digits: String,
  negative: bool,
  point: i64,
}

impl Decimal {
  /// Formats this decimal using fixed-point or scientific notation.
  ///
  /// The notation is chosen using JSON-style number formatting rules. Zero is
  /// always formatted as `0`. Scientific notation is used when the decimal
  /// exponent is less than `-4`, or when it is greater than or equal to
  /// `significant_digits`. Fixed-point notation is used otherwise.
  pub(crate) fn display(self, significant_digits: NonZeroUsize) -> String {
    if self.is_zero() {
      return "0".into();
    }

    let exponent = self.point - 1;

    let significant_digits = i64::try_from(significant_digits.get()).unwrap();

    if exponent < -4 || exponent >= significant_digits {
      self.scientific_string(exponent)
    } else {
      self.fixed_string()
    }
  }

  /// Formats this decimal using fixed-point notation.
  ///
  /// This inserts a decimal point according to `self.point`, adds any leading
  /// or trailing zeroes required to make the decimal point position valid and
  /// trims insignificant trailing zeroes from the fractional component.
  ///
  /// The sign is applied after the unsigned representation has been formatted.
  fn fixed_string(&self) -> String {
    let digits_len = i64::try_from(self.digits.len()).unwrap();

    let unsigned = match self.point {
      point if point <= 0 => {
        format!(
          "0.{}{}",
          "0".repeat(usize::try_from(-point).unwrap()),
          self.digits
        )
      }
      point if point >= digits_len => {
        format!(
          "{}{}",
          self.digits,
          "0".repeat(usize::try_from(point - digits_len).unwrap())
        )
      }
      point => {
        let (integer, fraction) =
          self.digits.split_at(usize::try_from(point).unwrap());

        format!("{integer}.{fraction}")
      }
    };

    self.with_sign(Self::trim_zeros(unsigned))
  }

  /// Formats an exponent for scientific notation.
  ///
  /// The exponent is always written with an explicit sign and at least two
  /// digits. For example, `5` is formatted as `+05`, while `-13` is formatted
  /// as `-13`.
  fn format_exponent(exponent: i64) -> String {
    format!(
      "{}{:02}",
      if exponent.is_negative() { '-' } else { '+' },
      exponent.abs()
    )
  }

  /// Converts a rational number to a finite decimal, if one exists.
  ///
  /// A rational number has a terminating decimal expansion exactly when its
  /// denominator has no prime factors other than `2` and `5`. If any other
  /// factor remains after removing all powers of `2` and `5`, then this returns
  /// `None`.
  ///
  /// Otherwise, the numerator is scaled so that the denominator is a power of
  /// ten, and the resulting integer is stored as decimal digits.
  pub(crate) fn from_rational(number: &Rational) -> Option<Self> {
    let mut denominator = number.denom().clone();

    let (twos, fives) = (
      Self::remove_factor(&mut denominator, 2),
      Self::remove_factor(&mut denominator, 5),
    );

    if denominator != 1 {
      return None;
    }

    let places = twos.max(fives);

    let mut scaled = number.numer().clone();

    for _ in 0..places.saturating_sub(twos) {
      scaled *= 2;
    }

    for _ in 0..places.saturating_sub(fives) {
      scaled *= 5;
    }

    let negative = scaled.is_negative();

    let scaled = if negative { -scaled } else { scaled };

    let digits = scaled.to_string();

    Some(Self {
      point: i64::try_from(digits.len()).ok()? - i64::try_from(places).ok()?,
      digits,
      negative,
    })
  }

  /// Returns true when this decimal represents zero.
  ///
  /// The sign is intentionally ignored. Negative zero is formatted as plain
  /// zero.
  fn is_zero(&self) -> bool {
    self.digits.bytes().all(|digit| digit == b'0')
  }

  /// Creates a new decimal from its raw parts.
  ///
  /// The `digits` string should contain only ASCII decimal digits. The `point`
  /// gives the number of digits before the decimal point. A negative `point`
  /// means the formatted number has leading zeroes after the decimal point.
  pub(crate) fn new(digits: String, negative: bool, point: i64) -> Self {
    Self {
      digits,
      negative,
      point,
    }
  }

  /// Removes repeated factors from `number`.
  ///
  /// This divides `number` by `factor` until it is no longer divisible by that
  /// factor, and returns the number of divisions performed.
  fn remove_factor(number: &mut Integer, factor: u32) -> usize {
    let mut count = 0;

    while number.is_divisible_u(factor) {
      *number /= factor;
      count += 1;
    }

    count
  }

  /// Formats this decimal using scientific notation.
  ///
  /// The mantissa is formed by placing a decimal point after the first digit and
  /// trimming any insignificant trailing zeroes. The exponent is formatted with
  /// [`Decimal::format_exponent`].
  fn scientific_string(&self, exponent: i64) -> String {
    let mantissa = if self.digits.len() == 1 {
      self.digits.clone()
    } else {
      let (integer, fraction) = self.digits.split_at(1);
      Self::trim_zeros(format!("{integer}.{fraction}"))
    };

    format!(
      "{}e{}",
      self.with_sign(mantissa),
      Self::format_exponent(exponent),
    )
  }

  /// Trims insignificant trailing zeroes from a decimal string.
  ///
  /// This only modifies strings containing a decimal point. If all fractional
  /// digits are trimmed, then the decimal point itself is removed too.
  fn trim_zeros(mut string: String) -> String {
    if !string.contains('.') {
      return string;
    }

    while string.ends_with('0') {
      string.pop();
    }

    if string.ends_with('.') {
      string.pop();
    }

    string
  }

  /// Applies this decimal's sign to `string`.
  ///
  /// The caller is responsible for ensuring that zero values do not receive a
  /// negative sign.
  fn with_sign(&self, string: String) -> String {
    if self.negative {
      format!("-{string}")
    } else {
      string
    }
  }
}

#[cfg(test)]
mod tests {
  use {super::*, pretty_assertions::assert_eq};

  fn digits(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).unwrap()
  }

  #[test]
  fn configured_digits_exponent_when_point_exceeds_digits() {
    assert_eq!(
      Decimal::new("1234567890".to_owned(), false, 11).display(digits(10)),
      "1.23456789e+10"
    );
  }

  #[test]
  fn configured_digits_fixed_when_exponent_within_digits() {
    assert_eq!(
      Decimal::new("1234567890".to_owned(), false, 10).display(digits(10)),
      "1234567890"
    );
  }

  #[test]
  fn display_adds_trailing_zeros() {
    assert_eq!(
      Decimal::new("123".to_owned(), false, 5).display(digits(16)),
      "12300"
    );
  }

  #[test]
  fn display_fraction_with_leading_zero() {
    assert_eq!(
      Decimal::new("123".to_owned(), false, 0).display(digits(16)),
      "0.123"
    );
  }

  #[test]
  fn display_large_fixed_boundary() {
    assert_eq!(
      Decimal::new("1".to_owned(), false, 16).display(digits(16)),
      "1000000000000000"
    );
  }

  #[test]
  fn display_large_scientific() {
    assert_eq!(
      Decimal::new("1".to_owned(), false, 17).display(digits(16)),
      "1e+16"
    );
  }

  #[test]
  fn display_positive_integer() {
    assert_eq!(
      Decimal::new("123".to_owned(), false, 3).display(digits(16)),
      "123"
    );
  }

  #[test]
  fn display_scientific_large_digits() {
    assert_eq!(
      Decimal::new("1234567890123456".to_owned(), false, 17)
        .display(digits(16)),
      "1.234567890123456e+16"
    );
  }

  #[test]
  fn display_scientific_small_digits() {
    assert_eq!(
      Decimal::new("3600216012960922".to_owned(), false, -12)
        .display(digits(16)),
      "3.600216012960922e-13"
    );
  }

  #[test]
  fn display_small_fixed_fraction() {
    assert_eq!(
      Decimal::new("123".to_owned(), false, -3).display(digits(16)),
      "0.000123"
    );
  }

  #[test]
  fn display_small_scientific_fraction() {
    assert_eq!(
      Decimal::new("123".to_owned(), false, -4).display(digits(16)),
      "1.23e-05"
    );
  }

  #[test]
  fn display_trims_fractional_zeros() {
    assert_eq!(
      Decimal::new("2300".to_owned(), false, 2).display(digits(16)),
      "23"
    );
  }

  #[test]
  fn display_with_decimal_point() {
    assert_eq!(
      Decimal::new("123".to_owned(), false, 1).display(digits(16)),
      "1.23"
    );
  }

  #[test]
  fn display_zero() {
    assert_eq!(
      Decimal::new("0".to_owned(), false, 1).display(digits(16)),
      "0"
    );
  }

  #[test]
  fn display_zero_ignores_negative_sign() {
    assert_eq!(
      Decimal::new("0".to_owned(), true, 1).display(digits(16)),
      "0"
    );
  }

  #[test]
  fn from_rational_decimal_fraction() {
    let actual = Decimal::from_rational(&Rational::from((1234, 100)))
      .map(|decimal| decimal.display(digits(16)));

    assert_eq!(actual.as_deref(), Some("12.34"));
  }

  #[test]
  fn from_rational_integer() {
    let actual = Decimal::from_rational(&Rational::from(123))
      .map(|decimal| decimal.display(digits(16)));

    assert_eq!(actual.as_deref(), Some("123"));
  }

  #[test]
  fn from_rational_negative_fraction() {
    let actual = Decimal::from_rational(&Rational::from((-1, 40)))
      .map(|decimal| decimal.display(digits(16)));

    assert_eq!(actual.as_deref(), Some("-0.025"));
  }

  #[test]
  fn from_rational_non_terminating() {
    let actual = Decimal::from_rational(&Rational::from((1, 3)))
      .map(|decimal| decimal.display(digits(16)));

    assert_eq!(actual.as_deref(), None);
  }

  #[test]
  fn from_rational_small_fraction() {
    let actual = Decimal::from_rational(&Rational::from((1, 1000)))
      .map(|decimal| decimal.display(digits(16)));

    assert_eq!(actual.as_deref(), Some("0.001"));
  }

  #[test]
  fn from_rational_small_scientific() {
    let actual = Decimal::from_rational(&Rational::from((1, 100_000)))
      .map(|decimal| decimal.display(digits(16)));

    assert_eq!(actual.as_deref(), Some("1e-05"));
  }

  #[test]
  fn from_rational_twentieth() {
    let actual = Decimal::from_rational(&Rational::from((1, 20)))
      .map(|decimal| decimal.display(digits(16)));

    assert_eq!(actual.as_deref(), Some("0.05"));
  }
}
