pub struct Vetor {
    a: (f32, f32),
    b: (f32, f32),
    thickness: f32,
}

impl Vetor {
    pub fn new(a: (f32, f32), b: (f32, f32), thickness: f32) -> Self {
        Self { a, b, thickness }
    }

    pub fn set_a(&mut self, a: (f32, f32)) {
        self.a = a
    }

    pub fn set_b(&mut self, b: (f32, f32)) {
        self.b = b
    }

    pub fn get_a(&self) -> (f32, f32) {
        self.a
    }

    pub fn get_b(&self) -> (f32, f32) {
        self.b
    }
}
