use na::{Vector3, Dot};
use ray::Ray;
use hitable::HitRecord;
use util;

pub trait Material: {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f64>, Ray)>;
}

pub struct Lambertian {
    pub albedo: Vector3<f64>
}

pub struct Metal {
    pub albedo: Vector3<f64>,
    pub fuzz: f64
}

pub struct Dielectric {
    pub ref_idx: f64
}

impl Lambertian {
    pub fn new(a: Vector3<f64>) -> Self {
        Lambertian { albedo: a }
    }
}

impl Metal {
    pub fn new(a: Vector3<f64>, f: f64) -> Self {
        Metal { 
            albedo: a,
            fuzz: if f < 1.0 {
                f
            } else {
                1.0
            }
        }
    }
}

impl Dielectric {
    pub fn new(ri: f64) -> Self {
        Dielectric { ref_idx: ri }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f64>, Ray)> {
        let target = rec.p + rec.normal + util::random_in_unit_sphere();
        let scattered = Ray::new(rec.p, (target - rec.p));
        Some((self.albedo, scattered))
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f64>, Ray)> {
        let reflected = util::reflect(util::unit_vector(r_in.direction), rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f64>, Ray)> {

        let attenuation = Vector3::new(1.0, 1.0, 1.0);
        let outward_normal;
        let ni_over_nt;

        if r_in.direction.dot(&rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
        }

        match util::refract(r_in.direction, outward_normal, ni_over_nt) {
            Some(refracted) => {
                Some((attenuation, Ray::new(rec.p, refracted)))
            }
            None => {
                Some((attenuation, Ray::new(rec.p, util::reflect(r_in.direction, rec.normal))))
            }
        }
    }
}
