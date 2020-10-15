use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hit::HitRecord;
use rand::Rng;

pub trait Material : MaterialClone {
    fn scatter(&self,
        ray: &Ray,
        rec: &HitRecord,
        atten: &mut Vec3,
        scatter: &mut Ray) -> bool;
}

pub trait MaterialClone {
    fn clone_box(&self) -> Box<dyn Material>;
}

impl <T: 'static + Material + Clone> MaterialClone for T {
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Lambert {
    albedo: Vec3
}

impl Lambert {
    pub fn new(albedo: Vec3) -> Self {
        Lambert {
            albedo
        }
    }
}

impl Material for Lambert {

    fn scatter(&self,
        _: &Ray,
        rec: &HitRecord,
        atten: &mut Vec3,
        scatter: &mut Ray) -> bool {

        let target = rec.p + rec.normal + rnd_in_sphere();
        *scatter = Ray::new(rec.p, target - rec.p);
        *atten = self.albedo;
        true
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz > 1.0 { 1.0 } else { fuzz }
        }
    }
}

impl Material for Metal {

    fn scatter(&self,
        ray: &Ray,
        rec: &HitRecord,
        atten: &mut Vec3,
        scatter: &mut Ray) -> bool {

        let reflected = reflect(&ray.direction().into_unit(), &rec.normal);
        *scatter = Ray::new(rec.p, reflected + rnd_in_sphere() * self.fuzz);
        *atten = self.albedo;
        scatter.direction().dot(&rec.normal) > 0.0
    }
}

// TODO: implement and benchmark a different point picking algo
fn rnd_in_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut result;
    let offset = Vec3::new_all(1.0);

    loop {
        result = Vec3::new(rng.gen(), rng.gen(), rng.gen()) * 2.0 - offset;
        if result.len_sq() < 1.0 {
            break;
        }
    }

    result
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * v.dot(n) * 2.0
}