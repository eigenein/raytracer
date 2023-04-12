use schemars::JsonSchema;
use serde::Deserialize;

use crate::math::uom::{Bare, Length, Temperature};
use crate::optics::consts::{BOLTZMANN, LIGHT_SPEED, PLANCK};
use crate::optics::material::property::Property;

/// Absorbs nothing by default.
#[derive(Deserialize, JsonSchema, Clone)]
#[serde(tag = "type")]
pub enum Spectrum {
    Constant {
        #[serde(default = "Spectrum::default_intensity")]
        intensity: Bare,
    },

    /// https://en.wikipedia.org/wiki/Spectral_line_shape#Lorentzian
    Lorentzian {
        #[serde(default = "Spectrum::default_intensity", alias = "intensity")]
        max_intensity: Bare,

        /// Wavelength of the maximum, meters.
        #[serde(alias = "max", alias = "maximum")]
        maximum_at: Length,

        /// https://en.wikipedia.org/wiki/Full_width_at_half_maximum
        #[serde(alias = "full_width_at_half_maximum")]
        fwhm: Length,
    },

    /// Black body radiation: https://en.wikipedia.org/wiki/Planck%27s_law.
    ///
    /// Do not confuse with black body **absorption** – for the latter use empty reflectance.
    #[serde(alias = "BlackBody")]
    BlackBodyRadiation {
        temperature: Temperature,
        scale: Bare,
    },

    /// Sum of the spectra.
    Sum { spectra: Vec<Spectrum> },
}

impl const Default for Spectrum {
    fn default() -> Self {
        Spectrum::Constant { intensity: Bare::from(1.0) }
    }
}

impl Spectrum {
    pub const fn default_intensity() -> Bare {
        Bare::from(1.0)
    }
}

impl Property<Bare> for Spectrum {
    /// TODO: split into emittance and absorption.
    fn at(&self, wavelength: Length) -> Bare {
        match self {
            Self::Constant { intensity } => *intensity,

            Self::Lorentzian {
                max_intensity,
                maximum_at,
                fwhm,
            } => {
                let x = (wavelength - *maximum_at) / *fwhm * 2.0;
                *max_intensity / (x.powi::<2>() + 1.0)
            }

            Self::BlackBodyRadiation { scale, temperature } => {
                let spectral_radiance = *scale * 2.0 * PLANCK * LIGHT_SPEED.powi::<2>()
                    / wavelength.powi::<5>()
                    / ((PLANCK * LIGHT_SPEED / wavelength / BOLTZMANN / *temperature).exp() - 1.0);
                // FIXME: the SI units of `B(ν)` are `W · sr−1 · m−2 · Hz−1`.
                Bare::from(f64::from(spectral_radiance))
            }

            Self::Sum { spectra } => spectra.iter().map(|spectrum| spectrum.at(wavelength)).sum(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lorentzian_ok() {
        let maximum_at = Length::from_nanos(450.0); // blue
        let fwhm = Length::from(1e-14);
        let spectrum = Spectrum::Lorentzian {
            max_intensity: Bare::from(1.0),
            maximum_at,
            fwhm,
        };

        let intensity_at_half_width = spectrum.at(maximum_at - fwhm / Bare::from(2.0));
        assert!(
            (intensity_at_half_width - Bare::from(0.5)).abs() < Bare::from(1e-8),
            "actual: {intensity_at_half_width}"
        );
    }

    #[test]
    fn black_body_ok() {
        let spectrum = Spectrum::BlackBodyRadiation {
            temperature: Temperature::from(5777.0),
            scale: Bare::from(1.0),
        };
        let intensity = spectrum.at(Length::from_nanos(500.0));
        assert!(
            (intensity - Bare::from(2.635e13)).abs() < Bare::from(1e10),
            "actual: {intensity}"
        );
    }
}
