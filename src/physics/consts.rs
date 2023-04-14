use crate::physics::units::*;

pub const LIGHT_SPEED: Velocity = Quantity(299_792_458.0);

/// <https://en.wikipedia.org/wiki/Boltzmann_constant>
pub const BOLTZMANN: Quantity<f64, -2, 2, 1, 0, -1, 0, 0> = Quantity(1.380649e-23);

/// <https://en.wikipedia.org/wiki/Planck_constant>
pub const PLANCK: Quantity<f64, -1, 2, 1, 0, 0, 0, 0> = Quantity(6.62607015e-34);

/// <https://en.wikipedia.org/wiki/Stefan%E2%80%93Boltzmann_law>
#[allow(unused)]
pub const STEFAN_BOLTZMANN: Quantity<f64, -3, 0, 1, 0, -4, 0, 0> =
    Bare::from(2.0 / 15.0) * Bare::PI.quintic() * BOLTZMANN.quartic()
        / LIGHT_SPEED.squared()
        / PLANCK.cubed();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stefan_boltzmann_ok() {
        assert!(
            (STEFAN_BOLTZMANN - Quantity(5.670374419e-8)).abs().0 < 1e-17,
            "actual: `{STEFAN_BOLTZMANN}`",
        );
    }
}
