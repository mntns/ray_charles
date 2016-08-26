extern crate nalgebra as na;
use na::Vector3;

#[derive(Clone, Copy)]
pub struct Ray {
  pub origin: Vector3<f64>,
  pub direction: Vector3<f64>
}

impl Ray {
  pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
    Ray { origin: origin, direction: direction }
  }
  pub fn point_at_parameter(&self, t: f64) -> Vector3<f64> {
    (self.origin + t*self.direction)
  }
}
