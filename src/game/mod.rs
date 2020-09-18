#[cfg(test)]
mod tests;

use crate::board::Board;
use crate::*;

struct Game {
    board: Board,
    current_turn: Color,
}

impl Game {
    fn new() -> Self {
        Game {
        }
    }
}
