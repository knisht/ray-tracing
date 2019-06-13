use super::Vec3;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new() -> Ray {
        Ray { origin: Vec3::new(), direction: Vec3::new() }
    }

    pub fn from(start: Vec3, direction: Vec3) -> Self {
        Ray { origin: start, direction }
    }

    pub fn point(&self, t: f32) -> Vec3 {
        (&self.origin) + (&self.direction) * t
    }
}