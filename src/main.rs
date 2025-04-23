mod camera;
mod color;
mod material;
mod ray;
mod utils;
mod vec;

use ray::{HittableList, Sphere};
use vec::*;

fn main() {
    // Objects
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(
        Vec3::new(VecTypes::Coordinates, 0.0, -100.5, -1.0),
        100.0,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(VecTypes::Coordinates, 0.0, 0.0, -1.0),
        0.5,
    )));
    let asp: f64 = 16.0 / 9.0;
    let mut c = camera::Camera::new(asp, 600, "image.png");
    let _ = c.render(&world);
}
