use crate::vec3::Vec3;
use crate::ray::Ray;
use std::f32::consts::PI;
use rand::Rng;

pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32
}

impl Camera {
    pub fn new(origin: Vec3,
        destination: Vec3,
        up: Vec3,
        vert_fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32) -> Self {

        let theta = vert_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = half_height * aspect_ratio;

        let w = (origin - destination).into_unit();
        let u = up.cross(&w).into_unit();
        let v = w.cross(&u);

        Camera {
            origin,
            lower_left: origin
                - (u * half_width * focus_dist)
                - (v * half_height * focus_dist)
                - (w * focus_dist),
            horizontal: u * 2.0 * half_width * focus_dist,
            vertical: v * 2.0 * half_height * focus_dist,
            u,
            v,
            w,
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = rnd_in_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(self.origin + offset,
            self.lower_left 
            + self.horizontal * s 
            + self.vertical * t 
            - self.origin 
            - offset)
    }
}

// TODO: implement and benchmark a different point picking algo
fn rnd_in_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut result;
    let offset = Vec3::new(1.0, 1.0, 0.0);

    loop {
        result = Vec3::new(rng.gen(), rng.gen(), 0.0) * 2.0 - offset;
        if result.dot(&result) < 1.0 {
            return result;
        }
    }
}