#![allow(dead_code)]

mod vec3;
mod ray;
mod hit;
mod sphere;

use vec3::Vec3;
use ray::Ray;
use hit::{ Hit, HitList, HitRecord };
use sphere::Sphere;
use std::fs::File;
use std::io::{ BufWriter, Write };



fn color(ray: &Ray, world:&HitList) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(&ray, 0.0, f32::MAX, &mut rec) {
        return Vec3::new(rec.normal.x() + 1.0,
            rec.normal.y() + 1.0,
            rec.normal.z() + 1.0) * 0.5
    }

    let dir = ray.direction().into_unit();
    let t = 0.5 * (dir.y() + 1.0);
    Vec3::new_all(1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut world = HitList::new();
    world.append(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.append(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let width = 200;
    let height = 100;

    let file = File::create("output.ppm")?;
    let mut out = BufWriter::new(file);

    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new_zero();

    write!(&mut out, "P3\n{} {}\n255\n", width, height)?;
    for j in (0..height).rev() {
        for i in 0..width {
            let u = i as f32 / width as f32;
            let v = j as f32 / height as f32;
            let ray = Ray::new(origin, lower_left + horizontal * u + vertical * v);
            let col = color(&ray, &world);
            write!(&mut out, "{} {} {}\n",
                (col.x() * 255.9) as u8,
                (col.y() * 255.9) as u8,
                (col.z() * 255.9) as u8)?;
        }
    }

    Ok(())
}
