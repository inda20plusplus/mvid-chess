#[cfg(test)]
mod tests;

mod moves;
pub enum Kind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}
