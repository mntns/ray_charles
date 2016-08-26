extern crate nalgebra as na;
extern crate rand;
extern crate backtrace;

use na::{Vector3};
use std::f64;
use rand::{Rng};
use std::thread;
use std::sync::Arc;
mod ray;
use ray::Ray;
mod sphere;
use sphere::Sphere;
mod hitable;
use hitable::Hitable;
use hitable::HitRecord;
mod hitable_list;
use hitable_list::HitableList;
mod camera;
use camera::Camera;

// Helper functions
fn unit_vector(vec: Vector3<f64>) -> Vector3<f64> {
    let len = vec.len() as f64;
    Vector3::new(vec.x / len, vec.y / len, vec.z / len)
}

fn random_in_unit_sphere(rng: &mut rand::ThreadRng) -> Vector3<f64> {
  let mut p = Vector3::new(0.0, 0.0, 0.0);
  while na::norm_squared(&p) >= 1.0 {
    p = 2.0*Vector3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) - Vector3::new(1.0, 1.0, 1.0);
  }
  return p;
}
    
fn color(r: &Ray, world: &HitableList) -> Vector3<f64> {
   let mut rec = HitRecord {t: 0.0, p: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new(1.0, 0.0, 0.0)};
   let mut rng = rand::thread_rng();

    if world.hit(&r, &0.001, &f64::MAX, &mut rec) {
      let target = rec.p + rec.normal + random_in_unit_sphere(&mut rng);
      let ray = Ray::new(rec.p, target - rec.p);
      return 0.5*color(&ray, &world);
      //return 0.5*Vector3::new(rec.normal.x+1.0, rec.normal.y+1.0, rec.normal.z+1.0);
    } else {
      let unit_direction = unit_vector(r.direction);
      let t = 0.5*(unit_direction.y) + 1.0;
      return (1.0-t)*Vector3::new(1.0, 1.0, 1.0) + t*Vector3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    print!("P3\n{} {}\n255\n", nx, ny);

    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);

    // Objects
    let mut hitable_list = HitableList::new();
    hitable_list.push(Arc::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)));
    hitable_list.push(Arc::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new(origin, lower_left_corner, horizontal, vertical);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u = (i as f64 + rand::random::<f64>()) / (nx as f64);
                let v = (j as f64 + rand::random::<f64>()) / (ny as f64);

                let r = cam.get_ray(u, v);
                //let p = r.point_at_parameter(2.0);
                // println!("row = {}, col = {}, s = {}", j, i, s);
                col += color(&r, &hitable_list);
            }
            col /= ns as f64;
            let ir = (255.99*col[0]) as u64;
            let ig = (255.99*col[1]) as u64;
            let ib = (255.99*col[2]) as u64;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}

