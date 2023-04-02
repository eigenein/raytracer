use std::ops::Mul;

use glam::DVec3;

/// https://en.wikipedia.org/wiki/SRGB#From_CIE_XYZ_to_sRGB
pub struct XyzColor(DVec3);

/// RGB color represented as a 3-vector.
pub struct RgbColor(DVec3);

impl const From<DVec3> for RgbColor {
    #[inline]
    fn from(value: DVec3) -> Self {
        Self(value)
    }
}

impl RgbColor {
    #[inline]
    pub fn apply_gamma(self, gamma: f64) -> Self {
        Self(self.0.powf(gamma))
    }
}

impl From<RgbColor> for ::image::Rgb<u16> {
    #[inline]
    fn from(value: RgbColor) -> Self {
        let value = value.0.clamp(DVec3::ZERO, DVec3::ONE);
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
