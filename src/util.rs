extern crate nalgebra as na;
extern crate rand;
extern crate backtrace;

use na::{Vector3, Dot};
use rand::Rng;

pub fn unit_vector(vec: Vector3<f64>) -> Vector3<f64> {
    let len = vec.len() as f64;
    Vector3::new(vec.x / len, vec.y / len, vec.z / len)
}

pub fn random_in_unit_sphere() -> Vector3<f64> {
    let mut p = Vector3::new(0.0, 0.0, 0.0);
    let mut rng = rand::thread_rng();
    while na::norm_squared(&p) >= 1.0 {
        p = 2.0 * Vector3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) -
            Vector3::new(1.0, 1.0, 1.0);
    }
    return p;
}

pub fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    v - (2.0 * v.dot(&n) * n)
}

pub fn refract(v: Vector3<f64>, n: Vector3<f64>, ni_over_nt: f64) -> Option<Vector3<f64>> {
    let uv = unit_vector(v);
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

pub fn schlick(cosine: f64, eta: f64) -> f64 {
    let r0 = (1.0 - eta) / (1.0 + eta);
    let r0_squared = r0 * r0;
    r0_squared + (1.0 - r0_squared) * (1.0 - cosine).powi(5)
}
