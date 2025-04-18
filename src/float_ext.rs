use super::*;

pub trait FloatExt {
  fn display(&self) -> String;
  fn to_f64(&self, rounding_mode: astro_float::RoundingMode) -> Option<f64>;
}

impl FloatExt for Float {
  fn display(&self) -> String {
    match () {
      _ if self.is_nan() => "nan".into(),
      _ if self.is_inf_pos() => "inf".into(),
      _ if self.is_inf_neg() => "-inf".into(),
      _ if self.is_zero() => "0".into(),
      _ => {
        let formatted = self
          .format(
            Radix::Dec,
            astro_float::RoundingMode::None,
            &mut Consts::new().unwrap(),
          )
          .unwrap();

        if !formatted.contains('e') {
          return formatted;
        }

        let (mantissa, exponent) = {
          let mut parts = formatted.split('e');
          (parts.next().unwrap(), parts.next().unwrap())
        };

        let exponent = exponent.parse::<i32>().unwrap();

        let (sign, mantissa) =
          mantissa.split_at(if mantissa.starts_with('-') { 1 } else { 0 });

        let digits = mantissa.replace('.', "");

        let length =
          mantissa.find('.').unwrap_or(mantissa.len()) as i32 + exponent;

        let result = match length {
          length if length <= 0 => {
            format!("{}0.{}{}", sign, "0".repeat((-length) as usize), digits)
          }
          length if (length as usize) >= digits.len() => {
            format!(
              "{}{}{}",
              sign,
              digits,
              "0".repeat(length as usize - digits.len())
            )
          }
          _ => {
            let (left, right) = digits.split_at(length as usize);
            format!("{}{}.{}", sign, left, right)
          }
        };

        if result.find('.').is_some() {
          return result
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string();
        }

        result
      }
    }
  }

  fn to_f64(&self, rounding_mode: astro_float::RoundingMode) -> Option<f64> {
    let mut big_float = self.clone();

    big_float.set_precision(64, rounding_mode).ok()?;

    let sign = big_float.sign()?;

    let exponent = big_float.exponent()? as isize;

    let mantissa_digits = big_float.mantissa_digits()?;

    if mantissa_digits.is_empty() {
      return Some(0.0);
    }

    let mantissa = mantissa_digits[0];

    if mantissa == 0 {
      return Some(0.0);
    }

    let mut exponent = exponent + 0b1111111111;

    let mut ret = 0u64;

    if exponent >= 0b11111111111 {
      Some(match sign {
        Sign::Pos => f64::INFINITY,
        Sign::Neg => f64::NEG_INFINITY,
      })
    } else if exponent <= 0 {
      let shift = -exponent;

      if shift < 52 {
        ret |= mantissa >> (shift + 12);

        if sign == Sign::Neg {
          ret |= 0x8000000000000000u64;
        }

        return Some(f64::from_bits(ret));
      } else {
        return Some(0.0);
      }
    } else {
      let mantissa = mantissa << 1;

      exponent -= 1;

      if sign == Sign::Neg {
        ret |= 1;
      }

      ret <<= 11;
      ret |= exponent as u64;
      ret <<= 52;
      ret |= mantissa >> 12;

      return Some(f64::from_bits(ret));
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn float_from_str(s: &str) -> Float {
    Float::parse(
      s,
      Radix::Dec,
      128,
      astro_float::RoundingMode::FromZero,
      &mut Consts::new().unwrap(),
    )
  }

  #[test]
  fn specials() {
    assert_eq!(format!("{}", Float::from(0).display()), "0");

    assert_eq!(format!("{}", Float::from(f64::INFINITY).display()), "inf");

    assert_eq!(
      format!("{}", Float::from(f64::NEG_INFINITY).display()),
      "-inf"
    );

    assert_eq!(format!("{}", Float::nan(None).display()), "nan");
  }

  #[test]
  fn integers() {
    assert_eq!(Float::from(1).display(), "1");
    assert_eq!(Float::from(-1).display(), "-1");
    assert_eq!(Float::from(123456789).display(), "123456789");
    assert_eq!(Float::from(-123456789).display(), "-123456789");
  }

  #[test]
  fn trailing_zeros() {
    assert_eq!(float_from_str("1.2300e2").display(), "123");
  }

  #[test]
  fn scientific_notation_positive_exponent() {
    assert_eq!(float_from_str("1.23e2").display(), "123");
    assert_eq!(float_from_str("1.23e3").display(), "1230");
  }

  #[test]
  fn scientific_notation_negative_exponent() {
    assert_eq!(float_from_str("1.23e-5").display(), "0.0000123");
    assert_eq!(float_from_str("-1.23e-2").display(), "-0.0123");
    assert_eq!(float_from_str("-1.23e-5").display(), "-0.0000123");
  }

  #[test]
  fn large_numbers() {
    assert_eq!(float_from_str("1e15").display(), "1000000000000000");
    assert_eq!(float_from_str("-1e15").display(), "-1000000000000000");
    assert_eq!(float_from_str("1.23e15").display(), "1230000000000000");
  }

  #[test]
  fn convert_to_double_precision() {
    assert_eq!(
      Float::from(0.0).to_f64(astro_float::RoundingMode::ToEven),
      Some(0.0)
    );

    assert_eq!(
      Float::from(1.0).to_f64(astro_float::RoundingMode::ToEven),
      Some(1.0)
    );

    assert_eq!(
      Float::from(-1.0).to_f64(astro_float::RoundingMode::ToEven),
      Some(-1.0)
    );
  }
}
