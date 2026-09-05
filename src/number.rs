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
    match (self, rhs) {
      (Self::Approx(lhs), Self::Approx(rhs)) => Self::Approx(
        Float::with_val_round(
          config.precision(),
          lhs + rhs,
          config.rounding_mode,
        )
        .0,
      ),
      (Self::Approx(lhs), Self::Exact(rhs))
      | (Self::Exact(rhs), Self::Approx(lhs)) => Self::Approx(
        Float::with_val_round(
          config.precision(),
          lhs + rhs,
          config.rounding_mode,
        )
        .0,
      ),
      (Self::Exact(lhs), Self::Exact(rhs)) => {
        Self::Exact((lhs + rhs).complete())
      }
    }
  }

  fn approx_pow(&self, rhs: &Self, config: Config) -> Self {
    self.approx_unary(config, |lhs, round| match rhs {
      Self::Exact(rhs) if rhs.is_integer() => {
        lhs.pow_assign_round(rhs.numer(), round)
      }
      _ => lhs.pow_assign_round(rhs.to_float(config), round),
    })
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
          Self::Approx(Float::with_val(config.precision(), number))
            .display(config)
        }
      }
    }
  }

  /// # Errors
  ///
  /// Returns [`Error::DivisionByZero`] if `rhs` is zero.
  pub fn div(
    &self,
    rhs: &Self,
    config: Config,
  ) -> std::result::Result<Self, Error> {
    if rhs.is_zero() {
      Err(Error::DivisionByZero)
    } else {
      Ok(match (self, rhs) {
        (Self::Approx(lhs), Self::Approx(rhs)) => Self::Approx(
          Float::with_val_round(
            config.precision(),
            lhs / rhs,
            config.rounding_mode,
          )
          .0,
        ),
        (Self::Approx(lhs), Self::Exact(rhs)) => Self::Approx(
          Float::with_val_round(
            config.precision(),
            lhs / rhs,
            config.rounding_mode,
          )
          .0,
        ),
        (Self::Exact(lhs), Self::Approx(rhs)) => Self::Approx(
          Float::with_val_round(
            config.precision(),
            lhs / rhs,
            config.rounding_mode,
          )
          .0,
        ),
        (Self::Exact(lhs), Self::Exact(rhs)) => {
          Self::Exact((lhs / rhs).complete())
        }
      })
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
    match (self, rhs) {
      (Self::Approx(lhs), Self::Approx(rhs)) => Self::Approx(
        Float::with_val_round(
          config.precision(),
          lhs * rhs,
          config.rounding_mode,
        )
        .0,
      ),
      (Self::Approx(lhs), Self::Exact(rhs))
      | (Self::Exact(rhs), Self::Approx(lhs)) => Self::Approx(
        Float::with_val_round(
          config.precision(),
          lhs * rhs,
          config.rounding_mode,
        )
        .0,
      ),
      (Self::Exact(lhs), Self::Exact(rhs)) => {
        Self::Exact((lhs * rhs).complete())
      }
    }
  }

  #[must_use]
  pub fn neg(&self) -> Self {
    match self {
      Self::Approx(number) => Self::Approx(-number.clone()),
      Self::Exact(number) => Self::Exact(-number.clone()),
    }
  }

  /// # Errors
  ///
  /// Returns [`Error::ZeroToNegativePower`] if `self` is zero and
  /// `rhs` is negative.
  pub fn pow(
    &self,
    rhs: &Self,
    config: Config,
  ) -> std::result::Result<Self, Error> {
    if self.is_zero() && rhs.is_negative() {
      return Err(Error::ZeroToNegativePower);
    }

    if let (Self::Exact(lhs), Self::Exact(rhs)) = (self, rhs)
      && rhs.is_integer()
      && let Some(rhs) = rhs.numer().to_i32()
    {
      return Ok(Self::Exact(lhs.clone().pow(rhs)));
    }

    Ok(self.approx_pow(rhs, config))
  }

  /// # Errors
  ///
  /// Returns [`Error::ModuloByZero`] if `rhs` is zero.
  pub fn rem(
    &self,
    rhs: &Self,
    config: Config,
  ) -> std::result::Result<Self, Error> {
    if rhs.is_zero() {
      return Err(Error::ModuloByZero);
    }

    match (self, rhs) {
      (Self::Exact(lhs), Self::Exact(rhs)) => {
        Ok(Self::Exact((lhs / rhs).complete().rem_floor() * rhs))
      }
      (Self::Approx(lhs), Self::Approx(rhs))
        if lhs.is_finite() && rhs.is_finite() =>
      {
        let precision = lhs.prec().max(rhs.prec());
        let remainder = Float::with_val(precision, lhs % rhs);

        let remainder = if !remainder.is_zero()
          && remainder.is_sign_negative() != rhs.is_sign_negative()
        {
          Float::with_val_round(
            config.precision(),
            &remainder + rhs,
            config.rounding_mode,
          )
          .0
        } else {
          Float::with_val_round(
            config.precision(),
            remainder,
            config.rounding_mode,
          )
          .0
        };

        Ok(Self::Approx(remainder))
      }
      (Self::Exact(lhs), Self::Approx(rhs)) if rhs.is_finite() => {
        let Some(rhs) = rhs.to_rational() else {
          unreachable!();
        };

        let remainder = (lhs / &rhs).complete().rem_floor() * &rhs;

        Ok(Self::Approx(
          Float::with_val_round(
            config.precision(),
            remainder,
            config.rounding_mode,
          )
          .0,
        ))
      }
      (Self::Approx(lhs), Self::Exact(rhs)) if lhs.is_finite() => {
        let Some(lhs) = lhs.to_rational() else {
          unreachable!();
        };

        let remainder = (&lhs / rhs).complete().rem_floor() * rhs;

        Ok(Self::Approx(
          Float::with_val_round(
            config.precision(),
            remainder,
            config.rounding_mode,
          )
          .0,
        ))
      }
      _ => Ok(Self::Approx(
        Float::with_val_round(
          config.precision(),
          &self.to_float(config) % &rhs.to_float(config),
          config.rounding_mode,
        )
        .0,
      )),
    }
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
      Self::Exact(number)
        if number.numer().is_perfect_square()
          && number.denom().is_perfect_square() =>
      {
        Self::Exact(Rational::from((
          number.numer().clone().sqrt(),
          number.denom().clone().sqrt(),
        )))
      }
      _ => self.approx_unary(config, Float::sqrt_round),
    }
  }

  #[must_use]
  pub fn sub(&self, rhs: &Self, config: Config) -> Self {
    match (self, rhs) {
      (Self::Approx(lhs), Self::Approx(rhs)) => Self::Approx(
        Float::with_val_round(
          config.precision(),
          lhs - rhs,
          config.rounding_mode,
        )
        .0,
      ),
      (Self::Approx(lhs), Self::Exact(rhs)) => Self::Approx(
        Float::with_val_round(
          config.precision(),
          lhs - rhs,
          config.rounding_mode,
        )
        .0,
      ),
      (Self::Exact(lhs), Self::Approx(rhs)) => Self::Approx(
        -Float::with_val_round(
          config.precision(),
          rhs - lhs,
          config.rounding_mode.reverse(),
        )
        .0,
      ),
      (Self::Exact(lhs), Self::Exact(rhs)) => {
        Self::Exact((lhs - rhs).complete())
      }
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
        config.precision(),
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
        Float::with_val_round(config.precision(), number, config.rounding_mode)
          .0
      }
      Self::Exact(number) => {
        Float::with_val_round(config.precision(), number, config.rounding_mode)
          .0
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
    match self {
      Self::Exact(number) if number.is_integer() => number.numer().to_usize(),
      Self::Exact(_) => None,
      Self::Approx(_) => self.to_integer()?.to_usize(),
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
      (Self::Exact(lhs), Self::Approx(rhs)) => lhs.partial_cmp(rhs),
      (Self::Approx(lhs), Self::Exact(rhs)) => lhs.partial_cmp(rhs),
    }
  }
}

