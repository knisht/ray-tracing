use super::Material;
use super::random_in_normalized_sphere;
use super::math::{Ray, Vec3};

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }

}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, point: &Vec3, normal: &Vec3) -> Option<(Vec3, Ray)> {
        let target = point + normal + random_in_normalized_sphere();
        let scattered = Ray { origin: point.clone(), direction: target - point };
        let attenuation = self.albedo.clone();
        Some((attenuation, scattered))
    }
}