use super::*;

#[derive(Clone, Debug)]
pub enum Number {
  Approx(Float),
  Exact(Rational),
}

impl Number {
  #[must_use]
  pub fn abs(&self) -> Self {
    match self {
      Self::Approx(number) => Self::Approx(number.clone().abs()),
      Self::Exact(number) => Self::Exact(number.clone().abs()),
    }
  }

  #[must_use]
  pub fn acos(&self, config: Config) -> Self {
    self.approx_unary(config, Float::acos_round)
  }

  #[must_use]
  pub fn add(&self, rhs: &Self, config: Config) -> Self {
    if let (Self::Exact(lhs), Self::Exact(rhs)) = (self, rhs) {
      Self::Exact((lhs + rhs).complete())
    } else {
      Self::Approx(
        Float::with_val_round(
          config.precision,
          &self.to_float(config) + &rhs.to_float(config),
          config.rounding_mode,
        )
        .0,
      )
    }
  }

  fn approx_pow(&self, rhs: &Self, config: Config) -> Self {
    Self::Approx(
      Float::with_val_round(
        config.precision,
        self.to_float(config).pow(rhs.to_float(config)),
        config.rounding_mode,
      )
      .0,
    )
  }

  fn approx_unary(
    &self,
    config: Config,
    f: impl FnOnce(&mut Float, Round) -> Ordering,
  ) -> Self {
    let mut number = self.to_float(config);
    f(&mut number, config.rounding_mode);
    Self::Approx(number)
  }

  #[must_use]
  pub fn asin(&self, config: Config) -> Self {
    self.approx_unary(config, Float::asin_round)
  }

  #[must_use]
  pub fn atan(&self, config: Config) -> Self {
    self.approx_unary(config, Float::atan_round)
  }

  #[must_use]
  pub fn ceil(&self) -> Self {
    match self {
      Self::Approx(number) => Self::Approx(number.clone().ceil()),
      Self::Exact(number) => Self::Exact(number.clone().ceil()),
    }
  }

  #[must_use]
  pub fn cos(&self, config: Config) -> Self {
    self.approx_unary(config, Float::cos_round)
  }

  #[must_use]
  pub fn cosh(&self, config: Config) -> Self {
    self.approx_unary(config, Float::cosh_round)
  }

  #[must_use]
  pub fn display(&self, config: Config) -> String {
    match self {
      Self::Approx(number) => {
        let (negative, digits, point) = number.to_sign_string_exp_round(
          10,
          Some(config.digits.get()),
          Round::Nearest,
        );

        match point {
          Some(point) => Decimal::new(digits, negative, i64::from(point))
            .display(config.digits),
          None if digits == "0" || digits == "NaN" => digits.to_lowercase(),
          None if negative => format!("-{digits}"),
          None => digits,
        }
      }
      Self::Exact(number) if number.is_integer() => number.numer().to_string(),
      Self::Exact(number) => {
        if let Some(decimal) = Decimal::from_rational(number) {
          decimal.display(config.digits)
        } else {
          Self::Approx(Float::with_val(config.precision, number))
            .display(config)
        }
      }
    }
  }

  #[must_use]
  pub fn div(&self, rhs: &Self, config: Config) -> Self {
    if let (Self::Exact(lhs), Self::Exact(rhs)) = (self, rhs) {
      Self::Exact((lhs / rhs).complete())
    } else {
      Self::Approx(
        Float::with_val_round(
          config.precision,
          &self.to_float(config) / &rhs.to_float(config),
          config.rounding_mode,
        )
        .0,
      )
    }
  }

  #[must_use]
  pub fn e(config: Config) -> Self {
    Self::from(1_i64).exp(config)
  }

  #[must_use]
  pub fn exp(&self, config: Config) -> Self {
    self.approx_unary(config, Float::exp_round)
  }

  #[must_use]
  pub fn floor(&self) -> Self {
    match self {
      Self::Approx(number) => Self::Approx(number.clone().floor()),
      Self::Exact(number) => Self::Exact(number.clone().floor()),
    }
  }

  #[must_use]
  pub fn is_negative(&self) -> bool {
    match self {
      Self::Approx(number) => {
        matches!(number.cmp0(), Some(std::cmp::Ordering::Less))
      }
      Self::Exact(number) => number.is_negative(),
    }
  }

  #[must_use]
  pub fn is_zero(&self) -> bool {
    match self {
      Self::Approx(number) => number.is_zero(),
      Self::Exact(number) => number.is_zero(),
    }
  }

  #[must_use]
  pub fn ln(&self, config: Config) -> Self {
    self.approx_unary(config, Float::ln_round)
  }

