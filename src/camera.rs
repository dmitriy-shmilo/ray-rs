use crate::vec3::Vec3;
use crate::ray::Ray;
use std::f32::consts::PI;

pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(vert_fov: f32, aspect_ratio: f32) -> Self {
        let theta = vert_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = half_height * aspect_ratio;


        Camera {
            origin: Vec3::new_zero(),
            lower_left: Vec3::new(-half_width, -half_height, -1.0),
            horizontal: Vec3::new(2.0 * half_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0 * half_height, 0.0)
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin,
            self.lower_left + self.horizontal * u + self.vertical * v - self.origin)
    }
}