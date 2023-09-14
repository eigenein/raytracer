use std::f64::consts::PI;
use std::ops::{Add, Sub};

use fastrand::Rng;

use crate::physics::units::quantity::Quantity;

/// Dimensionless quantity: <https://en.wikipedia.org/wiki/Dimensionless_quantity>.
pub type Bare = Quantity<0, 0, 0, 0, 0, 0, 0>;

impl From<Bare> for f64 {
    fn from(value: Bare) -> Self {
        value.0
    }
}

impl Add<f64> for Bare {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<f64> for Bare {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Bare {
    pub const PI: Self = Self(PI);

    #[inline]
    pub fn random(rng: &Rng) -> Self {
        Self(rng.f64())
    }

    #[inline]
    pub fn exp(self) -> Self {
        Self(self.0.exp())
    }

    #[inline]
    pub fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }

    #[inline]
    pub fn powf<X: Into<f64>>(self, x: X) -> Self {
        Self(self.0.powf(x.into()))
    }
}
