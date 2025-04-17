use super::*;

pub trait FloatExt {
  fn to_f64(&self, rounding_mode: astro_float::RoundingMode) -> Option<f64>;
}

impl FloatExt for BigFloat {
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
      return Some(match sign {
        Sign::Pos => f64::INFINITY,
        Sign::Neg => f64::NEG_INFINITY,
      });
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
