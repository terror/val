use super::*;

#[derive(Clone, Debug)]
pub(crate) struct Decimal {
  digits: String,
  negative: bool,
  point: i64,
}

impl Decimal {
  fn fixed_string(&self) -> String {
    let sign = self.sign();
    let digits_len = i64::try_from(self.digits.len()).unwrap();

    let result = if self.point <= 0 {
      let zeros = usize::try_from(-self.point).unwrap();
      format!("{sign}0.{}{}", "0".repeat(zeros), self.digits)
    } else if self.point >= digits_len {
      let zeros = usize::try_from(self.point - digits_len).unwrap();
      format!("{sign}{}{}", self.digits, "0".repeat(zeros))
    } else {
      let split_at = usize::try_from(self.point).unwrap();
      let (integer, fraction) = self.digits.split_at(split_at);
      format!("{sign}{integer}.{fraction}")
    };

    Self::trim_zeros(result)
  }

  fn format_exponent(exponent: i64) -> String {
    let sign = if exponent.is_negative() { '-' } else { '+' };

    format!("{sign}{:02}", exponent.abs())
  }

  pub(crate) fn from_rational(number: &Rational) -> Option<Self> {
    let mut denominator = number.denom().clone();

    let twos = Self::remove_factor(&mut denominator, 2);
    let fives = Self::remove_factor(&mut denominator, 5);

    if denominator != 1 {
      return None;
    }

    let places = twos.max(fives);
    let mut scaled = number.numer().clone();

    for _ in 0..places - twos {
      scaled *= 2;
    }

    for _ in 0..places - fives {
      scaled *= 5;
    }

    let negative = scaled.is_negative();
    let scaled = if negative { -scaled } else { scaled };
    let digits = scaled.to_string();
    let point =
      i64::try_from(digits.len()).ok()? - i64::try_from(places).ok()?;

    Some(Self {
      digits,
      negative,
      point,
    })
  }

  pub(crate) fn into_string(self, digits: usize) -> String {
    if self.digits.bytes().all(|digit| digit == b'0') {
      return "0".into();
    }

    let exponent = self.point - 1;
    let digits = i64::try_from(digits.max(1)).unwrap();

    if exponent < -4 || exponent >= digits {
      self.scientific_string(exponent)
    } else {
      self.fixed_string()
    }
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
      "{}{}e{}",
      self.sign(),
      mantissa,
      Self::format_exponent(exponent)
    )
  }

  fn sign(&self) -> &'static str {
    if self.negative { "-" } else { "" }
  }

  fn trim_zeros(mut result: String) -> String {
    if result.contains('.') {
      while result.ends_with('0') {
        result.pop();
      }

      if result.ends_with('.') {
        result.pop();
      }
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use {super::*, pretty_assertions::assert_eq};

  #[test]
  fn configured_digits_exponent_when_point_exceeds_digits() {
    assert_eq!(
      Decimal::new("1234567890".to_owned(), false, 11).into_string(10),
      "1.23456789e+10"
    );
  }

  #[test]
  fn configured_digits_fixed_when_exponent_within_digits() {
    assert_eq!(
      Decimal::new("1234567890".to_owned(), false, 10).into_string(10),
      "1234567890"
    );
  }

  #[test]
  fn from_rational_decimal_fraction() {
    let actual = Decimal::from_rational(&Rational::from((1234, 100)))
      .map(|decimal| decimal.into_string(16));

    assert_eq!(actual.as_deref(), Some("12.34"));
  }

  #[test]
  fn from_rational_integer() {
    let actual = Decimal::from_rational(&Rational::from(123))
      .map(|decimal| decimal.into_string(16));

    assert_eq!(actual.as_deref(), Some("123"));
  }

  #[test]
  fn from_rational_negative_fraction() {
    let actual = Decimal::from_rational(&Rational::from((-1, 40)))
      .map(|decimal| decimal.into_string(16));

    assert_eq!(actual.as_deref(), Some("-0.025"));
  }

  #[test]
  fn from_rational_non_terminating() {
    let actual = Decimal::from_rational(&Rational::from((1, 3)))
      .map(|decimal| decimal.into_string(16));

    assert_eq!(actual.as_deref(), None);
  }

  #[test]
  fn from_rational_small_fraction() {
    let actual = Decimal::from_rational(&Rational::from((1, 1000)))
      .map(|decimal| decimal.into_string(16));

    assert_eq!(actual.as_deref(), Some("0.001"));
  }

  #[test]
  fn from_rational_small_scientific() {
    let actual = Decimal::from_rational(&Rational::from((1, 100_000)))
      .map(|decimal| decimal.into_string(16));

    assert_eq!(actual.as_deref(), Some("1e-05"));
  }

  #[test]
  fn from_rational_twentieth() {
    let actual = Decimal::from_rational(&Rational::from((1, 20)))
      .map(|decimal| decimal.into_string(16));

    assert_eq!(actual.as_deref(), Some("0.05"));
  }

  #[test]
  fn into_string_adds_trailing_zeros() {
    assert_eq!(
      Decimal::new("123".to_owned(), false, 5).into_string(16),
      "12300"
    );
  }

  #[test]
  fn into_string_fraction_with_leading_zero() {
    assert_eq!(
      Decimal::new("123".to_owned(), false, 0).into_string(16),
      "0.123"
    );
  }

  #[test]
  fn into_string_large_fixed_boundary() {
    assert_eq!(
      Decimal::new("1".to_owned(), false, 16).into_string(16),
      "1000000000000000"
    );
  }

  #[test]
  fn into_string_large_scientific() {
    assert_eq!(
      Decimal::new("1".to_owned(), false, 17).into_string(16),
      "1e+16"
    );
  }

  #[test]
  fn into_string_positive_integer() {
    assert_eq!(
      Decimal::new("123".to_owned(), false, 3).into_string(16),
      "123"
    );
  }

  #[test]
  fn into_string_scientific_large_digits() {
    assert_eq!(
      Decimal::new("1234567890123456".to_owned(), false, 17).into_string(16),
      "1.234567890123456e+16"
    );
  }

  #[test]
  fn into_string_scientific_small_digits() {
    assert_eq!(
      Decimal::new("3600216012960922".to_owned(), false, -12).into_string(16),
      "3.600216012960922e-13"
    );
  }

  #[test]
  fn into_string_small_fixed_fraction() {
    assert_eq!(
      Decimal::new("123".to_owned(), false, -3).into_string(16),
      "0.000123"
    );
  }

  #[test]
  fn into_string_small_scientific_fraction() {
    assert_eq!(
      Decimal::new("123".to_owned(), false, -4).into_string(16),
      "1.23e-05"
    );
  }

  #[test]
  fn into_string_trims_fractional_zeros() {
    assert_eq!(
      Decimal::new("2300".to_owned(), false, 2).into_string(16),
      "23"
    );
  }

  #[test]
  fn into_string_with_decimal_point() {
    assert_eq!(
      Decimal::new("123".to_owned(), false, 1).into_string(16),
      "1.23"
    );
  }

  #[test]
  fn into_string_zero() {
    assert_eq!(Decimal::new("0".to_owned(), false, 1).into_string(16), "0");
  }

  #[test]
  fn into_string_zero_ignores_negative_sign() {
    assert_eq!(Decimal::new("0".to_owned(), true, 1).into_string(16), "0");
  }
}
