use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema, Copy, Clone)]
pub struct Spectrum {
    pub base: BaseSpectrum,
    pub intensity: f64,
}

#[derive(Deserialize, JsonSchema, Copy, Clone)]
pub enum BaseSpectrum {
    /// https://en.wikipedia.org/wiki/Spectral_line_shape#Lorentzian
    Lorentzian(f64),
}
