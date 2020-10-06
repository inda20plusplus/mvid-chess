use super::*;

pub fn create_test_board(positions: Vec<(Point, Piece)>) -> Board {
    let mut current: [Option<Piece>; 64] = [None; 64];
    for (point, piece) in positions {
        current[point.index()] = Some(piece);
    }
    Board {
        current,
        graveyard: [vec![], vec![]],
        height: (1..=8),
        width: (1..=8),
        enpassant: None,
    }
}

#[test]
fn test_detect_check() {
    let board = create_test_board(vec![
        (Point(4, 4), Piece::new(Color::White, Kind::King)),
        (Point(3, 4), Piece::new(Color::White, Kind::Rook)),
        (Point(2, 4), Piece::new(Color::Black, Kind::Rook)),
        (Point(7, 4), Piece::new(Color::Black, Kind::Rook)),
        (Point(5, 5), Piece::new(Color::Black, Kind::Pawn)),
        (Point(5, 3), Piece::new(Color::Black, Kind::Pawn)),
        (Point(2, 6), Piece::new(Color::Black, Kind::Bishop)),
        (Point(4, 6), Piece::new(Color::Black, Kind::Bishop)),
        (Point(7, 7), Piece::new(Color::Black, Kind::Bishop)),
        (Point(6, 5), Piece::new(Color::Black, Kind::Knight)),
    ]);

    assert_eq!(
        board.detect_check(&Color::White).unwrap().as_sorted(),
        vec![Point(2, 6), Point(5, 5), Point(6, 5), Point(7, 4)].as_sorted()
    );
}

#[test]
fn test_find_king() {
    let board = create_test_board(vec![
        (Point(3, 8), Piece::new(Color::White, Kind::King)),
        (Point(8, 3), Piece::new(Color::Black, Kind::King)),
    ]);

    assert_eq!(board.find_king(&Color::White), Point(3, 8));

    assert_eq!(board.find_king(&Color::Black), Point(8, 3));
}
