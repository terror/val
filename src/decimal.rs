use super::*;

#[derive(Clone, Debug)]
pub(crate) struct Decimal {
  digits: String,
  negative: bool,
  point: i64,
}

impl Decimal {
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

  fn format_exponent(exponent: i64) -> String {
    format!(
      "{}{:02}",
      if exponent.is_negative() { '-' } else { '+' },
      exponent.abs()
    )
  }

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

  fn is_zero(&self) -> bool {
    self.digits.bytes().all(|digit| digit == b'0')
  }

  pub(crate) fn new(digits: String, negative: bool, point: i64) -> Self {
    Self {
      digits,
      negative,
      point,
    }
  }

  fn remove_factor(number: &mut Integer, factor: u32) -> usize {
    let mut count = 0;

    while number.is_divisible_u(factor) {
      *number /= factor;
      count += 1;
    }

    count
  }

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
