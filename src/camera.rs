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
    pub fn new(origin: Vec3,
        destination: Vec3,
        up: Vec3,
        vert_fov: f32,
        aspect_ratio: f32) -> Self {

        let theta = vert_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = half_height * aspect_ratio;

        let w = (origin - destination).into_unit();
        let u = up.cross(&w);
        let v = w.cross(&u);



        Camera {
            origin,
            lower_left: origin - (u * half_width) - (v * half_height) - w,
            horizontal: u * 2.0 * half_width,
            vertical: v * 2.0 * half_height
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin,
            self.lower_left + self.horizontal * u + self.vertical * v - self.origin)
    }
}