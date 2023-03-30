use glam::DVec3;

use crate::hit::Hit;
use crate::math::random_unit_vector;

pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
    pub refractive_index: f64,
}

impl Ray {
    pub fn by_two_points(from: DVec3, to: DVec3) -> Self {
        Self {
            origin: from,
            direction: to - from,
            refractive_index: 1.0,
        }
    }

    pub fn at(&self, time: f64) -> DVec3 {
        self.origin + self.direction * time
    }

    /// See also:
    ///
    /// - <https://physics.stackexchange.com/a/436252/11966>
    /// - <https://en.wikipedia.org/wiki/Snell%27s_law#Vector_form>
    /// - <https://en.wikipedia.org/wiki/Lambertian_reflectance>
    pub fn scatter(&self, hit: &Hit) -> Self {
        let incident_direction = self.direction.normalize();
        let cosine_theta_1 = -hit.normal.dot(incident_direction);
        assert!(cosine_theta_1 >= 0.0);

        if let Some(to_refractive_index) = hit.material.refractive_index {
            // Transparent body, consider refracting the ray.
            let mu = if hit.from_outside {
                self.refractive_index / to_refractive_index
            } else {
                to_refractive_index / self.refractive_index
            };
            let sin_theta_2 = mu * (1.0 - cosine_theta_1.powi(2)).sqrt();

            if sin_theta_2 <= 1.0 {
                // Refraction is possible.
                let reflectance = {
                    // Schlick's approximation for reflectance:
                    let r0 = ((self.refractive_index - to_refractive_index)
                        / (self.refractive_index + to_refractive_index))
                        .powi(2);
                    r0 + (1.0 - r0) * (1.0 - cosine_theta_1).powi(5)
                };

                // TODO: consider casting 2 scattered rays:
                if reflectance < fastrand::f64() {
                    // The ray got refracted, apply the Shell's law:
                    let direction = {
                        let cosine_theta_2 = (1.0 - sin_theta_2.powi(2)).sqrt();
                        mu * incident_direction
                            + hit.normal * (mu * cosine_theta_1 - cosine_theta_2)
                    };
                    return Self {
                        origin: hit.location,
                        direction,
                        refractive_index: to_refractive_index,
                    };
                }
            }
        }

        // The ray couldn't or didn't get refracted:
        /// TODO: consider casting 2 scattered rays:
        if hit.material.diffusion_probability > fastrand::f64() {
            // Diffused reflection with Lambertian reflectance: <https://en.wikipedia.org/wiki/Lambertian_reflectance>.
            Self {
                origin: hit.location,
                direction: hit.normal + random_unit_vector(),
                refractive_index: self.refractive_index,
            }
        } else {
            // Normal reflectance:
            let mut reflected_ray = Self {
                origin: hit.location,
                direction: incident_direction + 2.0 * cosine_theta_1 * hit.normal,
                refractive_index: self.refractive_index,
            };
            if let Some(fuzz) = hit.material.reflective_fuzz {
                reflected_ray.direction +=
                    random_unit_vector() * fuzz * reflected_ray.direction.length();
            }
            reflected_ray
        }
    }
}
