#[cfg(test)]
mod tests;

use crate::*;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        use crate::board::MockBoard as Board;
    } else {
        use crate::board::Board;
    }
}

pub enum EndResult {
    Win(Color),
    Tie,
}

pub enum TurnResult {
    Moved,
    Checked,
    GameEnd(EndResult),
    Failed
}

pub struct Game {
    pub board: Board,
    pub turn: Color,
    pub checked: Option<Vec<Point>>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            turn: Color::White,
            checked: None,
        }
    }

    pub fn turn(&self, source: Point, target: Point) -> TurnResult {
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
