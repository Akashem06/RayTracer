use std::cmp::PartialEq;
use std::f64;
use std::ops::{Add, Div, Mul, Neg, Sub};

use rand::Rng;

use serde::{Deserialize, Serialize};

#[cfg(feature = "simd")]
use std::simd::f64x4;

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

#[cfg(not(feature = "simd"))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

#[cfg(feature = "simd")]
#[derive(Debug, Clone, Copy)]
pub struct Vector3D {
    data: f64x4,
}

#[cfg(feature = "simd")]
impl Serialize for Vector3D {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct Temp {
            x: f64,
            y: f64,
            z: f64,
        }

        let temp = Temp {
            x: self.data[0],
            y: self.data[1],
            z: self.data[2],
        };

        temp.serialize(serializer)
    }
}

#[cfg(feature = "simd")]
impl<'de> Deserialize<'de> for Vector3D {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Temp {
            x: f64,
            y: f64,
            z: f64,
        }

        let temp = Temp::deserialize(deserializer)?;

        Ok(Vector3D {
            data: f64x4::from_array([temp.x, temp.y, temp.z, 0.0]),
        })
    }
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        #[cfg(not(feature = "simd"))]
        {
            return Vector3D { x, y, z };
        }

        #[cfg(feature = "simd")]
        {
            return Vector3D {
                data: f64x4::from_array([x, y, z, 0.0]),
            };
        }
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
        #[cfg(not(feature = "simd"))]
        {
            return self.x;
        }

        #[cfg(feature = "simd")]
        {
            return self.data[0];
        }
    }

    pub fn get_y(&self) -> f64 {
        #[cfg(not(feature = "simd"))]
        {
            return self.y;
        }

        #[cfg(feature = "simd")]
        {
            return self.data[1];
        }
    }

    pub fn get_z(&self) -> f64 {
        #[cfg(not(feature = "simd"))]
        {
            return self.z;
        }

        #[cfg(feature = "simd")]
        {
            return self.data[2];
        }
    }

    pub fn set_x(&mut self, value: f64) {
        #[cfg(not(feature = "simd"))]
        {
            self.x = value;
        }

        #[cfg(feature = "simd")]
        {
            self.data[0] = value;
        }
    }

    pub fn set_y(&mut self, value: f64) {
        #[cfg(not(feature = "simd"))]
        {
            self.y = value;
        }

        #[cfg(feature = "simd")]
        {
            self.data[1] = value;
        }
    }

    pub fn set_z(&mut self, value: f64) {
        #[cfg(not(feature = "simd"))]
        {
            self.z = value;
        }

        #[cfg(feature = "simd")]
        {
            self.data[2] = value;
        }
    }

    pub fn distance(&self, other: &Vector3D) -> f64 {
        #[cfg(not(feature = "simd"))]
        {
            let x_distance = self.x - other.get_x();
            let y_distance = self.y - other.get_y();
            let z_distance = self.z - other.get_z();

            return ((x_distance * x_distance)
                + (y_distance * y_distance)
                + (z_distance * z_distance))
                .sqrt();
        }

        #[cfg(feature = "simd")]
        {
            // Using SIMD subtraction and multiplication
            let diff = self.data - other.data;
            let squared = diff * diff;
            return (squared[0] + squared[1] + squared[2]).sqrt();
        }
    }

    pub fn length(&self) -> f64 {
        #[cfg(not(feature = "simd"))]
        {
            return self.distance(&Vector3D::new(0.0, 0.0, 0.0));
        }

        #[cfg(feature = "simd")]
        {
            return self.length_squared().sqrt();
        }
    }

    pub fn unit_vector(&self) -> Vector3D {
        let length = self.length();
        return *self / length;
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        #[cfg(not(feature = "simd"))]
        {
            return (self.x * other.get_x()) + (self.y * other.get_y()) + (self.z * other.get_z());
        }

        #[cfg(feature = "simd")]
        {
            // Calculate dot product with SIMD
            let products = self.data * other.data;
            return products[0] + products[1] + products[2];
        }
    }

    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        #[cfg(not(feature = "simd"))]
        {
            return Vector3D::new(
                (self.y * other.get_z()) - (self.z * other.get_y()),
                (self.z * other.get_x()) - (self.x * other.get_z()),
                (self.x * other.get_y()) - (self.y * other.get_x()),
            );
        }

        #[cfg(feature = "simd")]
        {
            let result = f64x4::from_array([
                self.data[1] * other.data[2] - self.data[2] * other.data[1],
                self.data[2] * other.data[0] - self.data[0] * other.data[2],
                self.data[0] * other.data[1] - self.data[1] * other.data[0],
                0.0,
            ]);

            return Vector3D { data: result };
        }
    }

    pub fn length_squared(&self) -> f64 {
        #[cfg(not(feature = "simd"))]
        {
            return self.x * self.x + self.y * self.y + self.z * self.z;
        }

        #[cfg(feature = "simd")]
        {
            // Calculate length_squared with SIMD
            let squared = self.data * self.data;
            return squared[0] + squared[1] + squared[2];
        }
    }

    pub fn near_zero(&self) -> bool {
        #[cfg(not(feature = "simd"))]
        {
            self.x.abs() < f64::EPSILON
                && self.y.abs() < f64::EPSILON
                && self.z.abs() < f64::EPSILON
        }

        #[cfg(feature = "simd")]
        {
            self.data[0].abs() < f64::EPSILON
                && self.data[1].abs() < f64::EPSILON
                && self.data[2].abs() < f64::EPSILON
        }
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
        #[cfg(not(feature = "simd"))]
        {
            return Vector3D {
                x: self.x + other.get_x(),
                y: self.y + other.get_y(),
                z: self.z + other.get_z(),
            };
        }

        #[cfg(feature = "simd")]
        {
            return Vector3D {
                data: self.data + other.data,
            };
        }
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
        #[cfg(not(feature = "simd"))]
        {
            return Vector3D {
                x: self.x - other.get_x(),
                y: self.y - other.get_y(),
                z: self.z - other.get_z(),
            };
        }

        #[cfg(feature = "simd")]
        {
            return Vector3D {
                data: self.data - other.data,
            };
        }
    }
}

