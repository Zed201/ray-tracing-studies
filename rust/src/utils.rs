use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign};

#[derive(Debug)]
enum VecTypes {
    Color,
    Coordinates,
}

impl Default for VecTypes {
    fn default() -> Self {
        VecTypes::Coordinates
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    pub typ: VecTypes,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(typ: VecTypes, x: f32, y: f32, z: f32) -> Self {
        Vec3 { typ, x, y, z }
    }

    // retornar x, y, z pelo indice apenas
    pub fn get(i: u8) -> f32 {
        todo!()
    }

    pub fn vec_length() -> f32 {
        todo!()
    }

    pub fn sum(s: f32) -> Self {
        todo!()
    }

    pub fn mul(m: f32) -> Self {
        todo!()
    }

    pub fn dot(d: Self) -> f32 {
        todo!()
    }

    pub fn unit_vec(&self) -> Self {
        return self / self.z;
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        todo!()
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        todo!()
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        todo!()
    }
}
