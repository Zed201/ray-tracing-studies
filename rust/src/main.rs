use std::fs::File;
use std::io::Write;

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
            let r = (i as f32) / (wid as f32 - 1.0);
            let g = (j as f32) / (hei as f32 - 1.0);
            let b: f32 = 0.0;

            let ir = (255.999 * r) as u64;
            let ig = (255.999 * g) as u64;
            let ib = (255.999 * b) as u64;

            // println!("{ir} {ig} {ib}");
            let _ = writeln!(file, "{ir} {ig} {ib}");
        }
    }
    println!("Done")
}
