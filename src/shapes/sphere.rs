use super::math::{Ray, Vec3};
use super::world::{Hit, HitRecord};
use super::material::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let bias = &ray.origin - &self.center;
        let a = ray.direction.squared_norm();
        let b = bias.dot(&ray.direction);
        let c = bias.squared_norm() - self.radius.powi(2);
        let discr = b.powi(2) - a * c;
        if discr > 0. {
            let temp = (-b - (discr.sqrt())) / a;
            if temp < t_max && temp > t_min {
                let point = ray.point(temp);
                let normal = &(&point - &self.center) * self.radius.powi(-1);
                return Some(HitRecord { t: temp, point, normal, material: self.material.as_ref() });
            }
            let temp = (-b + discr.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = ray.point(temp);
                let normal = &(&point - &self.center) * self.radius.powi(-1);
                return Some(HitRecord { t: temp, point, normal, material: self.material.as_ref() });
            }
        }
        None
    }
}

