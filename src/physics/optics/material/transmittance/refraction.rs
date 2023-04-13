use schemars::JsonSchema;
use serde::Deserialize;

use crate::physics::optics::material::property::Property;
use crate::physics::units::*;

/// Absolute refraction index.
///
/// By default, it is that of vacuum.
#[derive(Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum AbsoluteRefractiveIndex {
    Constant {
        index: Bare,
    },

    /// <https://en.wikipedia.org/wiki/Cauchy%27s_equation>
    Cauchy2 {
        a: Bare,
        b: Quantity<f64, 0, 2, 0, 0, 0, 0, 0>,
    },

    /// <https://en.wikipedia.org/wiki/Cauchy%27s_equation>
    Cauchy4 {
        a: Bare,
        b: Quantity<f64, 0, 2, 0, 0, 0, 0, 0>,
        c: Quantity<f64, 0, 4, 0, 0, 0, 0, 0>,
        d: Quantity<f64, 0, 6, 0, 0, 0, 0, 0>,
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

impl const Default for AbsoluteRefractiveIndex {
    fn default() -> Self {
        Self::Constant { index: Bare::from(1.0) }
    }
}

impl Property<Bare> for AbsoluteRefractiveIndex {
    /// Get the absolute refractive index at the given wavelength.
    fn at(&self, wavelength: Length) -> Bare {
        match self {
            Self::Constant { index } => *index,

            Self::Cauchy2 { a, b } => *a + *b / wavelength.powi::<2>(),

            Self::Cauchy4 { a, b, c, d } => {
                *a + *b / wavelength.powi::<2>()
                    + *c / wavelength.powi::<4>()
                    + *d / wavelength.powi::<6>()
            }

            Self::Water => Self::Cauchy4 {
                a: Bare::from(1.3199),
                b: Quantity::from(6878e-18),
                c: Quantity::from(-1.132e-27),
                d: Quantity::from(1.11e-40),
            }
            .at(wavelength),

            Self::FusedQuartz => Self::Cauchy2 {
                a: Bare::from(1.4580),
                b: Quantity::from(3.54e-15),
            }
            .at(wavelength),
        }
    }
}

/// https://en.wikipedia.org/wiki/Refractive_index
pub struct RelativeRefractiveIndex {
    /// Absolute incident index.
    pub incident: Bare,

    /// Absolute refracted index.
    pub refracted: Bare,
}

impl RelativeRefractiveIndex {
    pub fn relative(&self) -> Bare {
        self.incident / self.refracted
    }

    /// Calculate Schlick's approximation for reflectance:
    /// https://en.wikipedia.org/wiki/Schlick%27s_approximation.
    pub fn reflectance(&self, cosine_theta_1: f64) -> Bare {
        let r0 = ((self.incident - self.refracted) / (self.incident + self.refracted)).powi::<2>();
        r0 + (Bare::from(1.0) - r0) * (Bare::from(1.0) - cosine_theta_1).powi::<5>()
    }
}
