use super::math::{Ray, Vec3};
use super::world::{Hit, HitRecord};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    fn new() -> Self {
        Sphere { center: Vec3::new(), radius: 0. }
    }
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
                return Some(HitRecord { t: temp, point, normal });
            }
            let temp = (-b + discr.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = ray.point(temp);
                let normal = &(&point - &self.center) * self.radius.powi(-1);
                return Some(HitRecord { t: temp, point, normal });
            }
        }
        None
    }
}

