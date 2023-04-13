use std::iter::Sum;
use std::ops::{Div, Mul};

use glam::DVec3;

use crate::color::cie_1964::WAVELENGTH_TO_XYZ;
use crate::physics::units::Length;

/// XYZ color: https://en.wikipedia.org/wiki/SRGB#Transformation.
#[derive(Debug)]
pub struct XyzColor(DVec3);

impl XyzColor {
    pub fn from_wavelength(wavelength: Length) -> Self {
        let nanos = wavelength.0 / 1e-9;
        let fract = nanos.fract();
        let nanos = nanos as usize - 360;
        assert!(nanos < 470, "actual: {nanos}, wavelength = {wavelength}");
        Self((1.0 - fract) * WAVELENGTH_TO_XYZ[nanos] + fract * WAVELENGTH_TO_XYZ[nanos + 1])
    }

    #[inline]
    pub fn max_element(&self) -> f64 {
        self.0.max_element()
    }
}

impl Sum<XyzColor> for XyzColor {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = DVec3::ZERO;
        for color in iter {
            sum += color.0;
        }
        Self(sum)
    }
}

impl Mul<f64> for XyzColor {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<f64> for XyzColor {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl const From<XyzColor> for DVec3 {
    fn from(value: XyzColor) -> Self {
        value.0
    }
}
