use crate::{
    color::Color,
    material::{DefaultMaterial, Material},
    vec::{self, Vec3},
};

#[derive(Debug, Clone, Copy, Default)]
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

// Hittable objects
pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, r_max: f64, r_min: f64, rec: &mut HitRecord) -> bool;
    // the idea is check if the t is t_min < t < t_max and save this in HitRecord
    // there best ways to do that but i will upgrade later
}

pub struct HittableList {
    objs: Vec<Box<dyn Hittable + Sync>>, // vec de objetos que tem a trait hittable
}
impl HittableList {
    pub fn new() -> Self {
        HittableList { objs: Vec::new() }
    }
    pub fn add(&mut self, h: Box<dyn Hittable + Sync>) {
        self.objs.push(h);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, r_max: f64, r_min: f64, rec: &mut HitRecord) -> bool {
        let mut hit_any = false;
        let mut closest_so_far = r_max;
        let mut tmp_record = HitRecord::default();

        for i in &self.objs {
            // find the some hit, update the r_max finding the the clesest
            if i.hit(r, closest_so_far, r_min, &mut tmp_record) {
                hit_any = true;
                closest_so_far = tmp_record.t;
                *rec = tmp_record.clone();
            }
        }

        return hit_any;
    }
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    // add material, but the trait is complex to add here
    pub mat: Box<dyn Material>,
}

impl Clone for HitRecord {
    fn clone(&self) -> Self {
        HitRecord {
            point: self.point.clone(),
            normal: self.normal.clone(),
            t: self.t.clone(),
            front_face: self.front_face.clone(),
            // Precisamos clonar o conteúdo da Box.
            // Isso requer que o trait Material também suporte clonagem de objetos (object-safe Clone).
            mat: self.mat.clone_box(),
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            point: Vec3::default(),
            normal: Vec3::default(),
            t: f64::default(),
            front_face: bool::default(),
            mat: Box::new(DefaultMaterial {}),
        }
    }
}

// the normal vector is point to outside/outward
// if is same direction of ray the ray is inside, cos is upper 0
// if is oposite of ray the ray is outside, cos is under 0
impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
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

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Box<dyn Material + Sync>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Box<dyn Material + Sync>) -> Self {
        Sphere {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }

    pub fn to_box(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, r_max: f64, r_min: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin;
        let a = r.direction.vec_length().powi(2);
        let h = r.direction.dot(&oc);
        let c = oc.vec_length().powi(2) - self.radius.powi(2);

        let delta = h.powi(2) - a * c;
        if delta < 0.0 {
            return false;
        }

        let sq = delta.sqrt();

        // checks wich root is in the Interval [r_min, r_max]
        // the hit only will "counts" if it is in this Interval, is like a "draw distance"
        // but add other objects make the limit vision with objects in front
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

        // can't impl copy basic for a trait
        rec.mat = self.mat.clone_box();
        return true;
    }
}
