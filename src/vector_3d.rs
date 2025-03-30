use std::cmp::PartialEq;
use std::f64;
use std::ops::{Add, Div, Mul, Neg, Sub};

use rand::Rng;

use serde::{Deserialize, Serialize};

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        return Vector3D { x: x, y: y, z: z };
    }

    pub fn random(min: f64, max: f64) -> Vector3D {
        let mut rng = rand::thread_rng();
        Vector3D::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere() -> Vector3D {
        loop {
            let p = Vector3D::random(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn get_x(&self) -> f64 {
        return self.x;
    }

    pub fn get_y(&self) -> f64 {
        return self.y;
    }

    pub fn get_z(&self) -> f64 {
        return self.z;
    }

    pub fn distance(&self, other: &Vector3D) -> f64 {
        let x_distance = self.x - other.get_x();
        let y_distance = self.y - other.get_y();
        let z_distance = self.z - other.get_z();

        return ((x_distance * x_distance) + (y_distance * y_distance) + (z_distance * z_distance))
            .sqrt();
    }

    pub fn length(&self) -> f64 {
        return self.distance(&Vector3D::new(0.0, 0.0, 0.0));
    }

    pub fn unit_vector(&self) -> Vector3D {
        let length = self.length();
        return Vector3D::new(self.x / length, self.y / length, self.z / length);
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        return (self.get_x() * other.get_x())
            + (self.get_y() * other.get_y())
            + (self.get_z() * other.get_z());
    }

    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        return Vector3D::new(
            (self.get_y() * other.get_z()) - (self.get_z() * other.get_y()),
            (self.get_z() * other.get_x()) - (self.get_x() * other.get_z()),
            (self.get_x() * other.get_y()) - (self.get_y() * other.get_x()),
        );
    }

    pub fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON && self.z.abs() < f64::EPSILON
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
        return Vector3D {
            x: self.x + other.get_x(),
            y: self.y + other.get_y(),
            z: self.z + other.get_z(),
        };
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
        return Vector3D {
            x: self.x - other.get_x(),
            y: self.y - other.get_y(),
            z: self.z - other.get_z(),
        };
    }
}

impl Mul<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn mul(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x * other.get_x(),
            y: self.y * other.get_y(),
            z: self.z * other.get_z(),
        }
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, scale: f64) -> Vector3D {
        Vector3D {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
}

impl Div for Vector3D {
    type Output = Vector3D;

    fn div(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x / other.get_x(),
            y: self.y / other.get_y(),
            z: self.z / other.get_z(),
        }
    }
}

impl Div<f64> for Vector3D {
    type Output = Vector3D;

    fn div(self, other: f64) -> Vector3D {
        Vector3D {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl PartialEq for Vector3D {
    fn eq(&self, other: &Vector3D) -> bool {
        self.x == other.get_x() && self.y == other.get_y() && self.z == other.get_z()
    }
}

impl Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Vector3D {
        Vector3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[test]
fn test_gen() {
    let vec_1 = Vector3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    assert_eq!(vec_1.get_x(), 1.0);
    assert_eq!(vec_1.get_y(), 2.0);
    assert_eq!(vec_1.get_z(), 3.0);

    let vec_2 = Vector3D::new(2.0, 3.0, 4.0);
    assert_eq!(vec_2.get_x(), 2.0);
    assert_eq!(vec_2.get_y(), 3.0);
    assert_eq!(vec_2.get_z(), 4.0);
}

#[test]
fn test_add() {
    let vec_1 = Vector3D::new(0.15, 0.2, 0.3);
    let vec_2 = Vector3D::new(0.25, 3.0, 0.4);
    let result = vec_1 + vec_2;
    assert_approx_eq!(result.get_x(), 0.4);
    assert_approx_eq!(result.get_y(), 3.2);
    assert_approx_eq!(result.get_z(), 0.7);
}

#[test]
fn test_sub() {
    let vec_1 = Vector3D::new(0.1, 0.2, 0.3);
    let vec_2 = Vector3D::new(0.2, 0.3, 0.4);
    let result = vec_1 - vec_2;
    assert_approx_eq!(result.get_x(), -0.1);
    assert_approx_eq!(result.get_y(), -0.1);
    assert_approx_eq!(result.get_z(), -0.1);
}

#[test]
fn test_mul() {
    let vec_1 = Vector3D::new(0.1, 0.25, 1.0);
    let vec_2 = Vector3D::new(2.0, 0.33, 0.4);
    let result = vec_1 * vec_2;
    assert_approx_eq!(result.get_x(), 0.2);
    assert_approx_eq!(result.get_y(), 0.0825);
    assert_approx_eq!(result.get_z(), 0.4);
}

#[test]
fn test_div() {
    let vec_1 = Vector3D::new(0.1, 0.2, 1.0);
    let vec_2 = Vector3D::new(2.5, 0.3, 0.001);
    let result = vec_1 / vec_2;
    assert_approx_eq!(result.get_x(), 0.04);
    assert_approx_eq!(result.get_y(), 0.6666666666666666);
    assert_approx_eq!(result.get_z(), 1000.0);
}

#[test]
fn test_dot() {
    let vec_1 = Vector3D::new(0.1, 0.2, 0.3);
    let vec_2 = Vector3D::new(0.2, 0.3, 0.4);
    assert_approx_eq!(vec_1.dot(&vec_2), 0.2);
}

#[test]
fn test_length_squared() {
    let vec_1 = Vector3D::new(0.1, 0.2, 0.3);
    assert_approx_eq!(vec_1.length_squared(), 0.14);
}

#[test]
fn test_neg() {
    let vec_1 = Vector3D::new(0.1, 0.2, 0.3);
    let vec_2 = -vec_1;
    assert_approx_eq!(vec_2.get_x(), -0.1);
    assert_approx_eq!(vec_2.get_y(), -0.2);
    assert_approx_eq!(vec_2.get_z(), -0.3);
}

#[test]
fn test_random() {
    let vec_1 = Vector3D::random(-1.0, 1.0);
    assert!(vec_1.get_x() >= -1.0 && vec_1.get_x() <= 1.0);
    assert!(vec_1.get_y() >= -1.0 && vec_1.get_y() <= 1.0);
    assert!(vec_1.get_z() >= -1.0 && vec_1.get_z() <= 1.0);
}
