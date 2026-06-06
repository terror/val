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

use crate::serdeize;
use crate::serdeize::{Data, PrecReq};
use crate::{Assign, Rational};
use serde::de::{Deserialize, Deserializer, Error as DeError};
use serde::ser::{Serialize, Serializer};

impl Serialize for Rational {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let data: Data = self.into();
        serdeize::serde::serialize("Rational", &data, serializer)
    }
}

impl<'de> Deserialize<'de> for Rational {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Rational, D::Error> {
        let data = serdeize::serde::deserialize("Rational", PrecReq::Zero, deserializer)?;
        let p: super::big::ParseIncomplete = data.try_into().map_err(DeError::custom)?;
        Ok(Rational::from(p))
    }

    fn deserialize_in_place<D: Deserializer<'de>>(
        deserializer: D,
        place: &mut Rational,
    ) -> Result<(), D::Error> {
        let data = serdeize::serde::deserialize("Rational", PrecReq::Zero, deserializer)?;
        let p: super::big::ParseIncomplete = data.try_into().map_err(DeError::custom)?;
        place.assign(p);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Assign, Rational};
    use az::StrictCast;
    use serde_json::json;

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
            use serde_test::Token;
            use std::io::Write;
            let tokens = [
                Token::Struct {
                    name: "Rational",
                    len: 2,
                },
                Token::Str("radix"),
                Token::I32(radix),
                Token::Str("value"),
                Token::Str(value),
                Token::StructEnd,
            ];
            let json = json!({
                "radix": radix,
                "value": value,
            });
            let mut bincode = Vec::<u8>::new();
            bincode.write_i32::<LittleEndian>(radix).unwrap();
            bincode
                .write_u64::<LittleEndian>(value.len().strict_cast())
                .unwrap();
            bincode.write_all(value.as_bytes()).unwrap();
            match self {
                Check::SerDe(r) => {
                    serde_test::assert_tokens(r, &tokens);
                    json_assert_value(r, &json, assert);
                    let in_place = Rational::from((0xbad, 3));
                    bincode_assert_value(r, &bincode, assert, in_place);
                }
                Check::De(r) => {
                    serde_test::assert_de_tokens(r, &tokens);
                    json_assert_de_value(r, json, assert);
                    bincode_assert_de_value(r, &bincode, assert);
                }
                Check::DeError(msg) => {
                    serde_test::assert_de_tokens_error::<Rational>(&tokens, msg);
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
