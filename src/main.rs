mod math;
mod world;
mod shapes;
use math::Vec3;
use math::Ray;
use crate::world::Hit;
use crate::shapes::{HittableList, Sphere};

fn color(ray : &Ray, hittable : &Hit) -> Vec3 {
    match hittable.hit(ray, 0., std::f32::INFINITY) {
        Some(record) => {
            &(record.normal + Vec3(1., 1., 1.)) * 0.5
        }
        None => {
            let direction = Vec3::normalized(&ray.direction);
            let t = 0.5 * (direction.y() + 1.);
            &(&Vec3(1., 1., 1.) * (1. - t))  + &(&Vec3(0.5, 0.7, 1.) * t)
        }
    }
}


fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let lower_left_corner = Vec3(-2., -1., -1.);
    let horizontal = Vec3(4., 0., 0.);
    let vertical = Vec3(0., 2., 0.);
    let origin = Vec3(0., 0., 0.);
    let s1 = Sphere {center: Vec3(0., 0., -1.), radius: 0.5};
    let s2 = Sphere {center: Vec3(0., -100.5, -1.), radius: 100.};
    let hits = HittableList{list: vec![&s1, &s2]};
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let ray = Ray { origin: origin.clone(), direction: (&lower_left_corner) + &(&horizontal * u) + &(&vertical * v)};
            let color = &color(&ray, &hits) * 255.99;
            let ir = color.r() as i32;
            let ig = color.g() as i32;
            let ib = color.b() as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
