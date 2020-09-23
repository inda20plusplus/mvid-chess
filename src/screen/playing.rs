use super::super::MainState;
use chess::game;
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use std::path;

pub fn playing(mainstate: &mut MainState, ctx: &mut ggez::Context){
    let trim = board_edge(ctx);
    graphics::draw(ctx, &trim, (na::Point2::new(0.0, 0.0),));
    for i in 0..8 {
        for j in 0..8 {
            let color = if (i + j) % 2 == 1 {
                graphics::Color::from_rgb(112, 162, 163)
            } else {
                graphics::Color::from_rgb(177, 228, 185)
            };
            draw_tile(ctx, &Position(i, j), color);
        }
    }
    match self.turn {
        Color::White => text(ctx, 875.0, 100.0, "White's turn."),
        Color::Black => text(ctx, 875.0, 100.0, "Black's turn."),
    }
    for i in self.board.0.iter() {
        draw_piece(ctx, i.1.clone(), i.0.clone());
    }
    match self.help.clone() {
        Overlay::Moves { selected, to } => {
            {
                let color = graphics::Color::from_rgba(100, 100, 100, 150);
                draw_tile(ctx, &selected, color);
            }
            for i in to.iter() {
                let color = graphics::Color::from_rgba(0, 200, 0, 150);
                draw_tile(ctx, i, color);
            }
        }
        Overlay::None => (),
    }
}