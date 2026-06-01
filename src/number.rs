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
      let lhs = self.to_float(config);
      let rhs = rhs.to_float(config);
      Self::Approx(
        Float::with_val_round(
          config.precision,
          &lhs + &rhs,
          config.rounding_mode,
        )
        .0,
      )
    }
  }

  fn approx_pow(&self, rhs: &Self, config: Config) -> Self {
    let lhs = self.to_float(config);
    let rhs = rhs.to_float(config);

    Self::Approx(
      Float::with_val_round(
        config.precision,
        lhs.pow(rhs),
        config.rounding_mode,
      )
      .0,
    )
  }

  fn approx_unary(
    &self,
    config: Config,
    f: impl FnOnce(&mut Float, Round) -> std::cmp::Ordering,
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
  pub fn display(&self) -> String {
    self.to_string()
  }

  fn display_float(number: &Float) -> String {
    let (negative, digits, point) =
      number.to_sign_string_exp_round(10, None, Round::Nearest);

    match point {
      Some(point) => {
        Decimal::new(digits, negative, i64::from(point)).into_string()
      }
      None if digits == "0" || digits == "NaN" => digits.to_lowercase(),
      None if negative => format!("-{digits}"),
      None => digits,
    }
  }

  fn display_rational(number: &Rational) -> String {
    Decimal::from_rational(number)
      .map_or_else(|| number.to_string(), Decimal::into_string)
  }

  #[must_use]
  pub fn div(&self, rhs: &Self, config: Config) -> Self {
    if let (Self::Exact(lhs), Self::Exact(rhs)) = (self, rhs) {
      Self::Exact((lhs / rhs).complete())
    } else {
      let lhs = self.to_float(config);
      let rhs = rhs.to_float(config);
      Self::Approx(
        Float::with_val_round(
          config.precision,
          &lhs / &rhs,
          config.rounding_mode,
        )
        .0,
      )
    }
  }

  #[must_use]
  pub fn e(config: Config) -> Self {
    Self::from_i64(1).exp(config)
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
  pub fn from_bool(value: bool) -> Self {
    Self::from_i64(i64::from(value))
  }

  #[must_use]
  pub fn from_i64(value: i64) -> Self {
    Self::Exact(Rational::from(value))
  }

  #[must_use]
  pub fn from_integer(value: Integer) -> Self {
    Self::Exact(Rational::from(value))
  }

  #[must_use]
  pub fn from_usize(value: usize) -> Self {
    Self::from_integer(Integer::from(value))
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
      let lhs = self.to_float(config);
      let rhs = rhs.to_float(config);
      Self::Approx(
        Float::with_val_round(
          config.precision,
          &lhs * &rhs,
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
  pub fn one() -> Self {
    Self::from_i64(1)
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
  pub fn phi(config: Config) -> Self {
    Self::one()
      .add(&Self::from_i64(5).sqrt(config), config)
      .div(&Self::from_i64(2), config)
  }

  #[must_use]
  pub fn pi(config: Config) -> Self {
    Self::Approx(
      Float::with_val_round(
        config.precision,
        Constant::Pi,
        config.rounding_mode,
      )
      .0,
    )
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
      let lhs = self.to_float(config);
      let rhs = rhs.to_float(config);
      Self::Approx(
        Float::with_val_round(
          config.precision,
          &lhs - &rhs,
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
    Self::pi(config).mul(&Self::from_i64(2), config)
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

  #[must_use]
  pub fn zero() -> Self {
    Self::from_i64(0)
  }
}

impl Display for Number {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Approx(number) => f.write_str(&Self::display_float(number)),
      Self::Exact(number) => f.write_str(&Self::display_rational(number)),
    }
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
  fn display_approx() {
    #[track_caller]
    fn case(number: Float, expected: &str) {
      let number = Number::Approx(number);

      assert_eq!(number.to_string(), expected);
      assert_eq!(number.display(), expected);
    }

    case(Float::with_val(8, 23), "23");
    case(Float::with_val(8, -0.0625), "-0.0625");
    case(Float::with_val(8, 4.8e4), "48130");
    case(Float::with_val(8, Special::Infinity), "inf");
    case(Float::with_val(8, Special::NegInfinity), "-inf");
    case(Float::with_val(8, Special::Nan), "nan");
  }

  #[test]
  fn list_indexes() {
    assert_eq!(
      Number::parse_decimal("1").unwrap().to_non_negative_usize(),
      Some(1)
    );
    assert_eq!(
      Number::parse_decimal("1.0")
        .unwrap()
        .to_non_negative_usize(),
      Some(1)
    );
    assert_eq!(
      Number::parse_decimal("1.5")
        .unwrap()
        .to_non_negative_usize(),
      None
    );
    assert_eq!(
      Number::parse_decimal("-1").unwrap().to_non_negative_usize(),
      None
    );
  }
}
