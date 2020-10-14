
use std::ops::{ Mul, Div, Add, Sub, Index };
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    data: [f32; 3]
}

impl Vec3 {
    pub fn new_zero() -> Self {
        Vec3 {
            data: [0.0 ;3]
        }
    }

    pub fn new_all(v: f32) -> Self {
        Vec3 {
            data: [v; 3]
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 {
            data: [x, y, z]
        }
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn z(&self) -> f32 {
        self.data[2]
    }

    pub fn r(&self) -> f32 {
        self.data[0]
    }

    pub fn g(&self) -> f32 {
        self.data[1]
    }

    pub fn b(&self) -> f32 {
        self[2]
    }

    pub fn len(&self) -> f32 {
        (self.data[0] * self.data[0]
            + self.data[1] * self.data[1]
            + self.data[2] * self.data[2])
        .sqrt()
    }

    pub fn len_sq(&self) -> f32 {
        self[0] * self[0]
            + self[1] * self[1]
            + self[2] * self[2]
    }

    pub fn into_unit(&self) -> Vec3 {
        let len = self.len();
        Vec3::new(self[0] / len, self[1] / len, self[2] / len)
    }

    pub fn dot(&self, other:&Vec3) -> f32 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn cross(&self, other:&Vec3) -> Vec3 {
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