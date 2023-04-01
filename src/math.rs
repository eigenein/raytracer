use fastrand::Rng;
use glam::DVec3;

pub fn random_unit_vector(rng: &Rng) -> DVec3 {
    loop {
        let vector = DVec3::new(rng.f64() - 0.5, rng.f64() - 0.5, rng.f64() - 0.5);
        if vector.length_squared() <= 0.25 {
            return vector.normalize();
        }
    }
}

pub const fn luminance(light: DVec3) -> f64 {
    0.2126 * light.x + 0.7152 * light.y + 0.0722 * light.z
}
