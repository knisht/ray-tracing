use super::math::{Vec3, Ray};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new() -> Self {
        Camera { origin: Vec3(0., 0., 0.), lower_left_corner: Vec3(-2., -1., -1.), vertical: Vec3(0., 2., 0.), horizontal: Vec3(4., 0., 0.)}
    }

    pub fn ray_at(&self, u : f32, v: f32) -> Ray {
        Ray {origin : self.origin.clone(), direction: &self.lower_left_corner + &self.horizontal * u + &self.vertical * v}
    }
}