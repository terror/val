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

#[cfg(feature = "integer")]
use crate::Integer;
#[cfg(feature = "rational")]
use crate::Rational;
use crate::complex::MiniComplex;
use crate::ext::xmpc;
use crate::ext::xmpc::{NEAREST2, OptComplex, Ordering2, Round2};
use crate::float::MiniFloat;
use crate::ops::{
    AddAssignRound, AddFrom, AddFromRound, AssignRound, CompleteRound, DivAssignRound, DivFrom,
    DivFromRound, MulAssignRound, MulFrom, MulFromRound, NegAssign, Pow, PowAssign, PowAssignRound,
    PowFrom, PowFromRound, SubAssignRound, SubFrom, SubFromRound,
};
use crate::{Complex, Float};
use az::{CheckedAs, CheckedCast};
use core::ffi::{c_long, c_ulong};
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Shl, ShlAssign, Shr, ShrAssign, Sub,
    SubAssign,
};

impl Neg for Complex {
    type Output = Complex;
    #[inline]
    fn neg(mut self) -> Complex {
        self.neg_assign();
        self
    }
}

impl NegAssign for Complex {
    #[inline]
    fn neg_assign(&mut self) {
        xmpc::neg(self, (), NEAREST2);
    }
}

impl<'a> Neg for &'a Complex {
    type Output = NegIncomplete<'a>;
    #[inline]
    fn neg(self) -> NegIncomplete<'a> {
        NegIncomplete { val: self }
    }
}

#[derive(Debug)]
pub struct NegIncomplete<'a> {
    val: &'a Complex,
}

impl AssignRound<NegIncomplete<'_>> for Complex {
    type Round = Round2;
    type Ordering = Ordering2;
    #[inline]
    fn assign_round(&mut self, src: NegIncomplete<'_>, round: Round2) -> Ordering2 {
        xmpc::neg(self, src.val, round)
    }
}

impl CompleteRound for NegIncomplete<'_> {
    type Completed = Complex;
    type Prec = (u32, u32);
    type Round = Round2;
    type Ordering = Ordering2;
    #[inline]
    fn complete_round(self, prec: (u32, u32), round: Round2) -> (Complex, Ordering2) {
        Complex::with_val_round(prec, self, round)
    }
}

arith_binary_self_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::add;
    Add { add }
    AddAssign { add_assign }
    AddAssignRound { add_assign_round }
    AddFrom { add_from }
    AddFromRound { add_from_round }
    AddIncomplete
}
arith_binary_self_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::sub;
    Sub { sub }
    SubAssign { sub_assign }
    SubAssignRound { sub_assign_round }
    SubFrom { sub_from }
    SubFromRound { sub_from_round }
    SubIncomplete
}
arith_binary_self_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::mul;
    Mul { mul }
    MulAssign { mul_assign }
    MulAssignRound { mul_assign_round }
    MulFrom { mul_from }
    MulFromRound { mul_from_round }
    MulIncomplete
}
arith_binary_self_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::div;
    Div { div }
    DivAssign { div_assign }
    DivAssignRound { div_assign_round }
    DivFrom { div_from }
    DivFromRound { div_from_round }
    DivIncomplete
}
arith_binary_self_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::pow;
    Pow { pow }
    PowAssign { pow_assign }
    PowAssignRound { pow_assign_round }
    PowFrom { pow_from }
    PowFromRound { pow_from_round }
    PowIncomplete
}

arith_mini_commut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    Add { add }
    AddAssign { add_assign }
    AddAssignRound { add_assign_round }
    AddFrom { add_from }
    AddFromRound { add_from_round }
    MiniComplex;
    AddMiniIncomplete, AddOwnedMiniIncomplete
}
arith_mini_noncommut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    Sub { sub }
    SubAssign { sub_assign }
    SubAssignRound { sub_assign_round }
    SubFrom { sub_from }
    SubFromRound { sub_from_round }
    MiniComplex;
    SubMiniIncomplete, SubOwnedMiniIncomplete;
    SubFromMiniIncomplete, SubFromOwnedMiniIncomplete
}
arith_mini_commut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    Mul { mul }
    MulAssign { mul_assign }
    MulAssignRound { mul_assign_round }
    MulFrom { mul_from }
    MulFromRound { mul_from_round }
    MiniComplex;
    MulMiniIncomplete, MulOwnedMiniIncomplete
}
arith_mini_noncommut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    Div { div }
    DivAssign { div_assign }
    DivAssignRound { div_assign_round }
    DivFrom { div_from }
    DivFromRound { div_from_round }
    MiniComplex;
    DivMiniIncomplete, DivOwnedMiniIncomplete;
    DivFromMiniIncomplete, DivFromOwnedMiniIncomplete
}
arith_mini_noncommut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    Pow { pow }
    PowAssign { pow_assign }
    PowAssignRound { pow_assign_round }
    PowFrom { pow_from }
    PowFromRound { pow_from_round }
    MiniComplex;
    PowMiniIncomplete, PowOwnedMiniIncomplete;
    PowFromMiniIncomplete, PowFromOwnedMiniIncomplete
}

