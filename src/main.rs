use image::{ImageBuffer, Rgb, RgbImage};
use pbr::ProgressBar;

mod color;
mod ray;
mod vec;

use color::Color;

fn main() {
    // image infos, aspect is 16:9
    let wid: u32 = 16;
    let hei: u32 = wid * 9 / 16;

    let mut buffer: RgbImage = ImageBuffer::new(wid, hei);

    // ViewPort
    //let v_heigth = 2.0;
    //let v_width = v_heigth * (wid as f64 / hei as f64);

    //let eye_point = vec::Vec3::default();
    // the viewport is ortogonal from eye_point in z-axis

    //Render in ppm
    let mut pb = ProgressBar::new((hei * wid) as u64);
    pb.format("=>");
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let r = (x as f64) / (wid as f64 - 1.0);
        let g = (y as f64) / (hei as f64 - 1.0);
        let b: f64 = 0.4;
        let c = Color::new(r, g, b);
        *pixel = Rgb::from(c);
        pb.inc();
    }
    buffer.save("image.png").unwrap();
    pb.finish_println("Done");
}
