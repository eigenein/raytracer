use schemars::JsonSchema;
use serde::Deserialize;

pub trait Spectrum {
    /// Get the intensity at the specified wavelength.
    fn intensity_at(wavelength: f64) -> f64;
}

/// Linear combination of spectra.
pub struct Spectra {}

/// Simple spectrum specified by its base and intensity.
#[derive(Deserialize, JsonSchema, Copy, Clone)]
pub struct SimpleSpectrum {
    pub base: BaseSpectrum,
    pub intensity: f64,
}

#[derive(Deserialize, JsonSchema, Copy, Clone)]
pub enum BaseSpectrum {
    /// Single spectral line, millimeters.
    Line(f64),
}
