extern crate nalgebra as na;
use na::{Vector3};
use ray::Ray;

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

pub struct HitRecord {
    pub t: f64,
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>
}
