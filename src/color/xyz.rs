use std::iter::Sum;
use std::ops::{Div, Mul};

use crate::color::cie_1964::WAVELENGTH_TO_XYZ;
use crate::math::vec3::Vec3;
use crate::physics::units::Length;

/// XYZ color: https://en.wikipedia.org/wiki/SRGB#Transformation.
#[derive(Debug)]
#[must_use]
pub struct XyzColor(Vec3);

impl XyzColor {
    pub fn from_wavelength(wavelength: Length) -> Self {
        let nanos = wavelength.0 / 1e-9;
        let fract = nanos.fract();
        let nanos = nanos as usize - 360;
        assert!(nanos < 470, "actual: {nanos}, wavelength = {wavelength}");
        Self((1.0 - fract) * WAVELENGTH_TO_XYZ[nanos] + fract * WAVELENGTH_TO_XYZ[nanos + 1])
    }

    #[inline]
    #[must_use]
    pub fn max_element(&self) -> f64 {
        self.0.max_element()
    }
}

impl Sum<XyzColor> for XyzColor {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = Vec3::ZERO;
        for color in iter {
            sum += color.0;
        }
        Self(sum)
    }
}

impl const Mul<f64> for XyzColor {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl const Div<f64> for XyzColor {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl const From<XyzColor> for Vec3 {
    fn from(value: XyzColor) -> Self {
        value.0
    }
}
