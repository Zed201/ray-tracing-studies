use rand::prelude::*;

pub const PI: f64 = 3.141592;
pub const INF: f64 = f64::INFINITY;

pub fn degrees_to_radians(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn randon_f64() -> f64 {
    let mut rng = rand::rng();

    rng.random::<f64>()
}

pub fn random_interval_f64(min: f64, max: f64) -> f64 {
    min + randon_f64() * (max - min)
}

pub struct interval {
    pub min: f64,
    pub max: f64,
}

impl interval {
    pub fn new(min: f64, max: f64) -> Self {
        interval { min, max }
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

impl Default for interval {
    fn default() -> Self {
        interval {
            min: -INF,
            max: INF,
        }
    }
}
