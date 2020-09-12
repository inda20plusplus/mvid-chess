#[cfg(test)]
mod tests;

pub mod moves;
pub enum Kind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}
