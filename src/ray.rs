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
        // if self.hit_sphere(Vec3::new(VecTypes::Coordinates, 0.0, 0.0, 5.0), 2.1) {
        //     return Color::new(1.0, 0.0, 0.0);
        // }
        // let t1 = self.hit_sphere(Vec3::new(VecTypes::Coordinates, 0.0, 0.0, -2.0), 1.1);
        //
        // if t1 > 0.0 {
        //     // get the normal vector where the ray hit the sphere, basicallu the r vector, the
        //     // (0,0,-1) is because the camera point
        //     let normal = self.at(t1).unit_vec();
        //     return Color::new(normal[0] + 1.0, normal[1] + 1.0, normal[2] + 1.0).mul(0.5);
        // }
        //
        let unit = self.direction.unit_vec();
        let a = 0.5 * (unit[1] + 1.0);
        return Color::new(1.0, 1.0, 1.0).mul(1.0 - a) + Color::new(0.5, 0.7, 1.0).mul(a);
    }
    // basically position a sphere in the image,
    // solve a 2 order equation envolve the ray and the sphere center,
    // if has 1 root theres in the border, if has 2 roots
    // theres in the center but if theres no real roots, the ray(with origin in camera)
    // the sphere is not touched, so will not solve but determine the discriminant(delta);
    // t^2*d^2 - 2*t*d*(C - Q) + ((C - Q)^2 - r^2) = 0
    // where Q: ray origin, C: sphere origin, r: sphere radius
    // this formula origin is from out 0,0,0 sphere 3d formula x^2 + y^2 + z^2 = r^2in
    // the simplifies/substitute in the equation is (h +- sqrt(h^2 - ac))/a where h = b/-2 = d*(C - Q)
    // pub fn hit_sphere(&self, center: Vec3, radius: f64) -> f64 {
    //     let oc = center - self.origin;
    //     // let a = self.direction.dot(&self.direction);
    //     let a = (self.direction.vec_length()).powi(2);
    //     let h = self.direction.dot(&oc); // old b
    //     //let c = oc.dot(&oc) - ((radius * radius) as f64);
    //     let c = (oc.vec_length()).powi(2) - (radius.powi(2));
    //     // let delta = b * b - 4.0 * a * c;
    //     let delta = h.powi(2) - a * c;
    //     if delta < 0.0 {
    //         return -1.0;
    //     } else {
    //         // return (b - delta.sqrt()) / a;
    //         return (h - delta.sqrt()) / a;
    //     }
    // }
}

// hittable objects
trait hittable {
    fn hit(&self, r: Ray, r_max: f64, r_min: f64, rec: &mut hit_record) -> bool;
    // the idea is check if the t is t_min < t < t_max and save this in hit_record
    // there best ways to do that but i will upgrade later
}

struct hit_record {
    point: Vec3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

// the normal vector is point to outside/outward
// if is same direction of ray the ray is inside, cos is upper 0
// if is oposite of ray the ray is outside, cos is under 0
impl hit_record {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // will calc the face where some ray hit the object
        // outward_normal have to be in unit length
        self.front_face = r.direction.dot(&outward_normal) < 0.0;
        // if the dot is minus 0, the cos is under 0 and the face is front, else is back

        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal.mul(-1.0)
        };
    }
}

struct sphere {
    center: Vec3,
    radius: f64,
}

impl sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        sphere {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl hittable for sphere {
    fn hit(&self, r: Ray, r_max: f64, r_min: f64, rec: &mut hit_record) -> bool {
        let oc = self.center - r.origin;
        let a = r.direction.vec_length().powi(2);
        let h = r.direction.dot(&oc);
        let c = oc.vec_length().powi(2) - self.radius.powi(2);

        let delta = h.powi(2) - a * c;
        if delta < 0.0 {
            return false;
        }

        let sq = delta.sqrt();

        // checks wich root is in the interval [r_min, r_max]
        // the hit only will "counts" if it is in this interval, is like a "draw distance"
        let mut root = (h - sq) / a;
        if root <= r_min || r_max <= root {
            root = (h + sq) / a;
            if root <= r_min || r_max <= root {
                return false;
            }
        }

        // get the closest point
        rec.t = root;
        rec.point = r.at(root);
        let outward_normal = (rec.point - self.center).div(self.radius);
        rec.set_face_normal(&r, outward_normal);

        return true;
    }
}
