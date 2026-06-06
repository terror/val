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

use crate::Rational;
use crate::serdeize::Data;
use borsh::io::{Error, ErrorKind, Read, Result, Write};
use borsh::{BorshDeserialize, BorshSerialize};

impl BorshSerialize for Rational {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        let data: Data = self.into();
        data.serialize(writer)
    }
}

impl BorshDeserialize for Rational {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let data = Data::deserialize_reader(reader)?;
        let p: super::big::ParseIncomplete = data
            .try_into()
            .map_err(|error| Error::new(ErrorKind::InvalidInput, error))?;
        Ok(Rational::from(p))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Assign, Rational};
    use az::StrictCast;

    fn assert(a: &Rational, b: &Rational) {
        assert_eq!(a, b);
    }

    enum Check<'a> {
        SerDe(&'a Rational),
        De(&'a Rational),
        DeError(&'a str),
    }

    impl Check<'_> {
        fn check(self, radix: i32, value: &'static str) {
            use crate::serdeize::test::*;
            use byteorder::{LittleEndian, WriteBytesExt};
            use std::io::Write;

            let mut bytes = Vec::<u8>::new();
            bytes.write_u8(0).unwrap();
            bytes.write_i32::<LittleEndian>(radix).unwrap();
            bytes
                .write_u32::<LittleEndian>(value.len().strict_cast())
                .unwrap();
            bytes.write_all(value.as_bytes()).unwrap();
            match self {
                Check::SerDe(r) => {
                    borsh_assert_value(r, &bytes, assert);
                }
                Check::De(r) => {
                    borsh_assert_de_value(r, &bytes, assert);
                }
                Check::DeError(msg) => {
                    borsh_assert_de_error::<Rational>(&bytes, msg);
                }
            }
        }
    }

    #[test]
    fn check() {
        Check::DeError("radix 1 less than minimum 2").check(1, "0");
        Check::DeError("radix 37 greater than maximum 36").check(37, "0");

        let mut r = Rational::new();
        Check::SerDe(&r).check(10, "0");
        Check::De(&r).check(10, "+0/1");

        r.assign((11_i64, -0xffff_ffff_i64));
        Check::SerDe(&r).check(10, "-11/4294967295");
        Check::De(&r).check(16, "-b/ffffffff");
        Check::De(&r).check(16, "-b0/ffffffff0");

        r.assign((-11_i64, -0x1_0000_0000_i64));
        Check::SerDe(&r).check(16, "b/100000000");
    }
}
