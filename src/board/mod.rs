mod tests;

use crate::pieces::{Kind, Piece};
use crate::{Color, Point};
use std::collections::HashMap;

pub struct Board {
    current: HashMap<Point, Piece>,
}

impl Board {
    fn new() -> Self {
        let mut starting_positions: Vec<(Point, Piece)> = vec![
            (Point(1, 1), Piece::new(Color::White, Kind::Rook)),
            (Point(2, 1), Piece::new(Color::White, Kind::Knight)),
            (Point(3, 1), Piece::new(Color::White, Kind::Bishop)),
            (Point(4, 1), Piece::new(Color::White, Kind::Queen)),
            (Point(5, 1), Piece::new(Color::White, Kind::King)),
            (Point(6, 1), Piece::new(Color::White, Kind::Bishop)),
            (Point(7, 1), Piece::new(Color::White, Kind::Knight)),
            (Point(8, 1), Piece::new(Color::White, Kind::Rook)),
            (Point(1, 8), Piece::new(Color::Black, Kind::Rook)),
            (Point(2, 8), Piece::new(Color::Black, Kind::Knight)),
            (Point(3, 8), Piece::new(Color::Black, Kind::Bishop)),
            (Point(4, 8), Piece::new(Color::Black, Kind::Queen)),
            (Point(5, 8), Piece::new(Color::Black, Kind::King)),
            (Point(6, 8), Piece::new(Color::Black, Kind::Bishop)),
            (Point(7, 8), Piece::new(Color::Black, Kind::Knight)),
            (Point(8, 8), Piece::new(Color::Black, Kind::Rook)),
        ];
        for i in 1..=8 {
            starting_positions.push((Point(i, 2), Piece::new(Color::White, Kind::Pawn)));
        }
        for i in 1..=8 {
            starting_positions.push((Point(i, 7), Piece::new(Color::Black, Kind::Pawn)));
        }

        let starting_board: HashMap<Point, Piece> = starting_positions.into_iter().collect();
        Board {
            current: starting_board,
        }
    }
}
