use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone)]
pub struct Vetor2 {
    x: f32,
    y: f32,
}

impl Vetor2 {
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

    pub fn normalize(&self) -> Self {
        let mag = self.mag();
        Self::new(self.x / mag, self.y / mag)
    }

    pub fn dot(&self, b: Self) -> f32 {
        self.x * b.get_x() + self.y + b.get_y()
    }

    pub fn cross(&self, v: Self) -> f32 {
        self.x * v.get_y() - self.y * v.get_x()
    }

    pub fn get_perpendicular(&self) -> Self {
        Self::new(-self.y, self.x)
    }

    pub fn projection(&self, v: Self) -> Self {
        let a = self.dot(v);
        let b = v.dot(v);
        let l = a / b;
        v * l
    }

    pub fn projection_normalized(&self, v_normalized: Self) -> Self {
        v_normalized * self.dot(v_normalized)
    }

    pub fn react(&self, n: Self, alpha: f32, beta: f32, is_normalized: bool) -> Self {
        let vn;
        if is_normalized {
            vn = self.projection_normalized(n);
        } else {
            vn = self.projection(n);
        }
        let vp = *self - vn;
        vp * alpha - vn * beta
    }

    pub fn slide(&self, n: Self, is_normalized: bool) -> Self {
        self.react(n, 1.0, 0.0, is_normalized)
    }

    pub fn reflect(&self, n: Self, is_normalized: bool) -> Self {
        self.react(n, 1.0, 1.0, is_normalized)
    }
}

// Adição, subtração e negação
impl Add<Self> for Vetor2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.get_x() + rhs.get_x(), self.get_y() + rhs.get_y())
    }
}

impl Sub<Self> for Vetor2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.get_x() - rhs.get_x(), self.get_y() - rhs.get_y())
    }
}

impl Neg for Vetor2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output::new(-self.get_x(), -self.get_x())
    }
}

// Multiplicação e divisão por escalar
impl Mul<f32> for Vetor2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.get_x() * rhs, self.get_y() * rhs)
    }
}

impl Div<f32> for Vetor2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output::new(self.get_x() / rhs, self.get_y() / rhs)
    }
}
