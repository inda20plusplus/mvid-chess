#[cfg(test)]
pub mod test_setup;

#[cfg(test)]
mod test_move_piece;

#[cfg(test)]
mod test_get_moves;

use crate::pieces::{Kind, Piece};
use crate::{Color, Point};
use std::collections::HashMap;

pub struct Board {
    pub current: HashMap<Point, Piece>,
    pub graveyard: HashMap<Color, Vec<Piece>>,
    pub king_pos: HashMap<Color, Point>,
}

impl Board {
    pub fn new() -> Self {
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
            graveyard: vec![(Color::White, vec![]), (Color::Black, vec![])]
                .into_iter()
                .collect(),
            king_pos: vec![(Color::White, Point(5, 1)), (Color::Black, Point(5, 8))]
                .into_iter()
                .collect(),
        }
    }

    pub fn move_piece(&mut self, source: Point, target: Point) -> bool {
        if !(1..=8).contains(&target.0) || !(1..=8).contains(&target.1) {
            return false;
        } else if source == target {
            return false;
        }

        let source_piece_ref = match self.current.get(&source) {
            Some(piece) => piece,
            None => return false,
        };

        if let Some(target_piece_ref) = self.current.get(&target) {
            if target_piece_ref.color == source_piece_ref.color {
                return false;
            } else {
                let target_piece = self.current.remove(&target).unwrap();
                let graveyard = self.graveyard.entry(target_piece.color.clone()).or_default();
                graveyard.push(target_piece);
            }
        }

        let mut source_piece = self.current.remove(&source).unwrap();
        source_piece.has_moved = true;

        if source_piece.kind == Kind::King {
            self.king_pos.insert(source_piece.color.clone(), target.clone());
        };

        self.current.insert(target, source_piece);

        true
    }
}
