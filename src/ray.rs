use crate::{color::Color, vec};

#[derive(Debug)]
pub struct Ray {
    // the central idea is represent
    // P(t) = A + tB, where A is the origin and B is the direction
    // of the ray, this function is impl in at function
    pub origin: vec::Vec3,
    pub direction: vec::Vec3,
}

impl Ray {
    pub fn new(origin: vec::Vec3, direction: vec::Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> vec::Vec3 {
        return self.origin + self.direction.mul(t);
    }

    pub fn color(&self) -> Color {
        let unit = self.direction.unit_vec();
        let a = 0.5 * (unit[1] + 1.0);
        return Color::new(1.0, 1.0, 1.0).mul(1.0 - a) + Color::new(0.2, 0.0, 0.0).mul(a);
    }
}
