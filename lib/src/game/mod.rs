use crate::board::Board;
use crate::pieces::{Kind, Piece};
use crate::*;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug)]
pub enum EndResult {
    Win(Color),
    Tie,
}

#[derive(PartialEq, Debug)]
pub enum TurnResult {
    Moved,
    Checked,
    GameEnd(EndResult),
    Promotion,
    Failed,
}

pub struct Game {
    board: Board,
    pub color: Color,
    promotion: Option<(Point, Point)>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            color: Color::White,
            promotion: None,
        }
    }

    pub fn turn(&mut self, source: Point, target: Point) -> TurnResult {
        if let Some(_) = self.promotion {
            return TurnResult::Failed;
        };

        if let Some(moves) = self.get_moves(&source) {
            if !moves.contains(&target) {
                return TurnResult::Failed;
            }
        } else {
            return TurnResult::Failed;
        }

        if let Some(piece) = self.board.at_point(&source) {
            if piece.color != self.color {
                return TurnResult::Failed;
            } else if piece.kind == Kind::Pawn {
                let last_row = match self.color {
                    Color::White => 8,
                    Color::Black => 1,
                };
                if target.1 == last_row {
                    self.promotion = Some((source, target));
                    return TurnResult::Promotion;
                };
            };
        };

        if !self.board.move_piece(source, target) {
            return TurnResult::Failed;
        }

        let opponent_can_move = self.color_can_move(&self.color.inverse());

        let opponent_is_checked = match self.board.detect_check(&self.color.inverse()) {
            Some(_) => true,
            None => false,
        };

        if opponent_is_checked && opponent_can_move {
            self.color = self.color.inverse();
            TurnResult::Checked
        } else if opponent_is_checked && !opponent_can_move {
            TurnResult::GameEnd(EndResult::Win(self.color.clone()))
        } else if !opponent_is_checked && !opponent_can_move {
            TurnResult::GameEnd(EndResult::Tie)
        } else {
            self.color = self.color.inverse();
            TurnResult::Moved
        }
    }

    pub fn promote(&mut self, kind: Kind) -> TurnResult {
        let (source, target) = match self.promotion.clone() {
            Some(points) => (points.0, points.1),
            None => return TurnResult::Failed,
        };

        self.board.current[source.index()] = Some(Piece::new(self.color.clone(), kind));

        self.promotion = None;

        self.turn(source, target)
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_moves(&mut self, source: &Point) -> Option<Vec<Point>> {
        if let Some(piece) = &self.board.current[source.index()] {
            if piece.color != self.color {
                return None;
            }
        }

        self.board.get_allowed_moves(source)
    }

    fn color_can_move(&mut self, color: &Color) -> bool {
        for x in self.board.width.clone() {
            for y in self.board.height.clone() {
                let point = Point(x, y);
                if let Some(piece) = self.board.at_point(&point) {
                    if &piece.color == color {
                        if let Some(_) = self.board.get_allowed_moves(&point) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}
