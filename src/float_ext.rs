use super::*;

pub trait FloatExt {
  fn to_f64(&self, rounding_mode: astro_float::RoundingMode) -> Option<f64>;
  fn display(&self) -> String;
}

impl FloatExt for BigFloat {
  fn display(&self) -> String {
    if let Some(parts) = self.as_raw_parts() {
      let number =
        BigFloat::from_raw_parts(parts.0, parts.1, parts.2, parts.3, parts.4);

      if number.is_nan() {
        return "NaN".into();
      }

      if number.is_inf_pos() {
        return "Inf".into();
      }

      if number.is_inf_neg() {
        return "-Inf".into();
      }

      if number.is_zero() {
        return "0".into();
      }

      let formatted = number
        .format(
          Radix::Dec,
          astro_float::RoundingMode::None,
          &mut Consts::new().expect("BigFloat constants cache"),
        )
        .unwrap();

      if !formatted.contains('e') {
        return formatted;
      }

      let (mant, exp) = {
        let mut parts = formatted.split('e');
        (parts.next().unwrap(), parts.next().unwrap())
      };

      let exp: i32 = exp.parse().unwrap();

      let (sign, mant) =
        mant.split_at(if mant.starts_with('-') { 1 } else { 0 });

      let digits = mant.replace('.', "");

      let new_int_len = mant.find('.').unwrap_or(mant.len()) as i32 + exp;

      let result = if new_int_len <= 0 {
        format!(
          "{}0.{}{}",
          sign,
          "0".repeat((-new_int_len) as usize),
          digits
        )
      } else if new_int_len as usize >= digits.len() {
        format!(
          "{}{}{}",
          sign,
          digits,
          "0".repeat(new_int_len as usize - digits.len())
        )
      } else {
        let (left, right) = digits.split_at(new_int_len as usize);
        format!("{}{}.{}", sign, left, right)
      };

      if result.find('.').is_some() {
        result
          .trim_end_matches('0')
          .trim_end_matches('.')
          .to_string()
      } else {
        result
      }
    } else {
      self.to_string()
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
