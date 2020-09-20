#[cfg(test)]
pub mod test_setup;

#[cfg(test)]
mod test_move_piece;

#[cfg(test)]
mod test_get_moves;

use crate::pieces::{Kind, Piece};
use crate::*;
use mockall::automock;
use std::collections::HashMap;

pub struct Board {
    pub current: HashMap<Point, Piece>,
    pub graveyard: HashMap<Color, Vec<Piece>>,
    pub height: std::ops::RangeInclusive<i8>,
    pub width: std::ops::RangeInclusive<i8>,
}

#[automock]
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
                    if piece.kind == Kind::King && &piece.color == color{
                        return current_point;
                    }
                }
            }
        };
        panic!("Couldn't find king");
    }

    pub fn detect_check(&self, color: &Color) -> Option<Vec<Point>> {
        None
    }

    fn raytrace_for_kinds(
        &self,
        source: &Point,
        direction: &Point,
        color: &Color,
        kinds: Option<Vec<Kind>>,
    ) -> Option<Point> {
        let mut current_point = source.clone().add(&direction);

        let kinds = match kinds {
            Some(vec) => vec,
            None => vec![
                Kind::Bishop,
                Kind::King,
                Kind::Knight,
                Kind::Pawn,
                Kind::Queen,
                Kind::Rook,
            ],
        };

        loop {
            if !self.is_in_bounds(&current_point) {
                break None;
            } else {
                if let Some(target_piece) = self.current.get(&current_point) {
                    if kinds.contains(&target_piece.kind) && &target_piece.color == color {
                        break Some(current_point);
                    } else {
                        break None;
                    };
                };
            };

            current_point = current_point.add(&direction);
        }
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

    pub fn get_moves(&self, source: &Point) -> Vec<Point> {
        let piece = match self.current.get(&source) {
            Some(p) => p,
            None => return vec![],
        };

        let mut moves: Vec<Point>;

        if piece.kind == Kind::Pawn {
            moves = self.get_moves_for_pawn(&source);
        } else {
            moves = self.get_moves_for_piece(&source);
        };

        if piece.kind == Kind::King {
            let mut allowed_moves: Vec<Point> = vec![];

            for mv in moves.clone() {
                if !self.covered_by_opponent(&mv, &piece.color) {
                    allowed_moves.push(mv.clone());
                };
            }

            moves.retain(|point| allowed_moves.contains(&point));
        } else {
            if let Some(allowed_moves) = self.check_if_protecting_king(&source) {
                moves.retain(|point| allowed_moves.contains(&point));
            };
        };

        moves
    }

    fn covered_by_opponent(&self, source: &Point, color: &Color) -> bool {
        let opponent: Color = color.inverse();

        let straight_directions: [Point; 4] =
            [Point(1, 0), Point(0, 1), Point(-1, 0), Point(0, -1)];

        let diagonal_directions: [Point; 4] =
            [Point(1, 1), Point(-1, 1), Point(-1, -1), Point(1, -1)];

        for direction in straight_directions.iter().chain(diagonal_directions.iter()) {
            if let Some(piece) = self.current.get(&source.add(&direction)) {
                if piece.kind == Kind::King && piece.color == opponent {
                    return true;
                };
            }

            let kinds: Option<Vec<Kind>> = if straight_directions.contains(&direction) {
                Some(vec![Kind::Queen, Kind::Rook])
            } else {
                Some(vec![Kind::Queen, Kind::Bishop])
            };

            if let Some(_) = self.raytrace_for_kinds(&source, direction, &opponent, kinds) {
                return true;
            }
        }

        let knight_moves = Piece::new(color.clone(), Kind::Knight).get_moves();
        for mv in knight_moves.iter() {
            if let Some(piece) = self.current.get(&source.add(&mv.0)) {
                if piece.kind == Kind::Knight && piece.color == opponent {
                    return true;
                };
            };
        }

        let possible_pawn_pos: [Point; 2] = match opponent {
            Color::White => [Point(-1, -1), Point(1, -1)],
            Color::Black => [Point(-1, 1), Point(1, 1)],
        };

        for pos in possible_pawn_pos.iter() {
            if let Some(piece) = self.current.get(&source.add(&pos)) {
                if piece.kind == Kind::Pawn && piece.color == opponent {
                    return true;
                };
            };
        }

        false
    }


    fn check_if_protecting_king(&self, source: &Point) -> Option<Vec<Point>> {
        let source_piece = match self.current.get(&source) {
            Some(piece) => piece,
            None => return None,
        };
        let king = self.find_king(&source_piece.color);

        if let Some(direction) = king.relative_direction(&source) {
            // Check if source is the first piece in this direction
            let is_first_piece: bool =
                match self.raytrace_for_kinds(&king, &direction, &source_piece.color, None) {
                    Some(point) => &point == source,
                    None => false,
                };

            if !is_first_piece {
                None
            } else {
                // Check if source is covering the king from an opposing piece
                let kinds = if direction.0 == 0 || direction.1 == 0 {
                    Some(vec![Kind::Queen, Kind::Rook])
                } else {
                    Some(vec![Kind::Queen, Kind::Bishop])
                };

                if let Some(target_point) = self.raytrace_for_kinds(
                    &source,
                    &direction,
                    &source_piece.color.inverse(),
                    kinds,
                ) {
                    let mut blocking_points: Vec<Point> = vec![];
                    let mut current_point = king;
                    loop {
                        current_point = current_point.add(&direction);
                        blocking_points.push(current_point.clone());
                        if current_point == target_point {
                            break Some(blocking_points);
                        }
                        if !self.is_in_bounds(&current_point) {
                            panic!("Went out of bounds while checking if piece protects king")
                        }
                    }
                } else {
                    None
                }
            }
        } else {
            None
        }
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
            panic!("Piece cannot be of type Pawn");
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
