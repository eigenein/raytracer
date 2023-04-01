/// https://en.wikipedia.org/wiki/Refractive_index
pub struct RefractiveIndex {
    /// Absolute incident index.
    pub incident: f64,

    /// Absolute refracted index.
    pub refracted: f64,
}

impl RefractiveIndex {
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
