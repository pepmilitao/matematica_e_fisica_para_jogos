use crate::vetor2::Vetor2;
use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
pub struct Ponto2 {
    x: f32,
    y: f32,
}

impl Ponto2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_point(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl Sub<Self> for Ponto2 {
    type Output = Vetor2;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output::new(self.get_x() - other.get_x(), self.get_y() - other.get_y())
    }
}

impl Sub<Vetor2> for Ponto2 {
    type Output = Self;

    fn sub(self, other: Vetor2) -> Self::Output {
        Self::Output::new(self.get_x() - other.get_x(), self.get_y() - other.get_y())
    }
}

impl Add<Vetor2> for Ponto2 {
    type Output = Self;

    fn add(self, other: Vetor2) -> Self::Output {
        Self::Output::new(self.get_x() + other.get_x(), self.get_y() + other.get_y())
    }
}