impl TryFrom<&str> for Number {
  type Error = Error;

  fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
    let s = s.trim();

    let (negative, s) = s
      .strip_prefix('-')
      .map(|s| (true, s))
      .or_else(|| s.strip_prefix('+').map(|s| (false, s)))
      .unwrap_or((false, s));

    let (s, exponent) = if let Some((s, exponent)) = s.split_once(['e', 'E']) {
      (
        s,
        exponent.parse::<i64>().map_err(|_| Error::InvalidDecimal)?,
      )
    } else {
      (s, 0)
    };

    let (integer, fraction) = s.split_once('.').unwrap_or((s, ""));

    if integer.is_empty() && fraction.is_empty() {
      return Err(Error::InvalidDecimal);
    }

    let mut numerator = Integer::from(0);

    for b in integer.bytes().chain(fraction.bytes()) {
      if !b.is_ascii_digit() {
        return Err(Error::InvalidDecimal);
      }

      numerator *= 10;
      numerator += b - b'0';
    }

    if negative {
      numerator = -numerator;
    }

    let exponent = exponent
      .checked_sub(
        i64::try_from(fraction.len()).map_err(|_| Error::InvalidDecimal)?,
      )
      .ok_or(Error::InvalidDecimal)?;

    let scale = Integer::from(10).pow(
      u32::try_from(exponent.unsigned_abs())
        .map_err(|_| Error::InvalidDecimal)?,
    );

