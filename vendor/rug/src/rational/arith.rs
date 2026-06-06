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
use crate::ext::xmpq::OptRational;
use crate::integer::MiniInteger;
use crate::integer::arith::AsLong;
use crate::ops::{AddFrom, DivFrom, MulFrom, NegAssign, Pow, PowAssign, SubFrom};
use crate::rational::MiniRational;
use crate::{Assign, Complete, Integer, Rational};
use az::{CheckedAs, CheckedCast};
use core::ffi::{c_long, c_ulong};
use core::iter::{Product, Sum};
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Shl, ShlAssign, Shr, ShrAssign, Sub,
    SubAssign,
};

arith_unary! {
    Rational;
    xmpq::neg;
    Neg { neg }
    NegAssign { neg_assign }
    NegIncomplete
}
arith_binary_self! {
    Rational;
    xmpq::add;
    Add { add }
    AddAssign { add_assign }
    AddFrom { add_from }
    AddIncomplete;
    rhs_has_more_alloc
}
arith_binary_self! {
    Rational;
    xmpq::sub;
    Sub { sub }
    SubAssign { sub_assign }
    SubFrom { sub_from }
    SubIncomplete;
    rhs_has_more_alloc
}
arith_binary_self! {
    Rational;
    xmpq::mul;
    Mul { mul }
    MulAssign { mul_assign }
    MulFrom { mul_from }
    MulIncomplete;
    rhs_has_more_alloc
}
arith_binary_self! {
    Rational;
    xmpq::div;
    Div { div }
    DivAssign { div_assign }
    DivFrom { div_from }
    DivIncomplete;
    rhs_has_more_alloc
}

arith_mini_commut! {
    Rational;
    Add { add }
    AddAssign { add_assign }
    AddFrom { add_from }
    MiniRational;
    AddMiniIncomplete, AddOwnedMiniIncomplete
}
arith_mini_noncommut! {
    Rational;
    Sub { sub }
    SubAssign { sub_assign }
    SubFrom { sub_from }
    MiniRational;
    SubMiniIncomplete, SubOwnedMiniIncomplete;
    SubFromMiniIncomplete, SubFromOwnedMiniIncomplete
}
arith_mini_commut! {
    Rational;
    Mul { mul }
    MulAssign { mul_assign }
    MulFrom { mul_from }
    MiniRational;
    MulMiniIncomplete, MulOwnedMiniIncomplete
}
arith_mini_noncommut! {
    Rational;
    Div { div }
    DivAssign { div_assign }
    DivFrom { div_from }
    MiniRational;
    DivMiniIncomplete, DivOwnedMiniIncomplete;
    DivFromMiniIncomplete, DivFromOwnedMiniIncomplete
}

arith_commut! {
    Rational;
    xmpq::add_z;
    Add { add }
    AddAssign { add_assign }
    AddFrom { add_from }
    Integer;
    AddIntegerIncomplete, AddOwnedIntegerIncomplete
}
arith_noncommut! {
    Rational;
    xmpq::sub_z;
    xmpq::z_sub;
    Sub { sub }
    SubAssign { sub_assign }
    SubFrom { sub_from }
    Integer;
    SubIntegerIncomplete, SubOwnedIntegerIncomplete;
    SubFromIntegerIncomplete, SubFromOwnedIntegerIncomplete
}
arith_commut! {
    Rational;
    xmpq::mul_z;
    Mul { mul }
    MulAssign { mul_assign }
    MulFrom { mul_from }
    Integer;
    MulIntegerIncomplete, MulOwnedIntegerIncomplete
}
arith_noncommut! {
    Rational;
    xmpq::div_z;
    xmpq::z_div;
    Div { div }
    DivAssign { div_assign }
    DivFrom { div_from }
    Integer;
    DivIntegerIncomplete, DivOwnedIntegerIncomplete;
    DivFromIntegerIncomplete, DivFromOwnedIntegerIncomplete
}

