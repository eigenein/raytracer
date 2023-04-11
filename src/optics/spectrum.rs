use schemars::JsonSchema;
use serde::Deserialize;

use crate::optics::consts::{BOLTZMANN, LIGHT_SPEED, LIGHT_SPEED_2, PLANCK};

/// Absorbs nothing by default.
#[derive(Deserialize, JsonSchema, Clone)]
#[serde(tag = "type")]
pub enum Spectrum {
    Constant {
        #[serde(default = "Spectrum::default_intensity")]
        intensity: f64,
    },

    /// https://en.wikipedia.org/wiki/Spectral_line_shape#Lorentzian
    Lorentzian {
        #[serde(default = "Spectrum::default_intensity", alias = "intensity")]
        max_intensity: f64,

        /// Wavelength of the maximum, meters.
        #[serde(alias = "max", alias = "maximum")]
        maximum_at: f64,

        /// https://en.wikipedia.org/wiki/Full_width_at_half_maximum
        #[serde(alias = "full_width_at_half_maximum")]
        fwhm: f64,
    },

    /// Black body radiation: https://en.wikipedia.org/wiki/Planck%27s_law.
    ///
    /// Do not confuse with black body **absorption** – for the latter use empty reflectance.
    #[serde(alias = "BlackBody")]
    BlackBodyRadiation { temperature: f64, scale: f64 },

    /// Sum of the spectra.
    Sum { spectra: Vec<Spectrum> },

    /// Multiplication of the spectra.
    Mul { spectra: Vec<Spectrum> },
}

impl const Default for Spectrum {
    fn default() -> Self {
        Spectrum::Constant { intensity: 1.0 }
    }
}

impl Spectrum {
    pub fn intensity_at(&self, wavelength: f64) -> f64 {
        match self {
            Self::Constant { intensity } => *intensity,

            Self::Lorentzian {
                max_intensity,
                maximum_at,
                fwhm,
            } => {
                let x = 2.0 * (wavelength - maximum_at) / fwhm;
                max_intensity / (1.0 + x * x)
            }

            Self::BlackBodyRadiation { scale, temperature } => {
                scale * 2.0 * PLANCK * LIGHT_SPEED_2
                    / wavelength.powi(5)
                    / ((PLANCK * LIGHT_SPEED / wavelength / BOLTZMANN / temperature).exp() - 1.0)
            }

            Self::Sum { spectra } => spectra
                .iter()
                .map(|spectrum| spectrum.intensity_at(wavelength))
                .sum(),

            Self::Mul { spectra } => spectra
                .iter()
                .map(|spectrum| spectrum.intensity_at(wavelength))
                .fold(1.0, |accumulator, intensity| accumulator * intensity),
        }
    }

    pub const fn default_intensity() -> f64 {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lorentzian_ok() {
        let maximum = 450e-9; // blue
        let fwhm = 1e-14;
        let spectrum = Spectrum::Lorentzian {
            max_intensity: 1.0,
            maximum_at: maximum,
            fwhm,
        };

        let intensity_at_half_width = spectrum.intensity_at(maximum - fwhm / 2.0);
        assert!(
            (intensity_at_half_width - 0.5).abs() < 1e-8,
            "actual: {intensity_at_half_width}"
        );
    }

    #[test]
    fn black_body_ok() {
        let spectrum = Spectrum::BlackBodyRadiation {
            temperature: 5777.0,
            scale: 1.0,
        };
        let intensity = spectrum.intensity_at(500e-9);
        assert!((intensity - 2.635e13).abs() < 1e10, "actual: {intensity}");
    }
}
