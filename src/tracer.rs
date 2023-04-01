use fastrand::Rng;
use glam::DVec3;
use itertools::iproduct;
use tracing::info;

use crate::args::TracerOptions;
use crate::hit::{Hit, Hittable};
use crate::math::random_unit_vector;
use crate::prelude::*;
use crate::progress::new_progress;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::viewport::Viewport;

pub struct Tracer {
    pub scene: Scene,
    pub options: TracerOptions,
    rng: Rng,
}

impl Tracer {
    pub fn new(scene: Scene, options: TracerOptions) -> Self {
        Self {
            options,
            scene,
            rng: Rng::new(),
        }
    }

    pub fn trace(&self, output_width: u32, output_height: u32) -> Result<Vec<DVec3>> {
        info!(self.options.samples_per_pixel, self.options.max_depth);
        info!(?self.scene.camera.location);
        info!(?self.scene.camera.look_at);
        info!(?self.scene.camera.up);
        info!(self.scene.camera.vertical_fov);

        let viewport = Viewport::new(&self.scene.camera, output_width, output_height);
        info!(?viewport.dx);
        info!(?viewport.dy);

        let progress = new_progress(output_height as u64, "tracing (rows)")?;
        let mut pixels = Vec::with_capacity(output_width as usize * output_height as usize);

        for (y, x) in iproduct!(0..output_height, 0..output_width) {
            let color = (0..self.options.samples_per_pixel)
                .map(|_| {
                    let viewport_point = self.scene.camera.look_at + viewport.cast_random_ray(x, y);
                    let ray = Ray::by_two_points(self.scene.camera.location, viewport_point);
                    self.trace_ray(&ray, self.options.max_depth)
                })
                .sum::<DVec3>()
                / self.options.samples_per_pixel as f64;
            pixels.push(color);
            progress.set_position(y as u64);
        }
        progress.finish();

        Ok(pixels)
    }

    /// Trace the ray and return the resulting color.
    #[inline]
    fn trace_ray(&self, ray: &Ray, depth_left: u16) -> DVec3 {
        // TODO: shadow acne problem, make an option.
        let distance_range = 0.000001..f64::INFINITY;

        let hit = self
            .scene
            .surfaces
            .iter()
            .filter_map(|surface| surface.hit(ray, &distance_range))
            .min_by(|hit_1, hit_2| hit_1.distance.total_cmp(&hit_2.distance));

        match hit {
            Some(hit) if depth_left != 0 => {
                // The ray hit a surface, scatter the ray:
                self.trace_scattered_ray(ray, &hit, depth_left - 1)
            }
            Some(_) => DVec3::ZERO,           // the depth limit is reached
            None => self.scene.ambient_color, // the ray didn't hit anything
        }
    }

    /// Trace a scattered ray and return amount of light.
    ///
    /// Notes:
    ///
    /// - The incident ray **must** be normalized.
    fn trace_scattered_ray(&self, incident_ray: &Ray, hit: &Hit, depth_left: u16) -> DVec3 {
        let emittance = if hit.from_outside {
            // TODO: possibly multiply by `cosine_theta_1`.
            // TODO: possibly divide by the distance squared.
            hit.material.emittance
        } else {
            DVec3::ZERO
        };

        if let Some(light) = self.trace_diffusion(incident_ray, hit, depth_left) {
            return emittance + light;
        }

        let cosine_theta_1 = -hit.normal.dot(incident_ray.direction);
        assert!(cosine_theta_1 >= 0.0);

        if let Some(light) = self.trace_refraction(incident_ray, hit, cosine_theta_1, depth_left) {
            return emittance + light;
        }

        emittance + self.trace_normal_reflection(incident_ray, hit, cosine_theta_1, depth_left)
    }

