pub mod bvh;
pub mod progress;
mod viewport;

use std::sync::{Arc, Mutex};

use fastrand::Rng;
use rayon::prelude::*;
use tracing::info;

use crate::args::TracerOptions;
use crate::color::xyz::XyzColor;
use crate::math::hit::*;
use crate::math::ray::Ray;
use crate::math::sequence::*;
use crate::math::vec2::Vec2;
use crate::math::vec3::Vec3;
use crate::physics::optics::material::emittance::Emittance;
use crate::physics::optics::material::property::Property;
use crate::physics::optics::material::transmittance::refraction::RelativeRefractiveIndex;
use crate::physics::units::*;
use crate::prelude::*;
use crate::scene::Camera;
use crate::surface::Surface;
use crate::tracer::bvh::Bvh;
use crate::tracer::progress::new_progress;
use crate::tracer::viewport::Viewport;

pub struct Tracer<'a> {
    bvh: Bvh<'a, Surface>,
    ambient_emittance: Emittance,
    camera: Camera,
    options: TracerOptions,
    output_width: u32,
    output_height: u32,
    viewport: Viewport,
}

impl<'a> Tracer<'a> {
    const MAX_WAVELENGTH: Length = Quantity::from_nanos(830.0);
    const MIN_WAVELENGTH: Length = Quantity::from_nanos(360.0);
    const SPECTRUM_WIDTH: Length = Quantity(Self::MAX_WAVELENGTH.0 - Self::MIN_WAVELENGTH.0);

    pub fn new(
        bvh: Bvh<'a, Surface>,
        ambient_emittance: Emittance,
        camera: Camera,
        options: TracerOptions,
        output_width: u32,
        output_height: u32,
    ) -> Self {
        let viewport = Viewport::new(&camera, output_width, output_height);

        Self {
            bvh,
            ambient_emittance,
            camera,
            options,
            output_width,
            output_height,
            viewport,
        }
    }

    pub fn trace(&self) -> Result<Vec<(u32, Vec<XyzColor>)>> {
        info!(self.options.n_samples_per_pixel);
        info!(self.options.n_max_bounces, self.options.min_hit_distance);
        info!(%self.camera.location);
        info!(%self.camera.look_at);
        info!(%self.camera.up);
        info!(self.camera.vertical_fov);
        info!(%self.viewport.dx);
        info!(%self.viewport.dy);

        let mut y_indices: Vec<u32> = (0..self.output_height).collect();
        fastrand::shuffle(&mut y_indices);

        let mut rows = Vec::with_capacity(self.output_width as usize);
        let progress =
            Arc::new(Mutex::new(new_progress(self.output_height as u64, "tracing rows")?));

        y_indices
            .into_par_iter()
            .map(|y| {
                let mut rng = Rng::new();
                let row: Vec<XyzColor> = (0..self.output_width)
                    .map(|x| self.render_pixel(x, y, &mut rng))
                    .collect();
                progress.lock().unwrap().inc(1);
                (y, row)
            })
            .collect_into_vec(&mut rows);

        progress.lock().unwrap().finish();
        Ok(rows)
    }

    #[inline]
    fn render_pixel(&self, x: u32, y: u32, rng: &mut Rng) -> XyzColor {
        let mut subpixel_sequence = Halton2::new(5, 3).offset(Vec2::new(rng.f64(), rng.f64()));
        let mut wavelength_sequence = VanDerCorput::new(2);
        let mut diffusion_sequence = RandomSequence::new();
        let mut effect_check_sequence = RandomSequence::new();

        (0..self.options.n_samples_per_pixel)
            .map(|_| {
                let ray = {
                    let subpixel = subpixel_sequence.next();
                    let viewport_point =
                        self.camera.look_at + self.viewport.cast_ray(x, y, subpixel);
                    Ray::with_two_points(self.camera.location, viewport_point)
                };
                let wavelength = Self::MIN_WAVELENGTH
                    + Self::SPECTRUM_WIDTH * Bare::from(wavelength_sequence.next());
                let density = self.trace_ray(
                    ray,
                    wavelength,
                    self.options.n_max_bounces,
                    &mut effect_check_sequence,
                    &mut diffusion_sequence,
                );
                XyzColor::from_wavelength(wavelength) * density.0
            })
            .sum::<XyzColor>()
    }

