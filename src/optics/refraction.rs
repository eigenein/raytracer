use schemars::JsonSchema;
use serde::Deserialize;

/// Absolute refraction index.
///
/// By default, it is of vacuum.
#[derive(Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum RefractiveIndex {
    Constant {
        index: f64,
    },

    /// https://en.wikipedia.org/wiki/Cauchy%27s_equation
    Cauchy {
        #[serde(alias = "c")]
        coefficients: Vec<f64>,
    },

    Cauchy2 {
        a: f64,
        b: f64,
    },

    Cauchy4 {
        a: f64,
        b: f64,
        c: f64,
        d: f64,
    },

    /// Alexey N. Bashkatov and Elina A. Genina
    /// "Water refractive index in dependence on temperature and wavelength: a simple approximation",
    /// Proc. SPIE 5068, Saratov Fall Meeting 2002: Optical Technologies in Biophysics and Medicine IV,
    /// (13 October 2003); <https://doi.org/10.1117/12.518857>.
    Water,

    /// - <https://en.wikipedia.org/wiki/Fused_quartz>
    /// - <https://en.wikipedia.org/wiki/Cauchy%27s_equation>
    #[serde(alias = "FusedSilica", alias = "QuartzGlass")]
    FusedQuartz,
}

impl const Default for RefractiveIndex {
    fn default() -> Self {
        Self::Constant { index: 1.0 }
    }
}

impl RefractiveIndex {
    /// Get the absolute refractive index at the given wavelength.
    pub fn at(&self, wavelength: f64) -> f64 {
        match self {
            Self::Constant { index } => *index,

            Self::Cauchy { coefficients } => coefficients
                .iter()
                .enumerate()
                .map(|(i, coefficient)| coefficient / wavelength.powi((i * 2) as i32))
                .sum(),

            Self::Cauchy2 { a, b } => a + b / wavelength.powi(2),

            Self::Cauchy4 { a, b, c, d } => {
                a + b / wavelength.powi(2) + c / wavelength.powi(4) + d / wavelength.powi(6)
            }

            Self::Water => Self::Cauchy4 {
                a: 1.3199,
                b: 6878e-18,
                c: -1.132e-27,
                d: 1.11e-40,
            }
            .at(wavelength),

            Self::FusedQuartz => Self::Cauchy2 { a: 1.4580, b: 3.54e-15 }.at(wavelength),
        }
    }
}

/// https://en.wikipedia.org/wiki/Refractive_index
pub struct RelativeRefractiveIndex {
    /// Absolute incident index.
    pub incident: f64,

    /// Absolute refracted index.
    pub refracted: f64,
}

impl RelativeRefractiveIndex {
    pub const fn relative(&self) -> f64 {
        self.incident / self.refracted
    }

    /// Calculate Schlick's approximation for reflectance:
    /// https://en.wikipedia.org/wiki/Schlick%27s_approximation.
    pub fn reflectance(&self, cosine_theta_1: f64) -> f64 {
        let r0 = ((self.incident - self.refracted) / (self.incident + self.refracted)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine_theta_1).powi(5)
    }
}
