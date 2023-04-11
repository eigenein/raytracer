use crate::math::uom::Quantity;

pub const LIGHT_SPEED: Quantity<f64, -1, 1, 0, 0, 0, 0, 0> = Quantity::from(299_792_458.0);

/// https://en.wikipedia.org/wiki/Boltzmann_constant
pub const BOLTZMANN: Quantity<f64, -2, 2, 1, 0, -1, 0, 0> = Quantity::from(1.380649e-23);

/// https://en.wikipedia.org/wiki/Planck_constant
pub const PLANCK: Quantity<f64, -1, 2, 1, 0, 0, 0, 0> = Quantity::from(6.62607015e-34);
