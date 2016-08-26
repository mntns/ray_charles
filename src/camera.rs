extern crate nalgebra as na;
use na::{Vector3};
use std::f64;
use ray::Ray;

pub struct Camera {
  pub origin: Vector3<f64>,
  pub lower_left_corner: Vector3<f64>,
  pub horizontal: Vector3<f64>,
  pub vertical: Vector3<f64>
}

impl Camera {
  pub fn new(origin: Vector3<f64>, lower_left_corner: Vector3<f64>, horizontal: Vector3<f64>, vertical: Vector3<f64>) -> Self {
    Camera { 
      origin: origin,
      lower_left_corner: lower_left_corner,
      horizontal: horizontal,
      vertical: vertical
    }
  }
  pub fn get_ray(&self, u: f64, v: f64) -> Ray {
    return Ray::new(self.origin, (self.lower_left_corner + u*self.horizontal +  v*self.vertical - self.origin));
  }
}