arith_prim_commut! {
    Rational;
    PrimOps::add;
    Add { add }
    AddAssign { add_assign }
    AddFrom { add_from }
    i8, AddI8Incomplete;
    i16, AddI16Incomplete;
    i32, AddI32Incomplete;
    i64, AddI64Incomplete;
    i128, AddI128Incomplete;
    isize, AddIsizeIncomplete;
    u8, AddU8Incomplete;
    u16, AddU16Incomplete;
    u32, AddU32Incomplete;
    u64, AddU64Incomplete;
    u128, AddU128Incomplete;
    usize, AddUsizeIncomplete;
}
arith_prim_noncommut! {
    Rational;
    PrimOps::sub, PrimOps::sub_from;
    Sub { sub }
    SubAssign { sub_assign }
    SubFrom { sub_from }
    i8, SubI8Incomplete, SubFromI8Incomplete;
    i16, SubI16Incomplete, SubFromI16Incomplete;
    i32, SubI32Incomplete, SubFromI32Incomplete;
    i64, SubI64Incomplete, SubFromI64Incomplete;
    i128, SubI128Incomplete, SubFromI128Incomplete;
    isize, SubIsizeIncomplete, SubFromIsizeIncomplete;
    u8, SubU8Incomplete, SubFromU8Incomplete;
    u16, SubU16Incomplete, SubFromU16Incomplete;
    u32, SubU32Incomplete, SubFromU32Incomplete;
    u64, SubU64Incomplete, SubFromU64Incomplete;
    u128, SubU128Incomplete, SubFromU128Incomplete;
    usize, SubUsizeIncomplete, SubFromUsizeIncomplete;
}
arith_prim_commut! {
    Rational;
    PrimOps::mul;
    Mul { mul }
    MulAssign { mul_assign }
    MulFrom { mul_from }
    i8, MulI8Incomplete;
    i16, MulI16Incomplete;
    i32, MulI32Incomplete;
    i64, MulI64Incomplete;
    i128, MulI128Incomplete;
    isize, MulIsizeIncomplete;
    u8, MulU8Incomplete;
    u16, MulU16Incomplete;
    u32, MulU32Incomplete;
    u64, MulU64Incomplete;
    u128, MulU128Incomplete;
    usize, MulUsizeIncomplete;
}
arith_prim_noncommut! {
    Rational;
    PrimOps::div, PrimOps::div_from;
    Div { div }
    DivAssign { div_assign }
    DivFrom { div_from }
    i8, DivI8Incomplete, DivFromI8Incomplete;
    i16, DivI16Incomplete, DivFromI16Incomplete;
    i32, DivI32Incomplete, DivFromI32Incomplete;
    i64, DivI64Incomplete, DivFromI64Incomplete;
    i128, DivI128Incomplete, DivFromI128Incomplete;
    isize, DivIsizeIncomplete, DivFromIsizeIncomplete;
    u8, DivU8Incomplete, DivFromU8Incomplete;
    u16, DivU16Incomplete, DivFromU16Incomplete;
    u32, DivU32Incomplete, DivFromU32Incomplete;
    u64, DivU64Incomplete, DivFromU64Incomplete;
    u128, DivU128Incomplete, DivFromU128Incomplete;
    usize, DivUsizeIncomplete, DivFromUsizeIncomplete;
}

