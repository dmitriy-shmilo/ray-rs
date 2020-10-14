use crate::hit::{ Hit, HitRecord};
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere {
            center,
            radius
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let diff = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = diff.dot(&ray.direction());
        let c = diff.dot(&diff) - self.radius * self.radius;
        let d = b * b - a * c;

        if d > 0.0 {
            let t = (-b - d.sqrt()) / a;

            if t < t_max && t > t_min {
                rec.t = t;
                rec.p = ray.point_at(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }

            let t = (-b + d.sqrt()) / a;
            if t < t_max && t > t_min {
                rec.t = t;
                rec.p = ray.point_at(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }

        false
    }
}