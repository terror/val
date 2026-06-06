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

use crate::{Complex, Float};
#[allow(deprecated)]
use az::UnwrappedCast;
use az::{Cast, CheckedCast, SaturatingCast, StrictCast};
use num_complex::Complex as NumComplex;

impl<T> Cast<NumComplex<T>> for Complex
where
    for<'a> &'a Float: Cast<T>,
{
    #[inline]
    fn cast(self) -> NumComplex<T> {
        (&self).cast()
    }
}

impl<T> Cast<NumComplex<T>> for &'_ Complex
where
    for<'a> &'a Float: Cast<T>,
{
    #[inline]
    fn cast(self) -> NumComplex<T> {
        NumComplex::new(self.real().cast(), self.imag().cast())
    }
}

impl<T> CheckedCast<NumComplex<T>> for Complex
where
    for<'a> &'a Float: CheckedCast<T>,
{
    #[inline]
    fn checked_cast(self) -> Option<NumComplex<T>> {
        (&self).checked_cast()
    }
}

impl<T> CheckedCast<NumComplex<T>> for &'_ Complex
where
    for<'a> &'a Float: CheckedCast<T>,
{
    #[inline]
    fn checked_cast(self) -> Option<NumComplex<T>> {
        if let (Some(r), Some(i)) = (self.real().checked_cast(), self.imag().checked_cast()) {
            Some(NumComplex::new(r, i))
        } else {
            None
        }
    }
}

impl<T> SaturatingCast<NumComplex<T>> for Complex
where
    for<'a> &'a Float: SaturatingCast<T>,
{
    #[inline]
    fn saturating_cast(self) -> NumComplex<T> {
        (&self).saturating_cast()
    }
}

impl<T> SaturatingCast<NumComplex<T>> for &'_ Complex
where
    for<'a> &'a Float: SaturatingCast<T>,
{
    #[inline]
    fn saturating_cast(self) -> NumComplex<T> {
        NumComplex::new(self.real().saturating_cast(), self.imag().saturating_cast())
    }
}

impl<T> StrictCast<NumComplex<T>> for Complex
where
    for<'a> &'a Float: StrictCast<T>,
{
    #[inline]
    fn strict_cast(self) -> NumComplex<T> {
        (&self).strict_cast()
    }
}

impl<T> StrictCast<NumComplex<T>> for &'_ Complex
where
    for<'a> &'a Float: StrictCast<T>,
{
    #[inline]
    fn strict_cast(self) -> NumComplex<T> {
        NumComplex::new(self.real().strict_cast(), self.imag().strict_cast())
    }
}

#[allow(deprecated)]
impl<T> UnwrappedCast<NumComplex<T>> for Complex
where
    for<'a> &'a Float: UnwrappedCast<T>,
{
    #[inline]
    fn unwrapped_cast(self) -> NumComplex<T> {
        (&self).unwrapped_cast()
    }
}

#[allow(deprecated)]
impl<T> UnwrappedCast<NumComplex<T>> for &'_ Complex
where
    for<'a> &'a Float: UnwrappedCast<T>,
{
    #[inline]
    fn unwrapped_cast(self) -> NumComplex<T> {
        NumComplex::new(self.real().unwrapped_cast(), self.imag().unwrapped_cast())
    }
}
