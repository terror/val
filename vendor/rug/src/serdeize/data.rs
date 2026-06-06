// Copyright © 2016–2026 Trevor Spiteri, Kartik Soneji

// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU Lesser General Public License and
// a copy of the GNU General Public License along with this program. If not, see
// <https://www.gnu.org/licenses/>.

#![allow(dead_code)]

#[cfg(feature = "complex")]
use crate::Complex;
#[cfg(feature = "float")]
use crate::Float;
#[cfg(feature = "integer")]
use crate::Integer;
#[cfg(feature = "rational")]
use crate::Rational;
use crate::serdeize::check_range;

pub enum PrecReq {
    Zero,
    One,
    Two,
}

#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub enum PrecVal {
    Zero,
    One(u32),
    Two((u32, u32)),
}

#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct Data {
    pub prec: PrecVal,
    pub radix: i32,
    pub value: String,
}

#[cfg(feature = "complex")]
impl From<&Complex> for Data {
    fn from(value: &Complex) -> Self {
        let prec = value.prec();
        let radix = if (prec.0 <= 32 || !value.real().is_normal())
            && (prec.1 <= 32 || !value.imag().is_normal())
        {
            10
        } else {
            16
        };
        let prec = PrecVal::Two(prec);
        let value = value.to_string_radix(radix, None);
        Data { prec, radix, value }
    }
}

#[cfg(feature = "complex")]
impl TryFrom<Data> for crate::complex::big::ParseIncomplete {
    type Error = String;

    fn try_from(Data { prec, radix, value }: Data) -> Result<Self, Self::Error> {
        use crate::float;

        let PrecVal::Two(prec) = prec else {
            unreachable!();
        };
        check_range(
            "real precision",
            prec.0,
            float::prec_min(),
            float::prec_max(),
        )?;
        check_range(
            "imaginary precision",
            prec.1,
            float::prec_min(),
            float::prec_max(),
        )?;
        check_range("radix", radix, 2, 36)?;

        Complex::parse_radix(value, radix).map_err(|e| e.to_string())
    }
}

#[cfg(feature = "float")]
impl From<&Float> for Data {
    fn from(value: &Float) -> Self {
        let prec = value.prec();
        let radix = if prec <= 32 || !value.is_normal() {
            10
        } else {
            16
        };
        let prec = PrecVal::One(prec);
        let value = value.to_string_radix(radix, None);
        Data { prec, radix, value }
    }
}

#[cfg(feature = "float")]
impl TryFrom<Data> for crate::float::big::ParseIncomplete {
    type Error = String;

    fn try_from(Data { prec, radix, value }: Data) -> Result<Self, Self::Error> {
        use crate::float;

        let PrecVal::One(prec) = prec else {
            unreachable!();
        };
        check_range("precision", prec, float::prec_min(), float::prec_max())?;
        check_range("radix", radix, 2, 36)?;
        Float::parse_radix(value, radix).map_err(|e| e.to_string())
    }
}

#[cfg(feature = "integer")]
impl From<&Integer> for Data {
    fn from(value: &Integer) -> Self {
        let prec = PrecVal::Zero;
        let radix = if value.significant_bits() <= 32 {
            10
        } else {
            16
        };
        let value = value.to_string_radix(radix);
        Data { prec, radix, value }
    }
}

#[cfg(feature = "integer")]
impl TryFrom<Data> for crate::integer::big::ParseIncomplete {
    type Error = String;

    fn try_from(Data { prec, radix, value }: Data) -> Result<Self, Self::Error> {
        match prec {
            PrecVal::Zero => {}
            _ => unreachable!(),
        }
        check_range("radix", radix, 2, 36)?;
        Integer::parse_radix(value, radix).map_err(|e| e.to_string())
    }
}

#[cfg(feature = "rational")]
impl From<&Rational> for Data {
    fn from(value: &Rational) -> Self {
        let prec = PrecVal::Zero;
        let radix =
            if value.numer().significant_bits() <= 32 && value.denom().significant_bits() <= 32 {
                10
            } else {
                16
            };
        let value = value.to_string_radix(radix);
        Data { prec, radix, value }
    }
}

#[cfg(feature = "rational")]
impl TryFrom<Data> for crate::rational::big::ParseIncomplete {
    type Error = String;

    fn try_from(Data { prec, radix, value }: Data) -> Result<Self, Self::Error> {
        match prec {
            PrecVal::Zero => {}
            _ => unreachable!(),
        }
        check_range("radix", radix, 2, 36)?;
        Rational::parse_radix(value, radix).map_err(|e| e.to_string())
    }
}
