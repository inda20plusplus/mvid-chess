use super::*;
use crate::Point;

#[test]
fn test_new_piece() {
    let piece = Piece::new(Color::White, Kind::King);
    assert_eq!(
        piece,
        Piece {
            color: Color::White,
            kind: Kind::King,
            has_moved: false,
        }
    );
}

#[test]
fn test_get_moves() {
    let piece = Piece::new(Color::Black, Kind::Rook);
    assert_eq!(
        piece.get_moves(),
        vec![
            DirectionalMove(Point(0, 1), true),
            DirectionalMove(Point(-1, 0), true),
            DirectionalMove(Point(1, 0), true),
            DirectionalMove(Point(0, -1), true),
        ]
    )
}