arith_prim! {
    Rational;
    xmpq::shl_i32;
    Shl { shl }
    ShlAssign { shl_assign }
    i32, ShlI32Incomplete;
}
arith_prim! {
    Rational;
    xmpq::shr_i32;
    Shr { shr }
    ShrAssign { shr_assign }
    i32, ShrI32Incomplete;
}
arith_prim! {
    Rational;
    xmpq::shl_u32;
    Shl { shl }
    ShlAssign { shl_assign }
    u32, ShlU32Incomplete;
}
arith_prim! {
    Rational;
    xmpq::shr_u32;
    Shr { shr }
    ShrAssign { shr_assign }
    u32, ShrU32Incomplete;
}
arith_prim! {
    Rational;
    xmpq::shl_isize;
    Shl { shl }
    ShlAssign { shl_assign }
    isize, ShlIsizeIncomplete;
}
arith_prim! {
    Rational;
    xmpq::shr_isize;
    Shr { shr }
    ShrAssign { shr_assign }
    isize, ShrIsizeIncomplete;
}
arith_prim! {
    Rational;
    xmpq::shl_usize;
    Shl { shl }
    ShlAssign { shl_assign }
    usize, ShlUsizeIncomplete;
}
arith_prim! {
    Rational;
    xmpq::shr_usize;
    Shr { shr }
    ShrAssign { shr_assign }
    usize, ShrUsizeIncomplete;
}
arith_prim! {
    Rational;
    xmpq::pow_i32;
    Pow { pow }
    PowAssign { pow_assign }
    i32, PowI32Incomplete;
}
arith_prim! {
    Rational;
    xmpq::pow_u32;
    Pow { pow }
    PowAssign { pow_assign }
    u32, PowU32Incomplete;
}

trait PrimOps<Long>: AsLong {
    fn add<O: OptRational>(rop: &mut Rational, op1: O, op2: Self);
    fn sub<O: OptRational>(rop: &mut Rational, op1: O, op2: Self);
    fn sub_from<O: OptRational>(rop: &mut Rational, op1: Self, op2: O);
    fn mul<O: OptRational>(rop: &mut Rational, op1: O, op2: Self);
    fn div<O: OptRational>(rop: &mut Rational, op1: O, op2: Self);
    fn div_from<O: OptRational>(rop: &mut Rational, op1: Self, op2: O);
}

macro_rules! forward {
    (fn $fn:ident() -> $deleg_long:path, $deleg:path) => {
        #[inline]
        fn $fn<O: OptRational>(rop: &mut Rational, op1: O, op2: Self) {
            if let Some(op2) = op2.checked_as() {
                $deleg_long(rop, op1, op2);
            } else {
                let mut small: MiniInteger = op2.into();
                $deleg(rop, op1, &*small.borrow_excl());
            }
        }
    };
}
macro_rules! reverse {
    (fn $fn:ident() -> $deleg_long:path, $deleg:path) => {
        #[inline]
        fn $fn<O: OptRational>(rop: &mut Rational, op1: Self, op2: O) {
            if let Some(op1) = op1.checked_as() {
                $deleg_long(rop, op1, op2);
            } else {
                let mut small: MiniInteger = op1.into();
                $deleg(rop, &*small.borrow_excl(), op2);
            }
        }
    };
}

impl<T> PrimOps<c_long> for T
where
    T: AsLong<Long = c_long> + CheckedCast<c_long> + Into<MiniInteger>,
{
    forward! { fn add() -> xmpq::add_si, xmpq::add_z }
    forward! { fn sub() -> xmpq::sub_si, xmpq::sub_z }
    reverse! { fn sub_from() -> xmpq::si_sub, xmpq::z_sub }
    forward! { fn mul() -> xmpq::mul_si, xmpq::mul_z }
    forward! { fn div() -> xmpq::div_si, xmpq::div_z }
    reverse! { fn div_from() -> xmpq::si_div, xmpq::z_div }
}

impl<T> PrimOps<c_ulong> for T
where
    T: AsLong<Long = c_ulong> + CheckedCast<c_ulong> + Into<MiniInteger>,
{
    forward! { fn add() -> xmpq::add_ui, xmpq::add_z }
    forward! { fn sub() -> xmpq::sub_ui, xmpq::sub_z }
    reverse! { fn sub_from() -> xmpq::ui_sub, xmpq::z_sub }
    forward! { fn mul() -> xmpq::mul_ui, xmpq::mul_z }
    forward! { fn div() -> xmpq::div_ui, xmpq::div_z }
    reverse! { fn div_from() -> xmpq::ui_div, xmpq::z_div }
}

