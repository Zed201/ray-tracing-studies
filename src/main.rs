mod camera;
mod color;
mod ray;
mod utils;
mod vec;

use ray::{hittable_list, sphere};
use vec::*;

fn main() {
    // Objects
    let mut world = hittable_list::new();

    world.add(Box::new(sphere::new(
        Vec3::new(VecTypes::Coordinates, 0.0, -100.5, -1.0),
        100.0,
    )));
    world.add(Box::new(sphere::new(
        Vec3::new(VecTypes::Coordinates, 0.0, 0.0, -1.0),
        0.5,
    )));
    let asp: f64 = 16.0 / 9.0;
    let mut c = camera::Camera::new(asp, 500, "image.png");
    let _ = c.render(&world);
}
