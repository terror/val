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

use crate::Float;
use crate::serdeize::{Data, PrecVal};
use borsh::io::{Error, ErrorKind, Read, Result, Write};
use borsh::{BorshDeserialize, BorshSerialize};

impl BorshSerialize for Float {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        let data: Data = self.into();
        data.serialize(writer)
    }
}

impl BorshDeserialize for Float {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let data = Data::deserialize_reader(reader)?;
        let prec = match &data.prec {
            PrecVal::One(prec) => *prec,
            _ => unreachable!(),
        };
        let p: super::big::ParseIncomplete = data
            .try_into()
            .map_err(|error| Error::new(ErrorKind::InvalidInput, error))?;
        Ok(Float::with_val(prec, p))
    }
}

#[cfg(test)]
mod tests {
    use crate::float;
    use crate::float::{FreeCache, Special};
    use crate::{Assign, Float};
    use az::StrictCast;

    fn assert(a: &Float, b: &Float) {
        assert_eq!(a.prec(), b.prec());
        assert_eq!(a.as_ord(), b.as_ord());
    }

    enum Check<'a> {
        SerDe(&'a Float),
        De(&'a Float),
        DeError(u32, &'a str),
    }

    impl Check<'_> {
        fn check(self, radix: i32, value: &'static str) {
            use crate::serdeize::test::*;
            use byteorder::{LittleEndian, WriteBytesExt};
            use std::io::Write;
            let prec = match self {
                Check::SerDe(f) | Check::De(f) => f.prec(),
                Check::DeError(p, _) => p,
            };
            let mut bytes = Vec::<u8>::new();
            bytes.write_u8(1).unwrap();
            bytes.write_u32::<LittleEndian>(prec).unwrap();
            bytes.write_i32::<LittleEndian>(radix).unwrap();
            bytes
                .write_u32::<LittleEndian>(value.len().strict_cast())
                .unwrap();
            bytes.write_all(value.as_bytes()).unwrap();
            match self {
                Check::SerDe(f) => {
                    borsh_assert_value(f, &bytes, assert);
                }
                Check::De(f) => {
                    borsh_assert_de_value(f, &bytes, assert);
                }
                Check::DeError(_, msg) => {
                    borsh_assert_de_error::<Float>(&bytes, msg);
                }
            }
        }
    }

    #[test]
    fn check() {
        let prec_err = format!("precision 0 less than minimum {}", float::prec_min());
        Check::DeError(0, &prec_err).check(10, "0");
        Check::DeError(40, "radix 1 less than minimum 2").check(1, "0");
        Check::DeError(40, "radix 37 greater than maximum 36").check(37, "0");

        let mut f = Float::new(40);
        Check::SerDe(&f).check(10, "0");
        Check::De(&f).check(10, "+0.0e5");

        f = -f;
        Check::SerDe(&f).check(10, "-0");
        Check::De(&f).check(16, "-0");

        f.assign(Special::Nan);
        Check::SerDe(&f).check(10, "NaN");
        Check::De(&f).check(10, "+@nan@");
        f = -f;
        Check::SerDe(&f).check(10, "-NaN");

        f.assign(15.0);
        Check::SerDe(&f).check(16, "f.0000000000");
        Check::De(&f).check(10, "15");
        Check::De(&f).check(15, "10");

        f.set_prec(32);
        Check::SerDe(&f).check(10, "15.000000000");
        Check::De(&f).check(16, "f");
        Check::De(&f).check(16, "0.f@1");
        Check::De(&f).check(15, "1.0@1");

        float::free_cache(FreeCache::All);
    }
}
