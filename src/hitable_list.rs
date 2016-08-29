use ray::Ray;
use std::sync::Arc;
use hitable::Hitable;
use hitable::HitRecord;
use material::Material;

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
    pub fn hit(&self, r: &Ray, t_min: &f64, t_max: &f64) -> Option<(HitRecord, Arc<Material>)> {
        let mut closest_so_far = t_max.clone();
        let mut last_hit: Option<(HitRecord, Arc<Material>)> = None;

        for object in &self.list {
            if let Some(record) = object.hit(&r, &t_min, &closest_so_far) {
                closest_so_far = record.t;
                last_hit = Some((record, object.material()))
            }
        }
        last_hit
    }
}
