use crate::material::Material;
use crate::ray::Ray;
use crate::vector_3d::Vector3D;

pub struct ObjectHitRecord<'material> {
    pub t: f64,
    pub point: Vector3D,
    pub normal: Vector3D,
    pub front_face: bool,
    pub material: &'material Material,
    // pub u: f64,
    // pub v: f64,
}

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<ObjectHitRecord>;
}
