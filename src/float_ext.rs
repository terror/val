use super::*;

pub trait FloatExt {
  fn display(&self) -> String;
  fn to_f64(&self, rounding_mode: astro_float::RoundingMode) -> Option<f64>;
}

impl FloatExt for Float {
  fn display(&self) -> String {
    if self.is_nan() {
      return "nan".into();
    }

    if self.is_inf_pos() {
      return "inf".into();
    }

    if self.is_inf_neg() {
      return "-inf".into();
    }

    if self.is_zero() {
      return "0".into();
    }

    let formatted = with_consts(|consts| {
      self.format(Radix::Dec, astro_float::RoundingMode::None, consts)
    })
    .expect("failed to format Float as decimal");

    let Some((mantissa_with_sign, exponent_str)) = formatted.split_once('e')
    else {
      return formatted;
    };

    let Ok(exponent) = exponent_str.parse::<i32>() else {
      return formatted;
    };

    let (sign, mantissa) =
      if let Some(rest) = mantissa_with_sign.strip_prefix('-') {
        ("-", rest)
      } else if let Some(rest) = mantissa_with_sign.strip_prefix('+') {
        ("", rest)
      } else {
        ("", mantissa_with_sign)
      };

    let mut parts = mantissa.split('.');
    let int_part = parts.next().unwrap_or("");
    let frac_part = parts.next().unwrap_or("");

    let mut digits = String::with_capacity(int_part.len() + frac_part.len());
    digits.push_str(int_part);
    digits.push_str(frac_part);

    let length = int_part.len() as i32 + exponent;
    let digits_len = digits.len() as i32;

    let mut result = if length <= 0 {
      let zeros = (-length) as usize;
      let mut out =
        String::with_capacity(sign.len() + 2 + zeros + digits.len());
      out.push_str(sign);
      out.push('0');
      out.push('.');
      out.extend(std::iter::repeat_n('0', zeros));
      out.push_str(&digits);
      out
    } else if length >= digits_len {
      let zeros = (length - digits_len) as usize;
      let mut out = String::with_capacity(sign.len() + digits.len() + zeros);
      out.push_str(sign);
      out.push_str(&digits);
      out.extend(std::iter::repeat_n('0', zeros));
      out
    } else {
      let split_at = length as usize;
      let (left, right) = digits.split_at(split_at);
      let mut out =
        String::with_capacity(sign.len() + left.len() + 1 + right.len());
      out.push_str(sign);
      out.push_str(left);
      out.push('.');
      out.push_str(right);
      out
    };

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

  fn to_f64(&self, rounding_mode: astro_float::RoundingMode) -> Option<f64> {
    if self.is_nan() {
      return None;
    }

    if self.is_inf_pos() {
      return Some(f64::INFINITY);
    }

    if self.is_inf_neg() {
      return Some(f64::NEG_INFINITY);
    }

    if self.is_zero() {
      return Some(0.0);
    }

    let mut big_float = self.clone();
    big_float.set_precision(64, rounding_mode).ok()?;

    let sign = big_float.sign()?;
    let mut exponent = big_float.exponent()? as isize;
    let mantissa_digits = big_float.mantissa_digits()?;
    let mantissa = *mantissa_digits.first().unwrap_or(&0);

    const F64_EXPONENT_BIAS: isize = 0x3ff;
    const F64_EXPONENT_MAX: isize = 0x7ff;
    const F64_SIGNIFICAND_BITS: usize = 52;
    const INTERNAL_SHIFT: usize = 12;
    const SIGN_MASK: u64 = 1u64 << 63;

    if mantissa == 0 {
      return Some(if sign == Sign::Neg {
        f64::from_bits(SIGN_MASK)
      } else {
        0.0
      });
    }

    exponent += F64_EXPONENT_BIAS;

    if exponent >= F64_EXPONENT_MAX {
      return Some(match sign {
        Sign::Pos => f64::INFINITY,
        Sign::Neg => f64::NEG_INFINITY,
      });
    }

    let sign_bit = if sign == Sign::Neg { SIGN_MASK } else { 0 };

    if exponent <= 0 {
      let shift = (-exponent) as usize;

      if shift >= F64_SIGNIFICAND_BITS {
        return Some(f64::from_bits(sign_bit));
      }

      let fraction = mantissa >> (shift + INTERNAL_SHIFT);

      return Some(f64::from_bits(sign_bit | fraction));
    }

    let adjusted_mantissa = mantissa << 1;
    let adjusted_exponent = (exponent - 1) as u64;
    let exponent_bits = adjusted_exponent << F64_SIGNIFICAND_BITS;
    let fraction_bits = adjusted_mantissa >> INTERNAL_SHIFT;

    Some(f64::from_bits(sign_bit | exponent_bits | fraction_bits))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn float_from_str(s: &str) -> Float {
    with_consts(|consts| {
      Float::parse(
        s,
        Radix::Dec,
        128,
        astro_float::RoundingMode::FromZero,
        consts,
      )
    })
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

  #[test]
  fn convert_special_values_to_double_precision() {
    assert_eq!(
      Float::from(f64::INFINITY).to_f64(astro_float::RoundingMode::ToEven),
      Some(f64::INFINITY)
    );

    assert_eq!(
      Float::from(f64::NEG_INFINITY).to_f64(astro_float::RoundingMode::ToEven),
      Some(f64::NEG_INFINITY)
    );

    assert_eq!(
      Float::nan(None).to_f64(astro_float::RoundingMode::ToEven),
      None
    );
  }

  #[test]
  fn convert_underflow_preserves_sign() {
    let tiny_negative = float_from_str("-1e-4000");

    let result = tiny_negative.to_f64(astro_float::RoundingMode::ToEven);

    assert!(result.is_some());

    let value = result.unwrap();

    assert!(value.is_sign_negative());

    assert_eq!(value, -0.0);
  }
}
