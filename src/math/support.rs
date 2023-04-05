use glam::DVec3;

pub fn random_unit_vector() -> DVec3 {
    loop {
        let vector =
            DVec3::new(fastrand::f64() - 0.5, fastrand::f64() - 0.5, fastrand::f64() - 0.5);
        if vector.length_squared() <= 0.25 {
            return vector.normalize();
        }
    }
}
