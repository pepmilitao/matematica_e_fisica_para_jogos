use std::ops::{Div, Mul, Neg, Sub};

use raylib::ffi::sqrtf;

#[derive(Copy, Clone)]
pub struct Vetor {
    x: f32,
    y: f32,
}

impl Vetor {
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

    pub fn mag(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn norm(&self) -> Self {
        let mag = self.mag();
        Self::new(self.x / mag, self.y / mag)
    }

    pub fn get_perpendicular(&self) -> Self {
        Self::new(-self.y, self.x)
    }
}

impl Mul<Vetor> for Vetor {
    type Output = Vetor;

    fn mul(self, rhs: Vetor) -> Vetor {
        Vetor::new(self.get_x() * rhs.get_x(), self.get_y() * rhs.get_y())
    }
}

impl Mul<f32> for Vetor {
    type Output = Vetor;

    fn mul(self, rhs: f32) -> Vetor {
        Vetor::new(self.get_x() * rhs, self.get_y() * rhs)
    }
}

impl Div for Vetor {
    type Output = Vetor;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.get_x() / rhs.get_x(), self.get_y() / rhs.get_y())
    }
}

impl Neg for Vetor {
    type Output = Vetor;

    fn neg(self) -> Self::Output {
        Self::Output::new(-self.get_x(), -self.get_x())
    }
}
