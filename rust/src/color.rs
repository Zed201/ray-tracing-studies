use core::fmt;
use std::fmt::Formatter;

use crate::vec;

#[derive(Clone, Copy)]
pub struct Color {
    pub rgb: vec::Vec3,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color {
            rgb: vec::Vec3::new(vec::VecTypes::Color, r, g, b),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let rbyte = (self.rgb[0] * 255.999) as u8;
        let gbyte = (self.rgb[1] * 255.999) as u8;
        let bbyte = (self.rgb[2] * 255.999) as u8;

        writeln!(f, "{} {} {}", rbyte, gbyte, bbyte)
    }
}
