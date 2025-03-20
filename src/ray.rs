use crate::vector_3d::Vector3D;

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector3D,
    pub direction: Vector3D,
}

impl Ray {
    pub fn new(origin: Vector3D, direction: Vector3D) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vector3D {
        self.origin + self.direction * t
    }
}

#[test]
fn test_ray() {
    let vec_1 = Vector3D::new(1.0, 2.0, 3.0);
    let vec_2 = Vector3D::new(2.0, 3.0, 4.0);

    let ray = Ray::new(vec_1, vec_2);

    assert_approx_eq!(ray.origin.get_x(), 1.0);
    assert_approx_eq!(ray.origin.get_y(), 2.0);
    assert_approx_eq!(ray.origin.get_z(), 3.0);
    assert_approx_eq!(ray.direction.get_x(), 2.0);
    assert_approx_eq!(ray.direction.get_y(), 3.0);
    assert_approx_eq!(ray.direction.get_z(), 4.0);
}

#[test]
fn test_ray_at() {
    let vec_1 = Vector3D::new(0.0, 0.0, 0.0);
    let vec_2 = Vector3D::new(1.0, 2.0, 3.0);

    let ray = Ray::new(vec_1, vec_2);
    let s = ray.at(0.5);

    assert_approx_eq!(s.get_x(), 0.5);
    assert_approx_eq!(s.get_y(), 1.0);
    assert_approx_eq!(s.get_z(), 1.5);
}
