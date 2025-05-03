use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub};

use crate::utils::random_Interval_f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VecTypes {
    Color,
    #[default]
    Coordinates,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub typ: VecTypes,
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(typ: VecTypes, x: f64, y: f64, z: f64) -> Self {
        Vec3 { typ, x, y, z }
    }

    // retornar x, y, z pelo indice apenas
    pub fn get(&self, i: u8) -> f64 {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Error on index Vec3 componenets"),
        }
    }

    pub fn vec_length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // soma a partir de um numero, retornando um novo Vec3
    pub fn sum(&self, s: f64) -> Self {
        Vec3 {
            typ: self.typ,
            x: self.x + s,
            y: self.y + s,
            z: self.z + s,
        }
    }

    pub fn mul(&self, m: f64) -> Self {
        Vec3 {
            typ: self.typ,
            x: self.x * m,
            y: self.y * m,
            z: self.z * m,
        }
    }

    pub fn dot(&self, d: &Self) -> f64 {
        if self.typ != d.typ {
            panic!("Diferents Vec3Types try dotted");
        }
        self.x * d.x + self.y * d.y + self.z * d.z
    }

    pub fn div(&self, u: f64) -> Self {
        Vec3 {
            typ: self.typ,
            x: self.x / u,
            y: self.y / u,
            z: self.z / u,
        }
    }

    pub fn unit_vec(&self) -> Self {
        self.div(self.vec_length())
    }

    pub fn random() -> Self {
        Vec3::random_max_min(0.0, 1.0)
    }

    pub fn random_max_min(min: f64, max: f64) -> Self {
        Vec3 {
            typ: VecTypes::Coordinates,
            x: random_Interval_f64(min, max),
            y: random_Interval_f64(min, max),
            z: random_Interval_f64(min, max),
        }
    }

    pub fn random_unit_vec() -> Self {
        loop {
            let p = Vec3::random_max_min(-1.0, 1.0);
            let lensq = p.vec_length().powi(2);
            if lensq <= 1.0 && 1e-160 < lensq {
                // the first is for get a vec into a sphere with r = 1
                // the second is for evite infinite cords
                return p.unit_vec();
            }
        }
    }

    // get a random direction vector pointed to outside hemisphere,
    // based on the dot op begin > 0, the angle is (90, -90)
    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let p = Self::random_unit_vec();
        if p.dot(normal) > 0.0 {
            return p;
        }
        // invert the random
        // need this syntax because the theres the mul trait
        Vec3::mul(&p, -1.0)
    }

    // return tre is the vector is very close to zero in all cords
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    // ideal reflect of a ray into a metal surface, like a glass
    pub fn reflected_vec(&self, normal: &Vec3) -> Self {
        *self - Self::mul(normal, 2.0 * self.dot(normal))
    }

    // refract a vector using lens law, where refraction_const is N/N'
    pub fn refract(&self, normal: &Vec3, refraction_const: f64) -> Self {
        // angle of normal and self, both need to be as unit
        let cos_theta = self.mul(-1.0).dot(normal).min(1.0);
        // perpendicular comp of refracted vector
        let r_out_perp = Self::mul(&self.add(normal.mul(cos_theta)), refraction_const);
        // parallel comp
        let r_out_para = Self::mul(normal, (1.0 - r_out_perp.vec_length().powi(2)).abs().sqrt());
        r_out_perp + Self::mul(&r_out_para, -1.0)
    }

    // for defocus blur
    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(
                VecTypes::Coordinates,
                random_Interval_f64(-1.0, 1.0),
                random_Interval_f64(-1.0, 1.0),
                0.0,
            );
            if p.vec_length() < 1.0 {
                return p;
            }
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            typ: self.typ,
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            typ: self.typ,
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            typ: self.typ,
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Error on index Vec3 componenets"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Error on index Vec3 componenets"),
        }
    }
}
