use std::f64::consts::FRAC_PI_2;

use glam::{DQuat, DVec3};
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
        info!(?self.scene.camera.location);
        info!(?self.scene.camera.look_at);
        info!(?self.scene.camera.up);
        info!(self.scene.camera.vertical_fov);

        let (viewport_dx, viewport_dy) = self.get_viewport_plane(output_height);
        info!(?viewport_dx);
        info!(?viewport_dy);

        let half_image_width = output_width as f64 / 2.0;
        let half_image_height = output_height as f64 / 2.0;

        let progress = new_progress(output_height as u64, "tracing (rows)")?;
        let mut pixels = Vec::with_capacity(output_width as usize * output_height as usize);

        for y in 0..output_height {
            for x in 0..output_width {
                let color = (0..self.options.samples_per_pixel)
                    .map(|_| {
                        let image_x = x as f64 - half_image_width + fastrand::f64() - 0.5;
                        let image_y = y as f64 - half_image_height + fastrand::f64() - 0.5;
                        let viewport_point = self.scene.camera.look_at
                            + image_x * viewport_dx
                            + image_y * viewport_dy;
                        let ray = Ray::by_two_points(self.scene.camera.location, viewport_point);
                        self.trace_ray(ray, self.options.max_depth)
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

    /// Calculate and return the viewport's `dx` and `dy` vectors,
    /// which represent how much space the image pixel takes in the scene world.
    ///
    /// The resulting vectors are relative to the camera direction point.
    fn get_viewport_plane(&self, output_height: u32) -> (DVec3, DVec3) {
        let principal_axis = self.scene.camera.location - self.scene.camera.look_at;
        let focal_length = principal_axis.length();
        let principal_axis = principal_axis / focal_length;

        let dx = principal_axis.cross(self.scene.camera.up).normalize();
        let dy = DQuat::from_axis_angle(principal_axis, FRAC_PI_2).mul_vec3(dx);

        // Finally, scale the vectors to the actual field-of-view angle:
        let viewport_height =
            2.0 * focal_length * (self.scene.camera.vertical_fov / 2.0).to_radians().sin();
        let scale = viewport_height / output_height as f64;
        (dx * scale, dy * scale)
    }

    /// Trace the ray and return the resulting color.
    #[inline]
    fn trace_ray(&self, mut ray: Ray, depth_left: u16) -> DVec3 {
        ray.normalize();
        let distance_range = 0.000001..f64::INFINITY; // TODO: shadow acne problem, make an option.;

        let hit = self
            .scene
            .surfaces
            .iter()
            .filter_map(|surface| surface.hit(&ray, &distance_range))
            .min_by(|hit_1, hit_2| hit_1.distance.total_cmp(&hit_2.distance));

        match hit {
            Some(hit) if depth_left != 0 => {
                // The ray hit a surface, scatter the ray:
                self.trace_scattered_rays(&ray, &hit, depth_left - 1)
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
    /// - Shell's law in vector form: <https://physics.stackexchange.com/a/436252/11966>
    /// - Shell's law in vector form: <https://en.wikipedia.org/wiki/Snell%27s_law#Vector_form>
    /// - Lambertian reflectance: <https://en.wikipedia.org/wiki/Lambertian_reflectance>
    pub fn trace_scattered_rays(&self, incident_ray: &Ray, hit: &Hit, depth_left: u16) -> DVec3 {
        let mut reflectance = 1.0;
        let mut total_light = DVec3::ZERO;

        let cosine_theta_1 = -hit.normal.dot(incident_ray.direction);
        assert!(cosine_theta_1 >= 0.0);

        if let Some(transmittance) = &hit.material.transmittance {
            // Transparent body, the ray may refract:
            let current_refractive_index = incident_ray
                .current_refractive_index()
                .unwrap_or(self.scene.refractive_index);

            let mu = if hit.from_outside {
                // Entering the medium:
                current_refractive_index / transmittance.refractive_index
            } else {
                // Leaving the medium:
                let outer_refractive_index = incident_ray
                    .outer_refractive_index()
                    .unwrap_or(self.scene.refractive_index);
                transmittance.refractive_index / outer_refractive_index
            };
            let sin_theta_2 = mu * (1.0 - cosine_theta_1.powi(2)).sqrt();

            if sin_theta_2 <= 1.0 {
                // Refraction is possible, adjust the reflectance:
                reflectance = {
                    // Schlick's approximation for reflectance:
                    let r0 = ((current_refractive_index - transmittance.refractive_index)
                        / (current_refractive_index + transmittance.refractive_index))
                        .powi(2);
                    r0 + (1.0 - r0) * (1.0 - cosine_theta_1).powi(5)
                };

                // Shell's law:
                let direction = {
                    let cosine_theta_2 = (1.0 - sin_theta_2.powi(2)).sqrt();
                    mu * incident_ray.direction
                        + hit.normal * (mu * cosine_theta_1 - cosine_theta_2)
                };
                let ray = {
                    let mut refractive_indexes = incident_ray.refractive_indexes.clone();
                    {
                        // Update the index of the refracted ray:
                        let refractive_indexes = refractive_indexes.to_mut();
                        if hit.from_outside {
                            refractive_indexes.push(transmittance.refractive_index);
                        } else {
                            refractive_indexes
                                .pop()
                                .expect("cannot leave a medium without entering it first");
                        }
                    }
                    Ray {
                        origin: hit.location,
                        direction,
                        refractive_indexes,
                    }
                };

                let mut transmitted_light = self.trace_ray(ray, depth_left) * (1.0 - reflectance);
                if !hit.from_outside {
                    // Hit from inside, apply the attenuation coefficient:
                    if let Some(coefficient) = transmittance.coefficient {
                        // Exponential decay:
                        transmitted_light *= (-hit.distance * coefficient).exp();
                    }
                }
                total_light += transmitted_light
                    * transmittance
                        .attenuation
                        .unwrap_or(hit.material.reflectance.attenuation);
            }
        }

        if let Some(diffusion) = hit.material.reflectance.diffusion {
            // Diffused reflection with Lambertian reflectance:
            // <https://en.wikipedia.org/wiki/Lambertian_reflectance>.
            let ray = Ray {
                origin: hit.location,
                direction: hit.normal + random_unit_vector(),
                refractive_indexes: incident_ray.refractive_indexes.clone(),
            };
            total_light += self.trace_ray(ray, depth_left)
                * reflectance
                * diffusion
                * hit.material.reflectance.attenuation;
        }

        // And finally, normal reflectance:
        {
            let mut ray = Ray {
                origin: hit.location,
                direction: incident_ray.direction + 2.0 * cosine_theta_1 * hit.normal,
                refractive_indexes: incident_ray.refractive_indexes.clone(),
            };
            if let Some(fuzz) = hit.material.reflectance.fuzz {
                ray.direction += random_unit_vector() * fuzz;
            }
            total_light += self.trace_ray(ray, depth_left)
                * reflectance
                * (1.0 - hit.material.reflectance.diffusion.unwrap_or_default())
                * hit.material.reflectance.attenuation;
        }

        total_light
    }
}
