use rand::random;

use super::{reflect, refract, schlick};
use super::Material;
use super::math::{Ray, Vec3};

pub struct Dielectric {
    ref_index: f32,
}

impl Dielectric {
    pub fn new(ref_index: f32) -> Self {
        Dielectric { ref_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, point: &Vec3, normal: &Vec3) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&ray.direction, normal);
        let attenuation = Vec3(1., 1., 1.);
        let (outward_normal, quotient, cosine) = if ray.direction.dot(normal) > 0. {
            let temp_cos = self.ref_index * ray.direction.dot(normal) / ray.direction.norm();
            (-normal, self.ref_index, temp_cos)
        } else {
            let temp_cos = -ray.direction.dot(normal) / ray.direction.norm();
            (normal.clone(), 1. / self.ref_index, temp_cos)
        };
        let direction = match refract(&ray.direction, &outward_normal, quotient) {
            Some(refracted) => {
                let reflect_prob = schlick(cosine, self.ref_index);
                if random::<f32>() < reflect_prob {
                    reflected
                } else {
                    refracted
                }
            }
            None => reflected
        };
        Some((attenuation, Ray { origin: point.clone(), direction }))
    }
}