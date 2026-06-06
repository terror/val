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

use crate::ext::xmpq;
use crate::ext::xmpz;
use crate::misc;
use crate::misc::StringLike;
#[allow(deprecated)]
use crate::rational::SmallRational;
use crate::rational::big;
use crate::rational::{MiniRational, ParseRationalError, TryFromFloatError};
use crate::{Assign, Integer, Rational};
use az::CheckedCast;
use core::fmt::{
    Binary, Debug, Display, Formatter, LowerHex, Octal, Result as FmtResult, UpperHex,
};
use core::hash::{Hash, Hasher};
use core::mem;
use core::mem::MaybeUninit;
use core::str::FromStr;
use gmp_mpfr_sys::gmp;

impl Default for Rational {
    #[inline]
    fn default() -> Rational {
        Rational::new()
    }
}

impl Clone for Rational {
    #[inline]
    fn clone(&self) -> Rational {
        unsafe {
            let mut dst = MaybeUninit::uninit();
            let inner_ptr = misc::cast_ptr_mut(dst.as_mut_ptr());
            let num = misc::cast_ptr_mut(gmp::mpq_numref(inner_ptr));
            xmpz::init_set(num, self.numer());
            let den = misc::cast_ptr_mut(gmp::mpq_denref(inner_ptr));
            xmpz::init_set(den, self.denom());
            dst.assume_init()
        }
    }

    #[inline]
    fn clone_from(&mut self, src: &Rational) {
        self.assign(src);
    }
}

impl Drop for Rational {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            xmpq::clear(self);
        }
    }
}

impl Hash for Rational {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.numer().hash(state);
        self.denom().hash(state);
    }
}

impl FromStr for Rational {
    type Err = ParseRationalError;
    #[inline]
    fn from_str(src: &str) -> Result<Rational, ParseRationalError> {
        Ok(Rational::from(Rational::parse(src)?))
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_radix(self, f, 10, false, "")
    }
}

impl Debug for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_radix(self, f, 10, false, "")
    }
}

impl Binary for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_radix(self, f, 2, false, "0b")
    }
}

impl Octal for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_radix(self, f, 8, false, "0o")
    }
}

impl LowerHex for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_radix(self, f, 16, false, "0x")
    }
}

impl UpperHex for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_radix(self, f, 16, true, "0x")
    }
}

impl Assign for Rational {
    #[inline]
    fn assign(&mut self, src: Rational) {
        drop(mem::replace(self, src));
    }
}

impl Assign<&Rational> for Rational {
    #[inline]
    fn assign(&mut self, src: &Rational) {
        xmpq::set(self, src);
    }
}

impl From<&Rational> for Rational {
    #[inline]
    fn from(src: &Rational) -> Self {
        unsafe {
            let mut dst = MaybeUninit::uninit();
            xmpq::init_set(dst.as_mut_ptr(), src);
            dst.assume_init()
        }
    }
}

impl Assign<MiniRational> for Rational {
    #[inline]
    fn assign(&mut self, mut src: MiniRational) {
        self.assign(src.borrow_excl());
    }
}

impl Assign<&MiniRational> for Rational {
    #[inline]
    fn assign(&mut self, src: &MiniRational) {
        self.assign(&*src.borrow());
    }
}

impl From<MiniRational> for Rational {
    #[inline]
    fn from(mut src: MiniRational) -> Self {
        Rational::from(src.borrow_excl())
    }
}

impl From<&MiniRational> for Rational {
    #[inline]
    fn from(src: &MiniRational) -> Self {
        Rational::from(&*src.borrow())
    }
}

#[allow(deprecated)]
impl Assign<SmallRational> for Rational {
    #[inline]
    fn assign(&mut self, src: SmallRational) {
        self.assign(&*src);
    }
}

#[allow(deprecated)]
impl Assign<&SmallRational> for Rational {
    #[inline]
    fn assign(&mut self, src: &SmallRational) {
        self.assign(&**src);
    }
}

#[allow(deprecated)]
impl From<SmallRational> for Rational {
    #[inline]
    fn from(src: SmallRational) -> Self {
        Rational::from(&*src)
    }
}

#[allow(deprecated)]
impl From<&SmallRational> for Rational {
    #[inline]
    fn from(src: &SmallRational) -> Self {
        Rational::from(&**src)
    }
}

impl<Num> Assign<Num> for Rational
where
    Integer: Assign<Num>,
{
    #[inline]
    fn assign(&mut self, src: Num) {
        // Safety: no need to canonicalize, as denominator will be 1.
        let (num, den) = unsafe { self.as_mut_numer_denom_no_canonicalization() };
        num.assign(src);
        xmpz::set_1(den);
    }
}

