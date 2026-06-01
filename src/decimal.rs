use super::*;

#[derive(Clone, Debug)]
pub(crate) struct Decimal {
  digits: String,
  negative: bool,
  point: i64,
}

impl Decimal {
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

  pub(crate) fn into_string(self) -> String {
    if self.digits.bytes().all(|digit| digit == b'0') {
      return "0".into();
    }

    let sign = if self.negative { "-" } else { "" };
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
  fn from_rational() {
    #[track_caller]
    fn case(number: &Rational, expected: Option<&str>) {
      let actual = Decimal::from_rational(number).map(Decimal::into_string);

      assert_eq!(actual.as_deref(), expected);
    }

    case(&Rational::from(123), Some("123"));
    case(&Rational::from((1234, 100)), Some("12.34"));
    case(&Rational::from((1, 1000)), Some("0.001"));
    case(&Rational::from((1, 2)), Some("0.5"));
    case(&Rational::from((1, 8)), Some("0.125"));
    case(&Rational::from((1, 20)), Some("0.05"));
    case(&Rational::from((-1, 40)), Some("-0.025"));
    case(&Rational::from((1, 3)), None);
  }

  #[test]
  fn into_string() {
    #[track_caller]
    fn case(digits: &str, negative: bool, point: i64, expected: &str) {
      assert_eq!(
        Decimal::new(digits.to_owned(), negative, point).into_string(),
        expected
      );
    }

    case("0", false, 1, "0");
    case("0", true, 1, "0");
    case("123", false, 3, "123");
    case("123", true, 3, "-123");
    case("123", false, 1, "1.23");
    case("123", false, 0, "0.123");
    case("123", false, -2, "0.00123");
    case("123", false, 5, "12300");
    case("2300", false, 2, "23");
  }
}
