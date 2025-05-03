mod camera;
mod color;
mod material;
mod ray;
mod utils;
mod vec;

use std::{f64::consts::PI, fs::File};

use color::Color;
use image::{Delay, Frame, ImageBuffer, RgbImage, Rgba, RgbaImage, codecs::gif::GifEncoder};
use material::{Dieletric, Lambertian, Material, Metal};
use ray::{HittableList, Sphere};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use vec::*;

fn main() {
    let world = HittableList::new();
    let asp: f64 = 16.0 / 9.0;
    let mut c = camera::Camera::new(asp, 300);
    c.lookfrom = Vec3::new(VecTypes::Coordinates, 5.0, 2.0, 3.0);
    c.lookat = Vec3::new(VecTypes::Coordinates, 0.0, 0.0, 0.0);
    c.vup = Vec3::new(VecTypes::Coordinates, 0.0, 1.0, 0.0);
    c.vfov = 60.0;
    let i: RgbImage = c.render(&world);

    if let Err(e) = i.save("image.png") {
        println!("ERro ao salvar image {:?}", e);
    }
}

fn _rgb_rgba(i: &RgbImage) -> RgbaImage {
    ImageBuffer::from_fn(i.width(), i.height(), |x, y| {
        let p = i.get_pixel(x, y);
        Rgba([p[0], p[1], p[2], 255])
    })
}

// gif with 3 Sphere's
fn _gif_part_1() {
    let fs: Vec<Frame> = (0..100)
        .into_par_iter()
        .map(|p| {
            let mut world = HittableList::new();
            let pp = p as f64 / 100.0;
            let m_g = Lambertian::new(Color::new(1.0, 0.3, 0.5));
            let g = Sphere::new(
                Vec3::new(VecTypes::Coordinates, 0.0, -103.0, 0.0),
                100.0,
                m_g.clone_box(),
            );
            world.add(g.boxed());
            let angle = 2.0 * PI * pp;
            let ra = 2.0;
            let v = Vec3::new(
                VecTypes::Coordinates,
                ra * angle.cos(),
                0.0,
                ra * angle.sin(),
            );
            let m = Dieletric::new(1.5);
            let s = Sphere::new(
                Vec3::new(VecTypes::Coordinates, 0.0, 0.0, 0.0),
                1.0,
                m.clone_box(),
            );
            world.add(s.boxed());

            let m = Dieletric::new(1.0 / 1.5);
            let s = Sphere::new(
                Vec3::new(VecTypes::Coordinates, 0.0, 0.0, 0.0),
                0.8,
                m.clone_box(),
            );
            world.add(s.boxed());

            let m2 = Lambertian::new(Color::new(0.81, 0.23, 0.12));
            let s = Sphere::new(v.mul(-1.0), 0.7, m2.clone_box());
            world.add(s.boxed());

            let m3 = Metal::new(Color::new(0.3, 0.25, 0.87), 0.0);
            let s = Sphere::new(v, 0.7, m3.clone_box());
            world.add(s.boxed());

            let asp: f64 = 16.0 / 9.0;
            let mut c = camera::Camera::new(asp, 1200);
            c.lookfrom = Vec3::new(VecTypes::Coordinates, 5.0, 2.0, 3.0);
            c.lookat = Vec3::new(VecTypes::Coordinates, 0.0, 0.0, 0.0);
            c.vup = Vec3::new(VecTypes::Coordinates, 0.0, 1.0, 0.0);
            c.vfov = 60.0;
            let i: RgbImage = c.render(&world);
            let a = _rgb_rgba(&i);
            let d = Delay::from_numer_denom_ms(100, 1);

            Frame::from_parts(a, 0, 0, d)
        })
        .collect();
    let f = File::create("test.gif").unwrap();
    let mut r = GifEncoder::new(f);
    let _ = r.encode_frames(fs);
}