impl<T> Sum<T> for Rational
where
    Rational: AddAssign<T>,
{
    fn sum<I>(iter: I) -> Rational
    where
        I: Iterator<Item = T>,
    {
        let mut ret = Rational::new();
        for i in iter {
            ret.add_assign(i);
        }
        ret
    }
}

impl<T> Product<T> for Rational
where
    Rational: MulAssign<T>,
{
    fn product<I>(iter: I) -> Rational
    where
        I: Iterator<Item = T>,
    {
        let mut ret = Rational::from(1);
        for i in iter {
            ret.mul_assign(i);
        }
        ret
    }
}

#[inline]
fn rhs_has_more_alloc(lhs: &Rational, rhs: &Rational) -> bool {
    // This can overflow:
    //     lhs.num.alloc + lhs.den.alloc < rhs.num.alloc + rhs.den.alloc
    // Since all alloc are non-negative signed integers (c_int), this
    // cannot overflow:
    //     lhs.num.alloc - rhs.den.alloc < rhs.num.alloc - lhs.den.alloc
    lhs.numer().inner().alloc - rhs.denom().inner().alloc
        < rhs.numer().inner().alloc - lhs.denom().inner().alloc
}

#[cfg(test)]
mod tests {
    use crate::ops::Pow;
    use crate::ops::{AddFrom, SubFrom};
    use crate::rational::MiniRational;
    use crate::{Assign, Complete, Integer, Rational};

    macro_rules! test_ref_op {
        ($first:expr, $second:expr) => {
            assert_eq!(
                Rational::from($first),
                $second,
                "({}) != ({})",
                stringify!($first),
                stringify!($second)
            );
        };
    }

    #[test]
    fn check_ref_op() {
        let lhs = Rational::from((-13, 27));
        let rhs = Rational::from((15, 101));
        let pu = 30_u32;
        let pi = -15_i32;
        test_ref_op!(-&lhs, -lhs.clone());
        test_ref_op!(&lhs + &rhs, lhs.clone() + &rhs);
        test_ref_op!(&lhs - &rhs, lhs.clone() - &rhs);
        test_ref_op!(&lhs * &rhs, lhs.clone() * &rhs);
        test_ref_op!(&lhs / &rhs, lhs.clone() / &rhs);

        test_ref_op!(&lhs << pu, lhs.clone() << pu);
        test_ref_op!(&lhs >> pu, lhs.clone() >> pu);
        test_ref_op!((&lhs).pow(pu), lhs.clone().pow(pu));

        test_ref_op!(&lhs << pi, lhs.clone() << pi);
        test_ref_op!(&lhs >> pi, lhs.clone() >> pi);
        test_ref_op!((&lhs).pow(pi), lhs.clone().pow(pi));
    }

    macro_rules! test_numer_denom {
        ($first:expr, $second:expr) => {{
            let a = $first;
            let b = $second;
            assert_eq!(
                a.numer(),
                b.numer(),
                "({}).numer() != ({}).numer()",
                stringify!($first),
                stringify!($second)
            );
            assert_eq!(
                a.denom(),
                b.denom(),
                "({}).denom() != ({}).denom()",
                stringify!($first),
                stringify!($second)
            );
        }};
    }

