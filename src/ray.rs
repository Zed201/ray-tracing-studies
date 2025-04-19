use crate::vec;

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
}
