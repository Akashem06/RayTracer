use std::env;

use ray_tracer::camera::{Camera, CameraConfig};
use ray_tracer::sphere::Sphere;
use ray_tracer::vector_3d::Vector3D;
use ray_tracer::world::World;
use ray_tracer::renderer::Renderer;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, world!");

    if args.len() != 2 {
        println!("Usage: {} <output_file>", args[0]);
        return;
    }

    let camera_config = CameraConfig {
        aspect: 800.0 / 600.0,
        image_width: 800,
        vertical_fov: 70.0,
        vector_up: Vector3D::new(0.0, 1.0, 0.0),
        look_from: Vector3D::new(0.0, 0.0, 0.0),
        look_at: Vector3D::new(0.0, 0.0, -1.0),
    };

    let camera = Camera::from(camera_config);

    let mut world = World::new();
    let renderer = Renderer::new(8);

    world.add(Sphere::new(Vector3D::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vector3D::new(0.0, -100.5, -1.0), 100.0));

    let pixels = renderer.render(&camera, &world);
    
    renderer.write_image(&args[1], &pixels, camera.image_width, camera.image_height).expect("Failed to write image");
}
