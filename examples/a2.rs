use matematica_e_fisica_para_jogos::{
    draw::{PosTexto, draw_arrow},
    ponto2::Ponto2,
    vetor2::Vetor2,
};
use rand::seq::SliceRandom;
use raylib::prelude::*;

fn main() {
    let width = 760;
    let height = (width as f32 / 1.3333333) as i32;
    let (mut rl, thread) = raylib::init().size(width, height).title("A2").build();

    rl.set_target_fps(1);

    let r = || rand::random_range(150..500) as f32;

    // Definição dos pontos aleatórios
    let ponto_inicial = Ponto2::new(r(), r());
    let ponto_inicial_v = Ponto2::new(r(), r());
    let ponto_inicial_w = Ponto2::new(r(), r());
    let ponto_final_w = Ponto2::new(r(), r());

    // Vetores
    let u = ponto_inicial_v - ponto_inicial;
    let v = ponto_inicial_w - ponto_inicial_v;
    let w = ponto_final_w - ponto_inicial_w;
    let mut vetores = vec![
        (u, "u", Color::BLUE),
        (v, "v", Color::GREEN),
        (w, "w", Color::PURPLE),
    ];
    let soma = u + v + w;

    while !rl.window_should_close() {
        vetores.shuffle(&mut rand::rng());
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        draw_arrow(
            &mut d,
            ponto_inicial,
            vetores[0].0,
            1.0,
            vetores[0].2,
            Some((vetores[0].1.to_string(), PosTexto::Meio)),
            None,
        );

        let ponto_inicial1 = ponto_inicial + vetores[0].0;

        draw_arrow(
            &mut d,
            ponto_inicial1,
            vetores[1].0,
            1.0,
            vetores[1].2,
            Some((vetores[1].1.to_string(), PosTexto::Meio)),
            None,
        );

        let ponto_inicial2 = ponto_inicial1 + vetores[1].0;

        draw_arrow(
            &mut d,
            ponto_inicial2,
            vetores[2].0,
            1.0,
            vetores[2].2,
            Some((vetores[2].1.to_string(), PosTexto::Meio)),
            None,
        );

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
