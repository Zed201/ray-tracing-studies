use std::fs::File;
use std::io::Write;

use pbr::ProgressBar;

mod color;
mod ray;
mod vec;

use color::Color;

fn main() {
    // image infos, aspect is 16:9
    let wid: u64 = 1920;
    let hei: u64 = wid * 9 / 16;
    let name = String::from("some.ppm");
    let mut file = File::create(name.as_str()).unwrap();

    // ViewPort
    let v_heigth = 2.0;
    let v_width = v_heigth * (wid as f64 / hei as f64);

    let eye_point = vec::Vec3::default();
    // the viewport is ortogonal from eye_point in z-axis

    //Render in ppm
    let _ = writeln!(file, "P3\n{} {}\n255", wid, hei);
    let mut pb = ProgressBar::new(hei);
    pb.format("=>");
    for j in 0..hei {
        pb.inc();
        for i in 0..wid {
            let r = (i as f64) / (wid as f64 - 1.0);
            let g = (j as f64) / (hei as f64 - 1.0);
            let b: f64 = 0.0;
            let c = Color::new(r, g, b);

            let _ = writeln!(file, "{}", c);
        }
    }
    pb.finish_println("Done");
}
