extern crate nalgebra as na;
extern crate rand;
extern crate backtrace;

use na::{Vector3};
use std::f64;
use std::sync::Arc;
mod ray;
use ray::Ray;
mod sphere;
use sphere::Sphere;
mod hitable;
mod hitable_list;
use hitable_list::HitableList;
mod camera;
use camera::Camera;
mod util;
mod material;
use material::{Lambertian, Metal, Dielectric};

fn color(r: &Ray, world: &HitableList, depth: i64) -> Vector3<f64> {
    match world.hit(&r, &0.0001, &f64::MAX) {
        Some((t, material)) => {
            if depth < 50 {
                match material.scatter(r, &t) {
                    Some((attenuation, scattered)) => {
                        attenuation*color(&scattered, world, (depth+1))
                    }
                    None => Vector3::new(1.0, 0.0, 0.0)
                }
            } else {
                Vector3::new(0.0, 1.0, 0.0)
            }
        }
        None => {
            let unit_direction = util::unit_vector(r.direction);
            let t = 0.5*(unit_direction.y) + 1.0;
            return (1.0-t)*Vector3::new(1.0, 1.0, 1.0) + t*Vector3::new(0.5, 0.7, 1.0)
        }
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
    hitable_list.push(Arc::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, Arc::new(Lambertian::new(Vector3::new(0.8, 0.3, 0.3))))));
    hitable_list.push(Arc::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, Arc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0))))));
    hitable_list.push(Arc::new(Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, Arc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.8)))));
    hitable_list.push(Arc::new(Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, Arc::new(Dielectric::new(1.5)))));


    // Camera
    let cam = Camera::new(origin, lower_left_corner, horizontal, vertical);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (i as f64 + rand::random::<f64>()) / (nx as f64);
                let v = (j as f64 + rand::random::<f64>()) / (ny as f64);

                let r = cam.get_ray(u, v);
                //let p = r.point_at_parameter(2.0);
                col += color(&r, &hitable_list, 0);
            }
            col /= ns as f64;
            col = Vector3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
            let ir = (255.99*col[0]) as u8;
            let ig = (255.99*col[1]) as u8;
            let ib = (255.99*col[2]) as u8;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}

