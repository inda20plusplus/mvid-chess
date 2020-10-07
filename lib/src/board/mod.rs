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
    pub current: [Option<Piece>; 64],
    pub graveyard: HashMap<Color, Vec<Piece>>,
    pub height: std::ops::RangeInclusive<i8>,
    pub width: std::ops::RangeInclusive<i8>,
    pub enpassant: Option<[Point; 2]>,
}

impl Default for Board {
    fn default() -> Self {
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

        let mut starting_board: [Option<Piece>; 64] = [None; 64];
        for (pos, piece) in starting_positions {
            starting_board[pos.index()] = Some(piece);
        }

        Board {
            current: starting_board,
            graveyard: vec![(Color::White, vec![]), (Color::Black, vec![])]
                .into_iter()
                .collect(),
            height: (1..=8),
            width: (1..=8),
            enpassant: None,
        }
    }
}

impl Board {
    pub fn is_in_bounds(&self, point: &Point) -> bool {
        self.width.contains(&point.0) && self.height.contains(&point.1)
    }

    fn find_king(&self, color: &Color) -> Point {
        for x in self.width.clone() {
            for y in self.height.clone() {
                let current_point = Point(x, y);
                if let Some(piece) = self.current[current_point.index()] {
                    if piece.kind == Kind::King && &piece.color == color {
                        return current_point;
                    }
                }
            }
        }
        panic!("Couldn't find king");
    }

    // pub fn at_index(&self, index: usize) -> Option<Piece> {
    //     self.current[index]
    // }

    pub fn at_point(&self, point: &Point) -> Option<Piece> {
        self.current[point.index()]
    }

    pub fn detect_check(&self, color: &Color) -> Option<Vec<Point>> {
        let king = self.find_king(&color);

        let points: Vec<Point> = self.covered_by_opponent(&king, &color);

        if !points.is_empty() {
            Some(points)
        } else {
            None
        }
    }

    pub fn covered_by_opponent(&self, source: &Point, color: &Color) -> Vec<Point> {
        let opponent: Color = color.inverse();

        let mut covering_pieces: Vec<Point> = vec![];

        for mv in pieces::moves::ALL.iter() {
            let mut current_point = source.add(&mv.0);

            while self.is_in_bounds(&current_point) {
                if let Some(piece) = self.current[current_point.index()] {
                    if piece.color == opponent
                        && self.get_moves_for_piece(&current_point).contains(&source)
                    {
                        covering_pieces.push(current_point);
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
        if !self.is_in_bounds(&target) || source == target {
            return false;
        }

        let source_index = source.index();
        let target_index = target.index();

        let source_piece = match self.current[source_index] {
            Some(piece) => piece,
            None => return false,
        };

        if let Some(target_piece) = self.current[target_index] {
            if target_piece.color == source_piece.color {
                return false;
            } else {
                let graveyard = self.graveyard.entry(target_piece.color).or_default();
                graveyard.push(target_piece);
            }
        }

        let mut new_target_piece = source_piece;
        new_target_piece.has_moved = true;

        self.current[target_index] = Some(new_target_piece);
        self.current[source_index] = None;

        true
    }

    pub fn get_allowed_moves(&mut self, source: &Point) -> Option<Vec<Point>> {
        let piece = match &self.current[source.index()] {
            Some(p) => *p,
            None => return None,
        };

        let mut moves: Vec<Point> = self.get_moves_for_piece(&source);

        let original = self.current;

        let mut allowed_moves: Vec<Point> = vec![];

        for mv in &moves {
            self.move_piece(*source, *mv);

            if self.detect_check(&piece.color).is_none() {
                allowed_moves.push(*mv);
            };
            self.current = original;
        }

        moves.retain(|point| allowed_moves.contains(&point));

        if moves.is_empty() {
            None
        } else {
            Some(moves)
        }
    }

    fn get_moves_for_pawn(&self, source: &Point) -> Vec<Point> {
        let piece = self.current[source.index()].unwrap();
        if piece.kind != Kind::Pawn {
            panic!("Piece is not of kind pawn");
        };

        let direction = match piece.color {
            Color::White => Point(0, 1),
            Color::Black => Point(0, -1),
        };

        let mut moves: Vec<Point> = vec![];

        if self.at_point(&source.add(&direction)).is_none() {
            moves.push(source.add(&direction));
        };

        let plus = source.add(&direction.add(&Point(1, 0)));
        if self.is_in_bounds(&plus) {
            if let Some(target) = self.at_point(&plus) {
                if target.color != piece.color {
                    moves.push(plus);
                };
            };
        }

        let minus = source.add(&direction.add(&Point(-1, 0)));
        if self.is_in_bounds(&minus) {
            if let Some(target) = self.at_point(&minus) {
                if target.color != piece.color {
                    moves.push(minus);
                };
            };
        }

        let one_forward = source.add(&direction);
        let two_forward = one_forward.add(&direction);

        if !piece.has_moved
            && self.is_in_bounds(&one_forward)
            && self.at_point(&one_forward).is_none()
            && self.is_in_bounds(&two_forward)
            && self.at_point(&two_forward).is_none()
        {
            moves.push(two_forward);
        };

        moves
    }

    fn get_moves_for_piece(&self, source: &Point) -> Vec<Point> {
        let piece = self.current[source.index()].unwrap();
        if piece.kind == Kind::Pawn {
            return self.get_moves_for_pawn(&source);
        };

        let mut moves: Vec<Point> = vec![];

        for mv in piece.get_moves() {
            let mut current_point = source.add(&mv.0);

            while self.is_in_bounds(&current_point) {
                match self.at_point(&current_point) {
                    Some(target_piece) => {
                        if target_piece.color != piece.color {
                            moves.push(current_point);
                        }
                        break;
                    }
                    None => moves.push(current_point),
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

    fn is_enpassant_capture(&self, source: &Point, target: &Point) -> bool {
        false
    }

    fn is_pawn_double_move(&self, source: &Point, target: &Point) -> bool {
        false
    }

    fn is_castling(&self, source: &Point, target: &Point) -> bool {
        false
    }
}
