use crate::{
    color::Color,
    ray::{HitRecord, Ray},
};

pub trait Material {
    fn reflect(r_in: &Ray, r_ref: &Ray, rec: &HitRecord, attenuation: &Color) -> bool;
}
