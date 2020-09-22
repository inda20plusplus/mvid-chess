use crate::*;
use crate::board::Board;

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
    Failed,
}

pub struct Game {
    pub board: Board,
    pub color: Color,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            color: Color::White,
        }
    }

    pub fn turn(&mut self, source: Point, target: Point) -> TurnResult {
        // Check if move is valid, return TurnResult::Failed
        // Make move
        // Set turn to opposing player
        // set self.checked to contain points of any pieces checking the current color
        // Check for game over
        // May be reordered and changed ^
        TurnResult::Moved
    }

    pub fn get_moves(&self, source: Point) -> Option<Vec<Point>> {
        // return Some(vector of points that source can move to)
        // returns None if there is no piece there
        None
    }
}
