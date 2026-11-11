use std::array;

use raylib::prelude::*;
mod vetor;

fn main() {
    let width = 640.0;
    let height = 480.0;
    let safe_zone = 0.8;
    let max_y = 3;
    let max_x = 5;
    let tam_unit = 100;
    let y_lines: [[(f32, f32); 2]; 5] = array::from_fn(|i| {
        [
            ((i * tam_unit) as f32 + 10.0, height),
            ((i * tam_unit) as f32 + 10.0, 0.0),
        ]
    }); // Os eixos por enquanto serão linhas normais, mas devo mudar isso depois
    let x_lines: [[(f32, f32); 2]; 3] = array::from_fn(|i| {
        [
            (0.0 as f32, height - 10.0 - (i * tam_unit) as f32),
            (width as f32, height - 10.0 - (i * tam_unit) as f32),
        ]
    }); // Os eixos por enquanto serão linhas normais, mas devo mudar isso depois

    let (mut rl, thread) = raylib::init()
        .size(width as i32, height as i32)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        let mouse_position = rl.get_mouse_position();
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        for line in x_lines {
            d.draw_line_ex(
                Vector2::from(line[0]),
                Vector2::from(line[1]),
                5.0,
                Color::BLACK,
            );
        }
        for line in y_lines {
            d.draw_line_ex(
                Vector2::from(line[0]),
                Vector2::from(line[1]),
                5.0,
                Color::BLACK,
            );
        }
    }
}