arith_commut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::add_fr;
    Add { add }
    AddAssign { add_assign }
    AddAssignRound { add_assign_round }
    AddFrom { add_from }
    AddFromRound { add_from_round }
    Float;
    AddFloatIncomplete, AddOwnedFloatIncomplete
}
arith_noncommut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::sub_fr, xmpc::fr_sub;
    Sub { sub }
    SubAssign { sub_assign }
    SubAssignRound { sub_assign_round }
    SubFrom { sub_from }
    SubFromRound { sub_from_round }
    Float;
    SubFloatIncomplete, SubOwnedFloatIncomplete;
    SubFromFloatIncomplete, SubFromOwnedFloatIncomplete
}
arith_commut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::mul_fr;
    Mul { mul }
    MulAssign { mul_assign }
    MulAssignRound { mul_assign_round }
    MulFrom { mul_from }
    MulFromRound { mul_from_round }
    Float;
    MulFloatIncomplete, MulOwnedFloatIncomplete
}
arith_noncommut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::div_fr, xmpc::fr_div;
    Div { div }
    DivAssign { div_assign }
    DivAssignRound { div_assign_round }
    DivFrom { div_from }
    DivFromRound { div_from_round }
    Float;
    DivFloatIncomplete, DivOwnedFloatIncomplete;
    DivFromFloatIncomplete, DivFromOwnedFloatIncomplete
}
arith_forward_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::pow_fr;
    Pow { pow }
    PowAssign { pow_assign }
    PowAssignRound { pow_assign_round }
    Float;
    PowFloatIncomplete, PowOwnedFloatIncomplete
}
#[cfg(feature = "integer")]
arith_forward_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::pow_z;
    Pow { pow }
    PowAssign { pow_assign }
    PowAssignRound { pow_assign_round }
    Integer;
    PowIntegerIncomplete, PowOwnedIntegerIncomplete
}

arith_prim_commut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    PrimOps::add;
    Add { add }
    AddAssign { add_assign }
    AddAssignRound { add_assign_round }
    AddFrom { add_from }
    AddFromRound { add_from_round }
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
    f32, AddF32Incomplete;
    f64, AddF64Incomplete;
}
#[cfg(feature = "nightly-float")]
arith_prim_commut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    PrimOps::add;
    Add { add }
    AddAssign { add_assign }
    AddAssignRound { add_assign_round }
    AddFrom { add_from }
    AddFromRound { add_from_round }
    f16, AddF16Incomplete;
    f128, AddF128Incomplete;
}
arith_prim_noncommut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    PrimOps::sub, PrimOps::sub_from;
    Sub { sub }
    SubAssign { sub_assign }
    SubAssignRound { sub_assign_round }
    SubFrom { sub_from }
    SubFromRound { sub_from_round }
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
    f32, SubF32Incomplete, SubFromF32Incomplete;
    f64, SubF64Incomplete, SubFromF64Incomplete;
}
#[cfg(feature = "nightly-float")]
arith_prim_noncommut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    PrimOps::sub, PrimOps::sub_from;
    Sub { sub }
    SubAssign { sub_assign }
    SubAssignRound { sub_assign_round }
    SubFrom { sub_from }
    SubFromRound { sub_from_round }
    f16, SubF16Incomplete, SubFromF16Incomplete;
    f128, SubF128Incomplete, SubFromF128Incomplete;
}
arith_prim_commut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    PrimOps::mul;
    Mul { mul }
    MulAssign { mul_assign }
    MulAssignRound { mul_assign_round }
    MulFrom { mul_from }
    MulFromRound { mul_from_round }
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
    f32, MulF32Incomplete;
    f64, MulF64Incomplete;
}
#[cfg(feature = "nightly-float")]
arith_prim_commut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    PrimOps::mul;
    Mul { mul }
    MulAssign { mul_assign }
    MulAssignRound { mul_assign_round }
    MulFrom { mul_from }
    MulFromRound { mul_from_round }
    f16, MulF16Incomplete;
    f128, MulF128Incomplete;
}
arith_prim_noncommut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    PrimOps::div, PrimOps::div_from;
    Div { div }
    DivAssign { div_assign }
    DivAssignRound { div_assign_round }
    DivFrom { div_from }
    DivFromRound { div_from_round }
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
    f32, DivF32Incomplete, DivFromF32Incomplete;
    f64, DivF64Incomplete, DivFromF64Incomplete;
}
#[cfg(feature = "nightly-float")]
arith_prim_noncommut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    PrimOps::div, PrimOps::div_from;
    Div { div }
    DivAssign { div_assign }
    DivAssignRound { div_assign_round }
    DivFrom { div_from }
    DivFromRound { div_from_round }
    f16, DivF16Incomplete, DivFromF16Incomplete;
    f128, DivF128Incomplete, DivFromF128Incomplete;
}
arith_prim_noncommut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    PrimOps::pow, PrimOps::pow_from;
    Pow { pow }
    PowAssign { pow_assign }
    PowAssignRound { pow_assign_round }
    PowFrom { pow_from }
    PowFromRound { pow_from_round }
    i8, PowI8Incomplete, PowFromI8Incomplete;
    i16, PowI16Incomplete, PowFromI16Incomplete;
    i32, PowI32Incomplete, PowFromI32Incomplete;
    i64, PowI64Incomplete, PowFromI64Incomplete;
    i128, PowI128Incomplete, PowFromI128Incomplete;
    isize, PowIsizeIncomplete, PowFromIsizeIncomplete;
    u8, PowU8Incomplete, PowFromU8Incomplete;
    u16, PowU16Incomplete, PowFromU16Incomplete;
    u32, PowU32Incomplete, PowFromU32Incomplete;
    u64, PowU64Incomplete, PowFromU64Incomplete;
    u128, PowU128Incomplete, PowFromU128Incomplete;
    usize, PowUsizeIncomplete, PowFromUsizeIncomplete;
    f32, PowF32Incomplete, PowFromF32Incomplete;
    f64, PowF64Incomplete, PowFromF64Incomplete;
}
#[cfg(feature = "nightly-float")]
arith_prim_noncommut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    PrimOps::pow, PrimOps::pow_from;
    Pow { pow }
    PowAssign { pow_assign }
    PowAssignRound { pow_assign_round }
    PowFrom { pow_from }
    PowFromRound { pow_from_round }
    f16, PowF16Incomplete, PowFromF16Incomplete;
    f128, PowF128Incomplete, PowFromF128Incomplete;
}

