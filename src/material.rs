use crate::{
    color::Color,
    ray::{Ray, hit_record},
};

pub trait Material {
    fn reflect(r_in: &Ray, r_ref: &Ray, rec: &hit_record, attenuation: &Color) -> bool;
}
