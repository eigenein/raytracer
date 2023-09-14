use std::f64::consts::FRAC_PI_2;

use crate::math::vec2::Vec2;
use crate::math::vec3::Vec3;
use crate::scene::Camera;

pub struct Viewport {
    pub dx: Vec3,
    pub dy: Vec3,

    image_half_size: Vec2,
}

impl Viewport {
    /// Calculate and return the viewport's `dx` and `dy` vectors,
    /// which represent how much space the image pixel takes in the scene world.
    ///
    /// The resulting vectors are relative to the camera direction point.
    pub fn new(camera: &Camera, image_width: u32, image_height: u32) -> Self {
        let image_height = image_height as f64;

        let principal_axis = camera.location - camera.look_at;
        let focal_length = principal_axis.length();
        let principal_axis = principal_axis / focal_length;

        let dx = principal_axis.cross(camera.up).normalize();
        let dy = dx.rotate_about(principal_axis, FRAC_PI_2);

        // Finally, scale the vectors to the actual field-of-view angle:
        let viewport_height = 2.0 * focal_length * (camera.vertical_fov / 2.0).to_radians().sin();
        let scale = viewport_height / image_height;

        Self {
            dx: dx * scale,
            dy: dy * scale,
            image_half_size: Vec2::new(image_width as f64 / 2.0, image_height / 2.0),
        }
    }

    /// Calculate the viewport point based on the image coordinates.
    #[inline]
    pub fn at(&self, image_point: Vec2) -> Vec3 {
        image_point.x * self.dx + image_point.y * self.dy
    }

    /// Cast a ray to the specified image pixel coordinates and return the viewport vector.
    ///
    /// # Notes
    ///
    /// You still **need** to add the resulting vector to the «look at» point.
    #[inline]
    pub fn cast_ray(&self, image_x: u32, image_y: u32, subpixel: Vec2) -> Vec3 {
        self.at(Vec2::new(image_x, image_y) - self.image_half_size + subpixel)
    }
}
