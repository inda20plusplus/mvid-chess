use super::super::{Color, MainState, Overlay, Position, Piece, State};
use super::tools;
use chess::game;
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::Color as PColor;
use ggez::nalgebra as na;
use std::path;
fn cbackground() -> PColor {
    PColor::from_rgb(0, 0, 66)
}
fn cdarktile() -> PColor {
    PColor::from_rgb(112, 162, 163)
}
fn clighttile() -> PColor {
    PColor::from_rgb(177, 228, 185)
}
fn cselect() -> PColor {
    PColor::from_rgba(100, 100, 100, 150)
}
fn cpossible() -> PColor {
    PColor::from_rgba(0, 200, 0, 150)
}
pub fn playing(mainstate: &mut MainState, ctx: &mut ggez::Context) {
    super::tools::background(ctx, cbackground());
    tools::draw_tile(ctx, &Position(9, 4), cdarktile());
    tools::draw_tile(ctx, &Position(10, 5), cdarktile());
    tools::draw_tile(ctx, &Position(9, 5), clighttile());
    tools::draw_tile(ctx, &Position(10, 4), clighttile());
    tools::draw_piece(ctx, Position(9, 4), Piece::Queen(mainstate.turn.clone()));
    tools::draw_piece(ctx, Position(10, 5), Piece::Knight(mainstate.turn.clone()));
    tools::draw_piece(ctx, Position(9, 5), Piece::Bishop(mainstate.turn.clone()));
    tools::draw_piece(ctx, Position(10, 4), Piece::Rook(mainstate.turn.clone()));
    for i in 0..8 {
        for j in 0..8 {
            tools::draw_tile(
                ctx,
                &Position(i, j),
                if (i + j) % 2 == 1 {
                    cdarktile()
                } else {
                    clighttile()
                },
            );
        }
    }
    match mainstate.state {
        State::Playing{promotion, check}=> {
            if !promotion{
                tools::draw_tile(ctx, &Position(9, 4), cselect());
                tools::draw_tile(ctx, &Position(10, 5), cselect());
                tools::draw_tile(ctx, &Position(9, 5), cselect());
                tools::draw_tile(ctx, &Position(10, 4), cselect());
                for i in 0..8 {
                    for j in 0..8 {
                        tools::draw_tile(
                            ctx,
                            &Position(i, j),
                            cselect()
                        );
                    }
                }
            };
            match check {
                true => tools::text(ctx, 875.0, 150.0, "Check."),
                false => tools::text(ctx, 875.0, 150.0, "Not Check"),
            }

        },
        _=>()
    };
    match mainstate.turn {
        Color::White => tools::text(ctx, 875.0, 100.0, "White's turn."),
        Color::Black => tools::text(ctx, 875.0, 100.0, "Black's turn."),
        _ => (),
    }
    for i in mainstate.board.0.iter() {
        tools::draw_piece(ctx, i.1.clone(), i.0.clone());
    }
    
    match mainstate.help.clone() {
        Overlay::Moves { selected, to } => {
            {
                tools::draw_tile(ctx, &selected, cselect());
            }
            for i in to.iter() {
                tools::draw_tile(ctx, i, cpossible());
            }
        }
        Overlay::None => (),
    }
}
