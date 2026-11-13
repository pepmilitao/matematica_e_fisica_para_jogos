use matematica_e_fisica_para_jogos::{ponto::Ponto, vetor::Vetor};
use raylib::prelude::*;

fn draw_arrow(
    draw_handle: &mut RaylibDrawHandle,
    ponto_inicial: Ponto,
    direcao: Vetor,
    expessura: f32,
    cor: Color,
) {
    let ponto_final = ponto_inicial + direcao;
    draw_handle.draw_line_ex(
        ponto_inicial.get_point(),
        ponto_final.get_point(),
        expessura,
        cor,
    );
    let direcao_norm = direcao.norm();
    let perpendicular = direcao_norm.get_perpendicular();

    let ponto_final_x = ponto_final.get_x();
    let ponto_final_y = ponto_final.get_y();
    let direcao_norm_x = direcao_norm.get_x();
    let direcao_norm_y = direcao_norm.get_y();
    let perpendicular_x = perpendicular.get_x();
    let perpendicular_y = perpendicular.get_y();

    draw_handle.draw_triangle(
        (
            ponto_final_x - 10.0 * direcao_norm_x + 4.0 * perpendicular_x,
            ponto_final_y - 10.0 * direcao_norm_y + 4.0 * perpendicular_y,
        ),
        ponto_final.get_point(),
        (
            ponto_final_x - 10.0 * direcao_norm_x - 4.0 * perpendicular_x,
            ponto_final_y - 10.0 * direcao_norm_y - 4.0 * perpendicular_y,
        ),
        cor,
    );
}

fn main() {
    let width = 640;
    let height = 480;
    let grid_spacing = 90;
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
            Ponto::new(
                0.5 * grid_spacing as f32,
                (height - 1 * grid_spacing) as f32,
            ),
            Vetor::new(500.0, 0.0),
            1.0,
            Color::BLACK,
        );

        // Eixo y
        draw_arrow(
            &mut d,
            Ponto::new(
                (1 * grid_spacing) as f32,
                height as f32 - 0.5 * grid_spacing as f32,
            ),
            Vetor::new(0.0, -400.0),
            1.0,
            Color::BLACK,
        );

        // Pontos
        let a = Ponto::new(1.0, 3.0);
        let b = Ponto::new(4.0, 1.0);

        // Vetor
        let mut v = b - a;
        v.set_y(-v.get_y());

        // Desenhar vetor
        draw_arrow(
            &mut d,
            Ponto::new(
                (a.get_x() as i32 * grid_spacing + grid_spacing) as f32,
                (height - a.get_y() as i32 * grid_spacing - grid_spacing) as f32,
            ),
            v * grid_spacing as f32,
            3.0,
            Color::PURPLE,
        );

        // Desenhar pontos
        d.draw_circle(
            a.get_x() as i32 * grid_spacing + grid_spacing,
            height - a.get_y() as i32 * grid_spacing - grid_spacing,
            5.0,
            Color::BLUE,
        );

        d.draw_circle(
            b.get_x() as i32 * grid_spacing + grid_spacing,
            height - b.get_y() as i32 * grid_spacing - grid_spacing,
            5.0,
            Color::BLUE,
        );
    }
}