  #[must_use]
  pub fn log10(&self, config: Config) -> Self {
    self.approx_unary(config, Float::log10_round)
  }

  #[must_use]
  pub fn log2(&self, config: Config) -> Self {
    self.approx_unary(config, Float::log2_round)
  }

  #[must_use]
  pub fn mul(&self, rhs: &Self, config: Config) -> Self {
    if let (Self::Exact(lhs), Self::Exact(rhs)) = (self, rhs) {
      Self::Exact((lhs * rhs).complete())
    } else {
      Self::Approx(
        Float::with_val_round(
          config.precision,
          &self.to_float(config) * &rhs.to_float(config),
          config.rounding_mode,
        )
        .0,
      )
    }
  }

  #[must_use]
  pub fn neg(&self) -> Self {
    match self {
      Self::Approx(number) => Self::Approx(-number.clone()),
      Self::Exact(number) => Self::Exact(-number.clone()),
    }
  }

  #[must_use]
  pub fn parse_decimal(s: &str) -> Option<Self> {
    let s = s.trim();

    let (negative, s) = if let Some(s) = s.strip_prefix('-') {
      (true, s)
    } else if let Some(s) = s.strip_prefix('+') {
      (false, s)
    } else {
      (false, s)
    };

    let (integer, fraction) = match s.split_once('.') {
      Some((integer, fraction)) => (integer, fraction),
      None => (s, ""),
    };

    if integer.is_empty() && fraction.is_empty() {
      return None;
    }

    if !integer.chars().all(|c| c.is_ascii_digit())
      || !fraction.chars().all(|c| c.is_ascii_digit())
    {
      return None;
    }

    let digits = format!("{integer}{fraction}");
    let digits = if digits.is_empty() { "0" } else { &digits };

    let mut numerator = Integer::from_str_radix(digits, 10).ok()?;

    if negative {
      numerator = -numerator;
    }

    let mut denominator = Integer::from(1);

    for _ in 0..fraction.len() {
      denominator *= 10;
    }

    Some(Self::Exact(Rational::from((numerator, denominator))))
  }

  #[must_use]
  pub fn pow(&self, rhs: &Self, config: Config) -> Self {
    match (self, rhs) {
      (Self::Exact(lhs), Self::Exact(exponent)) => {
        if exponent.is_integer()
          && let Some(exponent) = exponent.numer().to_i32()
        {
          return Self::Exact(lhs.clone().pow(exponent));
        }

        self.approx_pow(rhs, config)
      }
      _ => self.approx_pow(rhs, config),
    }
  }

  #[must_use]
  pub fn rem(&self, rhs: &Self, config: Config) -> Self {
    self.sub(&self.div(rhs, config).floor().mul(rhs, config), config)
  }

  #[must_use]
  pub fn sin(&self, config: Config) -> Self {
    self.approx_unary(config, Float::sin_round)
  }

  #[must_use]
  pub fn sinh(&self, config: Config) -> Self {
    self.approx_unary(config, Float::sinh_round)
  }

  #[must_use]
  pub fn sqrt(&self, config: Config) -> Self {
    match self {
      Self::Exact(number) => {
        let (numerator, denominator) = number.clone().into_numer_denom();

        if numerator.is_perfect_square() && denominator.is_perfect_square() {
          return Self::Exact(Rational::from((
            numerator.sqrt(),
            denominator.sqrt(),
          )));
        }

        self.approx_unary(config, Float::sqrt_round)
      }
      Self::Approx(_) => self.approx_unary(config, Float::sqrt_round),
    }
  }

  #[must_use]
  pub fn sub(&self, rhs: &Self, config: Config) -> Self {
    if let (Self::Exact(lhs), Self::Exact(rhs)) = (self, rhs) {
      Self::Exact((lhs - rhs).complete())
    } else {
      Self::Approx(
        Float::with_val_round(
          config.precision,
          &self.to_float(config) - &rhs.to_float(config),
          config.rounding_mode,
        )
        .0,
      )
    }
  }

  #[must_use]
  pub fn tan(&self, config: Config) -> Self {
    self.approx_unary(config, Float::tan_round)
  }

  #[must_use]
  pub fn tanh(&self, config: Config) -> Self {
    self.approx_unary(config, Float::tanh_round)
  }

  #[must_use]
  pub fn tau(config: Config) -> Self {
    Self::Approx(
      Float::with_val_round(
        config.precision,
        Constant::Pi,
        config.rounding_mode,
      )
      .0,
    )
    .mul(&Self::from(2_i64), config)
  }

  #[must_use]
  pub fn to_approx(&self, config: Config) -> Self {
    Self::Approx(self.to_float(config))
  }