#[cfg(feature = "integer")]
arith_commut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::add_z;
    Add { add }
    AddAssign { add_assign }
    AddAssignRound { add_assign_round }
    AddFrom { add_from }
    AddFromRound { add_from_round }
    Integer;
    AddIntegerIncomplete, AddOwnedIntegerIncomplete
}
#[cfg(feature = "integer")]
arith_noncommut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::sub_z, xmpc::z_sub;
    Sub { sub }
    SubAssign { sub_assign }
    SubAssignRound { sub_assign_round }
    SubFrom { sub_from }
    SubFromRound { sub_from_round }
    Integer;
    SubIntegerIncomplete, SubOwnedIntegerIncomplete;
    SubFromIntegerIncomplete, SubFromOwnedIntegerIncomplete
}
#[cfg(feature = "integer")]
arith_commut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::mul_z;
    Mul { mul }
    MulAssign { mul_assign }
    MulAssignRound { mul_assign_round }
    MulFrom { mul_from }
    MulFromRound { mul_from_round }
    Integer;
    MulIntegerIncomplete, MulOwnedIntegerIncomplete
}
#[cfg(feature = "integer")]
arith_forward_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::div_z;
    Div { div }
    DivAssign { div_assign }
    DivAssignRound { div_assign_round }
    Integer;
    DivIntegerIncomplete, DivOwnedIntegerIncomplete
}
#[cfg(feature = "rational")]
arith_commut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::add_q;
    Add { add }
    AddAssign { add_assign }
    AddAssignRound { add_assign_round }
    AddFrom { add_from }
    AddFromRound { add_from_round }
    Rational;
    AddRationalIncomplete, AddOwnedRationalIncomplete
}
#[cfg(feature = "rational")]
arith_noncommut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::sub_q, xmpc::q_sub;
    Sub { sub }
    SubAssign { sub_assign }
    SubAssignRound { sub_assign_round }
    SubFrom { sub_from }
    SubFromRound { sub_from_round }
    Rational;
    SubRationalIncomplete, SubOwnedRationalIncomplete;
    SubFromRationalIncomplete, SubFromOwnedRationalIncomplete
}
#[cfg(feature = "rational")]
arith_commut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::mul_q;
    Mul { mul }
    MulAssign { mul_assign }
    MulAssignRound { mul_assign_round }
    MulFrom { mul_from }
    MulFromRound { mul_from_round }
    Rational;
    MulRationalIncomplete, MulOwnedRationalIncomplete
}
#[cfg(feature = "rational")]
arith_forward_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::div_q;
    Div { div }
    DivAssign { div_assign }
    DivAssignRound { div_assign_round }
    Rational;
    DivRationalIncomplete, DivOwnedRationalIncomplete
}

arith_prim_exact_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::shl_u32;
    Shl { shl }
    ShlAssign { shl_assign }
    u32, ShlU32Incomplete;
}
arith_prim_exact_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::shr_u32;
    Shr { shr }
    ShrAssign { shr_assign }
    u32, ShrU32Incomplete;
}
arith_prim_exact_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::shl_i32;
    Shl { shl }
    ShlAssign { shl_assign }
    i32, ShlI32Incomplete;
}
arith_prim_exact_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::shr_i32;
    Shr { shr }
    ShrAssign { shr_assign }
    i32, ShrI32Incomplete;
}
arith_prim_exact_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::shl_usize;
    Shl { shl }
    ShlAssign { shl_assign }
    usize, ShlUsizeIncomplete;
}
arith_prim_exact_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::shr_usize;
    Shr { shr }
    ShrAssign { shr_assign }
    usize, ShrUsizeIncomplete;
}
arith_prim_exact_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::shl_isize;
    Shl { shl }
    ShlAssign { shl_assign }
    isize, ShlIsizeIncomplete;
}
arith_prim_exact_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    xmpc::shr_isize;
    Shr { shr }
    ShrAssign { shr_assign }
    isize, ShrIsizeIncomplete;
}

mul_op_commut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    add_mul;
    Add { add }
    AddAssign { add_assign }
    AddAssignRound { add_assign_round }
    AddFrom { add_from }
    AddFromRound { add_from_round }
    MulIncomplete;
    AddMulIncomplete
}
mul_op_noncommut_round! {
    Complex, (u32, u32), Round2, NEAREST2, Ordering2;
    sub_mul, mul_sub;
    Sub { sub }
    SubAssign { sub_assign }
    SubAssignRound { sub_assign_round }
    SubFrom { sub_from }
    SubFromRound { sub_from_round }
    MulIncomplete;
    SubMulIncomplete, SubMulFromIncomplete
}

