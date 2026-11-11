use raylib::prelude::*;
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
                Vector2::from((0.0, (height - i * grid_spacing) as f32)),
                Vector2::from((width as f32, (height - i * grid_spacing) as f32)),
                Color::LIGHTGRAY,
            );
        }
        for i in 0..(width / grid_spacing + 1) {
            d.draw_line_v(
                Vector2::from(((i * grid_spacing) as f32, height as f32)),
                Vector2::from(((i * grid_spacing) as f32, 0.0)),
                Color::LIGHTGRAY,
            );
        }

        // Eixo x
        d.draw_line_ex(
            Vector2::from((
                0.5 * grid_spacing as f32,
                (height - 1 * grid_spacing) as f32,
            )),
            Vector2::from(((width - 50) as f32, (height - 1 * grid_spacing) as f32)),
            3.0,
            Color::BLACK,
        );

        // Eixo y
        d.draw_line_ex(
            Vector2::from((
                (1 * grid_spacing) as f32,
                height as f32 - 0.5 * grid_spacing as f32,
            )),
            Vector2::from(((1 * grid_spacing) as f32, 50.0 as f32)),
            3.0,
            Color::BLACK,
        );

        // Pontos
        let a = Ponto::new(3.0, 2.0);
    }
}
