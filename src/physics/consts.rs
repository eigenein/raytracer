use crate::physics::units::*;

pub const LIGHT_SPEED: Velocity = Quantity(299_792_458.0);

/// <https://en.wikipedia.org/wiki/Boltzmann_constant>
pub const BOLTZMANN: Quantity<-2, 2, 1, 0, -1, 0> = Quantity(1.380649e-23);

/// <https://en.wikipedia.org/wiki/Planck_constant>
pub const PLANCK: Quantity<-1, 2, 1, 0, 0, 0> = Quantity(6.62607015e-34);

/// <https://en.wikipedia.org/wiki/Stefan%E2%80%93Boltzmann_law>
#[allow(dead_code)]
pub const STEFAN_BOLTZMANN: Quantity<-3, 0, 1, 0, -4, 0> = Quantity(5.670374419e-8);
