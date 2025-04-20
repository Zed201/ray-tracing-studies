use image::{ImageBuffer, Rgb, RgbImage};
use pbr::ProgressBar;

mod color;
mod ray;
mod vec;

use color::Color;
use vec::*;

fn main() {
    // image infos, aspect is 16:9
    let wid: u32 = 1920;
    let hei: u32 = wid * 9 / 16;

    let mut buffer: RgbImage = ImageBuffer::new(wid, hei);

    // camera and viewport
    let focal_len = 1.0;
    let view_hei = 2.0;
    let view_wid = view_hei * (wid as f64 / hei as f64);
    let camera_point = vec::Vec3::new(VecTypes::Coordinates, 0.0, 0.0, 0.0); // 0,0,0
    // the viewport is ortogonal from eye_point in z-axis at focal_len distance

    // aux vector, refering the axis x and y, in dccs u -> x and v -> y
    let view_x = Vec3::new(vec::VecTypes::Coordinates, view_wid, 0.0, 0.0);
    let view_y = Vec3::new(vec::VecTypes::Coordinates, 0.0, -view_hei, 0.0);

    // delta's, size of each pixel
    let delta_x = view_x.div(wid as f64);
    let delta_y = view_y.div(hei as f64);
    // upper left viewport point
    let view_upper_left = camera_point
        - Vec3::new(VecTypes::Coordinates, 0.0, 0.0, focal_len)
        - view_x.div(2.0)
        - view_y.div(2.0);

    let pixel00_loc = view_upper_left + (delta_y + delta_x).mul(0.5);
    //Render in png
    let mut pb = ProgressBar::new((hei * wid) as u64);
    pb.format("=>");

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        // basically will calc the ray from the camera to the especific point
        let pixel_center = pixel00_loc + delta_x.mul(x as f64) + delta_y.mul(y as f64);
        let ray_dir = pixel_center - camera_point;
        let r = ray::Ray::new(camera_point, ray_dir.unit_vec());

        let c = r.color();
        *pixel = Rgb::from(c);

        pb.inc();
    }
    buffer.save("image.png").unwrap();
    pb.finish_println("Done");
}
