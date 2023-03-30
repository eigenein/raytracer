use std::ops::Range;

use glam::DVec3;
use tracing::info;

use crate::args::TracerOptions;
use crate::hit::Hit;
use crate::math::random_unit_vector;
use crate::prelude::*;
use crate::progress::new_progress;
use crate::ray::Ray;
use crate::scene::Scene;

pub struct Tracer {
    pub scene: Scene,
    pub options: TracerOptions,
}

impl Tracer {
    pub const fn new(scene: Scene, options: TracerOptions) -> Self {
        Self { options, scene }
    }

    pub fn trace(&self, output_width: u32, output_height: u32) -> Result<Vec<DVec3>> {
        info!(self.options.samples_per_pixel, self.options.max_depth);

        let mut pixels = Vec::with_capacity(output_width as usize * output_height as usize);

        // Vectors to convert the image's pixel coordinates to the viewport's ones:
        let x_vector = DVec3::new(self.scene.viewport.width, 0.0, 0.0) / output_width as f64;
        let y_vector = DVec3::new(0.0, -x_vector.x, 0.0);
        info!(?x_vector, ?y_vector);

        let viewport_center = DVec3::new(0.0, 0.0, -self.scene.viewport.distance);
        info!(?viewport_center);

        let eye_position =
            DVec3::new(0.0, 0.0, -self.scene.viewport.focal_length - self.scene.viewport.distance);
        info!(?eye_position);

        let half_image_width = output_width as f64 / 2.0;
        let half_image_height = output_height as f64 / 2.0;
        let distance = 0.000001..f64::INFINITY; // FIXME: shadow acne problem.

        let progress = new_progress(output_height as u64, "tracing (rows)")?;

        for y in 0..output_height {
            for x in 0..output_width {
                let color = (0..self.options.samples_per_pixel)
                    .map(|_| {
                        let image_x = x as f64 - half_image_width + fastrand::f64() - 0.5;
                        let image_y = y as f64 - half_image_height + fastrand::f64() - 0.5;
                        let viewport_point =
                            viewport_center + image_x * x_vector + image_y * y_vector;
                        self.trace_ray(
                            Ray::by_two_points(eye_position, viewport_point),
                            self.options.max_depth,
                            &distance,
                        )
                    })
                    .sum::<DVec3>()
                    / self.options.samples_per_pixel as f64;
                pixels.push(color);
            }
            progress.inc(1);
        }
        progress.finish();

        Ok(pixels)
    }

    /// Trace the ray and return the resulting color.
    #[inline]
    fn trace_ray(&self, mut ray: Ray, depth_left: u16, distance_range: &Range<f64>) -> DVec3 {
        ray.normalize();

        let hit = self
            .scene
            .surfaces
            .iter()
            .filter_map(|surface| surface.hit(&ray, distance_range))
            .min_by(|hit_1, hit_2| hit_1.distance.total_cmp(&hit_2.distance));

        match hit {
            Some(hit) if depth_left != 0 => {
                // The ray hit a surface, scatter the ray:
                self.trace_scattered_rays(&ray, &hit, depth_left - 1, distance_range)
            }
            Some(_) => DVec3::ZERO,           // the depth limit is reached
            None => self.scene.ambient_color, // the ray didn't hit anything
        }
    }

    /// Notes:
    ///
    /// - The incident ray **must** be normalized.
    ///
    /// See also:
    ///
    /// - <https://physics.stackexchange.com/a/436252/11966>
    /// - <https://en.wikipedia.org/wiki/Snell%27s_law#Vector_form>
    /// - <https://en.wikipedia.org/wiki/Lambertian_reflectance>
    pub fn trace_scattered_rays(
        &self,
        incident_ray: &Ray,
        hit: &Hit,
        depth_left: u16,
        distance_range: &Range<f64>,
    ) -> DVec3 {
        let mut reflectance = 1.0;
        let mut total_color = DVec3::ZERO;

        let cosine_theta_1 = -hit.normal.dot(incident_ray.direction);
        assert!(cosine_theta_1 >= 0.0);

        if let Some(to_refractive_index) = hit.material.refractive_index {
            // Transparent body, the ray may refract:
            let mu = if hit.from_outside {
                incident_ray.refractive_index / to_refractive_index
            } else {
                to_refractive_index / incident_ray.refractive_index
            };
            let sin_theta_2 = mu * (1.0 - cosine_theta_1.powi(2)).sqrt();

            if sin_theta_2 <= 1.0 {
                // Refraction is possible.
                reflectance = {
                    // Schlick's approximation for reflectance:
                    let r0 = ((incident_ray.refractive_index - to_refractive_index)
                        / (incident_ray.refractive_index + to_refractive_index))
                        .powi(2);
                    r0 + (1.0 - r0) * (1.0 - cosine_theta_1).powi(5)
                };

                // Shell's law:
                let direction = {
                    let cosine_theta_2 = (1.0 - sin_theta_2.powi(2)).sqrt();
                    mu * incident_ray.direction
                        + hit.normal * (mu * cosine_theta_1 - cosine_theta_2)
                };
                let ray = Ray {
                    origin: hit.location,
                    direction,
                    refractive_index: to_refractive_index,
                };

                total_color +=
                    self.trace_ray(ray, depth_left, distance_range) * (1.0 - reflectance);
            }
        }

        if let Some(diffusion_probability) = hit.material.diffusion_probability {
            // Diffused reflection with Lambertian reflectance:
            // <https://en.wikipedia.org/wiki/Lambertian_reflectance>.
            let ray = Ray {
                origin: hit.location,
                direction: hit.normal + random_unit_vector(),
                refractive_index: incident_ray.refractive_index,
            };
            total_color += self.trace_ray(ray, depth_left, distance_range)
                * reflectance
                * diffusion_probability;
        }

        // And finally, normal reflectance:
        {
            let mut ray = Ray {
                origin: hit.location,
                direction: incident_ray.direction + 2.0 * cosine_theta_1 * hit.normal,
                refractive_index: incident_ray.refractive_index,
            };
            if let Some(fuzz) = hit.material.reflective_fuzz {
                ray.direction += random_unit_vector() * fuzz;
            }
            total_color += self.trace_ray(ray, depth_left, distance_range)
                * reflectance
                * (1.0 - hit.material.diffusion_probability.unwrap_or_default());
        }

        total_color * hit.material.attenuation * hit.material.albedo
    }
}