trait PrimOps<Long>: AsLong {
    fn add<O: OptComplex>(rop: &mut Complex, op1: O, op2: Self, rnd: Round2) -> Ordering2;
    fn sub<O: OptComplex>(rop: &mut Complex, op1: O, op2: Self, rnd: Round2) -> Ordering2;
    fn sub_from<O: OptComplex>(rop: &mut Complex, op1: Self, op2: O, rnd: Round2) -> Ordering2;
    fn mul<O: OptComplex>(rop: &mut Complex, op1: O, op2: Self, rnd: Round2) -> Ordering2;
    fn div<O: OptComplex>(rop: &mut Complex, op1: O, op2: Self, rnd: Round2) -> Ordering2;
    fn div_from<O: OptComplex>(rop: &mut Complex, op1: Self, op2: O, rnd: Round2) -> Ordering2;
    fn pow<O: OptComplex>(rop: &mut Complex, op1: O, op2: Self, rnd: Round2) -> Ordering2;
    fn pow_from<O: OptComplex>(rop: &mut Complex, op1: Self, op2: O, rnd: Round2) -> Ordering2;
}

trait AsLong: Copy {
    type Long;
}

macro_rules! as_long {
    ($Long:ty: $($Prim:ty)*) => { $(
        impl AsLong for $Prim {
            type Long = $Long;
        }
    )* }
}

as_long! { c_long: i8 i16 i32 i64 i128 isize }
as_long! { c_ulong: u8 u16 u32 u64 u128 usize }
as_long! { f64: f32 f64 }
#[cfg(feature = "nightly-float")]
as_long! { f64: f16 }
#[cfg(feature = "nightly-float")]
as_long! { f128: f128 }

macro_rules! forward {
    (fn $fn:ident() -> $deleg_long:path, $deleg:path) => {
        #[inline]
        fn $fn<O: OptComplex>(rop: &mut Complex, op1: O, op2: Self, rnd: Round2) -> Ordering2 {
            if let Some(op2) = op2.checked_as() {
                $deleg_long(rop, op1, op2, rnd)
            } else {
                let mut small: MiniFloat = op2.into();
                $deleg(rop, op1, small.borrow_excl(), rnd)
            }
        }
    };
    (fn $fn:ident() -> $deleg:path) => {
        #[inline]
        fn $fn<O: OptComplex>(rop: &mut Complex, op1: O, op2: Self, rnd: Round2) -> Ordering2 {
            let mut small: MiniFloat = op2.into();
            $deleg(rop, op1, small.borrow_excl(), rnd)
        }
    };
    (f64: fn $fn:ident() -> $deleg:path) => {
        #[inline]
        fn $fn<O: OptComplex>(rop: &mut Complex, op1: O, op2: Self, rnd: Round2) -> Ordering2 {
            let f = f64::from(op2);
            $deleg(rop, op1, f, rnd)
        }
    };
}
macro_rules! reverse {
    (fn $fn:ident() -> $deleg_long:path, $deleg:path) => {
        #[inline]
        fn $fn<O: OptComplex>(rop: &mut Complex, op1: Self, op2: O, rnd: Round2) -> Ordering2 {
            if let Some(op1) = op1.checked_as() {
                $deleg_long(rop, op1, op2, rnd)
            } else {
                let mut small: MiniFloat = op1.into();
                $deleg(rop, small.borrow_excl(), op2, rnd)
            }
        }
    };
    (fn $fn:ident() -> $deleg:path) => {
        #[inline]
        fn $fn<O: OptComplex>(rop: &mut Complex, op1: Self, op2: O, rnd: Round2) -> Ordering2 {
            let mut small: MiniFloat = op1.into();
            $deleg(rop, small.borrow_excl(), op2, rnd)
        }
    };
    (f64: fn $fn:ident() -> $deleg:path) => {
        #[inline]
        fn $fn<O: OptComplex>(rop: &mut Complex, op1: Self, op2: O, rnd: Round2) -> Ordering2 {
            let f = f64::from(op1);
            $deleg(rop, f, op2, rnd)
        }
    };
}

impl<T> PrimOps<c_long> for T
where
    T: AsLong<Long = c_long> + CheckedCast<c_long> + Into<MiniFloat> + Into<MiniComplex>,
{
    forward! { fn add() -> xmpc::add_si, xmpc::add_fr }
    forward! { fn sub() -> xmpc::sub_si, xmpc::sub_fr }
    reverse! { fn sub_from() -> xmpc::si_sub, xmpc::fr_sub }
    forward! { fn mul() -> xmpc::mul_si, xmpc::mul_fr }
    forward! { fn div() -> xmpc::div_si, xmpc::div_fr }
    reverse! { fn div_from() -> xmpc::si_div, xmpc::fr_div }

    #[inline]
    fn pow<O: OptComplex>(rop: &mut Complex, op1: O, op2: Self, rnd: Round2) -> Ordering2 {
        let mut small: MiniFloat = op2.into();
        xmpc::pow_fr(rop, op1, small.borrow_excl(), rnd)
    }

    #[inline]
    fn pow_from<O: OptComplex>(rop: &mut Complex, op1: Self, op2: O, rnd: Round2) -> Ordering2 {
        let mut small: MiniComplex = op1.into();
        xmpc::pow(rop, small.borrow_excl(), op2, rnd)
    }
}

