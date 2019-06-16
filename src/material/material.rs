use super::math::{Ray, Vec3};
use rand::{prelude::ThreadRng, Rng};

pub trait Material {
    fn scatter(&self, ray: &Ray, point: &Vec3, normal: &Vec3) -> Option<(Vec3, Ray)>;
}

pub fn random_in_normalized_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    random_in_normalized_sphere_impl(&mut rng)
}

fn random_vec(rng: &mut ThreadRng) -> Vec3 {
    Vec3(rng.gen_range(0., 1.), rng.gen_range(0., 1.), rng.gen_range(0., 1.))
}

fn random_in_normalized_sphere_impl(rng: &mut ThreadRng) -> Vec3 {
    let p = &random_vec(rng) * 2. - Vec3(1., 1., 1.);
    if p.squared_norm() >= 1. {
        random_in_normalized_sphere_impl(rng)
    } else {
        p
    }
}
