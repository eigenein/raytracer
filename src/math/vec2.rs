use std::ops::{Add, Sub};

#[repr(simd)]
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    #[inline]
    pub const fn new<V: ~const Into<f64>>(x: V, y: V) -> Self {
        Self { x: x.into(), y: y.into() }
    }

    #[inline]
    pub const fn splat(value: f64) -> Self {
        Self { x: value, y: value }
    }

    #[inline]
    pub fn fastrand() -> Self {
        Self {
            x: fastrand::f64(),
            y: fastrand::f64(),
        }
    }
}

impl const Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl const Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
