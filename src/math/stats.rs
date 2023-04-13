//! Subset of the probability theory tools.
//!
//! I will use them to mitigate the chromatic noise via separating path tracing
//! and wavelength sampling, and converting a complete spectrum to a color.

use std::f64::consts::FRAC_1_PI;
use std::ops::Range;

use crate::math::const_pow2;
use crate::physics::units::Temperature;

/// Probability density function.
#[const_trait]
pub trait Pdf {
    /// Get the value of the PDF at `x`.
    #[must_use]
    fn pdf(&self, x: f64) -> f64;
}

pub trait Sample {
    /// Sample the distribution and return a random value.
    #[must_use]
    fn sample(&self, domain: Range<f64>) -> f64;
}

/// Sort of [Rejection sampling][1] for any distribution which supports PDF.
/// Here, `Mg(x) = 1`, so `f(x) <= Mg(x)` always holds.
///
/// Implemented as a standalone function because of [the compiler bug][2].
///
/// [1]: https://en.wikipedia.org/wiki/Rejection_sampling
/// [2]: https://github.com/rust-lang/rust/issues/48869
fn sample<P: Pdf>(distribution: &P, domain: Range<f64>) -> f64 {
    loop {
        let x = domain.start + (domain.end - domain.start) * fastrand::f64();
        if fastrand::f64() <= distribution.pdf(x) {
            break x;
        }
    }
}

/// [Continuous uniform distribution][1].
///
/// Sampling outside the support range is **not allowed**.
///
/// [1]: https://en.wikipedia.org/wiki/Continuous_uniform_distribution
pub struct UniformDistribution(pub Range<f64>);

impl Sample for UniformDistribution {
    fn sample(&self, domain: Range<f64>) -> f64 {
        assert!(domain.start <= self.0.start && domain.end >= self.0.end);
        domain.start + (domain.end - domain.start) * fastrand::f64()
    }
}

/// [Cauchy distribution][1] aka *Lorentz distribution* aka *Breit–Wigner distribution*.
///
/// [1]: https://en.wikipedia.org/wiki/Cauchy_distribution
#[derive(Copy, Clone, Debug)]
pub struct CauchyDistribution {
    /// Scale parameter which specifies the *half-width at half-maximum* (HWHM).
    pub gamma: f64,

    pub median: f64,
}

impl const Pdf for CauchyDistribution {
    #[inline]
    #[must_use]
    fn pdf(&self, x: f64) -> f64 {
        FRAC_1_PI * self.gamma / (const_pow2(x - self.median) + const_pow2(self.gamma))
    }
}

impl Sample for CauchyDistribution {
    #[inline]
    fn sample(&self, domain: Range<f64>) -> f64 {
        sample(self, domain)
    }
}

/// Distribution of a [black-body radiation][1] given by [Planck's law][2] expressed
/// in terms of wavelength.
///
/// [1]: https://en.wikipedia.org/wiki/Black-body_radiation
/// [2]: https://en.wikipedia.org/wiki/Planck%27s_law
pub struct BlackBodyRadiation {
    pub temperature: Temperature,
}
