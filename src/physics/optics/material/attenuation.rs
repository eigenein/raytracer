use schemars::JsonSchema;
use serde::Deserialize;

use crate::physics::optics::material::property::Property;
use crate::physics::optics::spectrum::lorentzian;
use crate::physics::units::*;

/// Absorbs nothing by default.
#[derive(Deserialize, JsonSchema, Clone)]
#[serde(tag = "type")]
pub enum Attenuation {
    Constant {
        #[serde(default = "Attenuation::default_coefficient")]
        coefficient: Bare,
    },

    /// Lorentzian line: <https://en.wikipedia.org/wiki/Spectral_line_shape#Lorentzian>
    Lorentzian {
        #[serde(default = "Attenuation::default_coefficient")]
        coefficient: Bare,

        /// Wavelength of the maximum, meters.
        #[serde(alias = "max", alias = "maximum")]
        maximum_at: Length,

        /// <https://en.wikipedia.org/wiki/Full_width_at_half_maximum>
        #[serde(alias = "fwhm")]
        full_width_at_half_maximum: Length,
    },

    /// Sum of the spectra.
    Sum { spectra: Vec<Attenuation> },
}

impl Default for Attenuation {
    fn default() -> Self {
        Attenuation::Constant {
            coefficient: Self::default_coefficient(),
        }
    }
}

impl Attenuation {
    pub fn default_coefficient() -> Bare {
        Bare::from(1.0)
    }
}

impl Property<Bare> for Attenuation {
    fn at(&self, wavelength: Length) -> Bare {
        match self {
            Self::Constant { coefficient: intensity } => *intensity,

            Self::Lorentzian {
                coefficient,
                maximum_at,
                full_width_at_half_maximum,
            } => *coefficient * lorentzian(wavelength, *maximum_at, *full_width_at_half_maximum),

            Self::Sum { spectra } => spectra
                .iter()
                .map(|attenuation| attenuation.at(wavelength))
                .sum(),
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
        let spectrum = Attenuation::Lorentzian {
            coefficient: Bare::from(1.0),
            maximum_at,
            full_width_at_half_maximum: fwhm,
        };

        let intensity_at_half_width = spectrum.at(maximum_at - fwhm / Bare::from(2.0));
        assert!(
            (intensity_at_half_width - Bare::from(0.5)).abs() < Bare::from(1e-8),
            "actual: {intensity_at_half_width}"
        );
    }
}
