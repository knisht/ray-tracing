use std::f32::consts::PI;

use super::math::{Ray, Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Self {
        let theta = vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;
        let w = Vec3::normalized(&(&look_from - look_at));
        let u = Vec3::normalized(&vup.cross(&w));
        let v = w.cross(&u);
        Camera {
            lower_left_corner: &look_from - &u * half_width - &v * half_height - w,
            origin: look_from,
            vertical: &v * (half_height * 2.),
            horizontal: &u * (half_width * 2.),
        }
    }

    pub fn ray_at(&self, u: f32, v: f32) -> Ray {
        Ray { origin: self.origin.clone(), direction: &self.lower_left_corner + &self.horizontal * u + &self.vertical * v - &self.origin }
    }
}