use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub};

use crate::utils::{self, random_interval_f64, randon_f64};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VecTypes {
    Color,
    Coordinates,
}

impl Default for VecTypes {
    fn default() -> Self {
        VecTypes::Coordinates
    }
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
            0 => return self.x,
            1 => return self.y,
            2 => return self.z,
            _ => panic!("Error on index Vec3 componenets"),
        }
    }

    pub fn vec_length(&self) -> f64 {
        return ((self.x * self.x + self.y * self.y + self.z * self.z) as f64).sqrt();
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
        return self.x * d.x + self.y * d.y + self.z * d.z;
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
        return self.div(self.vec_length());
    }

    pub fn random() -> Self {
        Vec3::random_max_min(0.0, 1.0)
    }

    pub fn random_max_min(min: f64, max: f64) -> Self {
        Vec3 {
            typ: VecTypes::Coordinates,
            x: random_interval_f64(min, max),
            y: random_interval_f64(min, max),
            z: random_interval_f64(min, max),
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
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.typ != rhs.typ {
            panic!("Error add diferents VecTypes");
        }
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
        if self.typ != rhs.typ {
            panic!("Error add_assign diferents VecTypes");
        }
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.typ != rhs.typ {
            panic!("Error add diferents VecTypes");
        }
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
        if self.typ != rhs.typ {
            panic!("Error mult diferents VecTypes");
        }
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
            0 => return &self.x,
            1 => return &self.y,
            2 => return &self.z,
            _ => panic!("Error on index Vec3 componenets"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => return &mut self.x,
            1 => return &mut self.y,
            2 => return &mut self.z,
            _ => panic!("Error on index Vec3 componenets"),
        }
    }
}
