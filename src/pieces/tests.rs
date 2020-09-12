use super::*;

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
}
