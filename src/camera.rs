use std::fs::File;

use serde::{Deserialize, Serialize};

use palette::Pixel;
use palette::Srgb;

use image::ColorType;
use image::png::PNGEncoder;

use crate::anti_aliasing::AntiAliasing;
use crate::material::Scatterable;
use crate::ray::Ray;
use crate::vector_3d::Vector3D;
use crate::world::World;

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "CameraConfig")]
pub struct Camera {
    #[serde(skip_serializing)]
    pub origin: Vector3D, // Camera center
    #[serde(skip_serializing)]
    pub lower_left_corner: Vector3D, // Location of (pixel, 0, 0)
    #[serde(skip_serializing)]
    pub focal_length: f64,
    #[serde(skip_serializing)]
    pub horizontal: Vector3D,
    #[serde(skip_serializing)]
    pub vertical: Vector3D,
    #[serde(skip_serializing)]
    pub image_height: usize,
    pub image_width: usize,
    aspect: f64,
    vertical_fov: f64, // vertical field-of-view in degrees
    vector_up: Vector3D,
    look_from: Vector3D,
    look_at: Vector3D,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct CameraConfig {
    pub aspect: f64,
    pub image_width: usize,
    pub vertical_fov: f64, // Vertical FOV in degrees
    pub vector_up: Vector3D,
    pub look_from: Vector3D,
    pub look_at: Vector3D,
}

impl From<CameraConfig> for Camera {
    fn from(config: CameraConfig) -> Self {
        Camera::new(config)
    }
}

impl Camera {
    pub fn new(config: CameraConfig) -> Camera {
        let theta = config.vertical_fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = config.aspect * half_height;

        // Forward direction vector
        let w = (config.look_from - config.look_at).unit_vector();

        // Right direction vector (Cross product of forwards x updwards)
        let u = config.vector_up.cross(&w).unit_vector();

        // Up direction vector
        let v = w.cross(&u);

        let origin = config.look_from;

        // Left corner is origin - forward direction - (right direction * half width) - (up direction * half height)
        let lower_left_corner = origin - (u * half_width) - (v * half_height) - w;

        // Full width * right direction
        let horizontal = u * 2.0 * half_width;

        // Full height * up direction
        let vertical = v * 2.0 * half_height;

        let image_height = ((config.image_width as f64) / config.aspect).round() as usize;

        return Camera {
            origin,
            lower_left_corner,
            focal_length: (config.look_from - config.look_at).length(),
            horizontal,
            vertical,
            look_from: config.look_from,
            look_at: config.look_at,
            vector_up: config.vector_up,
            vertical_fov: config.vertical_fov,
            aspect: config.aspect,
            image_width: config.image_width,
            image_height: image_height,
        };
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - self.origin,
        );
    }

    pub fn ray_color(&self, ray: &Ray, world: &World, depth: i32) -> Srgb {
        if depth <= 0 {
            return Srgb::new(0.0, 0.0, 0.0);
        }

        let hit = world.hit(ray, 0.001, std::f64::MAX);
        match hit {
            Some(hit_record) => {
                let scattered = hit_record.material.scatter(ray, &hit_record);

                match scattered {
                    Some((scattered_ray, albedo)) => {
                        let target_color = self.ray_color(&scattered_ray, world, depth - 1);
                        return Srgb::new(
                            albedo.red * target_color.red,
                            albedo.green * target_color.green,
                            albedo.blue * target_color.blue,
                        );
                    }
                    None => {
                        return Srgb::new(0.0, 0.0, 0.0);
                    }
                }
            }
            None => {
                let t: f32 = 0.5 * (ray.direction.unit_vector().get_y() as f32 + 1.0);
                return Srgb::new(
                    (1.0 - t) * 1.0 + t * 0.5,
                    (1.0 - t) * 1.0 + t * 0.7,
                    (1.0 - t) * 1.0 + t * 1.0,
                );
            }
        }
    }

    pub fn render(&self, world: &World, anti_aliasing: &AntiAliasing) -> Vec<u8> {
        let mut pixels = vec![0; self.image_width * self.image_height * 3];

        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let color = anti_aliasing.anti_alias(x, y, self, world);

                let i = y * self.image_width + x;
                let pixel: [u8; 3] = color.into_format().into_raw();
                pixels[i * 3] = pixel[0];
                pixels[i * 3 + 1] = pixel[1];
                pixels[i * 3 + 2] = pixel[2];
            }
        }

        return pixels;
    }

    pub fn write_image(
        &self,
        filename: &str,
        pixels: &[u8],
        width: usize,
        height: usize,
    ) -> Result<(), std::io::Error> {
        let output = File::create(filename)?;
        let encoder = PNGEncoder::new(output);
        encoder.encode(pixels, width as u32, height as u32, ColorType::RGB(8))?;
        Ok(())
    }
}

#[test]
fn test_camera() {
    let camera_config = CameraConfig {
        aspect: 800.0 / 600.0,
        image_width: 800,
        vertical_fov: 90.0,
        vector_up: Vector3D::new(0.0, 1.0, 0.0),
        look_from: Vector3D::new(0.0, 0.0, 0.0),
        look_at: Vector3D::new(0.0, 0.0, -1.0),
    };

    let camera = Camera::from(camera_config);

    assert_eq!(camera.origin.get_x(), 0.0);
    assert_eq!(camera.origin.get_y(), 0.0);
    assert_eq!(camera.origin.get_z(), 0.0);

    assert_approx_eq!(camera.lower_left_corner.get_x(), -(1.0 + (1.0 / 3.0)));
    assert_approx_eq!(camera.lower_left_corner.get_y(), -1.0);
    assert_approx_eq!(camera.lower_left_corner.get_z(), -1.0);
}

#[test]
fn test_camera_get_ray() {
    let camera_config = CameraConfig {
        aspect: 800.0 / 600.0,
        image_width: 800,
        vertical_fov: 90.0,
        vector_up: Vector3D::new(0.0, 1.0, 0.0),
        look_from: Vector3D::new(-4.0, 4.0, 1.0),
        look_at: Vector3D::new(0.0, 0.0, -1.0),
    };

    let camera = Camera::from(camera_config);
    let ray = camera.get_ray(0.5, 0.5);

    assert_eq!(ray.origin.get_x(), -4.0);
    assert_eq!(ray.origin.get_y(), 4.0);
    assert_eq!(ray.origin.get_z(), 1.0);

    assert_approx_eq!(ray.direction.get_x(), (2.0 / 3.0));
    assert_approx_eq!(ray.direction.get_y(), -(2.0 / 3.0));
    assert_approx_eq!(ray.direction.get_z(), -(1.0 / 3.0));
}
