pub use material::{ random_in_normalized_sphere, Material};
use material::{reflect, refract, schlick};
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use dielectric::Dielectric;

use super::math;

mod material;
mod lambertian;
mod metal;
mod dielectric;