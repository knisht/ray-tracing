use super::math::Vec3;
use super::math::Ray;

pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}