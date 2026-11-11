pub struct Vetor {
    a: f32,
    b: f32,
}

impl Vetor {
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
