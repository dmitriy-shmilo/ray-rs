#![allow(dead_code)]

mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;
use std::fs::File;
use std::io::{ BufWriter, Write };

fn hit_sphere(center: Vec3, radius: f32, ray:&Ray) -> bool {
    let diff = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = diff.dot(&ray.direction()) * 2.0;
    let c = diff.dot(&diff) - radius * radius;
    let d = b * b - 4.0 * a * c;
    d > 0.0
}

fn color(ray: &Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0,0.0,-1.0), 0.5, ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let dir = ray.direction().into_unit();
    let t = 0.5 * (dir.y() + 1.0);
    Vec3::new_all(1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
            let col = color(&ray);
            write!(&mut out, "{} {} {}\n",
                (col.x() * 255.9) as u8,
                (col.y() * 255.9) as u8,
                (col.z() * 255.9) as u8)?;
        }
    }

    Ok(())
}