    #[test]
    fn check_arith_int() {
        let r = &Rational::from((-15, 128));
        let zero = &Integer::new();
        let minus_100 = &Integer::from(-100);

        test_numer_denom!(r.clone() + zero, r + Rational::from(zero));
        test_numer_denom!(r.clone() - zero, r - Rational::from(zero));
        test_numer_denom!(r.clone() * zero, r * Rational::from(zero));
        test_numer_denom!(r.clone() + minus_100, r + Rational::from(minus_100));
        test_numer_denom!(r.clone() - minus_100, r - Rational::from(minus_100));
        test_numer_denom!(r.clone() * minus_100, r * Rational::from(minus_100));
        test_numer_denom!(r.clone() / minus_100, r / Rational::from(minus_100));

        test_numer_denom!(zero + r.clone(), Rational::from(zero) + r);
        test_numer_denom!(zero - r.clone(), Rational::from(zero) - r);
        test_numer_denom!(zero * r.clone(), Rational::from(zero) * r);
        test_numer_denom!(zero / r.clone(), Rational::from(zero) / r);
        test_numer_denom!(minus_100 + r.clone(), Rational::from(minus_100) + r);
        test_numer_denom!(minus_100 - r.clone(), Rational::from(minus_100) - r);
        test_numer_denom!(minus_100 * r.clone(), Rational::from(minus_100) * r);
        test_numer_denom!(minus_100 / r.clone(), Rational::from(minus_100) / r);
    }

    #[test]
    fn check_neg_pow() {
        let a = Rational::from((-12, 7));
        let pow_pos = a.clone().pow(3i32);
        assert_eq!(
            pow_pos,
            MiniRational::from(((-12i32).pow(3), 7i32.pow(3u32)))
        );
        let pow_neg = a.pow(-3i32);
        assert_eq!(pow_neg, MiniRational::from(((-7i32).pow(3), 12i32.pow(3))));
    }

    macro_rules! check_u_s {
        ($list:expr, $against:expr) => {
            for &op in $list {
                let rat_op = Rational::from(op);
                for b in &$against {
                    test_numer_denom!(b.clone() + op, b.clone() + &rat_op);
                    test_numer_denom!(b.clone() - op, b.clone() - &rat_op);
                    test_numer_denom!(b.clone() * op, b.clone() * &rat_op);
                    if op != 0 {
                        test_numer_denom!(b.clone() / op, b.clone() / &rat_op);
                    }
                    test_numer_denom!(op + b.clone(), rat_op.clone() + b);
                    test_numer_denom!(op - b.clone(), rat_op.clone() - b);
                    test_numer_denom!(op * b.clone(), rat_op.clone() * b);
                    if !b.is_zero() {
                        test_numer_denom!(op / b.clone(), rat_op.clone() / b);
                    }
                }
            }
        };
    }

    fn num_den<T>(s: &[T]) -> impl Iterator<Item = Rational> + '_
    where
        T: Copy + Eq + From<bool>,
        Rational: From<(T, T)>,
    {
        let zero = T::from(false);
        let one = T::from(true);
        s.iter()
            .map(move |&den| if den == zero { one } else { den })
            .flat_map(move |den| s.iter().map(move |&num| Rational::from((num, den))))
    }

