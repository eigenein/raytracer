use glam::DVec3;
use image::{Rgba, RgbaImage};
use indicatif::ProgressBar;

use crate::ray::Ray;
use crate::scene::Scene;

pub fn render(scene: &Scene, into: &mut RgbaImage) {
    let progress = ProgressBar::new(into.width() as u64 * into.height() as u64);
    let eye_position = DVec3::new(0.0, 0.0, -scene.viewport.focal_length);

    for x in 0..into.width() {
        let viewport_x = ((x as f64 + 0.5) / into.width() as f64 - 0.5) * scene.viewport.width;
        for y in 0..into.height() {
            let viewport_y =
                ((y as f64 + 0.5) / into.height() as f64 - 0.5) * scene.viewport_height();
            let direction = eye_position - DVec3::new(viewport_x, viewport_y, 0.0);
            let ray = Ray {
                origin: eye_position,
                direction,
            };
            let pixel = trace_ray(&ray, scene);
            into.put_pixel(x, y, pixel);
            progress.inc(1);
        }
    }

    progress.finish_and_clear();
}

#[inline]
fn trace_ray(ray: &Ray, in_: &Scene) -> Rgba<u8> {
    for body in &in_.surfaces {
        if let Some(normal) = body.hit_by(ray) {
            return Rgba::from([
                (normal.x * 255.0) as u8,
                (normal.y * 255.0) as u8,
                (normal.z * 255.0) as u8,
                255,
            ]);
        }
    }
    Rgba::from([0, 0, 0, 255])
}
