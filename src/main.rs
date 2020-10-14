#![allow(dead_code)]

use std::fs::File;
use std::io::{ BufWriter, Write };

struct Vec3 {
    data: [f32; 3]
}

impl Vec3 {
    fn new_zero() -> Self {
        Vec3 {
            data: [0.0 ;3]
        }
    }

    fn new_all(v: f32) -> Self {
        Vec3 {
            data: [v; 3]
        }
    }

    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 {
            data: [x, y, z]
        }
    }

    fn x(&self) -> f32 {
        self.data[0]
    }

    fn y(&self) -> f32 {
        self.data[1]
    }

    fn z(&self) -> f32 {
        self.data[2]
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = 200;
    let height = 100;

    let file = File::create("output.ppm")?;
    let mut out = BufWriter::new(file);

    write!(&mut out, "P3\n{} {}\n255\n", width, height)?;
    for j in (0..height).rev() {
        for i in 0..width {
            let col = Vec3::new(i as f32 / width as f32, j as f32 / height as f32, 0.2);
            write!(&mut out, "{} {} {}\n",
                (col.x() * 255.9) as u8,
                (col.y() * 255.9) as u8,
                (col.z() * 255.9) as u8)?;
        }
    }

    Ok(())
}
