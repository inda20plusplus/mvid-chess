pub mod moves;
#[cfg(test)]
mod tests;

use crate::Color;
use moves::DirectionalMove;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Piece {
    pub color: Color,
    pub kind: Kind,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(color: Color, kind: Kind) -> Self {
        Piece {
            color,
            kind,
            has_moved: false,
        }
    }

    pub fn get_moves(&self) -> Vec<DirectionalMove> {
        match self.kind {
            Kind::King => moves::KING.to_vec(),
            Kind::Queen => moves::QUEEN.to_vec(),
            Kind::Bishop => moves::BISHOP.to_vec(),
            Kind::Knight => moves::KNIGHT.to_vec(),
            Kind::Rook => moves::ROOK.to_vec(),
            Kind::Pawn => moves::PAWN.to_vec(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Kind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}