    /// Trace a possible diffused ray.
    ///
    /// # Returns
    ///
    /// Amount of light returned by the scattered ray, or `None` if no diffused ray was traced.
    ///
    /// # See also
    ///
    /// - Lambertian reflectance: <https://en.wikipedia.org/wiki/Lambertian_reflectance>
    fn trace_diffusion(&self, incident_ray: &Ray, hit: &Hit, depth_left: u16) -> Option<DVec3> {
        let Some(probability) = hit.material.reflectance.diffusion else { return None };

        if fastrand::f64() < probability {
            let ray = Ray::new(
                hit.location,
                hit.normal + random_unit_vector(&self.rng),
                incident_ray.refractive_indexes.clone(),
            );
            Some(self.trace_ray(&ray, depth_left) * hit.material.reflectance.attenuation)
        } else {
            None
        }
    }

    /// Trace a possible refraction.
    ///
    /// # See also
    ///
    /// - Shell's law in vector form: <https://physics.stackexchange.com/a/436252/11966>
    //  - Shell's law in vector form: <https://en.wikipedia.org/wiki/Snell%27s_law#Vector_form>
    fn trace_refraction(
        &self,
        incident_ray: &Ray,
        hit: &Hit,
        cosine_theta_1: f64,
        depth_left: u16,
    ) -> Option<DVec3> {
        // Checking whether the body is dielectric:
        let Some(transmittance) = &hit.material.transmittance else { return None };

        let current_refractive_index = incident_ray
            .current_refractive_index()
            .unwrap_or(self.scene.refractive_index);

        // Check whether we're entering a new medium, or leaving the current medium:
        let mu = if hit.from_outside {
            // Entering the new medium:
            current_refractive_index / transmittance.refractive_index
        } else {
            // Leaving the current medium:
            let outer_refractive_index = incident_ray
                .outer_refractive_index()
                .unwrap_or(self.scene.refractive_index);
            transmittance.refractive_index / outer_refractive_index
        };

        let sin_theta_2 = mu * (1.0 - cosine_theta_1.powi(2)).sqrt();
        if sin_theta_2 > 1.0 {
            // Total internal reflection, refraction is not possible.
            return None;
        }

        let reflectance = {
            // Schlick's approximation for reflectance:
            let r0 = ((current_refractive_index - transmittance.refractive_index)
                / (current_refractive_index + transmittance.refractive_index))
                .powi(2); // FIXME
            r0 + (1.0 - r0) * (1.0 - cosine_theta_1).powi(5)
        };
        if reflectance > fastrand::f64() {
            // Reflectance wins.
            return None;
        }

        // Shell's law:
        let direction = {
            let cosine_theta_2 = (1.0 - sin_theta_2.powi(2)).sqrt();
            mu * incident_ray.direction + hit.normal * (mu * cosine_theta_1 - cosine_theta_2)
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
            Ray::new(hit.location, direction, refractive_indexes)
        };

        let mut transmitted_light = self.trace_ray(&ray, depth_left);
        if !hit.from_outside {
            // Hit from inside, apply the possible exponential decay coefficient:
            if let Some(coefficient) = transmittance.coefficient {
                transmitted_light *= (-hit.distance * coefficient).exp();
            }
        }
        let attenuation = transmittance
            .attenuation
            .unwrap_or(hit.material.reflectance.attenuation);
        Some(transmitted_light * attenuation)
    }

    fn trace_normal_reflection(
        &self,
        incident_ray: &Ray,
        hit: &Hit,
        cosine_theta_1: f64,
        depth_left: u16,
    ) -> DVec3 {
        let mut ray = Ray::new(
            hit.location,
            incident_ray.direction + 2.0 * cosine_theta_1 * hit.normal,
            incident_ray.refractive_indexes.clone(),
        );
        if let Some(fuzz) = hit.material.reflectance.fuzz {
            ray.direction += random_unit_vector(&self.rng) * fuzz;
        }
        self.trace_ray(&ray, depth_left) * hit.material.reflectance.attenuation
    }
}
