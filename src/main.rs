use std::env;

use palette::Srgb;

use ray_tracer::anti_aliasing::{AntiAliasing, AntiAliasingTechnique};
use ray_tracer::camera::{Camera, CameraConfig};
use ray_tracer::sphere::Sphere;
use ray_tracer::vector_3d::Vector3D;
use ray_tracer::world::World;
use ray_tracer::material::{Material, Lambertian, Metal};

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

    let anti_aliasing = AntiAliasing::new(10, AntiAliasingTechnique::SuperSampling);

    let mut world = World::new();

    let metal_1_config = (Srgb::new(0.5, 0.0, 0.5), 0.0);
    let metal_2_config = (Srgb::new(1.0, 0.0, 0.0), 0.25);
    let metal_3_config = (Srgb::new(0.0, 0.0, 1.0), 0.25);
    let lambertian_config = Srgb::new(0.5, 0.5, 0.5);

    world.add(Sphere::new(
        Vector3D::new(0.0, 0.0, -1.0),
        0.1,
        Material::Metal(Metal::new(metal_1_config.0, metal_1_config.1))
    ));

    world.add(Sphere::new(
        Vector3D::new(-0.5, 0.0, -1.0),
        0.25,
        Material::Metal(Metal::new(metal_2_config.0, metal_2_config.1))
    ));

    world.add(Sphere::new(
        Vector3D::new(0.5, 0.0, -1.0),
        0.25,
        Material::Metal(Metal::new(metal_3_config.0, metal_3_config.1))
    ));

    world.add(Sphere::new(
        Vector3D::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambertian(Lambertian::new(lambertian_config))
    ));

    let pixels = camera.render(&world, &anti_aliasing);

    camera
        .write_image(&args[1], &pixels, camera.image_width, camera.image_height)
        .expect("Failed to write image");
}