  #[must_use]
  pub fn to_float(&self, config: Config) -> Float {
    match self {
      Self::Approx(number) => {
        Float::with_val_round(config.precision, number, config.rounding_mode).0
      }
      Self::Exact(number) => {
        Float::with_val_round(config.precision, number, config.rounding_mode).0
      }
    }
  }

  #[must_use]
  pub fn to_i64(&self) -> Option<i64> {
    self.to_integer()?.to_i64()
  }

  #[must_use]
  pub fn to_integer(&self) -> Option<Integer> {
    match self {
      Self::Exact(number) => {
        if number.is_integer() {
          Some(number.numer().clone())
        } else {
          None
        }
      }
      Self::Approx(number) => {
        if number.is_finite() && number.is_integer() {
          Some(number.to_integer_round(Round::Zero)?.0)
        } else {
          None
        }
      }
    }
  }

  #[must_use]
  pub fn to_non_negative_usize(&self) -> Option<usize> {
    let number = self.to_integer()?;

    if number.is_negative() {
      None
    } else {
      number.to_usize()
    }
  }
}

impl Display for Number {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str(&self.display(Config::default()))
  }
}

impl From<bool> for Number {
  fn from(value: bool) -> Self {
    Self::from(i64::from(value))
  }
}

impl From<i64> for Number {
  fn from(value: i64) -> Self {
    Self::Exact(Rational::from(value))
  }
}

impl From<Integer> for Number {
  fn from(value: Integer) -> Self {
    Self::Exact(Rational::from(value))
  }
}

impl From<usize> for Number {
  fn from(value: usize) -> Self {
    Self::from(Integer::from(value))
  }
}

impl PartialEq for Number {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(std::cmp::Ordering::Equal)
  }
}

impl PartialOrd for Number {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    match (self, other) {
      (Self::Exact(lhs), Self::Exact(rhs)) => lhs.partial_cmp(rhs),
      (Self::Approx(lhs), Self::Approx(rhs)) => lhs.partial_cmp(rhs),
      (Self::Exact(lhs), Self::Approx(rhs)) => {
        Float::with_val(rhs.prec(), lhs).partial_cmp(rhs)
      }
      (Self::Approx(lhs), Self::Exact(rhs)) => {
        lhs.partial_cmp(&Float::with_val(lhs.prec(), rhs))
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use {super::*, pretty_assertions::assert_eq, rug::float::Special};

  #[test]
  fn display_approx_configured_digits() {
    let number = Number::from(2_i64)
      .to_approx(Config::default())
      .div(&Number::from(5_555_222_222_222_i64), Config::default());

    let config = Config {
      digits: NonZeroUsize::new(4).unwrap(),
      ..Config::default()
    };

    assert_eq!(number.display(config), "3.6e-13");
  }

  #[test]
  fn display_approx_infinity() {
    let number = Number::Approx(Float::with_val(8, Special::Infinity));

    assert_eq!(number.to_string(), "inf");
  }

  #[test]
  fn display_approx_nan() {
    let number = Number::Approx(Float::with_val(8, Special::Nan));

    assert_eq!(number.to_string(), "nan");
  }

  #[test]
  fn display_approx_negative_decimal() {
    let number = Number::Approx(Float::with_val(8, -0.0625));

    assert_eq!(number.to_string(), "-0.0625");
  }

  #[test]
  fn display_approx_negative_infinity() {
    let number = Number::Approx(Float::with_val(8, Special::NegInfinity));

    assert_eq!(number.to_string(), "-inf");
  }

  #[test]
  fn display_approx_positive_integer() {
    let number = Number::Approx(Float::with_val(8, 23));

    assert_eq!(number.to_string(), "23");
  }

  #[test]
  fn display_approx_rounded_large_integer() {
    let number = Number::Approx(Float::with_val(8, 4.8e4));

    assert_eq!(number.to_string(), "48128");
  }

  #[test]
  fn display_approx_small_scientific() {
    let number = Number::from(2_i64)
      .to_approx(Config::default())
      .div(&Number::from(5_555_222_222_222_i64), Config::default());

    assert_eq!(number.to_string(), "3.600216012960922e-13");
  }

  #[test]
  fn list_indexes_integer() {
    assert_eq!(
      Number::parse_decimal("1").unwrap().to_non_negative_usize(),
      Some(1)
    );
  }

  #[test]
  fn list_indexes_negative_integer() {
    assert_eq!(
      Number::parse_decimal("-1").unwrap().to_non_negative_usize(),
      None
    );
  }

  #[test]
  fn list_indexes_non_integer() {
    assert_eq!(
      Number::parse_decimal("1.5")
        .unwrap()
        .to_non_negative_usize(),
      None
    );
  }
}
