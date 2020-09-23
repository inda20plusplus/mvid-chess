use super::super::{Color, Piece, Position};
use chess::game;
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use std::path;
pub fn draw_piece(ctx: &mut ggez::Context, position: Position, piece: Piece) -> () {
    let mut path;
    match piece {
        Piece::King(color) => {
            path = if color == Color::White {
                "/pieces/Chess_klt60.png"
            } else {
                "/pieces/Chess_kdt60.png"
            }
        }
        Piece::Queen(color) => {
            path = if color == Color::White {
                "/pieces/Chess_qlt60.png"
            } else {
                "/pieces/Chess_qdt60.png"
            }
        }
        Piece::Rook(color) => {
            path = if color == Color::White {
                "/pieces/Chess_rlt60.png"
            } else {
                "/pieces/Chess_rdt60.png"
            }
        }
        Piece::Pawn(color) => {
            path = if color == Color::White {
                "/pieces/Chess_plt60.png"
            } else {
                "/pieces/Chess_pdt60.png"
            }
        }
        Piece::Bishop(color) => {
            path = if color == Color::White {
                "/pieces/Chess_blt60.png"
            } else {
                "/pieces/Chess_bdt60.png"
            }
        }
        Piece::Knight(color) => {
            path = if color == Color::White {
                "/pieces/Chess_nlt60.png"
            } else {
                "/pieces/Chess_ndt60.png"
            }
        }
        Piece::None => return,
    }
    let image = graphics::Image::new(ctx, path).unwrap();
    graphics::draw(
        ctx,
        &image,
        (na::Point2::new(
            70.0 + 100.0 * position.0 as f32,
            70.0 + 100.0 * (7 - position.1) as f32,
        ),),
    );
}
pub fn text(ctx: &mut ggez::Context, x: f32, y: f32, to_draw: &str) {
    let font = graphics::Font::new(ctx, "/DejaVuSansMono.ttf");
    let text = graphics::Text::new((to_draw, font.unwrap(), 48.0));
    let dest_point = na::Point2::new(x, y);
    graphics::draw(ctx, &text, (dest_point,));
}
pub fn draw_tile(ctx: &mut ggez::Context, position: &Position, color: graphics::Color) -> () {
    let mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect {
            x: 50.0 + 100.0 * (position.0) as f32,
            y: 50.0 + 100.0 * (7 - position.1) as f32,
            w: 100.0,
            h: 100.0,
        },
        color,
    )
    .unwrap();
    graphics::draw(ctx, &mesh, (na::Point2::new(0.0, 0.0),));
}

pub fn background(ctx: &mut ggez::Context, color: graphics::Color) {
    let mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: super::super::WindowSize.0,
            h: super::super::WindowSize.1,
        },
        color,
    )
    .unwrap();
    graphics::draw(ctx, &mesh, (na::Point2::new(0.0, 0.0),));
}
