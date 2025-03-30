use palette::Srgb;

use rand::Rng;

use crate::camera::Camera;
use crate::world::World;

pub enum AntiAliasingTechnique {
    SuperSampling, // Simple supersampling
    MonteCarlo,    // Advanced Monte Carlo sampling
    Spatial,       // Spatial anti-aliasing
    Temporal,      // Temporal anti-aliasing
    None,
}

pub struct AntiAliasing {
    samples_per_pixel: usize,
    technique: AntiAliasingTechnique,
}

impl AntiAliasing {
    pub fn new(samples_per_pixel: usize, technique: AntiAliasingTechnique) -> Self {
        return AntiAliasing {
            samples_per_pixel: samples_per_pixel,
            technique: technique,
        };
    }

    pub fn anti_alias(&self, x: usize, y: usize, camera: &Camera, world: &World) -> Srgb {
        let mut pixel_colors: Vec<f32> = vec![0.0; 3];
        let mut color = Srgb::new(0.0, 0.0, 0.0);

        match self.technique {
            AntiAliasingTechnique::SuperSampling => {
                let mut rng = rand::thread_rng();
                for _s in 0..self.samples_per_pixel {
                    // Get random ray close to the original x and y within a [-0.5, 0.5] square
                    let u = (x as f64 + rng.r#gen::<f64>()) / (camera.image_width as f64 - 1.0);
                    let v = (camera.image_height as f64 - (y as f64 + rng.r#gen::<f64>()))
                        / (camera.image_height as f64 - 1.0);

                    let r = camera.get_ray(u, v);
                    let c = camera.ray_color(&r, &world, 50);

                    pixel_colors[0] += c.red;
                    pixel_colors[1] += c.green;
                    pixel_colors[2] += c.blue;
                }

                // Average the color integration
                let scale = 1.0 / self.samples_per_pixel as f32;
                color = Srgb::new(
                    (scale * pixel_colors[0]).sqrt(),
                    (scale * pixel_colors[1]).sqrt(),
                    (scale * pixel_colors[2]).sqrt(),
                );
            }

            // Default case (Including the None option)
            _ => {
                let u = (x as f64) / (camera.image_width as f64 - 1.0);
                let v = ((camera.image_height as f64) - (y as f64)) / (camera.image_height as f64 - 1.0);
                let r = camera.get_ray(u, v);
                color = camera.ray_color(&r, &world, 50);
            }
        }

        return color;
    }
}
