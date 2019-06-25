use std::f32::consts::PI;

use rand::prelude::ThreadRng;
use rand::Rng;

use super::math::{Ray, Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Self {
        let theta = vfov * PI / 180.;

        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;
        let w = Vec3::normalized(&(&look_from - look_at));
        let u = Vec3::normalized(&vup.cross(&w));
        let v = w.cross(&u);
        Camera {
            lower_left_corner: &look_from - &(&u * half_width + &v * half_height + &w) * focus_dist,
            origin: look_from,
            vertical: &v * (half_height * 2.* focus_dist),
            horizontal: &u * (half_width * 2. * focus_dist),
            lens_radius: aperture / 2.,
            v,
            u,
        }
    }

    pub fn ray_at(&self, u: f32, v: f32) -> Ray {
        let rd = &Self::random_in_normalized_disc() * self.lens_radius;
        let offset = &self.u * rd.x()  + &self.v * rd.y() ;
        Ray { origin: &self.origin + &offset, direction: &self.lower_left_corner + &self.horizontal * u + &self.vertical * v - &self.origin - &offset }
    }

    pub fn random_in_normalized_disc() -> Vec3 {
        let mut rng = rand::thread_rng();
        Self::random_in_normalized_disc_impl(&mut rng)
    }

    fn random_vec(rng: &mut ThreadRng) -> Vec3 {
        Vec3(rng.gen_range(0., 1.), rng.gen_range(0., 1.), 0.)
    }

    fn random_in_normalized_disc_impl(rng: &mut ThreadRng) -> Vec3 {
        let p = &Self::random_vec(rng) * 2. - Vec3(1., 1., 0.);
        if p.squared_norm() >= 1. {
            Self::random_in_normalized_disc_impl(rng)
        } else {
            p
        }
    }
}