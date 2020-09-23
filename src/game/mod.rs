use crate::board::Board;
use crate::pieces::{Kind, Piece};
use crate::*;
use std::collections::HashMap;

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
        if let Some(piece) = self.board.current.get(&source) {
            if piece.color != self.color {
                return TurnResult::Failed
            };
        };

        if self.board.get_allowed_moves(&source).contains(&target) {
            if !self.board.move_piece(source, target) {
                return TurnResult::Failed;
            }
        } else {
            return TurnResult::Failed;
        }
        
        let opponent_can_move = self.color_can_move(&self.color.inverse());

        let opponent_is_checked = match self.board.detect_check(&self.color.inverse()) {
            Some(_) => true,
            None => false,
        };

        if opponent_is_checked && opponent_can_move {
            self.color = self.color.inverse();
            return TurnResult::Checked;
        } else if opponent_is_checked && !opponent_can_move {
            return TurnResult::GameEnd(EndResult::Win(self.color.clone()));
        } else if !opponent_is_checked && !opponent_can_move {
            return TurnResult::GameEnd(EndResult::Tie);
        } else {
            self.color = self.color.inverse();
            return TurnResult::Moved;
        }
    }

    pub fn get_board(&self) -> HashMap<Point, Piece> {
        self.board.current.clone()
    }

    pub fn get_moves(&mut self, source: &Point) -> Vec<Point> {
        let mut moves = self.board.get_allowed_moves(&source);

        moves
    }

    fn color_can_move(&mut self, color: &Color) -> bool {
        for x in self.board.width.clone() {
            for y in self.board.height.clone() {
                let point = Point(x, y);
                if let Some(piece) = self.board.current.get(&point) {
                    if &piece.color == color {
                        if self.board.get_allowed_moves(&point).len() > 0 {
                            return true
                        }
                    }
                }
            }
        }
        false 
    }
}
