//! Units of measurements.
//!
//! The purpose is very similar to that of `uom`, but:
//!
//! - I wanted to play with the `const` generics
//! - `uom` is difficult to use in `const` context
//! - `uom` doesn't play nice with `glam`

use std::fmt::{Debug, Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};

use schemars::JsonSchema;
use serde::Deserialize;

use crate::physics::units::bare::Bare;

#[derive(Copy, Clone, PartialEq, PartialOrd, Deserialize, JsonSchema)]
pub struct Quantity<
    V,
    const T: isize,
    const L: isize,
    const M: isize,
    const EC: isize,
    const TT: isize,
    const AS: isize,
    const LI: isize,
>(pub V);

impl<
    V: Display,
    const T: isize,
    const L: isize,
    const M: isize,
    const EC: isize,
    const TT: isize,
    const AS: isize,
    const LI: isize,
> Display for Quantity<V, T, L, M, EC, TT, AS, LI>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;
        if T != 0 {
            write!(f, " s^{T}")?;
        }
        if L != 0 {
            write!(f, " m^{L}")?;
        }
        if M != 0 {
            write!(f, " kg^{M}")?;
        }
        if EC != 0 {
            write!(f, " A^{EC}")?;
        }
        if TT != 0 {
            write!(f, " K^{TT}")?;
        }
        if AS != 0 {
            write!(f, " mol^{AS}")?;
        }
        if LI != 0 {
            write!(f, " cd^{LI}")?;
        }
        Ok(())
    }
}

impl<
    V: Debug,
    const T: isize,
    const L: isize,
    const M: isize,
    const EC: isize,
    const TT: isize,
    const AS: isize,
    const LI: isize,
> Debug for Quantity<V, T, L, M, EC, TT, AS, LI>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)?;
        if T != 0 {
            write!(f, " s^{T}")?;
        }
        if L != 0 {
            write!(f, " m^{L}")?;
        }
        if M != 0 {
            write!(f, " kg^{M}")?;
        }
        if EC != 0 {
            write!(f, " A^{EC}")?;
        }
        if TT != 0 {
            write!(f, " K^{TT}")?;
        }
        if AS != 0 {
            write!(f, " mol^{AS}")?;
        }
        if LI != 0 {
            write!(f, " cd^{LI}")?;
        }
        Ok(())
    }
}

impl<
    V,
    const T: isize,
    const L: isize,
    const M: isize,
    const EC: isize,
    const TT: isize,
    const AS: isize,
    const LI: isize,
> const From<V> for Quantity<V, T, L, M, EC, TT, AS, LI>
{
    fn from(value: V) -> Self {
        Self(value)
    }
}

impl<
    V: Mul<f64, Output = V>,
    const T: isize,
    const L: isize,
    const M: isize,
    const EC: isize,
    const TT: isize,
    const AS: isize,
    const LI: isize,
> Quantity<V, T, L, M, EC, TT, AS, LI>
{
    pub fn from_millis(value: V) -> Self {
        Self(value * 1e-3)
    }

    pub fn from_micros(value: V) -> Self {
        Self(value * 1e-6)
    }

    pub fn from_nanos(value: V) -> Self {
        Self(value * 1e-9)
    }
}

impl<
    const T: isize,
    const L: isize,
    const M: isize,
    const EC: isize,
    const TT: isize,
    const AS: isize,
    const LI: isize,
> Quantity<f64, T, L, M, EC, TT, AS, LI>
{
    pub fn powi<const P: isize>(
        self,
    ) -> Quantity<
        f64,
        { T * P },
        { L * P },
        { M * P },
        { EC * P },
        { TT * P },
        { AS * P },
        { LI * P },
    > {
        Quantity::<
            f64,
            { T * P },
            { L * P },
            { M * P },
            { EC * P },
            { TT * P },
            { AS * P },
            { LI * P },
        >(self.0.powi(P as i32))
    }

    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }
}

impl<
    V,
    const T: isize,
    const L: isize,
    const M: isize,
    const EC: isize,
    const TT: isize,
    const AS: isize,
    const LI: isize,
> Add<Self> for Quantity<V, T, L, M, EC, TT, AS, LI>
where
    V: Add<Output = V>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<
    V: AddAssign<V>,
    const T: isize,
    const L: isize,
    const M: isize,
    const EC: isize,
    const TT: isize,
    const AS: isize,
    const LI: isize,