    /// Trace the ray and return the resulting color.
    #[inline]
    fn trace_ray(
        &self,
        mut ray: Ray,
        wavelength: Length,
        n_bounces_left: u32,
        effect_check_sequence: &mut impl Sequence<f64>,
        diffusion_sequence: &mut impl Sequence<Vec2>,
    ) -> SpectralFluxDensity {
        let distance_range = self.options.min_hit_distance..f64::INFINITY;
        let scene_emittance = self.ambient_emittance.at(wavelength);

        let mut total_flux_density = SpectralFluxDensity::ZERO;
        let mut total_attenuation = Bare::from(1.0);

        for _ in 0..n_bounces_left {
            if total_attenuation < Bare::from(self.options.min_attenuation) {
                break;
            }
            let hit = self.bvh.hit(&ray, &distance_range, effect_check_sequence);
            let Some(hit) = hit else {
                // The ray didn't hit anything, finish the tracing:
                total_flux_density += total_attenuation * scene_emittance;
                break;
            };

            if hit.type_ == HitType::Enter && let Some(emittance) = &hit.material.emittance {
                total_flux_density += total_attenuation * emittance.at(wavelength);
            }

            let (scattered_ray, attenuation) = if let Some((ray, attenuation)) =
                Self::trace_refraction(&ray, wavelength, &hit, effect_check_sequence)
            {
                (ray, attenuation)
            } else if let Some((ray, attenuation)) =
                Self::trace_diffusion(&hit, wavelength, effect_check_sequence, diffusion_sequence)
            {
                (ray, attenuation)
            } else if let Some((ray, attenuation)) =
                Self::trace_specular_reflection(&ray, wavelength, &hit, diffusion_sequence)
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

        total_flux_density
    }

    /// Trace [Lambertian reflectance][1].
    ///
    /// [1]: https://en.wikipedia.org/wiki/Lambertian_reflectance
    fn trace_diffusion(
        hit: &Hit,
        wavelength: Length,
        effect_check_sequence: &mut impl Sequence<f64>,
        diffusion_sequence: &mut impl Sequence<Vec2>,
    ) -> Option<(Ray, Bare)> {
        let Some(reflectance) = &hit.material.reflectance else {
            // The material is not reflective.
            return None;
        };
        let Some(probability) = reflectance.diffusion else {
            // The reflectiveness is not diffuse.
            return None;
        };
        if effect_check_sequence.next() > probability {
            // Diffusion is not applied this time.
            return None;
        }

        let ray = Ray::new(hit.location, hit.normal + Vec3::sample_unit_vector(diffusion_sequence));
        // The «length / 2» accounts for its reflected intensity in the ray's direction (the max length is 1 + 1).
        let attenuation = reflectance.attenuation.at(wavelength) * ray.direction.length() / 2.0;
        Some((ray, attenuation))
    }

    /// Trace a possible refraction using [Snell's law][1] in [vector form][2].
    ///
    /// [1]: https://en.wikipedia.org/wiki/Snell%27s_law#Vector_form
    /// [2]: https://physics.stackexchange.com/a/436252/11966
    fn trace_refraction(
        incident_ray: &Ray,
        wavelength: Length,
        hit: &Hit,
        effect_check_sequence: &mut impl Sequence<f64>,
    ) -> Option<(Ray, Bare)> {
        // Checking whether the body is dielectric:
        let Some(transmittance) = &hit.material.transmittance else {
            return None;
        };

        let refractive_index = match hit.type_ {
            HitType::Enter => RelativeRefractiveIndex {
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

        if refractive_index.reflectance(cosine_theta_1) > Bare::from(effect_check_sequence.next()) {
            // Reflectance wins.
            return None;
        }

        // Snell's law:
        let direction = {
            let cosine_theta_2 = (1.0 - sin_theta_2.powi(2)).sqrt();
            let mu = refractive_index.relative().0;
            mu * incident_ray.direction + hit.normal * (mu * cosine_theta_1 - cosine_theta_2)
        };
        let ray = Ray::new(hit.location, direction);

        let attenuation = if hit.type_ == HitType::Leave {
            // Hit from inside, apply the exponential decay coefficient:
            (Length::from(-hit.distance) * transmittance.attenuation_coefficient.at(wavelength))
                .exp()
        } else {
            Bare::ONE
        };

        Some((ray, attenuation))
    }

    /// Trace [specular reflection][1].
    ///
    /// [1]: https://en.wikipedia.org/wiki/Specular_reflection
    fn trace_specular_reflection(
        incident_ray: &Ray,
        wavelength: Length,
        hit: &Hit,
        diffusion_sequence: &mut impl Sequence<Vec2>,
    ) -> Option<(Ray, Bare)> {
        let Some(reflectance) = &hit.material.reflectance else {
            return None;
        };
        let mut ray = Ray::new(hit.location, incident_ray.direction.reflect_about(hit.normal));
        if let Some(fuzz) = reflectance.fuzz {
            ray.direction =
                (ray.direction + Vec3::sample_unit_vector(diffusion_sequence) * fuzz).normalize();
        }
        let attenuation = reflectance.attenuation.at(wavelength);
        Some((ray, attenuation))
    }
}