impl<T> PrimOps<c_ulong> for T
where
    T: AsLong<Long = c_ulong> + CheckedCast<c_ulong> + Into<MiniFloat> + Into<MiniComplex>,
{
    forward! { fn add() -> xmpc::add_ui, xmpc::add_fr }
    forward! { fn sub() -> xmpc::sub_ui, xmpc::sub_fr }
    reverse! { fn sub_from() -> xmpc::ui_sub, xmpc::fr_sub }
    forward! { fn mul() -> xmpc::mul_ui, xmpc::mul_fr }
    forward! { fn div() -> xmpc::div_ui, xmpc::div_fr }
    reverse! { fn div_from() -> xmpc::ui_div, xmpc::fr_div }

    #[inline]
    fn pow<O: OptComplex>(rop: &mut Complex, op1: O, op2: Self, rnd: Round2) -> Ordering2 {
        let mut small: MiniFloat = op2.into();
        xmpc::pow_fr(rop, op1, small.borrow_excl(), rnd)
    }

    #[inline]
    fn pow_from<O: OptComplex>(rop: &mut Complex, op1: Self, op2: O, rnd: Round2) -> Ordering2 {
        let mut small: MiniComplex = op1.into();
        xmpc::pow(rop, small.borrow_excl(), op2, rnd)
    }
}

impl<T> PrimOps<f64> for T
where
    T: AsLong<Long = f64> + Into<MiniFloat> + Into<MiniComplex>,
    f64: From<T>,
{
    forward! { f64: fn add() -> xmpc::add_d }
    forward! { f64: fn sub() -> xmpc::sub_d }
    reverse! { f64: fn sub_from() -> xmpc::d_sub }
    forward! { f64: fn mul() -> xmpc::mul_d }
    forward! { f64: fn div() -> xmpc::div_d }
    reverse! { f64: fn div_from() -> xmpc::d_div }

    #[inline]
    fn pow<O: OptComplex>(rop: &mut Complex, op1: O, op2: Self, rnd: Round2) -> Ordering2 {
        let mut small: MiniFloat = op2.into();
        xmpc::pow_fr(rop, op1, small.borrow_excl(), rnd)
    }

    #[inline]
    fn pow_from<O: OptComplex>(rop: &mut Complex, op1: Self, op2: O, rnd: Round2) -> Ordering2 {
        let mut small: MiniComplex = op1.into();
        xmpc::pow(rop, small.borrow_excl(), op2, rnd)
    }
}

#[cfg(feature = "nightly-float")]
impl<T> PrimOps<f128> for T
where
    T: AsLong<Long = f128> + Into<MiniFloat> + Into<MiniComplex>,
{
    forward! { fn add() -> xmpc::add_fr }
    forward! { fn sub() -> xmpc::sub_fr }
    reverse! { fn sub_from() -> xmpc::fr_sub }
    forward! { fn mul() -> xmpc::mul_fr }
    forward! { fn div() -> xmpc::div_fr }
    reverse! { fn div_from() -> xmpc::fr_div }

    #[inline]
    fn pow<O: OptComplex>(rop: &mut Complex, op1: O, op2: Self, rnd: Round2) -> Ordering2 {
        let mut small: MiniFloat = op2.into();
        xmpc::pow_fr(rop, op1, small.borrow_excl(), rnd)
    }

    #[inline]
    fn pow_from<O: OptComplex>(rop: &mut Complex, op1: Self, op2: O, rnd: Round2) -> Ordering2 {
        let mut small: MiniComplex = op1.into();
        xmpc::pow(rop, small.borrow_excl(), op2, rnd)
    }
}

#[inline]
fn add_mul<O: OptComplex>(
    rop: &mut Complex,
    add: O,
    mul: MulIncomplete<'_>,
    rnd: Round2,
) -> Ordering2 {
    xmpc::fma(rop, mul.lhs, mul.rhs, add, rnd)
}

#[inline]
fn sub_mul<O: OptComplex>(
    rop: &mut Complex,
    add: O,
    mul: MulIncomplete<'_>,
    rnd: Round2,
) -> Ordering2 {
    xmpc::submul(rop, add, mul.lhs, mul.rhs, rnd)
}

