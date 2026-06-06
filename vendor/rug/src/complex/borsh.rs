// Copyright © 2016–2026 Trevor Spiteri

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

use crate::Complex;
use crate::serdeize::{Data, PrecVal};
use borsh::io::{Error, ErrorKind, Read, Result, Write};
use borsh::{BorshDeserialize, BorshSerialize};

impl BorshSerialize for Complex {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        let data: Data = self.into();
        data.serialize(writer)
    }
}

impl BorshDeserialize for Complex {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let data: Data = Data::deserialize_reader(reader)?;
        let prec = match &data.prec {
            PrecVal::Two(prec) => *prec,
            _ => unreachable!(),
        };
        let p: super::big::ParseIncomplete = data
            .try_into()
            .map_err(|error| Error::new(ErrorKind::InvalidInput, error))?;
        Ok(Complex::with_val(prec, p))
    }
}

#[cfg(test)]
mod tests {
    use crate::float::{FreeCache, Special};
    use crate::serdeize::test::*;
    use crate::{Assign, Complex, float};
    use az::StrictCast;

    fn assert(a: &Complex, b: &Complex) {
        assert_eq_float_handle_nan(a.real(), b.real());
        assert_eq_float_handle_nan(a.imag(), b.imag());
    }

    enum Check<'a> {
        SerDe(&'a Complex),
        De(&'a Complex),
        DeError((u32, u32), &'a str),
    }

    impl Check<'_> {
        fn check(self, radix: i32, value: &'static str) {
            use byteorder::{LittleEndian, WriteBytesExt};
            use std::io::Write;
            let prec = match self {
                Check::SerDe(c) | Check::De(c) => c.prec(),
                Check::DeError(p, _) => p,
            };

            let mut bytes = Vec::<u8>::new();
            bytes.write_u8(2).unwrap();
            bytes.write_u32::<LittleEndian>(prec.0).unwrap();
            bytes.write_u32::<LittleEndian>(prec.1).unwrap();
            bytes.write_i32::<LittleEndian>(radix).unwrap();
            bytes
                .write_u32::<LittleEndian>(value.len().strict_cast())
                .unwrap();
            bytes.write_all(value.as_bytes()).unwrap();

            match self {
                Check::SerDe(c) => {
                    borsh_assert_value(c, &bytes, assert);
                }
                Check::De(c) => {
                    borsh_assert_de_value(c, &bytes, assert);
                }
                Check::DeError(_, msg) => {
                    borsh_assert_de_error::<Complex>(&bytes, msg);
                }
            }
        }
    }

    #[test]
    fn check() {
        let prec_min = float::prec_min();
        let real_prec_err = format!("real precision 0 less than minimum {prec_min}");
        let imag_prec_err = format!("imaginary precision 0 less than minimum {prec_min}");
        Check::DeError((0, 32), &real_prec_err).check(10, "0");
        Check::DeError((40, 0), &imag_prec_err).check(10, "0");
        Check::DeError((40, 32), "radix 1 less than minimum 2").check(1, "0");
        Check::DeError((40, 32), "radix 37 greater than maximum 36").check(37, "0");

        let mut c = Complex::new((40, 32));
        Check::SerDe(&c).check(10, "(0 0)");
        Check::De(&c).check(10, "0");

        c = -c;
        Check::SerDe(&c).check(10, "(-0 -0)");
        Check::De(&c).check(16, "(-0 -0)");

        c.assign((Special::Nan, 15.0));
        Check::SerDe(&c).check(10, "(NaN 15.000000000)");
        Check::De(&c).check(10, "(+@nan@ 15)");
        c = -c;
        Check::SerDe(&c).check(10, "(-NaN -15.000000000)");

        c.assign((15.0, Special::Nan));
        Check::SerDe(&c).check(16, "(f.0000000000 @NaN@)");
        Check::De(&c).check(10, "(1.5e1 nan)");
        Check::De(&c).check(15, "(0.10@2 @nan@)");

        c <<= 100;
        Check::SerDe(&c).check(16, "(f.0000000000@25 @NaN@)");

        float::free_cache(FreeCache::All);
    }
}
