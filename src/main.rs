extern crate rand;


use rand::Rng;

use math::{Ray, Vec3};

use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::shapes::{HittableList, Sphere};
use crate::world::{Camera, Hit};

mod math;
mod world;
mod shapes;
mod material;


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


fn random_scene(range: i32) -> HittableList {
    let mut chooser = rand::thread_rng();
    let mut list = HittableList { list: vec![] };
    list.list.push(Box::new(Sphere { center: Vec3(0., -1000., 0.), radius: 1000., material: Box::new(Lambertian::new(Vec3(0.5, 0.8, 0.5)))}));
    for a in -range..range {
        for b in -range..range {
            let choice = chooser.gen_range(0, 3);
            let center = Vec3(a as f32 + 0.9 * chooser.gen_range(0., 1.), 0.2, b as f32 + 0.9 * chooser.gen_range(0., 1.));
            if (&center - Vec3(4., 0.2, 0.)).norm() > 0.9 {
                let material: Box<dyn Material> = match choice {
                    0 => Box::new(Lambertian::new(Vec3(chooser.gen_range(0., 1.), chooser.gen_range(0., 1.), chooser.gen_range(0., 1.)))),
                    1 => Box::new(Metal::new(Vec3((chooser.gen_range(0., 1.) + 1.) / 2., (chooser.gen_range(0., 1.) + 1.) / 2., (chooser.gen_range(0., 1.) + 1.) / 2.), 0.)),
                    2 => Box::new(Dielectric::new(1.5)),
                    _otherwise => unreachable!()
                };
                list.list.push(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material,
                }))
            }
        }
    }
    list.list.push(Box::new(Sphere { center: Vec3(0., 1., 0.), radius: 1., material: Box::new(Dielectric::new(1.5)) }));
    list.list.push(Box::new(Sphere { center: Vec3(-4., 1., 0.), radius: 1., material: Box::new(Lambertian::new(Vec3(0.4, 0.2, 0.1))) }));
    list.list.push(Box::new(Sphere { center: Vec3(4., 1., 0.), radius: 1., material: Box::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.)) }));
    list
}


fn main() {
    let nx = 1024;
    let ny = 512;
    let ns = 20;
    println!("P3\n{} {}\n255", nx, ny);
    let look_from = Vec3(11., 3., 7.);
    let look_at = Vec3(0., 0., -1.);
    let focus_dist = (&look_from - &look_at).norm();
    let aperture = 0.1;
    let camera = Camera::new(look_from, look_at, Vec3(0., 1., 0.), 20., nx as f32 / ny as f32, aperture, focus_dist);
    let hits = random_scene(4);
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
