mod camera;
mod color;
mod material;
mod ray;
mod utils;
mod vec;

use color::Color;
use material::{Dieletric, Lambertian, Material, Metal};
use ray::{HittableList, Sphere};
use vec::*;

fn main() {
    // Objects
    let mut world = HittableList::new();

    let m_ground = Lambertian::new(Color::new(0.0, 0.8, 0.4));
    let m_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let m_left = Dieletric::new(1.5);
    let m_bubble = Dieletric::new(1.0 / 1.5);
    let m_rigth = Metal::new(Color::new(0.5, 0.6, 0.2), 1.0);

    let g = Sphere::new(
        Vec3::new(VecTypes::Coordinates, 0.0, -100.5, -1.0),
        100.0,
        m_ground.clone_box(),
    );

    let c = Sphere::new(
        Vec3::new(VecTypes::Coordinates, 0.0, 0.0, -1.2),
        0.5,
        m_center.clone_box(),
    );
    let l = Sphere::new(
        Vec3::new(VecTypes::Coordinates, -1.0, 0.0, -1.0),
        0.5,
        m_left.clone_box(),
    );
    let l_inside = Sphere::new(
        Vec3::new(VecTypes::Coordinates, -1.0, 0.0, -1.0),
        0.4,
        m_bubble.clone_box(),
    );
    let r = Sphere::new(
        Vec3::new(VecTypes::Coordinates, 1.0, 0.0, -1.0),
        0.5,
        m_rigth.clone_box(),
    );
    world.add(g.to_box());
    world.add(c.to_box());
    world.add(l.to_box());
    world.add(r.to_box());
    world.add(l_inside.to_box());

    let asp: f64 = 16.0 / 9.0;
    let mut c = camera::Camera::new(asp, 800, "image.png");
    let _ = c.render(&world);
}
