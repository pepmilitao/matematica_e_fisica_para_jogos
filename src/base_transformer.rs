const VALORES: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

pub struct BaseTransformer {
    numero: String,
    base: u32,
}

impl BaseTransformer {
    pub fn new() -> Self {
        Self {
            numero: String::new(),
            base: 0,
        }
    }

    pub fn set_numero(&mut self, numero: String) {
        self.numero = numero;
    }

    pub fn get_numero(&self) -> String {
        self.numero.clone()
    }

    pub fn set_base(&mut self, base: u32) {
        self.base = base;
    }

    pub fn get_base(&self) -> u32 {
        self.base
    }

    pub fn to_10(&mut self) {
        assert_ne!(self.base, 10);

        let rev_numero: String = self.numero.chars().rev().collect();

        self.numero = rev_numero
            .char_indices()
            .fold(0, |acc, (indice, valor)| {
                acc + valor.to_digit(self.base).unwrap() * self.base.pow(indice as u32)
            })
            .to_string();
        self.base = 10;
    }

    pub fn to_base(&mut self, base: u32) {
        if self.base != 10 {
            self.to_10();
        }
        let mut numero = self.numero.parse::<u32>().unwrap();
        let mut novo_numero = String::new();
        while numero > 0 {
            novo_numero = VALORES[(numero % base) as usize].to_string() + novo_numero.as_str();
            numero /= base;
        }
        self.numero = novo_numero;
        self.base = base;
    }
}
