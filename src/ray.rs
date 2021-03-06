use crate::vec3::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3
}

impl Ray {

    pub fn new(a: Vec3, b: Vec3) -> Self {
        Ray {
            a,
            b
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
        self.a + self.b * t
    }
}