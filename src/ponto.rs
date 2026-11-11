use std::ops::Sub;

pub struct Ponto {
    a: f32,
    b: f32,
}

impl Ponto {
    pub fn new(a: f32, b: f32) -> Self {
        Self { a, b }
    }

    pub fn set_a(&mut self, a: f32) {
        self.a = a
    }

    pub fn set_b(&mut self, b: f32) {
        self.b = b
    }

    pub fn get_a(&self) -> f32 {
        self.a
    }

    pub fn get_b(&self) -> f32 {
        self.b
    }
}

impl Sub for Ponto {
    type Output = Vetor;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            a: self.a - other.a,
            b: self.b - other.b,
        }
    }
}
