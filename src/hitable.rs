extern crate nalgebra as na;
use na::{Vector3};
use ray::Ray;

pub trait Hitable: {
    fn hit(&self, r: &Ray, t_min: &f64, t_max: &f64, hit_record: &mut HitRecord) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>
}

impl HitRecord {
    pub fn new(t: f64, p: Vector3<f64>, normal: Vector3<f64>) -> Self {
        let hit_record = HitRecord { t: t, p: p, normal: normal };
        hit_record
    }
}

