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

use crate::Integer;
use crate::serdeize::Data;
use borsh::io::{Error, ErrorKind, Read, Result, Write};
use borsh::{BorshDeserialize, BorshSerialize};

impl BorshSerialize for Integer {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        let data: Data = self.into();
        data.serialize(writer)
    }
}

impl BorshDeserialize for Integer {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let data = Data::deserialize_reader(reader)?;
        let p: super::big::ParseIncomplete = data
            .try_into()
            .map_err(|error| Error::new(ErrorKind::InvalidInput, error))?;
        Ok(Integer::from(p))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Assign, Integer};
    use az::StrictCast;

    fn assert(a: &Integer, b: &Integer) {
        assert_eq!(a, b);
    }

    enum Check<'a> {
        SerDe(&'a Integer),
        De(&'a Integer),
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
                Check::SerDe(i) => {
                    borsh_assert_value(i, &bytes, assert);
                }
                Check::De(i) => {
                    borsh_assert_de_value(i, &bytes, assert);
                }
                Check::DeError(msg) => {
                    borsh_assert_de_error::<Integer>(&bytes, msg);
                }
            }
        }
    }

    #[test]
    fn check() {
        Check::DeError("radix 1 less than minimum 2").check(1, "0");
        Check::DeError("radix 37 greater than maximum 36").check(37, "0");

        let mut i = Integer::new();
        Check::SerDe(&i).check(10, "0");
        Check::De(&i).check(10, "+0");
        Check::De(&i).check(10, "-00");

        i.assign(-0xffff_ffff_i64);
        Check::SerDe(&i).check(10, "-4294967295");
        Check::De(&i).check(16, "-ffffffff");

        i = i.abs() + 1;
        Check::SerDe(&i).check(16, "100000000");
        Check::De(&i).check(10, "4294967296");
    }
}
