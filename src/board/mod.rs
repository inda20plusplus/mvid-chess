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
            king_pos: vec![(Color::White, Point(5, 1)), (Color::Black, Point(5, 8))]
                .into_iter()
                .collect(),
            height: (1..=8),
            width: (1..=8),
        }
    }

    pub fn move_piece(&mut self, source: Point, target: Point) -> bool {
        if !self.width.contains(&target.0) || !self.height.contains(&target.1) {
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

    pub fn get_moves(&self, source: Point) -> Vec<Point> {
        let piece = match self.current.get(&source) {
            Some(p) => p,
            None => return vec![]
        };

        let mut moves: Vec<Point>;

        if piece.kind == Kind::Pawn {
            moves = self.get_moves_for_pawn(&source);
        } else {
            moves = self.get_moves_for_piece(&source);
        }

        if piece.kind == Kind::King {

        } else {
            if let Some(allowed_moves) = self.check_if_protecting_king(&source, &piece.color) {
                moves.retain(|point| allowed_moves.contains(&point));
            }
        };

        moves
    }

    fn check_if_protecting_king(&self, source: &Point, color: &Color) -> Option<Vec<Point>> {
        let king = self.king_pos.get(&color).unwrap();

        if let Some(direction) = king.relative_direction(&source) {
            let mut current_position = source.add(&direction);
            let mut passed_points: Vec<Point> = vec![];
            loop {
                passed_points.push(current_position.clone());
                if !self.current.contains_key(&current_position) {
                    if !self.width.contains(&current_position.0) || !self.height.contains(&current_position.1) {
                        return None
                    } else {
                        current_position = current_position.add(&direction);
                        continue
                    };
                } else {
                    let target_piece = self.current.get(&current_position).unwrap();
                    if &target_piece.color == color {
                        return None
                    } else {
                        if direction.0 == 0 || direction.1 == 0 {
                            if [Kind::Queen, Kind::Rook].contains(&target_piece.kind) {
                                return Some(passed_points)
                            } else {
                                return None
                            };
                        } else {
                            if [Kind::Queen, Kind::Bishop].contains(&target_piece.kind) {
                                return Some(passed_points)
                            } else {
                                return None
                            };
                        };
                    };
                };
            };
        } else {
            return None
        };
    }

    fn get_moves_for_pawn(&self, source: &Point) -> Vec<Point> {
        let piece = self.current.get(&source).unwrap();
        if piece.kind != Kind::Pawn {
            panic!("Piece is not of kind pawn");
        };

        let direction = match piece.color {
            Color::White => Point(0, 1),
            Color::Black => Point(0, -1)
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

        if !piece.has_moved && !self.current.contains_key(&source.add(&direction).add(&direction)) {
            moves.push(source.add(&direction).add(&direction));
        };

        moves
    }

    fn get_moves_for_piece(&self, source: &Point) -> Vec<Point> {
        let piece = self.current.get(&source).unwrap();
        if piece.kind == Kind::Pawn {
            panic!("Piece cannot be of type Pawn");
        };

        let mut moves: Vec<Point> = vec![];
        
        for mv in piece.get_moves() {
            let mut current_point = source.add(&mv.0);
            loop {
                if !self.current.contains_key(&current_point) {
                    if !self.width.contains(&current_point.0) || !self.height.contains(&current_point.1) {
                        break
                    }
                    moves.push(current_point.clone());
                } else {
                    let target_piece = self.current.get(&current_point).unwrap();
                    if target_piece.color != piece.color {
                        moves.push(current_point);
                    }
                    break
                }

                if mv.1 {
                    current_point = current_point.add(&mv.0);
                } else {
                    break
                };
            }
        }

        moves
    }
}
