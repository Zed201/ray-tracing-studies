use crate::{
    color::Color,
    ray::{HitRecord, Ray},
    vec::Vec3,
};

pub trait Material {
    // or scatter
    fn reflect(
        &self,
        r_in: &Ray,
        r_ref: &mut Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
    ) -> bool;

    // used for "clone trait" in HitRecord
    fn clone_box(&self) -> Box<dyn Material>;
}

// a DefaultMaterial for default impl in HitRecord
pub struct DefaultMaterial {}

#[allow(unused)]
impl Material for DefaultMaterial {
    fn reflect(
        &self,
        r_in: &Ray,
        r_ref: &mut Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
    ) -> bool {
        true
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(Self {})
    }
}

// albedo -> latim of whiteness
// lambertian will be a diffuse Material
// with it albedo
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(Self {
            albedo: self.albedo,
        })
    }
    fn reflect(
        &self,
        r_in: &Ray,
        r_ref: &mut Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
    ) -> bool {
        let reflect_dir = r_in.direction.reflected_vec(&rec.normal);
        *r_ref = Ray::new(rec.point, reflect_dir);
        *attenuation = self.albedo;
        true
    }
}