> AddAssign<Self> for Quantity<V, T, L, M, EC, TT, AS, LI>
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<
    V: Default + AddAssign<V> + Add<Output = V>,
    const T: isize,
    const L: isize,
    const M: isize,
    const EC: isize,
    const TT: isize,
    const AS: isize,
    const LI: isize,
> Sum<Self> for Quantity<V, T, L, M, EC, TT, AS, LI>
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = V::default();
        for item in iter {
            sum += item.0;
        }
        Self(sum)
    }
}

impl<
    V,
    const T: isize,
    const L: isize,
    const M: isize,
    const EC: isize,
    const TT: isize,
    const AS: isize,
    const LI: isize,
> Sub<Self> for Quantity<V, T, L, M, EC, TT, AS, LI>
where
    V: Sub<Output = V>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<
    V: MulAssign<V>,
    const T: isize,
    const L: isize,
    const M: isize,
    const EC: isize,
    const TT: isize,
    const AS: isize,
    const LI: isize,
> MulAssign<Bare<V>> for Quantity<V, T, L, M, EC, TT, AS, LI>
{
    fn mul_assign(&mut self, rhs: Bare<V>) {
        self.0 *= rhs.0;
    }
}

impl<
    V,
    const T1: isize,
    const L1: isize,
    const M1: isize,
    const EC1: isize,
    const TT1: isize,
    const AS1: isize,
    const LI1: isize,
    const T2: isize,
    const L2: isize,
    const M2: isize,
    const EC2: isize,
    const TT2: isize,
    const AS2: isize,
    const LI2: isize,
> Mul<Quantity<V, T2, L2, M2, EC2, TT2, AS2, LI2>> for Quantity<V, T1, L1, M1, EC1, TT1, AS1, LI1>
where
    V: Mul<Output = V>,
    Quantity<
        V,
        { T1 + T2 },
        { L1 + L2 },
        { M1 + M2 },
        { EC1 + EC2 },
        { TT1 + TT2 },
        { AS1 + AS2 },
        { LI1 + LI2 },
    >: Sized,
{
    type Output = Quantity<
        V,
        { T1 + T2 },
        { L1 + L2 },
        { M1 + M2 },
        { EC1 + EC2 },
        { TT1 + TT2 },
        { AS1 + AS2 },
        { LI1 + LI2 },
    >;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: Quantity<V, T2, L2, M2, EC2, TT2, AS2, LI2>) -> Self::Output {
        Quantity::<
            V,
            { T1 + T2 },
            { L1 + L2 },
            { M1 + M2 },
            { EC1 + EC2 },
            { TT1 + TT2 },
            { AS1 + AS2 },
            { LI1 + LI2 },
        >(self.0 * rhs.0)
    }
}

impl<
    V,
    const T1: isize,
    const L1: isize,
    const M1: isize,
    const EC1: isize,
    const TT1: isize,
    const AS1: isize,
    const LI1: isize,
    const T2: isize,
    const L2: isize,
    const M2: isize,
    const EC2: isize,
    const TT2: isize,
    const AS2: isize,
    const LI2: isize,
> Div<Quantity<V, T2, L2, M2, EC2, TT2, AS2, LI2>> for Quantity<V, T1, L1, M1, EC1, TT1, AS1, LI1>
where
    V: Div<Output = V>,
    Quantity<
        V,
        { T1 - T2 },
        { L1 - L2 },
        { M1 - M2 },
        { EC1 - EC2 },
        { TT1 - TT2 },
        { AS1 - AS2 },
        { LI1 - LI2 },
    >: Sized,
{
    type Output = Quantity<
        V,
        { T1 - T2 },
        { L1 - L2 },
        { M1 - M2 },
        { EC1 - EC2 },
        { TT1 - TT2 },
        { AS1 - AS2 },
        { LI1 - LI2 },
    >;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Quantity<V, T2, L2, M2, EC2, TT2, AS2, LI2>) -> Self::Output {
        Quantity::<
            V,
            { T1 - T2 },
            { L1 - L2 },
            { M1 - M2 },
            { EC1 - EC2 },
            { TT1 - TT2 },
            { AS1 - AS2 },
            { LI1 - LI2 },
        >(self.0 / rhs.0)
    }
}