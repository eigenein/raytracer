use std::f64::consts::PI;
use std::ops::{Add, Sub};

use crate::physics::units::quantity::Quantity;

/// Dimensionless quantity: <https://en.wikipedia.org/wiki/Dimensionless_quantity>.
pub type Bare<V = f64> = Quantity<V, 0, 0, 0, 0, 0, 0, 0>;

impl const From<Bare<f64>> for f64 {
    fn from(value: Bare<f64>) -> Self {
        value.0
    }
}

impl<V: ~const Add<Output = V>> const Add<V> for Bare<V> {
    type Output = Self;

    fn add(self, rhs: V) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl<V: ~const Sub<Output = V>> const Sub<V> for Bare<V> {
    type Output = Self;

    fn sub(self, rhs: V) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Bare<f64> {
    pub const PI: Self = Bare::from(PI);

    #[inline]
    pub fn fastrand() -> Self {
        Self(fastrand::f64())
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
