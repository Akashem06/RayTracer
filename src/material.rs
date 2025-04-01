use serde::{Deserialize, Serialize};

use palette::Srgb;

use crate::object::ObjectHitRecord;
use crate::ray::Ray;
use crate::vector_3d::Vector3D;

// https://docs.rs/serde_with/1.9.4/serde_with/macro.serde_conv.html
serde_with::serde_conv!(
    SrgbAsArray,
    Srgb,
    |srgb: &Srgb| [srgb.red, srgb.green, srgb.blue],
    |value: [f32; 3]| -> Result<_, std::convert::Infallible> {
        Ok(Srgb::new(value[0], value[1], value[2]))
    }
);

fn reflect(vec_1: &Vector3D, vec_2: &Vector3D) -> Vector3D {
    *vec_1 - *vec_2 * (2.0 * vec_1.dot(vec_2))
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &ObjectHitRecord) -> Option<(Ray, Srgb)>;
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit_record: &ObjectHitRecord) -> Option<(Ray, Srgb)> {
        match self {
            Material::Lambertian(l) => l.scatter(ray, hit_record),
            Material::Metal(m) => m.scatter(ray, hit_record),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lambertian {
    #[serde(with = "SrgbAsArray")]
    pub albedo: Srgb,
}

impl Lambertian {
    pub fn new(albedo: Srgb) -> Lambertian {
        return Lambertian { albedo: albedo };
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &ObjectHitRecord) -> Option<(Ray, Srgb)> {
        let mut scatter_direction = hit_record.normal + Vector3D::random_in_unit_sphere();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let target = hit_record.point + scatter_direction;
        let scattered = Ray::new(hit_record.point, target - hit_record.point);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metal {
    #[serde(with = "SrgbAsArray")]
    pub albedo: Srgb,
    pub roughness: f64,
}

impl Metal {
    pub fn new(albedo: Srgb, roughness: f64) -> Metal {
        return Metal {
            albedo: albedo,
            roughness: roughness,
        };
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &ObjectHitRecord) -> Option<(Ray, Srgb)> {
        let reflected = reflect(&ray.direction, &hit_record.normal);
        let rough_direction = reflected + Vector3D::random_in_unit_sphere() * self.roughness;
        let scattered = Ray::new(hit_record.point, rough_direction);
        let attenuation = self.albedo;

        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
