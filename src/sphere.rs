extern crate nalgebra as na;
use na::{Vector3, Dot};
use ray::Ray;
use std::sync::Arc;
use hitable::Hitable;
use hitable::HitRecord;
use material::Material;

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    material: Arc<Material>,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: Arc<Material>) -> Self {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl Hitable for Sphere {
    fn material(&self) -> Arc<Material> {
        self.material.clone()
    }
    fn hit(&self, r: &Ray, t_min: &f64, t_max: &f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;

        let a: f64 = r.direction.dot(&r.direction);
        let b: f64 = 2.0 * r.direction.dot(&oc);
        let c: f64 = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant <= 0.0 {
            None
        } else {
            let ds = discriminant.sqrt();
            let temp_neg = (-b - ds) / (2.0 * a);
            let temp_pos = (-b + ds) / (2.0 * a);
            let temp = if temp_neg < *t_max && temp_neg > *t_min {
                Some(temp_neg)
            } else if temp_pos < *t_max && temp_pos > *t_min {
                Some(temp_pos)
            } else {
                None
            };

            match temp {
                Some(temp) => {
                    let point = r.point_at_parameter(temp);
                    let normal = (point - self.center) / self.radius;
                    let hit = HitRecord::new(temp, point, normal);
                    Some(hit)
                }
                None => None,
            }
        }
    }
}