    #[test]
    fn check_arith_u_s() {
        use crate::tests::{I8, I16, I32, I64, I128, ISIZE, U8, U16, U32, U64, U128, USIZE};
        let large = [(1, 3, 100), (-11, 5, 200), (33, 79, -150)];
        let against = (large.iter().map(|&(n, d, s)| Rational::from((n, d)) << s))
            .chain(num_den(I32))
            .chain(num_den(U32))
            .chain(num_den(I64))
            .chain(num_den(U64))
            .chain(num_den(I128))
            .chain(num_den(U128))
            .chain(num_den(ISIZE))
            .chain(num_den(USIZE))
            .collect::<Vec<Rational>>();

        check_u_s!(I8, against);
        check_u_s!(I16, against);
        check_u_s!(I32, against);
        check_u_s!(I64, against);
        check_u_s!(I128, against);
        check_u_s!(ISIZE, against);
        check_u_s!(U8, against);
        check_u_s!(U16, against);
        check_u_s!(U32, against);
        check_u_s!(U64, against);
        check_u_s!(USIZE, against);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn check_mini_ops() {
        let big = Rational::from((10, 3));
        let mini = MiniRational::from((3, 2));
        let mut bm = Rational::new();

        // commutative
        let sum = MiniRational::from((29, 6));
        assert_eq!(big.clone() + mini, sum);
        assert_eq!(big.clone() + &mini, sum);
        assert_eq!((&big + mini).complete(), sum);
        assert_eq!((&big + &mini).complete(), sum);

        bm.assign(&big);
        bm += mini;
        assert_eq!(bm, sum);
        bm.assign(&big);
        bm += &mini;
        assert_eq!(bm, sum);

        assert_eq!(mini + big.clone(), sum);
        assert_eq!(&mini + big.clone(), sum);
        assert_eq!((mini + &big).complete(), sum);
        assert_eq!((&mini + &big).complete(), sum);

        bm.assign(&big);
        bm.add_from(mini);
        assert_eq!(bm, sum);
        bm.assign(&big);
        bm.add_from(&mini);
        assert_eq!(bm, sum);

        // non-commutative
        let diff1 = MiniRational::from((11, 6));
        assert_eq!(big.clone() - mini, diff1);
        assert_eq!(big.clone() - &mini, diff1);
        assert_eq!((&big - mini).complete(), diff1);
        assert_eq!((&big - &mini).complete(), diff1);

        bm.assign(&big);
        bm -= mini;
        assert_eq!(bm, diff1);
        bm.assign(&big);
        bm -= &mini;
        assert_eq!(bm, diff1);

        let diff2 = MiniRational::from((-11, 6));
        assert_eq!(mini - big.clone(), diff2);
        assert_eq!(&mini - big.clone(), diff2);
        assert_eq!((mini - &big).complete(), diff2);
        assert_eq!((&mini - &big).complete(), diff2);

        bm.assign(&big);
        bm.sub_from(mini);
        assert_eq!(bm, diff2);
        bm.assign(&big);
        bm.sub_from(&mini);
        assert_eq!(bm, diff2);
    }

    #[test]
    fn check_shift_u_s() {
        let pos = &(Rational::from((13, 7)) >> 100u32);
        let neg = &(Rational::from((19, 101)) << 200u32);

        assert_eq!(pos.clone() << 10u32, pos.clone() << 10i32);
        assert_eq!(pos.clone() << 10u32, pos.clone() >> -10i32);
        assert_eq!(pos.clone() >> 10u32, pos.clone() >> 10i32);
        assert_eq!(pos.clone() >> 10u32, pos.clone() << -10i32);

        assert_eq!(neg.clone() << 10u32, neg.clone() << 10i32);
        assert_eq!(neg.clone() << 10u32, neg.clone() >> -10i32);
        assert_eq!(neg.clone() >> 10u32, neg.clone() >> 10i32);
        assert_eq!(neg.clone() >> 10u32, neg.clone() << -10i32);

        assert_eq!(pos.clone() << 10u32, pos.clone() << 10usize);
        assert_eq!(pos.clone() << 10u32, pos.clone() << 10isize);
        assert_eq!(pos.clone() << 10u32, pos.clone() >> -10isize);
        assert_eq!(pos.clone() >> 10u32, pos.clone() >> 10usize);
        assert_eq!(pos.clone() >> 10u32, pos.clone() >> 10isize);
        assert_eq!(pos.clone() >> 10u32, pos.clone() << -10isize);

        assert_eq!(neg.clone() << 10u32, neg.clone() << 10usize);
        assert_eq!(neg.clone() << 10u32, neg.clone() << 10isize);
        assert_eq!(neg.clone() << 10u32, neg.clone() >> -10isize);
        assert_eq!(neg.clone() >> 10u32, neg.clone() >> 10usize);
        assert_eq!(neg.clone() >> 10u32, neg.clone() >> 10isize);
        assert_eq!(neg.clone() >> 10u32, neg.clone() << -10isize);
    }
}
