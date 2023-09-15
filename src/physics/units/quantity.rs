//! Units of measurements.
//!
//! The purpose is very similar to that of `uom`, but:
//!
//! - I wanted to play with the `const` generics
//! - `uom` is difficult to use in `const` context
//! - `uom` doesn't play nice with `glam`

use std::fmt::{Debug, Display, Formatter, Write};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};

use schemars::JsonSchema;
use serde::Deserialize;

use crate::physics::units::bare::Bare;

#[derive(Copy, Clone, PartialEq, PartialOrd, Deserialize, JsonSchema)]
#[must_use]
pub struct Quantity<
    const T: isize = 0,
    const L: isize = 0,
    const M: isize = 0,
    const TT: isize = 0,
    const SA: isize = 0,
>(pub f64);

impl<const T: isize, const L: isize, const M: isize, const TT: isize, const SA: isize>
    Quantity<T, L, M, TT, SA>
{
    pub const ONE: Self = Quantity(1.0);
    pub const ZERO: Self = Quantity(0.0);
}

impl<const T: isize, const L: isize, const M: isize, const TT: isize, const SA: isize> Display
    for Quantity<T, L, M, TT, SA>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.0)?;
        Self::write_units(f)?;
        Ok(())
    }
}

impl<const T: isize, const L: isize, const M: isize, const TT: isize, const SA: isize> Debug
    for Quantity<T, L, M, TT, SA>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ", self.0)?;
        Self::write_units(f)?;
        Ok(())
    }
}

impl<const T: isize, const L: isize, const M: isize, const TT: isize, const SA: isize>
    Quantity<T, L, M, TT, SA>
{
    pub fn write_units(f: &mut Formatter<'_>) -> std::fmt::Result {
        Self::write_unit::<T>(f, "s")?;
        Self::write_unit::<L>(f, "m")?;
        Self::write_unit::<M>(f, "kg")?;
        Self::write_unit::<TT>(f, "K")?;
        Self::write_unit::<SA>(f, "sr")?;
        Ok(())
    }

    #[inline]
    fn write_unit<const U: isize>(f: &mut Formatter<'_>, symbol: &str) -> std::fmt::Result {
        if U != 0 {
            write!(f, "{}", symbol)?;
            for char in U.to_string().chars() {
                let char = match char {
                    '0' => '⁰',
                    '1' => '¹',
                    '2' => '²',
                    '3' => '³',
                    '4' => '⁴',
                    '5' => '⁵',
                    '6' => '⁶',
                    '7' => '⁷',
                    '8' => '⁸',
                    '9' => '⁹',
                    '-' => '⁻',
                    _ => char,
                };
                f.write_char(char)?;
            }
        }
        Ok(())
    }
}

impl<const T: isize, const L: isize, const M: isize, const TT: isize, const SA: isize> From<f64>
    for Quantity<T, L, M, TT, SA>
{
    #[inline]
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl<const T: isize, const L: isize, const M: isize, const TT: isize, const SA: isize>
    Quantity<T, L, M, TT, SA>
{
    #[inline]
    pub const fn from_millis(value: f64) -> Self {
        Self(value * 1e-3)
    }

    #[inline]
    pub const fn from_micros(value: f64) -> Self {
        Self(value * 1e-6)
    }

    #[inline]
    pub const fn from_nanos(value: f64) -> Self {
        Self(value * 1e-9)
    }
}

impl<const T: isize, const L: isize, const M: isize, const TT: isize, const SA: isize>
    Quantity<T, L, M, TT, SA>
{
    #[inline]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }
}

impl<const T: isize, const L: isize, const M: isize, const TT: isize, const SA: isize>
    Quantity<T, L, M, TT, SA>
{
    #[inline]
    pub const fn squared(
        self,
    ) -> Quantity<{ T * 2 }, { L * 2 }, { M * 2 }, { TT * 2 }, { SA * 2 }> {
        Quantity(self.0 * self.0)
    }

    #[inline]
    pub const fn cubed(self) -> Quantity<{ T * 3 }, { L * 3 }, { M * 3 }, { TT * 3 }, { SA * 3 }> {
        Quantity(self.0 * self.0 * self.0)
    }

    /// Raise the quantity to the 4-th degree.
    #[inline]
    pub const fn quartic(
        self,
    ) -> Quantity<{ T * 4 }, { L * 4 }, { M * 4 }, { TT * 4 }, { SA * 4 }> {
        Quantity(self.0 * self.0 * self.0 * self.0)
    }

    /// Raise the quantity to the 5-th degree.
    #[inline]
    pub const fn quintic(
        self,
    ) -> Quantity<{ T * 5 }, { L * 5 }, { M * 5 }, { TT * 5 }, { SA * 5 }> {
        Quantity(self.0 * self.0 * self.0 * self.0 * self.0)
    }

    /// Raise the quantity to the 6-th degree.
    #[inline]
    pub const fn sextic(self) -> Quantity<{ T * 6 }, { L * 6 }, { M * 6 }, { TT * 6 }, { SA * 6 }> {
        Quantity(self.0 * self.0 * self.0 * self.0 * self.0 * self.0)
    }
}

impl<const T: isize, const L: isize, const M: isize, const TT: isize, const SA: isize> Add<Self>
    for Quantity<T, L, M, TT, SA>
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<const T: isize, const L: isize, const M: isize, const TT: isize, const SA: isize>
    AddAssign<Self> for Quantity<T, L, M, TT, SA>
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<const T: isize, const L: isize, const M: isize, const TT: isize, const SA: isize> Sum<Self>
    for Quantity<T, L, M, TT, SA>
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = 0.0;
        for item in iter {
            sum += item.0;
        }
        Self(sum)
    }
}

