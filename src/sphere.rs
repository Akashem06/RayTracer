use crate::material::Material;
use crate::object::{Object, ObjectHitRecord};
use crate::ray::Ray;
use crate::vector_3d::Vector3D;

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

pub struct Sphere {
    center: Vector3D,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vector3D, radius: f64, material: Material) -> Sphere {
        return Sphere {
            center: center,
            radius: radius,
            material: material,
        };
    }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<ObjectHitRecord> {
        // Compute the vector from the center of the sphere to the rays origin
        let sphere_to_ray = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = sphere_to_ray.dot(&ray.direction);
        let c = sphere_to_ray.length_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp_soln = (-half_b - root) / a;

            // Check if its within the range of distance from camera
            if temp_soln < t_max && temp_soln > t_min {
                let intersect_point = ray.at(temp_soln);
                let normal = (intersect_point - self.center) / self.radius;

                // If the dot product against the normal is negative (90 < x < 270)
                // This means we are outisde the sphere, and want to keep the normal the same
                let front_face = ray.direction.dot(&normal) < 0.0;

                return Some(ObjectHitRecord {
                    t: temp_soln,
                    point: intersect_point,
                    normal: if front_face { normal } else { -normal },
                    front_face: front_face,
                    material: &self.material,
                });
            }
        }
        return None;
    }
}
