use rand::{prelude::ThreadRng, Rng};

use super::math::{Ray, Vec3};

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


pub fn reflect(falling: &Vec3, normal: &Vec3) -> Vec3 {
    falling - &(normal * falling.dot(normal)) * 2.
}

pub fn refract(falling: &Vec3, normal: &Vec3, up_to_down_quotient: f32) -> Option<Vec3> {
    let normalized_falling = Vec3::normalized(falling);
    let cos = normalized_falling.dot(normal);
    let discr = 1. - up_to_down_quotient.powi(2) * (1. - cos.powi(2));
    if discr > 0. {
        Some(&(normalized_falling - (normal * cos)) * up_to_down_quotient - normal * discr.sqrt())
    } else {
        None
    }
}

pub fn schlick(cosine: f32, ref_index: f32) -> f32 {
    let r0 = ((1. - ref_index) / (1. + ref_index)).powi(2);
    r0 + (1. - r0)*(1. - cosine).powi(5)
}