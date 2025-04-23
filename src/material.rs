use crate::{
    color::Color,
    ray::{HitRecord, Ray},
};

pub trait Material {
    fn reflect(&self, r_in: &Ray, r_ref: &Ray, rec: &HitRecord, attenuation: &Color) -> bool;

    // used for "clone trait" in HitRecord
    fn clone_box(&self) -> Box<dyn Material>;
}

// a DefaultMaterial for default impl in HitRecord
pub struct DefaultMaterial {}

impl Material for DefaultMaterial {
    fn reflect(&self, r_in: &Ray, r_ref: &Ray, rec: &HitRecord, attenuation: &Color) -> bool {
        true
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(Self {})
    }
}
