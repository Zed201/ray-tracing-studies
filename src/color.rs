use core::fmt;
use std::{
    fmt::Formatter,
    ops::{Add, AddAssign, Mul},
};

use image::Rgb;

use crate::{
    utils::Interval,
    vec::{self, Vec3},
};

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

    pub fn mul(&self, m: f64) -> Self {
        Color::new(self.rgb[0] * m, self.rgb[1] * m, self.rgb[2] * m)
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

impl From<Rgb<u8>> for Color {
    fn from(value: Rgb<u8>) -> Self {
        Self::new(
            value.0[0] as f64 / 255.999,
            value.0[1] as f64 / 255.999,
            value.0[2] as f64 / 255.999,
        )
    }
}

impl From<Color> for Rgb<u8> {
    fn from(value: Color) -> Self {
        // will format the color rgb in gamma 2, give more consistent colors
        let x = if value.rgb[0] > 0.0 {
            value.rgb[0].sqrt()
        } else {
            0.0
        };
        let y = if value.rgb[1] > 0.0 {
            value.rgb[1].sqrt()
        } else {
            0.0
        };
        let z = if value.rgb[2] > 0.0 {
            value.rgb[2].sqrt()
        } else {
            0.0
        };

        // serve to limit the colors values to [0.0, 0.999]
        let inten = Interval::new(0.0, 0.999);
        Self([
            (inten.clamp(x) * 256.0) as u8,
            (inten.clamp(y) * 256.0) as u8,
            (inten.clamp(z) * 256.0) as u8,
        ])
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Self::new(value[0], value[1], value[2])
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.rgb[0] + rhs.rgb[0],
            self.rgb[1] + rhs.rgb[1],
            self.rgb[2] + rhs.rgb[2],
        )
    }
}
impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.rgb[0] += rhs.rgb[0];
        self.rgb[1] += rhs.rgb[1];
        self.rgb[2] += rhs.rgb[2];
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            rgb: Vec3::default(),
        }
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(self.rgb * rhs.rgb)
    }
}