#[inline]
fn mul_sub<O: OptComplex>(
    rop: &mut Complex,
    mul: MulIncomplete<'_>,
    sub: O,
    rnd: Round2,
) -> Ordering2 {
    xmpc::mulsub(rop, mul.lhs, mul.rhs, sub, rnd)
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "integer")]
    use crate::Integer;
    #[cfg(feature = "rational")]
    use crate::Rational;
    use crate::complex::MiniComplex;
    use crate::float;
    use crate::float::arith::tests as float_tests;
    use crate::float::{FreeCache, Special};
    use crate::ops::{AddFrom, NegAssign, Pow, SubFrom};
    use crate::{Assign, Complex, Float};
    #[cfg(feature = "integer")]
    use core::str::FromStr;

    #[test]
    fn check_neg() {
        let mut a = Complex::with_val(53, (Special::Zero, Special::NegZero));
        assert!(a.real().is_sign_positive() && a.imag().is_sign_negative());
        a.neg_assign();
        assert!(a.real().is_sign_negative() && a.imag().is_sign_positive());
        let a = -a;
        assert!(a.real().is_sign_positive() && a.imag().is_sign_negative());
        let a = Complex::with_val(53, -&a);
        assert!(a.real().is_sign_negative() && a.imag().is_sign_positive());

        float::free_cache(FreeCache::All);
    }

    fn same(a: Complex, b: Complex) -> bool {
        let (re_a, im_a) = a.into_real_imag();
        let (re_b, im_b) = b.into_real_imag();
        float_tests::same(re_a, re_b) && float_tests::same(im_a, im_b)
    }

    macro_rules! test_ref_op {
        ($first:expr, $second:expr) => {
            assert_eq!(
                Complex::with_val(53, $first),
                $second,
                "({}) != ({})",
                stringify!($first),
                stringify!($second)
            );
        };
    }

    #[test]
    fn check_ref_op() {
        let lhs = Complex::with_val(53, (12.25, -1.375));
        let rhs = Complex::with_val(53, (-1.375, 13));
        let pu = 30_u32;
        let pi = -15_i32;
        let ps = 31.625_f32;
        let pd = -1.5_f64;
        test_ref_op!(-&lhs, -lhs.clone());
        test_ref_op!(&lhs + &rhs, lhs.clone() + &rhs);
        test_ref_op!(&lhs - &rhs, lhs.clone() - &rhs);
        test_ref_op!(&lhs * &rhs, lhs.clone() * &rhs);
        test_ref_op!(&lhs / &rhs, lhs.clone() / &rhs);
        test_ref_op!((&lhs).pow(&rhs), lhs.clone().pow(&rhs));

        test_ref_op!(&lhs + pu, lhs.clone() + pu);
        test_ref_op!(&lhs - pu, lhs.clone() - pu);
        test_ref_op!(&lhs * pu, lhs.clone() * pu);
        test_ref_op!(&lhs / pu, lhs.clone() / pu);
        test_ref_op!(&lhs << pu, lhs.clone() << pu);
        test_ref_op!(&lhs >> pu, lhs.clone() >> pu);
        test_ref_op!((&lhs).pow(pu), lhs.clone().pow(pu));

        test_ref_op!(pu + &lhs, pu + lhs.clone());
        test_ref_op!(pu - &lhs, pu - lhs.clone());
        test_ref_op!(pu * &lhs, pu * lhs.clone());
        test_ref_op!(pu / &lhs, pu / lhs.clone());

        test_ref_op!(&lhs + pi, lhs.clone() + pi);
        test_ref_op!(&lhs - pi, lhs.clone() - pi);
        test_ref_op!(&lhs * pi, lhs.clone() * pi);
        test_ref_op!(&lhs / pi, lhs.clone() / pi);
        test_ref_op!(&lhs << pi, lhs.clone() << pi);
        test_ref_op!(&lhs >> pi, lhs.clone() >> pi);
        test_ref_op!((&lhs).pow(pi), lhs.clone().pow(pi));

        test_ref_op!(pi + &lhs, pi + lhs.clone());
        test_ref_op!(pi - &lhs, pi - lhs.clone());
        test_ref_op!(pi * &lhs, pi * lhs.clone());
        test_ref_op!(pi / &lhs, pi / lhs.clone());

        test_ref_op!((&lhs).pow(ps), lhs.clone().pow(ps));
        test_ref_op!((&lhs).pow(pd), lhs.clone().pow(pd));

        float::free_cache(FreeCache::All);
    }

    macro_rules! check_others {
        (&$list:expr, $against:expr) => {
            for op in &$list {
                let cop = Complex::with_val(150, op);
                for b in &$against {
                    assert!(same(b.clone() + op, b.clone() + &cop));
                    assert!(same(op + b.clone(), cop.clone() + b));
                    assert!(same(b.clone() - op, b.clone() - &cop));
                    assert!(same(op - b.clone(), cop.clone() - b));
                    if b.real().is_finite() && b.imag().is_finite() {
                        assert!(same(b.clone() * op, b.clone() * &cop));
                        assert!(same(op * b.clone(), cop.clone() * b));
                        if *op != 0 {
                            assert!(same(b.clone() / op, b.clone() / &cop));
                        }
                    }
                }
            }
        };
        ($list:expr, $against:expr, $zero:expr) => {
            for op in $list {
                let cop = Complex::with_val(150, *op);
                for b in &$against {
                    assert!(same(b.clone() + *op, b.clone() + &cop));
                    assert!(same(*op + b.clone(), cop.clone() + b));
                    assert!(same(b.clone() - *op, b.clone() - &cop));
                    assert!(same(*op - b.clone(), cop.clone() - b));
                    if b.real().is_finite() && b.imag().is_finite() {
                        assert!(same(b.clone() * *op, b.clone() * &cop));
                        assert!(same(*op * b.clone(), cop.clone() * b));
                        if *op != $zero {
                            assert!(same(b.clone() / *op, b.clone() / &cop));
                        }
                        if *b != 0i32 {
                            assert!(same(*op / b.clone(), cop.clone() / b));
                        }
                    }
                }
            }
        };
    }

    #[test]
    fn check_arith_others() {
        use crate::tests::{
            F32, F64, I8, I16, I32, I64, I128, ISIZE, U8, U16, U32, U64, U128, USIZE,
        };
        let large = [
            Complex::with_val(20, (Special::Zero, 1.0)),
            Complex::with_val(20, (Special::NegZero, 1.0)),
            Complex::with_val(20, (Special::Infinity, 1.0)),
            Complex::with_val(20, (Special::NegInfinity, 1.0)),
            Complex::with_val(20, (Special::Nan, 1.0)),
            Complex::with_val(20, (1, 1.0)),
            Complex::with_val(20, (-1, 1.0)),
            Complex::with_val(20, (999_999e100, 1.0)),
            Complex::with_val(20, (999_999e-100, 1.0)),
            Complex::with_val(20, (-999_999e100, 1.0)),
            Complex::with_val(20, (-999_999e-100, 1.0)),
        ];
        #[cfg(feature = "integer")]
        let z = [
            Integer::from(0),
            Integer::from(1),
            Integer::from(-1),
            Integer::from_str("-1000000000000").unwrap(),
            Integer::from_str("1000000000000").unwrap(),
        ];
        #[cfg(feature = "rational")]
        let q = [
            Rational::from(0),
            Rational::from(1),
            Rational::from(-1),
            Rational::from_str("-1000000000000/33333333333").unwrap(),
            Rational::from_str("1000000000000/33333333333").unwrap(),
        ];
        let f = [
            Float::with_val(20, 0.0),
            Float::with_val(20, 1.0),
            Float::with_val(20, -1.0),
            Float::with_val(20, 12.5),
            Float::with_val(20, 12.5) << 10000,
            Float::with_val(20, Special::Infinity),
        ];

        let mut against = large
            .iter()
            .cloned()
            .chain(U32.iter().map(|&x| Complex::with_val(20, x)))
            .chain(I32.iter().map(|&x| Complex::with_val(20, x)))
            .chain(U64.iter().map(|&x| Complex::with_val(20, x)))
            .chain(I64.iter().map(|&x| Complex::with_val(20, x)))
            .chain(U128.iter().map(|&x| Complex::with_val(20, x)))
            .chain(I128.iter().map(|&x| Complex::with_val(20, x)))
            .chain(USIZE.iter().map(|&x| Complex::with_val(20, x)))
            .chain(ISIZE.iter().map(|&x| Complex::with_val(20, x)))
            .chain(F32.iter().map(|&x| Complex::with_val(20, x)))
            .chain(F64.iter().map(|&x| Complex::with_val(20, x)))
            .collect::<Vec<Complex>>();
        #[cfg(feature = "integer")]
        against.extend(z.iter().map(|x| Complex::with_val(20, x)));
        #[cfg(feature = "rational")]
        against.extend(q.iter().map(|x| Complex::with_val(20, x)));
        against.extend(f.iter().map(|x| Complex::with_val(20, x)));

        check_others!(I8, against, 0);
        check_others!(I16, against, 0);
        check_others!(I32, against, 0);
        check_others!(I64, against, 0);
        check_others!(I128, against, 0);
        check_others!(ISIZE, against, 0);
        check_others!(U8, against, 0);
        check_others!(U16, against, 0);
        check_others!(U32, against, 0);
        check_others!(U64, against, 0);
        check_others!(U128, against, 0);
        check_others!(USIZE, against, 0);
        check_others!(F32, against, 0.0);
        check_others!(F64, against, 0.0);
        #[cfg(feature = "integer")]
        check_others!(&z, against);
        #[cfg(feature = "rational")]
        check_others!(&q, against);
        check_others!(&f, against);

        float::free_cache(FreeCache::All);
    }

    macro_rules! check_pow {
        ($list:expr, $against:expr) => {
            for op in $list {
                let cop = Complex::with_val(150, *op);
                for b in &$against {
                    assert!(same(b.clone().pow(*op), b.clone().pow(&cop)));
                    assert!(same(op.pow(b.clone()), cop.clone().pow(b)));
                }
            }
        };
    }

    #[test]
    fn check_pow() {
        use crate::tests::{F32, I32};
        let large = [
            Complex::with_val(20, (Special::Zero, 1.0)),
            Complex::with_val(20, (Special::NegZero, 1.0)),
            Complex::with_val(20, (Special::Infinity, 1.0)),
            Complex::with_val(20, (Special::NegInfinity, 1.0)),
            Complex::with_val(20, (Special::Nan, 1.0)),
            Complex::with_val(20, (1, 1.0)),
            Complex::with_val(20, (-1, 1.0)),
            Complex::with_val(20, (999_999e100, 1.0)),
            Complex::with_val(20, (999_999e-100, 1.0)),
            Complex::with_val(20, (-999_999e100, 1.0)),
            Complex::with_val(20, (-999_999e-100, 1.0)),
        ];
        let f = [
            Float::with_val(20, 0.0),
            Float::with_val(20, 1.0),
            Float::with_val(20, -1.0),
            Float::with_val(20, 12.5),
            Float::with_val(20, 12.5) << 10000,
            Float::with_val(20, Special::Infinity),
        ];

        let against = large
            .iter()
            .cloned()
            .chain(I32.iter().map(|&x| Complex::with_val(20, x)))
            .chain(F32.iter().map(|&x| Complex::with_val(20, x)))
            .chain(f.iter().map(|x| Complex::with_val(20, x)))
            .collect::<Vec<Complex>>();

        check_pow!(&[-1001i32, -1, 0, 1, 1001], against);
        check_pow!(&[0u32, 1, 1001], against);
        check_pow!(
            &[
                0.0,
                1.0,
                -1.0,
                1000.5,
                f64::NAN,
                f64::INFINITY,
                f64::NEG_INFINITY
            ],
            against
        );

        float::free_cache(FreeCache::All);
    }

    #[cfg(feature = "integer")]
    #[test]
    fn check_pow_int() {
        use crate::tests::{F32, I32};
        let large = [
            Complex::with_val(20, (Special::Zero, 1.0)),
            Complex::with_val(20, (Special::NegZero, 1.0)),
            Complex::with_val(20, (Special::Infinity, 1.0)),
            Complex::with_val(20, (Special::NegInfinity, 1.0)),
            Complex::with_val(20, (Special::Nan, 1.0)),
            Complex::with_val(20, (1, 1.0)),
            Complex::with_val(20, (-1, 1.0)),
            Complex::with_val(20, (999_999e100, 1.0)),
            Complex::with_val(20, (999_999e-100, 1.0)),
            Complex::with_val(20, (-999_999e100, 1.0)),
            Complex::with_val(20, (-999_999e-100, 1.0)),
        ];
        let f = [
            Float::with_val(20, 0.0),
            Float::with_val(20, 1.0),
            Float::with_val(20, -1.0),
            Float::with_val(20, 12.5),
            Float::with_val(20, 12.5) << 10000,
            Float::with_val(20, Special::Infinity),
        ];
        let i = [
            Integer::from(0),
            Integer::from(1),
            Integer::from(-1),
            Integer::from(12),
            Integer::from(-12),
            Integer::from(12) << 100,
        ];

        let comp = large
            .iter()
            .cloned()
            .chain(I32.iter().map(|&x| Complex::with_val(20, x)))
            .chain(F32.iter().map(|&x| Complex::with_val(20, x)))
            .chain(f.iter().map(|x| Complex::with_val(20, x)))
            .chain(i.iter().map(|x| Complex::with_val(20, x)))
            .collect::<Vec<Complex>>();

        for a in &comp {
            for b in &i {
                let b_as_c = Complex::with_val(150, b);
                assert!(same(a.clone().pow(b), a.clone().pow(b_as_c)));
            }
        }

        float::free_cache(FreeCache::All);
    }

    #[test]
    fn check_shift_u_s() {
        let c = &Complex::with_val(53, (13.75, -1.92e-10));

        assert_eq!(c.clone() << 10u32, c.clone() << 10i32);
        assert_eq!(c.clone() << 10u32, c.clone() >> -10i32);
        assert_eq!(c.clone() >> 10u32, c.clone() >> 10i32);
        assert_eq!(c.clone() >> 10u32, c.clone() << -10i32);

        assert_eq!(c.clone() << 10u32, c.clone() << 10usize);
        assert_eq!(c.clone() << 10u32, c.clone() << 10isize);
        assert_eq!(c.clone() << 10u32, c.clone() >> -10isize);
        assert_eq!(c.clone() >> 10u32, c.clone() >> 10usize);
        assert_eq!(c.clone() >> 10u32, c.clone() >> 10isize);
        assert_eq!(c.clone() >> 10u32, c.clone() << -10isize);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn check_mini_ops() {
        let big = Complex::with_val(53, (10.5, -2.5));
        let mini = MiniComplex::from((3.25f32, 1.5f32));
        let mut bm = Complex::new(53);

        // commutative
        let sum = MiniComplex::from((13.75, -1.0));
        assert_eq!(big.clone() + mini, sum);
        assert_eq!(big.clone() + &mini, sum);
        assert_eq!(Complex::with_val(53, &big + mini), sum);
        assert_eq!(Complex::with_val(53, &big + &mini), sum);

        bm.assign(big.clone());
        bm += mini;
        assert_eq!(bm.clone(), sum);
        bm.assign(big.clone());
        bm += &mini;
        assert_eq!(bm.clone(), sum);

        assert_eq!(mini + big.clone(), sum);
        assert_eq!(&mini + big.clone(), sum);
        assert_eq!(Complex::with_val(53, mini + &big), sum);
        assert_eq!(Complex::with_val(53, &mini + &big), sum);

        bm.assign(big.clone());
        bm.add_from(mini);
        assert_eq!(bm.clone(), sum);
        bm.assign(big.clone());
        bm.add_from(&mini);
        assert_eq!(bm.clone(), sum);

        // non-commutative
        let diff1 = MiniComplex::from((7.25, -4.0));
        assert_eq!(big.clone() - mini, diff1);
        assert_eq!(big.clone() - &mini, diff1);
        assert_eq!(Complex::with_val(53, &big - mini), diff1);
        assert_eq!(Complex::with_val(53, &big - &mini), diff1);

        bm.assign(big.clone());
        bm -= mini;
        assert_eq!(bm.clone(), diff1);
        bm.assign(big.clone());
        bm -= &mini;
        assert_eq!(bm.clone(), diff1);

        let diff2 = MiniComplex::from((-7.25, 4.0));
        assert_eq!(mini - big.clone(), diff2);
        assert_eq!(&mini - big.clone(), diff2);
        assert_eq!(Complex::with_val(53, mini - &big), diff2);
        assert_eq!(Complex::with_val(53, &mini - &big), diff2);

        bm.assign(big.clone());
        bm.sub_from(mini);
        assert_eq!(bm.clone(), diff2);
        bm.assign(big.clone());
        bm.sub_from(&mini);
        assert_eq!(bm.clone(), diff2);
    }
}
