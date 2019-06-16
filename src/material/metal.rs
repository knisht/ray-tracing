use std::cmp::min;

use super::Material;
use super::random_in_normalized_sphere;
use super::math::{Ray, Vec3};

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Metal { albedo, fuzz: if fuzz < 1. {fuzz} else {1.} }
    }

    fn reflect(falling: &Vec3, normal: &Vec3) -> Vec3 {
        falling - &(normal * falling.dot(normal)) * 2.
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, point: &Vec3, normal: &Vec3) -> Option<(Vec3, Ray)> {
        let reflected = Self::reflect(&Vec3::normalized(&ray.direction), normal);
        let scattered = Ray { origin: point.clone(), direction: reflected + &random_in_normalized_sphere() * self.fuzz };
        let attenuation = self.albedo.clone();
        if scattered.direction.dot(normal) > 0. {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}