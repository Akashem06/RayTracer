use std::fs::File;

use image::ColorType;
use image::png::PNGEncoder;

use palette::Srgb;
use palette::Pixel;

use rand::Rng;

use crate::camera::Camera;
use crate::world::World;

pub struct Renderer {
    samples_per_pixel: usize,
}

impl Renderer {
    pub fn new(samples_per_pixel: usize) -> Self {
        Renderer { samples_per_pixel }
    }

    pub fn render(&self, camera: &Camera, world: &World) -> Vec<u8> {
        let mut pixels = vec![0; camera.image_width * camera.image_height * 3];
        let mut rng = rand::thread_rng();

        for y in 0..camera.image_height {
            for x in 0..camera.image_width {
                let mut pixel_colors: Vec<f32> = vec![0.0; 3];

                for _s in 0..self.samples_per_pixel {
                    let u = (x as f64 + rng.r#gen::<f64>()) / (camera.image_width as f64 - 1.0);
                    let v =
                        (camera.image_height as f64 - (y as f64 + rng.r#gen::<f64>())) / (camera.image_height as f64 - 1.0);
                    let r = camera.get_ray(u, v);
                    let c = camera.ray_color(&r, &world, 50);
                    pixel_colors[0] += c.red;
                    pixel_colors[1] += c.green;
                    pixel_colors[2] += c.blue;
                }

                let scale = 1.0 / self.samples_per_pixel as f32;
                let color = Srgb::new(
                    (scale * pixel_colors[0]).sqrt(),
                    (scale * pixel_colors[1]).sqrt(),
                    (scale * pixel_colors[2]).sqrt(),
                );

                let i = y * camera.image_width + x;
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