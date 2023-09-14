use fastrand::Rng;

use crate::math::vec2::Vec2;

pub trait Sequence<T> {
    fn next(&mut self) -> T;
}

pub struct RandomSequence(Rng);

impl RandomSequence {
    #[allow(dead_code)]
    #[inline]
    pub fn new() -> Self {
        Self(Rng::new())
    }
}

impl Sequence<f64> for RandomSequence {
    #[inline]
    fn next(&mut self) -> f64 {
        self.0.f64()
    }
}

impl Sequence<Vec2> for RandomSequence {
    #[inline]
    fn next(&mut self) -> Vec2 {
        Vec2::new(self.0.f64(), self.0.f64())
    }
}

/// [Van der Corput sequence][1].
///
/// This is just a re-write of [the Python implementation][2] for Halton sequence.
///
/// [1]: https://en.wikipedia.org/wiki/Van_der_Corput_sequence
/// [2]: https://en.wikipedia.org/wiki/Halton_sequence#Implementation
pub struct VanDerCorput {
    base: u64,
    offset: f64,
    n: u64,
    d: u64,
}

impl VanDerCorput {
    pub const fn new(base: u64) -> Self {
        Self { base, offset: 0.0, n: 0, d: 1 }
    }

    /// Set the offset (modulo 1).
    #[allow(dead_code)]
    #[inline]
    pub const fn offset(mut self, offset: f64) -> Self {
        self.offset = offset;
        self
    }
}

impl Sequence<f64> for VanDerCorput {
    fn next(&mut self) -> f64 {
        let x = self.d - self.n;
        if x == 1 {
            self.n = 1;
            self.d *= self.base
        } else {
            let mut y = self.d / self.base;
            while x <= y {
                y /= self.base;
            }
            self.n = (self.base + 1) * y - x
        }
        (self.n as f64 / self.d as f64 + self.offset) % 1.0
    }
}

pub struct Halton2 {
    corput_1: VanDerCorput,
    corput_2: VanDerCorput,
}

impl Halton2 {
    pub const fn new(base_1: u64, base_2: u64) -> Self {
        if base_1 == base_2 {
            panic!("different bases are expected");
        }
        Self {
            corput_1: VanDerCorput::new(base_1),
            corput_2: VanDerCorput::new(base_2),
        }
    }

    #[allow(dead_code)]
    pub const fn offset(mut self, offset: Vec2) -> Self {
        self.corput_1 = self.corput_1.offset(offset.x);
        self.corput_2 = self.corput_2.offset(offset.y);
        self
    }
}

impl Sequence<Vec2> for Halton2 {
    #[inline]
    fn next(&mut self) -> Vec2 {
        Vec2 {
            x: self.corput_1.next(),
            y: self.corput_2.next(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corput_ok() {
        let mut sequence = VanDerCorput::new(2);
        assert_eq!(sequence.next(), 0.5);
        assert_eq!(sequence.next(), 0.25);
        assert_eq!(sequence.next(), 0.75);
        assert_eq!(sequence.next(), 0.125);
        assert_eq!(sequence.next(), 0.625);
        assert_eq!(sequence.next(), 0.375);
        assert_eq!(sequence.next(), 0.875);
        assert_eq!(sequence.next(), 0.0625);
    }
}