    Ok(Self::Exact(if exponent.is_negative() {
      Rational::from((numerator, scale))
    } else {
      Rational::from(numerator * scale)
    }))
  }
}

#[cfg(test)]
mod tests {
  use {super::*, pretty_assertions::assert_eq, rug::float::Special};

  #[test]
  fn approx_remainder_preserves_operand_values() {
    #[track_caller]
    fn case(lhs: &Number, rhs: &Number, expected: i32) {
      let config = Config {
        precision: 4,
        ..Config::default()
      };

      assert_eq!(
        lhs.rem(rhs, config),
        Ok(Number::Approx(Float::with_val(4, expected)))
      );
    }

    case(
      &Number::Exact(Rational::from((31, 32))),
      &Number::Approx(Float::with_val(4, 1)),
      1,
    );

    case(
      &Number::Approx(Float::with_val(5, 31)),
      &Number::from(32_i64),
      32,
    );

    case(
      &Number::Approx(Float::with_val(5, 31)),
      &Number::Approx(Float::with_val(5, 32)),
      32,
    );

    case(
      &Number::Approx(Float::with_val(4, -5)),
      &Number::Approx(Float::with_val(4, 3)),
      1,
    );
  }

  #[test]
  fn display_approx_configured_digits() {
    let config = Config {
      digits: NonZeroUsize::new(4).unwrap(),
      ..Config::default()
    };

    let number = Number::from(2_i64)
      .to_approx(Config::default())
      .div(&Number::from(5_555_222_222_222_i64), Config::default())
      .unwrap();

    assert_eq!(number.display(config), "3.6e-13");
  }

  #[test]
  fn display_approx_infinity() {
    assert_eq!(
      Number::Approx(Float::with_val(8, Special::Infinity)).to_string(),
      "inf"
    );
  }

  #[test]
  fn display_approx_nan() {
    assert_eq!(
      Number::Approx(Float::with_val(8, Special::Nan)).to_string(),
      "nan"
    );
  }

  #[test]
  fn display_approx_negative_decimal() {
    assert_eq!(
      Number::Approx(Float::with_val(8, -0.0625)).to_string(),
      "-0.0625"
    );
  }

  #[test]
  fn display_approx_negative_infinity() {
    assert_eq!(
      Number::Approx(Float::with_val(8, Special::NegInfinity)).to_string(),
      "-inf"
    );
  }

  #[test]
  fn display_approx_positive_integer() {
    assert_eq!(Number::Approx(Float::with_val(8, 23)).to_string(), "23");
  }

  #[test]
  fn display_approx_rounded_large_integer() {
    assert_eq!(
      Number::Approx(Float::with_val(8, 4.8e4)).to_string(),
      "48128"
    );
  }

  #[test]
  fn display_approx_small_scientific() {
    assert_eq!(
      Number::from(2_i64)
        .to_approx(Config::default())
        .div(&Number::from(5_555_222_222_222_i64), Config::default())
        .unwrap()
        .to_string(),
      "3.600216012960922e-13"
    );
  }

  #[test]
  fn invalid_decimal_returns_error() {
    #[track_caller]
    fn case(value: &str) {
      assert_eq!(Number::try_from(value), Err(Error::InvalidDecimal));
    }

    case(".");
    case("foo");
    case("e1");
    case("1e");
    case("1e+");
    case("1e-");
    case("1e--2");
    case("1e+-2");
    case("1e1.5");
    case("1e2e3");
    case("1e 2");
    case("1e4294967296");
    case("1e-4294967296");
    case("1e9223372036854775808");
    case("1.0e-9223372036854775808");
  }

  #[test]
  fn list_indexes_integer() {
    assert_eq!(
      Number::try_from("1").unwrap().to_non_negative_usize(),
      Some(1)
    );
  }

  #[test]
  fn list_indexes_negative_integer() {
    assert_eq!(
      Number::try_from("-1").unwrap().to_non_negative_usize(),
      None
    );
  }

  #[test]
  fn list_indexes_non_integer() {
    assert_eq!(
      Number::try_from("1.5").unwrap().to_non_negative_usize(),
      None
    );
  }

  #[test]
  fn mixed_exact_approx_comparison_preserves_exact_value() {
    let approx = Number::Approx(Float::with_val(53, 9_007_199_254_740_992_i64));

    let equal = Number::from(9_007_199_254_740_992_i64);
    let greater = Number::from(9_007_199_254_740_993_i64);

    assert_eq!(approx, equal);
    assert_ne!(approx, greater);

    assert!(approx < greater);
    assert!(greater > approx);
  }

