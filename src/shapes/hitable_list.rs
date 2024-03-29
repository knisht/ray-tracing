use super::world::{Hit, HitRecord};
use super::math::Ray;



pub struct HittableList {
    pub list: Vec<Box<dyn Hit>>
}

impl Hit for HittableList {

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut result  = None;
        for hittable in &self.list {
            if let Some(record) = hittable.hit(ray, t_min, closest) {
                    closest = record.t;
                    result = Some(record)
            }
        }
        result
    }
}