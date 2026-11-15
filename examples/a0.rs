use matematica_e_fisica_para_jogos::{
    draw::{PosTexto, draw_arrow},
    ponto2::Ponto2,
    vetor2::Vetor2,
};
use raylib::prelude::*;

fn main() {
    let width = 760;
    let height = (width as f32 / 1.3333333) as i32;
    let grid_spacing = 100;
    let (mut rl, thread) = raylib::init().size(width, height).title("A0").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // Fundo
        d.clear_background(Color::WHITE);

        // Grid
        for i in 0..(height / grid_spacing + 1) {
            d.draw_line_v(
                (0.0, (height - i * grid_spacing) as f32),
                (width as f32, (height - i * grid_spacing) as f32),
                Color::LIGHTGRAY,
            );
        }
        for i in 0..(width / grid_spacing + 1) {
            d.draw_line_v(
                ((i * grid_spacing) as f32, height as f32),
                ((i * grid_spacing) as f32, 0.0),
                Color::LIGHTGRAY,
            );
        }

        // Eixo x
        draw_arrow(
            &mut d,
            Ponto2::new(grid_spacing as f32, (height - 1 * grid_spacing) as f32),
            Vetor2::new(500.0, 0.0),
            1.0,
            Color::BLACK,
            Some(("x".to_string(), PosTexto::Final)),
            Some(grid_spacing),
        );

        // Eixo y
        draw_arrow(
            &mut d,
            Ponto2::new(
                (1 * grid_spacing) as f32,
                height as f32 - grid_spacing as f32,
            ),
            Vetor2::new(0.0, -400.0),
            1.0,
            Color::BLACK,
            Some(("y".to_string(), PosTexto::Final)),
            Some(grid_spacing),
        );

        // Pontos
        let a = Ponto2::new(1.0, 3.0);
        let b = Ponto2::new(4.0, 1.0);

        // Vetor
        let mut v = b - a;
        v.set_y(-v.get_y());

        // Desenhar vetor
        draw_arrow(
            &mut d,
            Ponto2::new(
                (a.get_x() as i32 * grid_spacing + grid_spacing) as f32,
                (height - a.get_y() as i32 * grid_spacing - grid_spacing) as f32,
            ),
            v * grid_spacing as f32,
            3.0,
            Color::PURPLE,
            Some((String::from("v"), PosTexto::Meio)),
            None,
        );

        // Desenhar pontos
        d.draw_circle(
            a.get_x() as i32 * grid_spacing + grid_spacing,
            height - a.get_y() as i32 * grid_spacing - grid_spacing,
            5.0,
            Color::BLUE,
        );

        d.draw_text(
            "A",
            a.get_x() as i32 * grid_spacing + grid_spacing + 5,
            height - a.get_y() as i32 * grid_spacing - grid_spacing + 5,
            20,
            Color::BLACK,
        );

        d.draw_circle(
            b.get_x() as i32 * grid_spacing + grid_spacing,
            height - b.get_y() as i32 * grid_spacing - grid_spacing,
            5.0,
            Color::BLUE,
        );

        d.draw_text(
            "B",
            b.get_x() as i32 * grid_spacing + grid_spacing + 5,
            height - b.get_y() as i32 * grid_spacing - grid_spacing + 5,
            20,
            Color::BLACK,
        );
    }
}
