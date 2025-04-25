#![allow(dead_code, unused)]
use core::f64;

use rand::prelude::*;

use crate::vec::{Vec3, VecTypes};

pub const INF: f64 = f64::INFINITY;

pub fn degrees_to_radians(deg: f64) -> f64 {
    deg * f64::consts::PI / 180.0
}

pub fn randon_f64() -> f64 {
    let mut rng = rand::rng();

    rng.random::<f64>()
}

// so por preguica de trocar
#[allow(non_snake_case)]
pub fn random_Interval_f64(min: f64, max: f64) -> f64 {
    min + randon_f64() * (max - min)
}

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        } else if x > self.max {
            return self.max;
        }
        x
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min: -INF,
            max: INF,
        }
    }
}

// return a random vector in the square (-0.5,-0.5) -> (0.5, 0.5)
pub fn sample_square() -> Vec3 {
    Vec3::new(
        VecTypes::Coordinates,
        randon_f64() - 0.5,
        randon_f64() - 0.5,
        0.0,
    )
}
