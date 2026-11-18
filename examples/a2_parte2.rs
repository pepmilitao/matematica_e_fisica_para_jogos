use matematica_e_fisica_para_jogos::{
    draw::{PosTexto, draw_arrow},
    ponto2::Ponto2,
    vetor2::Vetor2,
};
use rand::seq::SliceRandom;
use raylib::prelude::*;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse::<usize>().unwrap() + 1;
    let width = 760;
    let height = (width as f32 / 1.3333333) as i32;
    let (mut rl, thread) = raylib::init().size(width, height).title("A2").build();

    rl.set_target_fps(1);

    let r = || rand::random_range(150..500) as f32;

    // Definição dos pontos aleatórios
    let mut pontos = Vec::<Ponto2>::new();
    for i in 0..n {
        pontos.push(Ponto2::new(r(), r()))
    }

    // Definição dos vetores
    let mut vetores = Vec::<Vetor2>::new();
    for i in 1..n {
        vetores.push(pontos[i] - pontos[i - 1])
    }
    vetores.push(Vetor2::new(0.0, 0.0));

    // Vetor soma
    let soma = vetores
        .iter()
        .fold(Vetor2::new(0.0, 0.0), |acc, i| acc + *i);
    let ponto_inicial = pontos[0];

    while !rl.window_should_close() {
        vetores.shuffle(&mut rand::rng());
        let mut d = rl.begin_drawing(&thread);
        let mut ponto_atual = ponto_inicial;

        d.clear_background(Color::WHITE);

        for i in &vetores {
            draw_arrow(&mut d, ponto_atual, *i, 1.0, Color::BLACK, None, None);
            ponto_atual = ponto_atual + *i;
        }

        draw_arrow(
            &mut d,
            ponto_inicial,
            soma,
            1.0,
            Color::RED,
            Some(("soma".to_string(), PosTexto::Meio)),
            None,
        );
    }
}
