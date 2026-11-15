use crate::vetor3::Vetor3;
use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
pub struct Ponto3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Ponto3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y
    }

    pub fn set_z(&mut self, z: f32) {
        self.z = z
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_z(&self) -> f32 {
        self.z
    }

    pub fn get_point(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
}

impl Sub<Self> for Ponto3 {
    type Output = Vetor3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            self.get_x() - rhs.get_x(),
            self.get_y() - rhs.get_y(),
            self.get_z() - rhs.get_z(),
        )
    }
}

impl Sub<Vetor3> for Ponto3 {
    type Output = Self;

    fn sub(self, rhs: Vetor3) -> Self::Output {
        Self::Output::new(
            self.get_x() - rhs.get_x(),
            self.get_y() - rhs.get_y(),
            self.get_z() - rhs.get_z(),
        )
    }
}

impl Add<Vetor3> for Ponto3 {
    type Output = Self;

    fn add(self, rhs: Vetor3) -> Self::Output {
        Self::Output::new(
            self.get_x() + rhs.get_x(),
            self.get_y() + rhs.get_y(),
            self.get_z() + rhs.get_z(),
        )
    }
}
