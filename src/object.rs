use crate::ray::Ray;
use crate::vector_3d::Vector3D;

pub struct ObjectHitRecord {
    pub t: f64,
    pub point: Vector3D,
    pub normal: Vector3D,
    pub front_face: bool,
    // pub material: &'material Material,
    // pub u: f64,
    // pub v: f64,
}

impl ObjectHitRecord {
    pub fn new(t: f64, point: Vector3D, normal: Vector3D, front_face: bool) -> ObjectHitRecord {
        return ObjectHitRecord {
            t: t,
            point: point,
            normal: normal,
            front_face: front_face,
        };
    }
}

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<ObjectHitRecord>;
}