impl<Num> From<Num> for Rational
where
    Integer: From<Num>,
{
    #[inline]
    fn from(src: Num) -> Self {
        // Safety: no need to canonicalize, as denominator will be 1.
        unsafe {
            let mut dst = MaybeUninit::uninit();
            let inner_ptr = misc::cast_ptr_mut(dst.as_mut_ptr());
            let num = misc::cast_ptr_mut::<_, Integer>(gmp::mpq_numref(inner_ptr));
            num.write(Integer::from(src));
            let den = misc::cast_ptr_mut(gmp::mpq_denref(inner_ptr));
            xmpz::init_set_u32(den, 1);
            dst.assume_init()
        }
    }
}

impl<Num, Den> Assign<(Num, Den)> for Rational
where
    Integer: Assign<Num> + Assign<Den>,
{
    #[inline]
    fn assign(&mut self, src: (Num, Den)) {
        self.mutate_numer_denom(move |num, den| {
            num.assign(src.0);
            den.assign(src.1);
        });
    }
}

impl<Num, Den> From<(Num, Den)> for Rational
where
    Integer: From<Num> + From<Den>,
{
    #[inline]
    fn from((num, den): (Num, Den)) -> Self {
        let (num, den) = (Integer::from(num), Integer::from(den));
        let mut dst = MaybeUninit::uninit();
        xmpq::write_num_den_canonicalize(&mut dst, num, den);
        // Safety: write_num_den_canonicalize initializes and canonicalizes dst.
        unsafe { dst.assume_init() }
    }
}

impl<'a, Num, Den> Assign<&'a (Num, Den)> for Rational
where
    Integer: Assign<&'a Num> + Assign<&'a Den>,
{
    #[inline]
    fn assign(&mut self, src: &'a (Num, Den)) {
        self.mutate_numer_denom(|num, den| {
            num.assign(&src.0);
            den.assign(&src.1);
        });
    }
}

impl<'a, Num, Den> From<&'a (Num, Den)> for Rational
where
    Integer: From<&'a Num> + From<&'a Den>,
{
    #[inline]
    fn from(src: &'a (Num, Den)) -> Self {
        let (num, den) = (Integer::from(&src.0), Integer::from(&src.1));
        let mut dst = MaybeUninit::uninit();
        xmpq::write_num_den_canonicalize(&mut dst, num, den);
        // Safety: write_num_den_canonicalize initializes and canonicalizes dst.
        unsafe { dst.assume_init() }
    }
}

impl TryFrom<f32> for Rational {
    type Error = TryFromFloatError;
    #[inline]
    fn try_from(value: f32) -> Result<Self, TryFromFloatError> {
        value
            .checked_cast()
            .ok_or(TryFromFloatError { _unused: () })
    }
}

impl TryFrom<f64> for Rational {
    type Error = TryFromFloatError;
    #[inline]
    fn try_from(value: f64) -> Result<Self, TryFromFloatError> {
        value
            .checked_cast()
            .ok_or(TryFromFloatError { _unused: () })
    }
}

fn fmt_radix(
    r: &Rational,
    f: &mut Formatter<'_>,
    radix: i32,
    to_upper: bool,
    prefix: &str,
) -> FmtResult {
    let mut s = StringLike::new_malloc();
    big::append_to_string(&mut s, r, radix, to_upper);
    let st = s.as_str();
    let neg = st.starts_with('-');
    let buf = if neg { &st[1..] } else { st };
    f.pad_integral(!neg, prefix, buf)
}

// Safety: mpq_t is thread safe as guaranteed by the GMP library.
unsafe impl Send for Rational {}
unsafe impl Sync for Rational {}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use crate::rational::MiniRational;
    use crate::{Assign, Rational};

    #[test]
    fn check_assign() {
        let mut r = Rational::from((1, 2));
        assert_eq!(r, MiniRational::from((1, 2)));
        let other = Rational::from((-2, 3));
        r.assign(&other);
        assert_eq!(r, MiniRational::from((-2, 3)));
        r.assign(-other);
        assert_eq!(r, MiniRational::from((2, 3)));
        let another = Rational::from(&r);
        assert_eq!(another, r);
    }

    #[test]
    fn check_fallible_conversions() {
        use crate::tests::{F32, F64};
        for &f in F32 {
            let r = Rational::try_from(f);
            assert_eq!(r.is_ok(), f.is_finite());
            #[cfg(feature = "float")]
            {
                if let Ok(r) = r {
                    assert_eq!(r, f);
                }
            }
        }
        for &f in F64 {
            let r = Rational::try_from(f);
            assert_eq!(r.is_ok(), f.is_finite());
            #[cfg(feature = "float")]
            {
                if let Ok(r) = r {
                    assert_eq!(r, f);
                }
            }
        }
    }
}
