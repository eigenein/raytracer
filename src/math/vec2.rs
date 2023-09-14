use std::ops::{Add, Sub};

use fastrand::Rng;

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    #[inline]
    pub fn new<V: Into<f64>>(x: V, y: V) -> Self {
        Self { x: x.into(), y: y.into() }
    }

    #[inline]
    pub const fn splat(value: f64) -> Self {
        Self { x: value, y: value }
    }

    #[inline]
    pub fn random(rng: &Rng) -> Self {
        Self { x: rng.f64(), y: rng.f64() }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
