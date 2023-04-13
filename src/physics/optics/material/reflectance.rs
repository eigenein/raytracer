use schemars::JsonSchema;
use serde::Deserialize;

use crate::physics::optics::material::property::Property;
use crate::physics::optics::spectrum::lorentzian;
use crate::physics::units::*;

/// Absorbs nothing by default.
#[derive(Deserialize, JsonSchema, Clone)]
#[serde(tag = "type")]
pub enum ReflectanceAttenuation {
    Constant {
        #[serde(default = "ReflectanceAttenuation::default_intensity")]
        intensity: Bare,
    },

    /// <https://en.wikipedia.org/wiki/Spectral_line_shape#Lorentzian>
    Lorentzian {
        #[serde(
            default = "ReflectanceAttenuation::default_intensity",
            alias = "intensity"
        )]
        max_intensity: Bare,

        /// Wavelength of the maximum, meters.
        #[serde(alias = "max", alias = "maximum")]
        maximum_at: Length,

        /// <https://en.wikipedia.org/wiki/Full_width_at_half_maximum>
        #[serde(alias = "fwhm")]
        full_width_at_half_maximum: Length,
    },

    /// Sum of the spectra.
    Sum {
        spectra: Vec<ReflectanceAttenuation>,
    },
}

impl const Default for ReflectanceAttenuation {
    fn default() -> Self {
        ReflectanceAttenuation::Constant { intensity: Bare::from(1.0) }
    }
}

impl ReflectanceAttenuation {
    pub const fn default_intensity() -> Bare {
        Bare::from(1.0)
    }
}

impl Property<Bare> for ReflectanceAttenuation {
    fn at(&self, wavelength: Length) -> Bare {
        match self {
            Self::Constant { intensity } => *intensity,

            Self::Lorentzian {
                max_intensity,
                maximum_at,
                full_width_at_half_maximum,
            } => *max_intensity * lorentzian(wavelength, *maximum_at, *full_width_at_half_maximum),

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
        let spectrum = ReflectanceAttenuation::Lorentzian {
            max_intensity: Bare::from(1.0),
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
