use rayon::prelude::*;
use std::fmt::Display;
use std::ops::Sub;

pub struct Conjunto<T> {
    conjunto: Vec<T>,
}

impl<T: Clone + Copy + Sync + Send + PartialEq + Display> Conjunto<T> {
    pub fn new() -> Self {
        Self {
            conjunto: Vec::new(),
        }
    }

    pub fn add(&mut self, n: T) {
        if !self.conjunto.contains(&n) {
            self.conjunto.push(n);
        }
    }

    pub fn remove(&mut self, n: T) {
        if !self.conjunto.contains(&n) {
            self.conjunto.retain(|x| *x != n);
        }
    }

    pub fn intersection(&self, b: Conjunto<T>) -> Self {
        let output = self;
        output
            .conjunto
            .par_iter()
            .cloned()
            .filter(|x| b.conjunto.contains(x))
            .collect()
    }

    pub fn union(&self, b: Conjunto<T>) -> Self {
        let mut output = self.clone();
        for i in b.conjunto {
            output.add(i);
        }
        output
    }

    pub fn contains(&self, n: T) -> bool {
        self.conjunto.contains(&n)
    }

    pub fn is_subset(&self, b: Self) -> bool {
        self.conjunto.par_iter().all(|e| b.contains(*e))
    }

    pub fn proper_subsets(&self) -> Vec<Self> {
        let mut output = Vec::new();
        let n = self.conjunto.len();
        for mascara in 0..(1 << n) {
            output.push(
                self.conjunto
                    .par_iter()
                    .enumerate()
                    .filter_map(|(indice, valor)| {
                        if (mascara & (1 << indice)) != 0 {
                            Some(*valor)
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
        }
        output
    }

    pub fn print(&self) {
        let mut first = true;
        print!("{{ ");
        for e in self.conjunto.clone() {
            if !first {
                print!(", ");
            }
            print!("{}", e);
            first = false;
        }
        println!(" }}");
    }
}

impl<T: Clone> Clone for Conjunto<T> {
    fn clone(&self) -> Self {
        Self {
            conjunto: self.conjunto.clone(),
        }
    }
}

impl<T: Send> FromParallelIterator<T> for Conjunto<T> {
    fn from_par_iter<I>(par_iter: I) -> Self
    where
        I: IntoParallelIterator<Item = T>,
    {
        Self {
            conjunto: par_iter.into_par_iter().collect(),
        }
    }
}

impl<T: Clone + Sync + Send + PartialEq> Sub<Self> for Conjunto<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let output = self;
        output
            .conjunto
            .par_iter()
            .cloned()
            .filter(|x| !rhs.conjunto.contains(x))
            .collect()
    }
}
