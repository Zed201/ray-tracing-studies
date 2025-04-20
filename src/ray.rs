use crate::{
    color::Color,
    vec::{self, Vec3, VecTypes},
};

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
        if self.hit_sphere(Vec3::new(VecTypes::Coordinates, 0.0, 0.0, 5.0), 2.1) {
            return Color::new(1.0, 0.0, 0.0);
        }
        //
        let unit = self.direction.unit_vec();
        let a = 0.5 * (unit[1] + 1.0);
        return Color::new(1.0, 1.0, 1.0).mul(1.0 - a) + Color::new(0.9, 0.8, 0.9).mul(a);
    }
    // basically position a sphere in the image,
    // solve a 2 order equation envolve the ray and the sphere center,
    // if has 1 root theres in the border, if has 2 roots
    // theres in the center but if theres no real roots, the ray(with origin in camera)
    // the sphere is not touched, so will not solve but determine the discriminant(delta);
    // t^2*d^2 - 2*t*d*(C - Q) + ((C - Q)^2 - r^2) = 0
    // where Q: ray origin, C: sphere origin, r: sphere radius
    // this formula origin is from out 0,0,0 sphere 3d formula x^2 + y^2 + z^2 = r^2
    pub fn hit_sphere(&self, center: Vec3, radius: f64) -> bool {
        let oc = center - self.origin;
        let a = self.direction.dot(&self.direction);
        let b = -2.0 * self.direction.dot(&oc);
        let c = oc.dot(&oc) - ((radius * radius) as f64);
        let delta = b * b - 4.0 * a * c;
        return delta >= 0.0;
    }
}
