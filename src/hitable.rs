extern crate nalgebra as na;
use na::{Vector3};
use ray::Ray;

pub trait Hitable {
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &'a mut HitRecord) -> bool;
    //fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>
}

impl HitRecord {
    pub fn new(t: f64, p: Vector3<f64>, normal: Vector3<f64>) -> Self {
        HitRecord { t: t, p: p, normal: normal }
    }
}

