use super::world::{Hit, HitRecord};
use super::math::Ray;



pub struct HittableList<'a> {
    pub list: Vec<&'a dyn Hit>
}

impl Hit for HittableList<'_> {

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest = t_max;
        let mut result  = None;
        for hittable in &self.list {
            if let Some(record) = hittable.hit(ray, t_min, closest) {
                    hit_anything = true;
                    closest = record.t;
                    result = Some(record)
            }
        }
        result
    }
}