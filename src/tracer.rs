pub mod progress;
mod viewport;

use std::sync::{Arc, Mutex};

use rayon::prelude::*;
use tracing::info;

use crate::args::TracerOptions;
use crate::color::xyz::XyzColor;
use crate::math::vec::random_unit_vector;
use crate::physics::optics::hit::{Hit, HitType, Hittable};
use crate::physics::optics::material::property::Property;
use crate::physics::optics::material::transmittance::refraction::RelativeRefractiveIndex;
use crate::physics::optics::ray::Ray;
use crate::physics::units::*;
use crate::prelude::*;
use crate::scene::Scene;
use crate::tracer::progress::new_progress;
use crate::tracer::viewport::Viewport;

pub struct Tracer {
    scene: Scene,
    options: TracerOptions,
    output_width: u32,
    output_height: u32,
    viewport: Viewport,
    wavelength_step: Length,
}

impl Tracer {
    pub fn new(
        scene: Scene,
        options: TracerOptions,
        output_width: u32,
        output_height: u32,
    ) -> Self {
        let viewport = Viewport::new(&scene.camera, output_width, output_height);
        let wavelength_step =
            Length::from(830.0e-9 - 360.0e-9) / Bare::from(options.samples_per_pixel as f64);

        Self {
            options,
            scene,
            output_width,
            output_height,
            viewport,
            wavelength_step,
        }
    }

    pub fn trace(&self) -> Result<Vec<(u32, Vec<XyzColor>)>> {
        info!(self.options.samples_per_pixel);
        info!(self.options.n_max_bounces, self.options.min_hit_distance);
        info!(?self.scene.camera.location);
        info!(?self.scene.camera.look_at);
        info!(?self.scene.camera.up);
        info!(self.scene.camera.vertical_fov);
        info!(?self.viewport.dx);
        info!(?self.viewport.dy);
        info!(?self.wavelength_step);

        let mut y_indices: Vec<u32> = (0..self.output_height).collect();
        fastrand::shuffle(&mut y_indices);

        let mut rows = Vec::with_capacity(self.output_width as usize);
        let progress =
            Arc::new(Mutex::new(new_progress(self.output_height as u64, "tracing (rows)")?));

        y_indices
            .into_par_iter()
            .map(|y| {
                let row: Vec<XyzColor> = (0..self.output_width)
                    .map(|x| self.render_pixel(x, y, self.wavelength_step))
                    .collect();
                progress.lock().unwrap().inc(1);
                (y, row)
            })
            .collect_into_vec(&mut rows);

        progress.lock().unwrap().finish();
        Ok(rows)
    }

    #[inline]
    fn render_pixel(&self, x: u32, y: u32, wavelength_step: Length) -> XyzColor {
        (0..self.options.samples_per_pixel)
            .map(|i| {
                let viewport_point =
                    self.scene.camera.look_at + self.viewport.cast_random_ray(x, y);

                // Stratified random wavelength:
                let wavelength = Length::from_nanos(360.0)
                    + wavelength_step * Bare::from(i as f64 + fastrand::f64());

                let ray = Ray::by_two_points(self.scene.camera.location, viewport_point);
                let radiance = self.trace_ray(ray, wavelength, self.options.n_max_bounces);
                XyzColor::from_wavelength(wavelength) * radiance.0
            })
            .sum::<XyzColor>()
    }

