extern crate rand;

use rand::prelude::ThreadRng;
use rand::Rng;

use math::{Ray, Vec3};

use crate::material::{Lambertian, Metal};
use crate::shapes::{HittableList, Sphere};
use crate::world::{Camera, Hit};

mod math;
mod world;
mod shapes;
mod material;

fn random_vec(rng: &mut ThreadRng) -> Vec3 {
    Vec3(rng.gen_range(0., 1.), rng.gen_range(0., 1.), rng.gen_range(0., 1.))
}

fn random_in_normalized_sphere(rng: &mut ThreadRng) -> Vec3 {
    let p = &random_vec(rng) * 2. - Vec3(1., 1., 1.);
    if p.squared_norm() >= 1. {
        random_in_normalized_sphere(rng)
    } else {
        p
    }
}


fn color(ray: &Ray, hittable: &Hit, depth: i32) -> Vec3 {
    match hittable.hit(ray, 0.001, std::f32::INFINITY) {
        Some(record) => {
            if depth >= 50 {
                return Vec3::new();
            } else {
                match record.material.scatter(ray, &record.point, &record.normal) {
                    Some((attenuation, scattered)) => attenuation * color(&scattered, hittable, depth + 1),
                    None => Vec3::new()
                }
            }
        }
        None => {
            let direction = Vec3::normalized(&ray.direction);
            let t = 0.5 * (direction.y() + 1.);
            &(&Vec3(1., 1., 1.) * (1. - t)) + &(&Vec3(0.5, 0.7, 1.) * t)
        }
    }
}


fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let s1 = Sphere { center: Vec3(0., 0., -1.), radius: 0.5, material: &Lambertian::new(Vec3(0.8, 0.3, 0.3)) };
    let s2 = Sphere { center: Vec3(0., -100.5, -1.), radius: 100., material: &Lambertian::new(Vec3(0.8, 0.8, 0.)) };
    let s3 = Sphere { center: Vec3(1., 0., -1.), radius: 0.5, material: &Metal::new(Vec3(0.8, 0.6, 0.2), 0.3) };
    let s4 = Sphere { center: Vec3(-1., 0., -1.), radius: 0.5, material: &Metal::new(Vec3(0.8, 0.8, 0.8), 1.) };
    let hits = HittableList { list: vec![&s1, &s2, &s3, &s4] };
    let camera = Camera::new();
    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color_vec = Vec3::new();
            for _s in 0..ns {
                let u = (rng.gen_range(0., 1.) + i as f32) / nx as f32;
                let v = (rng.gen_range(0., 1.) + j as f32) / ny as f32;
                let ray = camera.ray_at(u, v);
                color_vec += &color(&ray, &hits, 0);
            }
            color_vec *= 1. / ns as f32;
            let color_vec = &Vec3(color_vec.0.sqrt(), color_vec.1.sqrt(), color_vec.2.sqrt()) * 255.99;
            let ir = color_vec.r() as i32;
            let ig = color_vec.g() as i32;
            let ib = color_vec.b() as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
