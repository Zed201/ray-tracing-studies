use std::fs::File;
use std::io::Write;

use color::Color;

mod color;
mod vec;

fn main() {
    // image infos
    let wid: u8 = 255;
    let hei: u8 = 255;
    let name = String::from("some.ppm");
    let mut file = File::create(name.as_str()).unwrap();

    //Render in ppm
    // println!("P3");
    // println!("{} {}\n255", wid, hei);
    let _ = writeln!(file, "P3\n{} {}\n255", wid, hei);

    for j in 0..hei {
        println!("Progress: {} ", hei - j);
        for i in 0..wid {
            let r = (i as f64) / (wid as f64 - 1.0);
            let g = (j as f64) / (hei as f64 - 1.0);
            let b: f64 = 1.0;
            let c = Color::new(r, g, b);

            // println!("{ir} {ig} {ib}");
            let _ = writeln!(file, "{}", c);
        }
    }
    println!("Done")
}