impl<const T: isize, const L: isize, const M: isize, const TT: isize, const SA: isize> Sub<Self>
    for Quantity<T, L, M, TT, SA>
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<const T: isize, const L: isize, const M: isize, const TT: isize, const SA: isize>
    MulAssign<Bare> for Quantity<T, L, M, TT, SA>
{
    #[inline]
    fn mul_assign(&mut self, rhs: Bare) {
        self.0 *= rhs.0;
    }
}

impl<
    const T1: isize,
    const L1: isize,
    const M1: isize,
    const TT1: isize,
    const SA1: isize,
    const T2: isize,
    const L2: isize,
    const M2: isize,
    const TT2: isize,
    const SA2: isize,
> Mul<Quantity<T2, L2, M2, TT2, SA2>> for Quantity<T1, L1, M1, TT1, SA1>
where
    Quantity<{ T1 + T2 }, { L1 + L2 }, { M1 + M2 }, { TT1 + TT2 }, { SA1 + SA2 }>: Sized,
{
    type Output = Quantity<{ T1 + T2 }, { L1 + L2 }, { M1 + M2 }, { TT1 + TT2 }, { SA1 + SA2 }>;

    #[inline]
    fn mul(self, rhs: Quantity<T2, L2, M2, TT2, SA2>) -> Self::Output {
        Quantity(self.0 * rhs.0)
    }
}

impl<const T: isize, const L: isize, const M: isize, const TT: isize, const SA: isize> Mul<f64>
    for Quantity<T, L, M, TT, SA>
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl<
    const T1: isize,
    const L1: isize,
    const M1: isize,
    const TT1: isize,
    const SA1: isize,
    const T2: isize,
    const L2: isize,
    const M2: isize,
    const TT2: isize,
    const SA2: isize,
> Div<Quantity<T2, L2, M2, TT2, SA2>> for Quantity<T1, L1, M1, TT1, SA1>
where
    Quantity<{ T1 - T2 }, { L1 - L2 }, { M1 - M2 }, { TT1 - TT2 }, { SA1 - SA2 }>: Sized,
{
    type Output = Quantity<{ T1 - T2 }, { L1 - L2 }, { M1 - M2 }, { TT1 - TT2 }, { SA1 - SA2 }>;

    #[inline]
    fn div(self, rhs: Quantity<T2, L2, M2, TT2, SA2>) -> Self::Output {
        Quantity(self.0 / rhs.0)
    }
}

impl<const T: isize, const L: isize, const M: isize, const TT: isize, const SA: isize> Div<f64>
    for Quantity<T, L, M, TT, SA>
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::physics::units::*;

    #[test]
    fn format_units_ok() {
        assert_eq!(format!("{}", Velocity::from(1.0)), "1 s⁻¹m¹");
        assert_eq!(format!("{}", Length::from(1.0)), "1 m¹");
        assert_eq!(format!("{}", Temperature::from(1.0)), "1 K¹");
        assert_eq!(format!("{}", SpectralRadiance::from(1.0)), "1 s⁻³m⁻¹kg¹sr⁻¹");
    }
}
