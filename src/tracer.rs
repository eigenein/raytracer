pub mod progress;
mod viewport;

use std::sync::{Arc, Mutex};

use rayon::prelude::*;
use tracing::info;

use crate::args::TracerOptions;
use crate::color::xyz::XyzColor;
use crate::math::vec3::Vec3;
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
}

impl Tracer {
    const MAX_WAVELENGTH: Length = Quantity::from_nanos(830.0);
    const MIN_WAVELENGTH: Length = Quantity::from_nanos(360.0);
    const SPECTRUM_WIDTH: Length = Self::MAX_WAVELENGTH - Self::MIN_WAVELENGTH;

    pub fn new(
        scene: Scene,
        options: TracerOptions,
        output_width: u32,
        output_height: u32,
    ) -> Self {
        let viewport = Viewport::new(&scene.camera, output_width, output_height);

        Self {
            options,
            scene,
            output_width,
            output_height,
            viewport,
        }
    }

    pub fn trace(&self) -> Result<Vec<(u32, Vec<XyzColor>)>> {
        info!(self.options.samples_per_pixel);
        info!(self.options.n_max_bounces, self.options.min_hit_distance);
        info!(%self.scene.camera.location);
        info!(%self.scene.camera.look_at);
        info!(%self.scene.camera.up);
        info!(self.scene.camera.vertical_fov);
        info!(%self.viewport.dx);
        info!(%self.viewport.dy);

        let mut y_indices: Vec<u32> = (0..self.output_height).collect();
        fastrand::shuffle(&mut y_indices);

        let mut rows = Vec::with_capacity(self.output_width as usize);
        let progress =
            Arc::new(Mutex::new(new_progress(self.output_height as u64, "tracing (rows)")?));

        y_indices
            .into_par_iter()
            .map(|y| {
                let row: Vec<XyzColor> = (0..self.output_width)
                    .map(|x| self.render_pixel(x, y))
                    .collect();
                progress.lock().unwrap().inc(1);
                (y, row)
            })
            .collect_into_vec(&mut rows);

        progress.lock().unwrap().finish();
        Ok(rows)
    }

    #[inline]
    fn render_pixel(&self, x: u32, y: u32) -> XyzColor {
        (0..self.options.samples_per_pixel)
            .map(|_| {
                let viewport_point =
                    self.scene.camera.look_at + self.viewport.cast_random_ray(x, y);
                let wavelength = Self::MIN_WAVELENGTH + Self::SPECTRUM_WIDTH * Bare::fastrand();
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

            let (scattered_ray, attenuation) =
                if let Some((ray, attenuation)) = Self::trace_refraction(&ray, wavelength, &hit) {
                    (ray, attenuation)
                } else if let Some((ray, attenuation)) = Self::trace_diffusion(&hit, wavelength) {
                    (ray, attenuation)
                } else if let Some((ray, attenuation)) =
                    Self::trace_specular_reflection(&ray, wavelength, &hit)
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

        if fastrand::f64() < probability {
            let ray = Ray::new(hit.location, hit.normal + Vec3::random_unit_vector());
            let attenuation = reflectance.attenuation.at(wavelength);
            Some((ray, attenuation))
        } else {
            None
        }
    }

    /// Trace a possible refraction.
    ///
    /// # See also
    ///
    /// - Shell's law in vector form: <https://physics.stackexchange.com/a/436252/11966>
    /// - Shell's law in vector form: <https://en.wikipedia.org/wiki/Snell%27s_law#Vector_form>
    fn trace_refraction(incident_ray: &Ray, wavelength: Length, hit: &Hit) -> Option<(Ray, Bare)> {
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

        let cosine_theta_1 = (-hit.normal.dot(incident_ray.direction)).min(1.0);
        assert!(cosine_theta_1 >= 0.0);

        let sin_theta_2 = refractive_index.relative().0 * (1.0 - cosine_theta_1.powi(2)).sqrt();
        if sin_theta_2 > 1.0 {
            // Total internal reflection, refraction is not possible.
            return None;
        }

        if refractive_index.reflectance(cosine_theta_1) > Bare::fastrand() {
            // Reflectance wins.
            return None;
        }

        // Shell's law:
        let direction = {
            let cosine_theta_2 = (1.0 - sin_theta_2.powi(2)).sqrt();
            let mu = refractive_index.relative().0;
            mu * incident_ray.direction + hit.normal * (mu * cosine_theta_1 - cosine_theta_2)
        };
        let ray = Ray::new(hit.location, direction);

        let mut attenuation = transmittance.attenuation.at(wavelength);
        if hit.type_ == HitType::Leave && let Some(coefficient) = transmittance.coefficient {
            // Hit from inside, apply the possible exponential decay coefficient:
            attenuation *= (Length::from(-hit.distance) * coefficient.at(wavelength)).exp();
        }

        Some((ray, attenuation))
    }

    /// Specular reflection: <https://en.wikipedia.org/wiki/Specular_reflection>.
    fn trace_specular_reflection(
        incident_ray: &Ray,
        wavelength: Length,
        hit: &Hit,
    ) -> Option<(Ray, Bare)> {
        let Some(reflectance) = &hit.material.reflectance else { return None };
        let mut ray = Ray::new(hit.location, incident_ray.direction.reflect_about(hit.normal));
        if let Some(fuzz) = reflectance.fuzz {
            ray.direction += Vec3::random_unit_vector() * fuzz;
        }
        let attenuation = reflectance.attenuation.at(wavelength);
        Some((ray, attenuation))
    }
}
