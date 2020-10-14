#![allow(dead_code)]

use std::fs::File;
use std::io::{ BufWriter, Write };
use std::ops::{ Mul, Div, Add, Sub, Index };
use std::fmt;

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

    fn r(&self) -> f32 {
        self.data[0]
    }

    fn g(&self) -> f32 {
        self.data[1]
    }

    fn b(&self) -> f32 {
        self[2]
    }

    fn len(&self) -> f32 {
        (self.data[0] * self.data[0]
            + self.data[1] * self.data[1]
            + self.data[2] * self.data[2])
        .sqrt()
    }

    fn len_sq(&self) -> f32 {
        self[0] * self[0]
            + self[1] * self[1]
            + self[2] * self[2]
    }

    fn into_unit(&self) -> Vec3 {
        let len = self.len();
        Vec3::new(self[0] / len, self[1] / len, self[2] / len)
    }

    fn dot(&self, other:&Vec3) -> f32 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    fn cross(&self, other:&Vec3) -> Vec3 {
        Vec3::new(self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0])
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2])
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2])
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3::new(self[0] * scalar, self[1] * scalar, self[2] * scalar)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self[0] * other[0], self[1] * other[1], self[2] * other[2])
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f32) -> Vec3 {
        Vec3::new(self[0] / scalar, self[1] / scalar, self[2] / scalar)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(self[0] / other[0], self[1] / other[1], self[2] / other[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32{
        &self.data[i]
    }
}

impl Index<i32> for Vec3 {
    type Output = f32;

    fn index(&self, i: i32) -> &f32{
        &self.data[i as usize]
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.data[0], self.data[1], self.data[2])
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
