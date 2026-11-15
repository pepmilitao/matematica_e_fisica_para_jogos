use crate::{ponto2::Ponto2, vetor2::Vetor2};
use raylib::prelude::*;

pub enum PosTexto {
    Meio,
    Final,
}

pub fn draw_arrow(
    draw_handle: &mut RaylibDrawHandle,
    ponto_inicial: Ponto2,
    direcao: Vetor2,
    expessura: f32,
    cor: Color,
    nome: Option<(String, PosTexto)>,
    numeros: Option<i32>,
) {
    let ponto_final = ponto_inicial + direcao;
    draw_handle.draw_line_ex(
        ponto_inicial.get_point(),
        ponto_final.get_point(),
        expessura,
        cor,
    );
    let direcao_norm = direcao.normalize();
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

    match nome {
        Some(v) => {
            let ponto_texto;
            match v.1 {
                PosTexto::Meio => ponto_texto = ponto_inicial + direcao / 2.0 + perpendicular * 4.0,
                PosTexto::Final => ponto_texto = ponto_final + perpendicular * 4.0,
            }
            draw_handle.draw_text(
                v.0.as_str(),
                ponto_texto.get_x() as i32,
                ponto_texto.get_y() as i32,
                20,
                Color::BLACK,
            );
        }
        None => {}
    }

    match numeros {
        Some(grid_spacing) => {
            let n = (direcao.mag() / grid_spacing as f32).ceil() as i32;
            for i in 1..n {
                let ponto_numero =
                    ponto_inicial + direcao_norm * (grid_spacing * i) as f32 + perpendicular * 4.0;
                draw_handle.draw_text(
                    &i.to_string(),
                    ponto_numero.get_x() as i32,
                    ponto_numero.get_y() as i32,
                    20,
                    cor,
                );
            }
        }
        None => {}
    }
}
