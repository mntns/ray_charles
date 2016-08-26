extern crate nalgebra as na;
use na::{Vector3};
use std::f64;
use std::rc::Rc;
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

fn unit_vector(vec: Vector3<f64>) -> Vector3<f64> {
    let len = vec.len() as f64;
    Vector3::new(vec.x / len, vec.y / len, vec.z / len)
}

fn color(r: Ray, world: &HitableList) -> Vector3<f64> {
    let mut rec = HitRecord {t: 0.0, p: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new(1.0, 0.0, 0.0)};

    if world.hit(&r, 0.0, f64::MAX, &mut rec) {
        return 0.5*Vector3::new(rec.normal.x+1.0, rec.normal.y+1.0, rec.normal.z+1.0);
    }

    let unit_direction: Vector3<f64> = unit_vector(r.direction);
    let t = 0.5*(unit_direction.y) + 1.0;
    return (1.0-t)*Vector3::new(1.0, 1.0, 1.0) + t*Vector3::new(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 200;
    let ny = 100;

    print!("P3\n{} {}\n255\n", nx, ny);

    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);

    // Objects
    let mut hitable_list = HitableList::new();
    //let sphere = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5);
    let sphere = Rc::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5));
    let sphere2 = Rc::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0));
    hitable_list.push(sphere);
    hitable_list.push(sphere2);

    // Camera
    let camera = Camera::new(origin, lower_left_corner, horizontal, vertical);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);

            let r = camera.get_ray(u, v);
            let col = color(r, &hitable_list);

            let ir = (255.99*col[0]) as i64;
            let ig = (255.99*col[1]) as i64;
            let ib = (255.99*col[2]) as i64;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}

