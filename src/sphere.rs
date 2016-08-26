extern crate nalgebra as na;
use na::{Vector3, Dot};
use ray::Ray;
use hitable::Hitable;
use hitable::HitRecord;

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64) -> Self {
        Sphere { center: center, radius: radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;

        let a: f64 = r.direction.dot(&r.direction);
        let b: f64 = oc.dot(&r.direction);
        let c: f64 = oc.dot(&oc) - self.radius*self.radius;

        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < *t_max && temp > *t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < *t_max && temp > *t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }
        return false;
    }
}
