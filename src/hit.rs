use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::{ Material, Lambert };
use std::vec::Vec;

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Box<dyn Material>
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            t: 0.0,
            p: Vec3::new_zero(),
            normal: Vec3::new_zero(),
            material: Box::new(Lambert::new(Vec3::new_zero()))
        }
    }
}

pub struct HitList {
    list: Vec<Box<dyn Hit>>
}

impl HitList {
    pub fn new() -> Self {
        HitList {
            list: Vec::new()
        }
    }

    pub fn append(&mut self, body: Box<dyn Hit>) {
        self.list.push(body)
    }
}

impl Hit for HitList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp = HitRecord::new();
        let mut any_hit = false;
        let mut closest = t_max;

        for hit in &self.list {
            if hit.hit(ray, t_min, closest, &mut temp) {
                any_hit = true;
                closest = temp.t;
            }
        }

        if any_hit {
            *rec = temp;
        }

        any_hit
    }
}