use super::*;
use std::collections::HashMap;

pub fn create_test_board(positions: Vec<(Point, Piece)>) -> Board {
    Board {
        current: positions.into_iter().collect(),
        graveyard: vec![(Color::White, vec![]), (Color::Black, vec![])]
            .into_iter()
            .collect(),
        height: (1..=8),
        width: (1..=8),
    }
}