    /// Trace the ray and return the resulting color.
    #[inline]
    fn trace_ray(
        &self,
        mut ray: Ray,
        wavelength: Length,
        n_bounces_left: u16,
    ) -> SpectralRadiancePerMeter {
        let distance_range = self.options.min_hit_distance..f64::INFINITY;
        let scene_emittance = self.scene.ambient_emittance.at(wavelength);

        let mut total_radiance = SpectralRadiancePerMeter::from(0.0);
        let mut total_attenuation = Bare::from(1.0);

        for _ in 0..n_bounces_left {
            if total_attenuation < Bare::from(self.options.min_attenuation) {
                break;
            }
            let hit = self
                .scene
                .surfaces
                .iter()
                .filter_map(|surface| surface.hit(&ray, &distance_range))
                .min_by(|hit_1, hit_2| hit_1.distance.total_cmp(&hit_2.distance));
            let Some(hit) = hit else {
                // The ray didn't hit anything, finish the tracing:
                total_radiance += total_attenuation * scene_emittance;
                break;
            };

            if hit.type_ == HitType::Enter && let Some(emittance) = &hit.material.emittance {
                total_radiance += total_attenuation * emittance.at(wavelength);
            }

            let cosine_theta_1 = (-hit.normal.dot(ray.direction)).min(1.0);
            assert!(
                cosine_theta_1 >= 0.0,
                "cos θ₁ = {cosine_theta_1}, normal: {:?}, ray: {:?}",
                hit.normal,
                ray.direction,
            );

            let (scattered_ray, attenuation) = if let Some((ray, attenuation)) =
                Self::trace_refraction(&ray, wavelength, &hit, cosine_theta_1)
            {
                (ray, attenuation)
            } else if let Some((ray, attenuation)) = Self::trace_diffusion(&hit, wavelength) {
                (ray, attenuation)
            } else if let Some((ray, attenuation)) =
                Self::trace_specular_reflection(&ray, wavelength, &hit, cosine_theta_1)
            {
                (ray, attenuation)
            } else {
                // There's no scattered ray (for example, the surface is not reflective nor refractive).
                break;
            };
            assert!(scattered_ray.direction.is_finite());

            total_attenuation *= attenuation;
            ray = scattered_ray;
        }

        total_radiance
    }

    /// Lambertian reflectance: <https://en.wikipedia.org/wiki/Lambertian_reflectance>.
    fn trace_diffusion(hit: &Hit, wavelength: Length) -> Option<(Ray, Bare)> {
        let Some(reflectance) = &hit.material.reflectance else { return None };
        let Some(probability) = reflectance.diffusion else { return None };
        (fastrand::f64() < probability).then(|| {
            let ray = Ray::new(hit.location, hit.normal + random_unit_vector());
            let intensity = reflectance.attenuation.at(wavelength);
            (ray, intensity)
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
        wavelength: Length,
        hit: &Hit,
        cosine_theta_1: f64,
    ) -> Option<(Ray, Bare)> {
        // Checking whether the body is dielectric:
        let Some(transmittance) = &hit.material.transmittance else { return None };

        let refractive_index = match hit.type_ {
            HitType::Enter | HitType::Refract => RelativeRefractiveIndex {
                incident: transmittance.incident_index.at(wavelength),
                refracted: transmittance.refracted_index.at(wavelength),
            },
            HitType::Leave => RelativeRefractiveIndex {
                incident: transmittance.refracted_index.at(wavelength),
                refracted: transmittance.incident_index.at(wavelength),
            },
        };

        let sin_theta_2 = refractive_index.relative() * (1.0 - cosine_theta_1.powi(2)).sqrt();
        if sin_theta_2 > Bare::from(1.0) {
            // Total internal reflection, refraction is not possible.
            return None;
        }

        if refractive_index.reflectance(cosine_theta_1) > Bare::from(fastrand::f64()) {
            // Reflectance wins.
            return None;
        }

        // Shell's law:
        let mu = f64::from(refractive_index.relative());
        let direction = {
            let cosine_theta_2 = (Bare::from(1.0) - sin_theta_2.powi::<2>()).sqrt();
            mu * incident_ray.direction
                + hit.normal * (mu * cosine_theta_1 - f64::from(cosine_theta_2))
        };
        let ray = Ray::new(hit.location, direction);

        let mut intensity = transmittance.attenuation.at(wavelength);
        if hit.type_ == HitType::Leave && let Some(coefficient) = transmittance.coefficient {
            // Hit from inside, apply the possible exponential decay coefficient:
            intensity *= (Length::from(-hit.distance) * coefficient.at(wavelength)).exp();
        }

        Some((ray, intensity))
    }

    /// Specular reflection: <https://en.wikipedia.org/wiki/Specular_reflection>.
    fn trace_specular_reflection(
        incident_ray: &Ray,
        wavelength: Length,
        hit: &Hit,
        cosine_theta_1: f64,
    ) -> Option<(Ray, Bare)> {
        let Some(reflectance) = &hit.material.reflectance else { return None };
        let mut ray =
            Ray::new(hit.location, incident_ray.direction + 2.0 * cosine_theta_1 * hit.normal);
        if let Some(fuzz) = reflectance.fuzz {
            ray.direction += random_unit_vector() * fuzz;
        }
        let intensity = reflectance.attenuation.at(wavelength);
        Some((ray, intensity))
    }
}
