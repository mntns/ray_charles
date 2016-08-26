extern crate nalgebra as na;
use na::{Vector3};
use ray::Ray;
use std::sync::Arc;
use hitable::Hitable;
use hitable::HitRecord;


pub struct HitableList {
  pub list: Vec<Arc<Hitable>>,
}

impl HitableList {
    pub fn new() -> Self {
        let list = Vec::new();
        HitableList { list: list }
    }
    pub fn push(&mut self, sphere: Arc<Hitable>) {
        self.list.push(sphere);
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max.clone();
        let mut temp_rec = HitRecord::new(0.0, Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 0.0));

        for object in &self.list {
            if object.hit(&r, &t_min, &closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        return hit_anything;
    }
}