  #[test]
  fn mixed_exact_approx_operations_round_once() {
    #[track_caller]
    fn case(actual: &Number, expected: f64) {
      let expected = Number::Approx(Float::with_val(2, expected));
      assert_eq!(actual, &expected);
    }

    let config = Config {
      precision: 2,
      ..Config::default()
    };

    let approx = |value| Number::Approx(Float::with_val(2, value));

    let exact = |numerator, denominator| {
      Number::Exact(Rational::from((numerator, denominator)))
    };

    case(&approx(1.0).add(&exact(13, 50), config), 1.5);
    case(&exact(13, 50).add(&approx(1.0), config), 1.5);

    case(&approx(1.0).sub(&exact(9, 25), config), 0.75);
    case(&exact(9, 25).sub(&approx(1.0), config), -0.75);

    case(&approx(1.5).mul(&exact(21, 50), config), 0.75);
    case(&exact(21, 50).mul(&approx(1.5), config), 0.75);

    case(&approx(1.0).div(&exact(13, 8), config).unwrap(), 0.5);
    case(&exact(9, 10).div(&approx(1.5), config).unwrap(), 0.5);
  }

  #[test]
  fn mixed_exact_approx_subtraction_signed_zero() {
    #[track_caller]
    fn case(rounding_mode: Round, negative: bool) {
      let config = Config {
        precision: 2,
        rounding_mode,
        ..Config::default()
      };

      let Number::Approx(result) =
        Number::from(1_i64).sub(&Number::Approx(Float::with_val(2, 1)), config)
      else {
        panic!("expected approximate number");
      };

      assert!(result.is_zero());
      assert_eq!(result.is_sign_negative(), negative);
    }

    case(Round::Down, true);
    case(Round::Up, false);
  }

  #[test]
  fn power_preserves_large_exact_exponent() {
    #[track_caller]
    fn case(lhs: &Number, exponent: &str) {
      let config = Config {
        precision: 53,
        ..Config::default()
      };

      assert_eq!(
        lhs.pow(&Number::try_from(exponent).unwrap(), config),
        Ok(Number::Approx(Float::with_val(53, -1)))
      );
    }

    case(&Number::from(-1_i64), "9007199254740993");

    let lhs = Number::Approx(Float::with_val(53, -1));

    case(&lhs, "9007199254740993");
    case(&lhs, "-9007199254740993");
    case(&lhs, "18446744073709551617");
  }

  #[test]
  fn power_rounds_with_configured_mode() {
    #[track_caller]
    fn case(lhs: &Number, rhs: &Number, rounding_mode: Round, expected: f64) {
      let config = Config {
        precision: 2,
        rounding_mode,
        ..Config::default()
      };

      let Number::Approx(number) = lhs.pow(rhs, config).unwrap() else {
        panic!("expected approximate number");
      };

      assert_eq!(number, expected);
      assert_eq!(number.prec(), config.precision());
    }

    let lhs = Number::Approx(Float::with_val(2, 1.5));

    case(&lhs, &Number::from(2_i64), Round::Up, 3.0);

    case(&lhs, &Number::Approx(Float::with_val(2, 2)), Round::Up, 3.0);

    case(
      &Number::from(2_i64),
      &Number::Exact(Rational::from((1, 2))),
      Round::Down,
      1.0,
    );
  }

  #[test]
  fn scientific_notation() {
    #[track_caller]
    fn case(value: &str, numerator: i64, denominator: i64) {
      let Number::Exact(number) = Number::try_from(value).unwrap() else {
        panic!("expected exact number");
      };

      assert_eq!(number, Rational::from((numerator, denominator)));
    }

    case("1e-05", 1, 100_000);
    case("1E5", 100_000, 1);
    case("1e+05", 100_000, 1);
    case("1.25e1", 25, 2);
    case("1.25e-1", 1, 8);
    case("1.25e2", 125, 1);
    case("-1e2", -100, 1);
    case("+1e0", 1, 1);
    case("0e0", 0, 1);
    case("9007199254740993e0", 9_007_199_254_740_993, 1);
  }

  #[test]
  fn undefined_exact_arithmetic_returns_error() {
    let zero = Number::from(0_i64);

    assert_eq!(
      Number::from(1_i64).div(&zero, Config::default()),
      Err(Error::DivisionByZero)
    );

    assert_eq!(
      Number::from(1_i64).rem(&zero, Config::default()),
      Err(Error::ModuloByZero)
    );

    assert_eq!(
      zero.pow(&Number::from(-1_i64), Config::default()),
      Err(Error::ZeroToNegativePower)
    );
  }

  #[test]
  fn zero_precision_uses_minimum() {
    let config = Config {
      precision: 0,
      ..Config::default()
    };

    let Number::Approx(number) = Number::e(config) else {
      panic!("expected approximate number");
    };

    assert_eq!(number.prec(), 1);
  }
}
