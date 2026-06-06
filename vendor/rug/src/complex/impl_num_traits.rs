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
use num_traits::ops::inv::Inv;
use num_traits::ops::mul_add::{MulAdd, MulAddAssign};

impl Inv for Complex {
    type Output = Self;

    #[inline]
    fn inv(self) -> Self::Output {
        self.recip()
    }
}

impl MulAdd for Complex {
    type Output = Complex;

    #[inline]
    fn mul_add(self, a: Complex, b: Complex) -> Complex {
        self.mul_add(&a, &b)
    }
}

impl MulAddAssign for Complex {
    #[inline]
    fn mul_add_assign(&mut self, a: Complex, b: Complex) {
        self.mul_add_mut(&a, &b);
    }
}

impl MulAdd<&Complex, &Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn mul_add(self, a: &Complex, b: &Complex) -> Complex {
        self.mul_add(a, b)
    }
}

impl MulAddAssign<&Complex, &Complex> for Complex {
    #[inline]
    fn mul_add_assign(&mut self, a: &Complex, b: &Complex) {
        self.mul_add_mut(a, b);
    }
}
