extern crate nalgebra as na;
use na::{Vector3};
use ray::Ray;
use std::sync::Arc;
use material::Material;

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: &f64, t_max: &f64) -> Option<HitRecord>;
    fn material(&self) -> Arc<Material>;
}

#[derive(Debug, Copy, Clone)]
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

