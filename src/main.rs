#![allow(dead_code)]

mod vec3;
mod ray;
mod hit;
mod sphere;
mod camera;
mod material;

use vec3::Vec3;
use ray::Ray;
use hit::{ Hit, HitList, HitRecord };
use sphere::Sphere;
use camera::Camera;
use material::{ Lambert, Metal };
use std::fs::File;
use std::io::{ BufWriter, Write };
use rand::Rng;

fn color(ray: &Ray, world:&HitList, depth: u8) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(&ray, 0.001, f32::MAX, &mut rec) {
        let mut scatter = Ray::new(Vec3::new_zero(), Vec3::new_zero());
        let mut atten = Vec3::new_zero();

        if depth < 50 && rec.material.scatter(ray, &rec, &mut atten, &mut scatter) {
            return atten * color(&scatter, world, depth + 1);
        }

        return Vec3::new_zero();
    }

    let dir = ray.direction().into_unit();
    let t = 0.5 * (dir.y() + 1.0);
    Vec3::new_all(1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut world = HitList::new();
    world.append(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(Metal::new(Vec3::new(0.4, 0.1, 0.7), 0.1)))));
    world.append(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0),
            0.3,
            Box::new(Metal::new(Vec3::new(0.4, 0.9, 0.7), 0.6)))));
    world.append(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(Lambert::new(Vec3::new(0.1, 0.1, 0.1))))));

    let width = 200;
    let height = 100;
    let aa_rays = 50;
    let camera = Camera::new();

    let mut rng = rand::thread_rng();

    let file = File::create("output.ppm")?;
    let mut out = BufWriter::new(file);

    write!(&mut out, "P3\n{} {}\n255\n", width, height)?;
    for j in (0..height).rev() {
        for i in 0..width {
            let mut col = Vec3::new_zero();
            for _ in 0..aa_rays {
                let u = (i as f32 + rng.gen::<f32>()) / width as f32;
                let v = (j as f32 + rng.gen::<f32>()) / height as f32;
                let ray = camera.get_ray(u, v);

                // TODO: implement AddAssign for Vec3
                col = col + color(&ray, &world, 0);
            }
            // TODO: implement DivAssign for Vec3
            col = col / aa_rays as f32;

            col = Vec3::new(col.r().sqrt(),
                col.g().sqrt(),
                col.b().sqrt());
            
            write!(&mut out, "{} {} {}\n",
                (col.r() * 255.99) as u8,
                (col.g() * 255.99) as u8,
                (col.b() * 255.99) as u8)?;
        }
    }

    Ok(())
}
