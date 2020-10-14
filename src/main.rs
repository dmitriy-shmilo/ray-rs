#![allow(dead_code)]

mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;
use std::fs::File;
use std::io::{ BufWriter, Write };

fn hit_sphere(center: Vec3, radius: f32, ray:&Ray) -> f32 {
    let diff = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = diff.dot(&ray.direction()) * 2.0;
    let c = diff.dot(&diff) - radius * radius;
    let d = b * b - 4.0 * a * c;
    if d < 0.0 {
        -1.0
    } else {
        (-b - d.sqrt()) / 2.0 / a
    }
}

fn color(ray: &Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0,0.0,-1.0), 0.5, ray);
    if t > 0.0 {
        let n = (ray.point_at(t) - Vec3::new(0.0, 0.0, -1.0)).into_unit();
        return Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
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
