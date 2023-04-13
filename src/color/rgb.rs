use std::ops::Mul;

use crate::color::cie_1964::*;
use crate::color::xyz::XyzColor;
use crate::math::vec::Vec3;
use crate::physics::units::Length;

/// RGB color represented as a 3-vector.
#[derive(Debug)]
pub struct RgbColor(Vec3);

impl const From<Vec3> for RgbColor {
    #[inline]
    fn from(value: Vec3) -> Self {
        Self(value)
    }
}

impl From<XyzColor> for RgbColor {
    /// - https://en.wikipedia.org/wiki/SRGB#From_CIE_XYZ_to_sRGB
    /// - https://stackoverflow.com/a/39446403/359730
    #[inline]
    fn from(value: XyzColor) -> Self {
        let value = Vec3::from(value);
        let srgb = Vec3::new(
            Self::srgb_gamma_correction(value.dot(XYZ_TO_RED)),
            Self::srgb_gamma_correction(value.dot(XYZ_TO_GREEN)),
            Self::srgb_gamma_correction(value.dot(XYZ_TO_BLUE)),
        );
        Self(srgb.clamp(Vec3::ZERO, Vec3::ONE))
    }
}

impl RgbColor {
    #[inline]
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
    }

    #[inline]
    pub fn abs_diff_eq(&self, rhs: &Self, max_abs_diff: f64) -> bool {
        self.0.abs_diff_eq(rhs.0, max_abs_diff)
    }

    #[inline]
    pub fn apply_gamma(self, gamma: f64) -> Self {
        Self(self.0.powf(gamma))
    }

    #[inline]
    pub fn from_wavelength(wavelength: Length) -> Self {
        Self::from(XyzColor::from_wavelength(wavelength))
    }

    #[inline]
    fn srgb_gamma_correction(linear_color: f64) -> f64 {
        if linear_color <= 0.0031308 {
            12.92 * linear_color
        } else {
            1.055 * linear_color.powf(1.0 / 2.4) - 0.055
        }
    }
}

impl From<RgbColor> for image::Rgb<u16> {
    #[inline]
    fn from(value: RgbColor) -> Self {
        let value = value.0.clamp(Vec3::ZERO, Vec3::ONE);
        let value = value * u16::MAX as f64;
        let value = value.round();
        Self::from([value.x as u16, value.y as u16, value.z as u16])
    }
}

impl Mul<f64> for RgbColor {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn red_limit_ok() {
        let color = RgbColor::from_wavelength(Length::from_nanos(700.0));
        assert!(color.abs_diff_eq(&RgbColor::new(0.18, 0.0, 0.0), 0.01), "actual: {color:?}");
    }

    #[test]
    fn blue_ok() {
        let color = RgbColor::from_wavelength(Length::from_nanos(450.0));
        assert!(color.abs_diff_eq(&RgbColor::new(0.29, 0.0, 1.0), 0.01), "actual: {color:?}");
    }

    #[test]
    fn violet_limit_ok() {
        let color = RgbColor::from_wavelength(Length::from_nanos(400.0));
        assert!(color.abs_diff_eq(&RgbColor::new(0.13, 0.0, 0.33), 0.01), "actual: {color:?}");
    }
}
