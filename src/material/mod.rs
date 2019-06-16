pub use material::{ random_in_normalized_sphere, Material};
pub use lambertian::Lambertian;
pub use metal::Metal;

use super::math;

mod material;
mod lambertian;
mod metal;
