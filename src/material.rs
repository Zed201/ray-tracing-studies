use crate::{
    color::Color,
    ray::{HitRecord, Ray},
    vec::Vec3,
};

pub trait Material: Sync {
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

#[allow(unused)]
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
        // let reflect_dir = r_in.direction.reflected_vec(&rec.normal);
        // *r_ref = Ray::new(rec.point, reflect_dir);
        // *attenuation = self.albedo;
        // true
        let mut ref_dir = rec.normal + Vec3::random_unit_vec();
        if ref_dir.near_zero() {
            ref_dir = rec.normal;
        }

        *r_ref = Ray::new(rec.point, ref_dir);
        *attenuation = self.albedo;
        true
    }
}

// TODO: put albedo color in trait and new funciton(see how place funciton in trait)

pub struct Metal {
    albedo: Color,
    // fuzz is [0.0, 1.0], 1.0 is total random, like matte metal, 0.0 is total reflected metal
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let f = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz: f }
    }
}

impl Material for Metal {
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(Self {
            albedo: self.albedo,
            fuzz: self.fuzz,
        })
    }
    fn reflect(
        &self,
        r_in: &Ray,
        r_ref: &mut Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
    ) -> bool {
        let mut reflect_dir = r_in.direction.reflected_vec(&rec.normal);
        // add fuzzy reflection, let the metal matte
        reflect_dir = reflect_dir.unit_vec() + Vec3::random_unit_vec().mul(self.fuzz);
        *r_ref = Ray::new(rec.point, reflect_dir);
        *attenuation = self.albedo;
        true
    }
}
