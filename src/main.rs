extern crate rand;

use rand::Rng;

use math::{Ray, Vec3};
//use rand::Rng;

use crate::shapes::{HittableList, Sphere};
use crate::world::{Camera, Hit};

mod math;
mod world;
mod shapes;

fn color(ray: &Ray, hittable: &Hit) -> Vec3 {
    match hittable.hit(ray, 0., std::f32::INFINITY) {
        Some(record) => {
            &(record.normal + Vec3(1., 1., 1.)) * 0.5
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
    let lower_left_corner = Vec3(-2., -1., -1.);
    let horizontal = Vec3(4., 0., 0.);
    let vertical = Vec3(0., 2., 0.);
    let origin = Vec3(0., 0., 0.);
    let s1 = Sphere { center: Vec3(0., 0., -1.), radius: 0.5 };
    let s2 = Sphere { center: Vec3(0., -100.5, -1.), radius: 100. };
    let hits = HittableList { list: vec![&s1, &s2] };
    let camera = Camera::new();
    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color_vec = Vec3::new();
            for s in 0..ns {
                let u = (rng.gen_range(0., 1.) + i as f32) / nx as f32;
                let v = (rng.gen_range(0., 1.) + j as f32) / ny as f32;
                let ray = camera.ray_at(u, v);
                color_vec += &color(&ray, &hits);
            }
            color_vec *= (255.99 / ns as f32);
            let ir = color_vec.r() as i32;
            let ig = color_vec.g() as i32;
            let ib = color_vec.b() as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
