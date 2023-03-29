use std::ops::Range;

use glam::DVec3;
use itertools::iproduct;
use tracing::info;

use crate::args::TracerOptions;
use crate::constants::LIGHT_SPEED;
use crate::hit::Hit;
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

        let progress = new_progress(output_height as u64 * output_width as u64, "tracing")?;
        for (y, x) in iproduct!(0..output_height, 0..output_width) {
            let color = (0..self.options.samples_per_pixel)
                .map(|_| {
                    let image_x = x as f64 - half_image_width + fastrand::f64() - 0.5;
                    let image_y = y as f64 - half_image_height + fastrand::f64() - 0.5;
                    let viewport_point = viewport_center + image_x * x_vector + image_y * y_vector;
                    self.trace_ray(
                        Ray::by_two_points(eye_position, viewport_point),
                        self.options.max_depth,
                        &(0.0..f64::INFINITY),
                    )
                })
                .sum::<DVec3>();
            pixels.push(color);
            progress.inc(1);
        }
        progress.finish();

        Ok(pixels)
    }

    /// Trace the ray and return the resulting color.
    #[inline]
    fn trace_ray(&self, mut ray: Ray, n_depth_left: u16, time_range: &Range<f64>) -> DVec3 {
        ray.direction = LIGHT_SPEED * ray.direction.normalize();
        self.scene
            .surfaces
            .iter()
            .filter_map(|surface| surface.hit(&ray, time_range))
            .min_by(|hit_1, hit_2| hit_1.time.total_cmp(&hit_2.time))
            .map_or(self.scene.ambient_color, |hit| {
                if n_depth_left != 0 {
                    self.trace_secondary_rays(&ray, &hit, n_depth_left - 1)
                } else {
                    DVec3::ZERO
                }
            })
    }

    #[inline]
    fn trace_secondary_rays(&self, incident_ray: &Ray, hit: &Hit, n_depth_left: u16) -> DVec3 {
        let time_range = (0.001 / LIGHT_SPEED)..f64::INFINITY; // FIXME: shadow acne problem.
        let mut color_sum = DVec3::ZERO;

        let (reflected_ray, reflectance, refracted_ray) = incident_ray.refract_and_reflect(
            hit,
            hit.material.refractive_index,
            hit.material.reflective_fuzz,
            hit.material.diffusion_probability,
        );

        // TODO: since `refract_and_reflect()` would return a single randomly chosen ray,
        // TODO: kill the recursion: the incident ray could just get transformed in a loop.
        let attenuation = hit.material.albedo * hit.material.attenuation;
        color_sum +=
            self.trace_ray(reflected_ray, n_depth_left, &time_range) * reflectance * attenuation;
        if let Some(refracted_ray) = refracted_ray {
            color_sum += self.trace_ray(refracted_ray, n_depth_left, &time_range)
                * (1.0 - reflectance)
                * attenuation;
        }

        color_sum
    }
}