impl Mul<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn mul(self, other: Vector3D) -> Vector3D {
        #[cfg(not(feature = "simd"))]
        {
            Vector3D {
                x: self.x * other.get_x(),
                y: self.y * other.get_y(),
                z: self.z * other.get_z(),
            }
        }

        #[cfg(feature = "simd")]
        {
            Vector3D {
                data: self.data * other.data,
            }
        }
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, scale: f64) -> Vector3D {
        #[cfg(not(feature = "simd"))]
        {
            Vector3D {
                x: self.x * scale,
                y: self.y * scale,
                z: self.z * scale,
            }
        }

        #[cfg(feature = "simd")]
        {
            Vector3D {
                data: self.data * f64x4::splat(scale),
            }
        }
    }
}

impl Div for Vector3D {
    type Output = Vector3D;

    fn div(self, other: Vector3D) -> Vector3D {
        #[cfg(not(feature = "simd"))]
        {
            Vector3D {
                x: self.x / other.get_x(),
                y: self.y / other.get_y(),
                z: self.z / other.get_z(),
            }
        }

        #[cfg(feature = "simd")]
        {
            Vector3D {
                data: self.data / other.data,
            }
        }
    }
}

impl Div<f64> for Vector3D {
    type Output = Vector3D;

    fn div(self, other: f64) -> Vector3D {
        #[cfg(not(feature = "simd"))]
        {
            Vector3D {
                x: self.x / other,
                y: self.y / other,
                z: self.z / other,
            }
        }

        #[cfg(feature = "simd")]
        {
            Vector3D {
                data: self.data / f64x4::splat(other),
            }
        }
    }
}

impl PartialEq for Vector3D {
    fn eq(&self, other: &Vector3D) -> bool {
        #[cfg(not(feature = "simd"))]
        {
            (self.x - other.x).abs() < f64::EPSILON
                && (self.y - other.y).abs() < f64::EPSILON
                && (self.z - other.z).abs() < f64::EPSILON
        }

        #[cfg(feature = "simd")]
        {
            (self.data[0] - other.data[0]).abs() < f64::EPSILON
                && (self.data[1] - other.data[1]).abs() < f64::EPSILON
                && (self.data[2] - other.data[2]).abs() < f64::EPSILON
        }
    }
}

impl Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Vector3D {
        #[cfg(not(feature = "simd"))]
        {
            Vector3D {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            }
        }

        #[cfg(feature = "simd")]
        {
            Vector3D { data: -self.data }
        }
    }
}

#[test]
fn test_gen() {
    #[cfg(feature = "simd")]
    {
        use std::simd::f64x4;
        let vec_1 = Vector3D {
            data: f64x4::from_array([1.0, 2.0, 3.0, 0.0]),
        };
        assert_approx_eq!(vec_1.get_x(), 1.0);
        assert_approx_eq!(vec_1.get_y(), 2.0);
        assert_approx_eq!(vec_1.get_z(), 3.0);
    }

    #[cfg(not(feature = "simd"))]
    {
        let vec_1 = Vector3D::new(0.15, 0.2, 0.3);
        let vec_2 = Vector3D::new(0.25, 3.0, 0.4);
        let result = vec_1 + vec_2;
        assert_approx_eq!(result.get_x(), 0.4);
        assert_approx_eq!(result.get_y(), 3.2);
        assert_approx_eq!(result.get_z(), 0.7);
    }
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
