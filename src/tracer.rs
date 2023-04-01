use fastrand::Rng;
use glam::DVec3;
use smallvec::{smallvec, SmallVec};
use tracing::info;

use crate::args::TracerOptions;
use crate::hit::{Hit, HitType, Hittable};
use crate::math::random_unit_vector;
use crate::prelude::*;
use crate::progress::new_progress;
use crate::ray::Ray;
use crate::refraction::RefractiveIndex;
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

    pub fn trace(&self, output_width: u32, output_height: u32) -> Result<Vec<(u32, u32, DVec3)>> {
        info!(
            self.options.samples_per_pixel,
            self.options.n_max_bounces, self.options.min_hit_distance
        );
        info!(?self.scene.camera.location);
        info!(?self.scene.camera.look_at);
        info!(?self.scene.camera.up);
        info!(self.scene.camera.vertical_fov);

        let viewport = Viewport::new(&self.scene.camera, output_width, output_height);
        info!(?viewport.dx);
        info!(?viewport.dy);

        let progress = new_progress(output_height as u64, "tracing (rows)")?;
        let mut pixels = Vec::with_capacity(output_width as usize * output_height as usize);

        let mut y_indices: Vec<u32> = (0..output_height).collect();
        fastrand::shuffle(&mut y_indices);

        for y in y_indices {
            for x in 0..output_width {
                let color = (0..self.options.samples_per_pixel)
                    .map(|_| {
                        let viewport_point =
                            self.scene.camera.look_at + viewport.cast_random_ray(x, y);
                        let ray = Ray::by_two_points(self.scene.camera.location, viewport_point);
                        self.trace_ray(ray, self.options.n_max_bounces)
                    })
                    .sum::<DVec3>()
                    / self.options.samples_per_pixel as f64;
                pixels.push((x, y, color));
            }
            progress.inc(1);
        }
        progress.finish();

        Ok(pixels)
    }

    /// Trace the ray and return the resulting color.
    #[inline]
    fn trace_ray(&self, mut ray: Ray, n_bounces_left: u16) -> DVec3 {
        let distance_range = self.options.min_hit_distance..f64::INFINITY;

        let mut refractive_indexes = smallvec![self.scene.refractive_index];
        let mut total_emitted = DVec3::ZERO;
        let mut total_attenuation = DVec3::ONE;

        for _ in 0..n_bounces_left {
            let hit = self
                .scene
                .surfaces
                .iter()
                .filter_map(|surface| surface.hit(&ray, &distance_range))
                .min_by(|hit_1, hit_2| hit_1.distance.total_cmp(&hit_2.distance));
            let Some(hit) = hit else {
                // The ray didn't hit anything, finish the tracing:
                total_emitted += total_attenuation * self.scene.ambient_color;
                break;
            };

            let cosine_theta_1 = -hit.normal.dot(ray.direction);
            assert!(cosine_theta_1 >= 0.0);

            let (scattered_ray, attenuation) = if let Some((ray, attenuation)) =
                Self::trace_refraction(&ray, &hit, cosine_theta_1, &mut refractive_indexes)
            {
                (ray, attenuation)
            } else if let Some((ray, attenuation)) = self.trace_diffusion(&hit) {
                (ray, attenuation)
            } else {
                self.trace_specular_reflection(&ray, &hit, cosine_theta_1)
            };

            if hit.type_ == HitType::Enter {
                total_emitted += total_attenuation * hit.material.emittance;
            }
            total_attenuation *= attenuation;
            ray = scattered_ray;
        }

        total_emitted
    }

    /// Lambertian reflectance: <https://en.wikipedia.org/wiki/Lambertian_reflectance>.
    fn trace_diffusion(&self, hit: &Hit) -> Option<(Ray, DVec3)> {
        let Some(probability) = hit.material.reflectance.diffusion else { return None };
        (fastrand::f64() < probability).then(|| {
            let ray = Ray::new(hit.location, hit.normal + random_unit_vector(&self.rng));
            (ray, hit.material.reflectance.attenuation)
        })
    }

    /// Trace a possible refraction.
    ///
    /// # See also
    ///
    /// - Shell's law in vector form: <https://physics.stackexchange.com/a/436252/11966>
    /// - Shell's law in vector form: <https://en.wikipedia.org/wiki/Snell%27s_law#Vector_form>
    fn trace_refraction(
        incident_ray: &Ray,
        hit: &Hit,
        cosine_theta_1: f64,
        refractive_indexes: &mut SmallVec<[f64; 4]>,
    ) -> Option<(Ray, DVec3)> {
        // Checking whether the body is dielectric:
        let Some(transmittance) = &hit.material.transmittance else { return None };

        let refractive_index = match hit.type_ {
            HitType::Enter => RefractiveIndex {
                incident: *refractive_indexes.last().unwrap(),
                refracted: transmittance.refractive_index,
            },
            HitType::Leave => RefractiveIndex {
                incident: transmittance.refractive_index,
                refracted: refractive_indexes[refractive_indexes.len() - 2],
            },
        };

        let sin_theta_2 = refractive_index.relative() * (1.0 - cosine_theta_1.powi(2)).sqrt();
        if sin_theta_2 > 1.0 {
            // Total internal reflection, refraction is not possible.
            return None;
        }

        if refractive_index.reflectance(cosine_theta_1) > fastrand::f64() {
            // Reflectance wins.
            return None;
        }

        // Refraction wins, update the refractive index stack:
        if hit.type_ == HitType::Enter {
            refractive_indexes.push(transmittance.refractive_index);
        } else if hit.type_ == HitType::Leave {
            refractive_indexes.pop();
        };

        // Shell's law:
        let mu = refractive_index.relative();
        let direction = {
            let cosine_theta_2 = (1.0 - sin_theta_2.powi(2)).sqrt();
            mu * incident_ray.direction + hit.normal * (mu * cosine_theta_1 - cosine_theta_2)
        };
        let ray = Ray::new(hit.location, direction);

        let mut attenuation = transmittance
            .attenuation
            .unwrap_or(hit.material.reflectance.attenuation);
        if hit.type_ == HitType::Leave && let Some(coefficient) = transmittance.coefficient {
            // Hit from inside, apply the possible exponential decay coefficient:
            attenuation *= (-hit.distance * coefficient).exp();
        }

        Some((ray, attenuation))
    }

    /// Specular reflection: <https://en.wikipedia.org/wiki/Specular_reflection>.
    fn trace_specular_reflection(
        &self,
        incident_ray: &Ray,
        hit: &Hit,
        cosine_theta_1: f64,
    ) -> (Ray, DVec3) {
        let mut ray =
            Ray::new(hit.location, incident_ray.direction + 2.0 * cosine_theta_1 * hit.normal);
        if let Some(fuzz) = hit.material.reflectance.fuzz {
            ray.direction += random_unit_vector(&self.rng) * fuzz;
        }
        (ray, hit.material.reflectance.attenuation)
    }
}
