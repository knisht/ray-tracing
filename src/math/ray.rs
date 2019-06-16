use super::Vec3;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point(&self, t: f32) -> Vec3 {
        (&self.origin) + (&self.direction) * t
    }
}