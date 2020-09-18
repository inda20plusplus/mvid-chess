#[cfg(test)]
mod tests;

use crate::*;
use crate::board::Board;

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
