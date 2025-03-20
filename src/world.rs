use crate::object::{Object, ObjectHitRecord};
use crate::ray::Ray;

pub struct World {
    objects: Vec<Box<dyn Object>>,
}

impl World {
    pub fn new() -> Self {
        World {
            objects: Vec::new(),
        }
    }

    pub fn add<T: Object + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<ObjectHitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record = None;
        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_record = Some(hit);
            }
        }
        return hit_record;
    }
}
