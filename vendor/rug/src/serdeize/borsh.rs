// Copyright © 2025 Kartik Soneji

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

#[cfg(test)]
pub mod test {
    use borsh::{BorshDeserialize, BorshSerialize};

    pub fn borsh_assert_value<T, F>(t: &T, val: &[u8], test: F)
    where
        T: BorshSerialize + BorshDeserialize,
        F: Fn(&T, &T),
    {
        let mut enc = vec![];
        t.serialize(&mut enc).unwrap();
        assert_eq!(enc, val);

        let dec = T::deserialize(&mut enc.as_slice()).unwrap();
        test(t, &dec);
    }

    pub fn borsh_assert_de_value<T, F>(t: &T, val: &[u8], test: F)
    where
        T: BorshDeserialize,
        F: Fn(&T, &T),
    {
        let dec = T::try_from_slice(val).unwrap();
        test(t, &dec);
    }

    pub fn borsh_assert_de_error<T>(val: &[u8], err: &str)
    where
        T: BorshDeserialize,
    {
        let actual_err = T::try_from_slice(val).err().unwrap();
        assert_eq!(actual_err.to_string(), err);
    }
}
