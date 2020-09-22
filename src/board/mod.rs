#[cfg(test)]
pub mod tests;

#[cfg(test)]
mod test_move_piece;

#[cfg(test)]
mod test_get_moves;

use crate::pieces::{Kind, Piece};
use crate::*;
use std::collections::HashMap;

pub struct Board {
    pub current: HashMap<Point, Piece>,
    pub graveyard: HashMap<Color, Vec<Piece>>,
    pub height: std::ops::RangeInclusive<i8>,
    pub width: std::ops::RangeInclusive<i8>,
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
            height: (1..=8),
            width: (1..=8),
        }
    }

    pub fn is_in_bounds(&self, point: &Point) -> bool {
        self.width.contains(&point.0) && self.height.contains(&point.1)
    }

    fn find_king(&self, color: &Color) -> Point {
        for x in self.width.clone() {
            for y in self.height.clone() {
                let current_point = Point(x, y);
                if let Some(piece) = self.current.get(&current_point) {
                    if piece.kind == Kind::King && &piece.color == color {
                        return current_point;
                    }
                }
            }
        }
        panic!("Couldn't find king");
    }

    pub fn detect_check(&self, color: &Color) -> Option<Vec<Point>> {
        let king = self.find_king(&color);

        let points: Vec<Point> = self.covered_by_opponent(&king, &color);

        if points.len() > 0 {
            Some(points)
        } else {
            None
        }
    }

    pub fn covered_by_opponent(&self, source: &Point, color: &Color) -> Vec<Point> {
        let opponent: Color = color.inverse();

        let mut covering_pieces: Vec<Point> = vec![];

        for mv in pieces::moves::ALL.iter() {
            let mut current_point: Point = source.clone().add(&mv.0);

            while self.is_in_bounds(&current_point) {
                if let Some(piece) = self.current.get(&current_point) {
                    if piece.color == opponent {
                        if self.get_moves_for_piece(&current_point).contains(&source) {
                            covering_pieces.push(current_point.clone());
                        }
                    }
                }

                if mv.1 {
                    current_point = current_point.add(&mv.0);
                } else {
                    break;
                }
            }
        }

        covering_pieces
    }

    pub fn move_piece(&mut self, source: Point, target: Point) -> bool {
        if !self.is_in_bounds(&target) {
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
                let graveyard = self
                    .graveyard
                    .entry(target_piece.color.clone())
                    .or_default();
                graveyard.push(target_piece);
            }
        }

        let mut source_piece = self.current.remove(&source).unwrap();

        source_piece.has_moved = true;

        self.current.insert(target, source_piece);

        true
    }

    pub fn get_allowed_moves(&mut self, source: &Point) -> Vec<Point> {
        let piece = match self.current.get(&source) {
            Some(p) => p.clone(),
            None => return vec![],
        };

        let mut moves: Vec<Point> = self.get_moves_for_piece(&source);

        let original = self.current.clone();

        let mut allowed_moves: Vec<Point> = vec![];

        for mv in moves.clone() {
            self.move_piece(source.clone(), mv.clone());

            if let None = self.detect_check(&piece.color) {
                allowed_moves.push(mv);
            };
            self.current = original.clone();
        }

        moves.retain(|point| allowed_moves.contains(&point));

        moves
    }

    fn get_moves_for_pawn(&self, source: &Point) -> Vec<Point> {
        let piece = self.current.get(&source).unwrap();
        if piece.kind != Kind::Pawn {
            panic!("Piece is not of kind pawn");
        };

        let direction = match piece.color {
            Color::White => Point(0, 1),
            Color::Black => Point(0, -1),
        };

        let mut moves: Vec<Point> = vec![];

        if !self.current.contains_key(&source.add(&direction)) {
            moves.push(source.add(&direction));
        };

        if let Some(target) = self.current.get(&source.add(&direction.add(&Point(1, 0)))) {
            if target.color != piece.color {
                moves.push(source.add(&direction.add(&Point(1, 0))));
            };
        };

        if let Some(target) = self.current.get(&source.add(&direction.add(&Point(-1, 0)))) {
            if target.color != piece.color {
                moves.push(source.add(&direction.add(&Point(-1, 0))));
            };
        };

        if !piece.has_moved
            && !self
                .current
                .contains_key(&source.add(&direction).add(&direction))
        {
            moves.push(source.add(&direction).add(&direction));
        };

        moves
    }

    fn get_moves_for_piece(&self, source: &Point) -> Vec<Point> {
        let piece = self.current.get(&source).unwrap();
        if piece.kind == Kind::Pawn {
            return self.get_moves_for_pawn(&source);
        };

        let mut moves: Vec<Point> = vec![];

        for mv in piece.get_moves() {
            let mut current_point = source.add(&mv.0);

            while self.is_in_bounds(&current_point) {
                if !self.current.contains_key(&current_point) {
                    moves.push(current_point.clone());
                } else {
                    let target_piece = self.current.get(&current_point).unwrap();
                    if target_piece.color != piece.color {
                        moves.push(current_point);
                    }
                    break;
                }

                if mv.1 {
                    current_point = current_point.add(&mv.0);
                } else {
                    break;
                };
            }
        }

        moves
    }
}
