use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema, Copy, Clone)]
pub enum Spectrum {
    /// https://en.wikipedia.org/wiki/Spectral_line_shape#Lorentzian
    Lorentzian {
        #[serde(default = "Spectrum::default_intensity")]
        intensity: f64,

        /// Wavelength of the maximum, meters.
        #[serde(alias = "max")]
        maximum: f64,

        /// https://en.wikipedia.org/wiki/Full_width_at_half_maximum
        #[serde(alias = "full width at half maximum")]
        fwhm: f64,
    },
}

impl Spectrum {
    pub fn intensity_at(&self, wavelength: f64) -> f64 {
        match self {
            Self::Lorentzian {
                intensity,
                maximum,
                fwhm,
            } => {
                let x = 2.0 * (wavelength - maximum) / fwhm;
                intensity / (1.0 + x * x)
            }
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
        let fwhm = 2e-5; // around 200kHz
        let spectrum = Spectrum::Lorentzian {
            intensity: 1.0,
            maximum,
            fwhm,
        };

        let intensity_at_half_width = spectrum.intensity_at(maximum - fwhm / 2.0);
        assert!(
            (intensity_at_half_width - 0.5).abs() < 0.001,
            "actual: {intensity_at_half_width}"
        );
    }
}
