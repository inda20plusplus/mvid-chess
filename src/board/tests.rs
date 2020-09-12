use super::{Board, Color, Kind, Piece, Point};

#[test]
fn test_move_to_empty_and_sets_has_moved() {
    let mut board = Board {
        current: vec![(Point(1, 1), Piece::new(Color::White, Kind::Rook))]
            .into_iter()
            .collect(),
    };

    let result: bool = board.move_piece(Point(1, 1), Point(2, 1));

    assert!(result);
    assert_eq!(board.current.get(&Point(1, 1)), None);
    assert_eq!(
        board.current.get(&Point(2, 1)),
        Some(&Piece {
            color: Color::White,
            kind: Kind::Rook,
            has_moved: true,
        })
    );
}

#[test]
fn test_move_to_occupied_by_same_color() {
    let mut board = Board {
        current: vec![
            (Point(1, 1), Piece::new(Color::White, Kind::Rook)),
            (Point(2, 1), Piece::new(Color::White, Kind::Pawn)),
        ]
        .into_iter()
        .collect(),
    };

    let result: bool = board.move_piece(Point(1, 1), Point(2, 1));

    assert!(!result);
    assert_eq!(
        board.current.get(&Point(1, 1)),
        Some(&Piece {
            color: Color::White,
            kind: Kind::Rook,
            has_moved: false,
        })
    );
    assert_eq!(
        board.current.get(&Point(2, 1)),
        Some(&Piece {
            color: Color::White,
            kind: Kind::Pawn,
            has_moved: false,
        })
    );
}

#[test]
fn test_move_to_occupied_by_other_color() {
    let mut board = Board {
        current: vec![
            (Point(1, 1), Piece::new(Color::White, Kind::Rook)),
            (Point(2, 1), Piece::new(Color::Black, Kind::Pawn)),
        ]
        .into_iter()
        .collect(),
    };

    let result: bool = board.move_piece(Point(1, 1), Point(2, 1));

    assert!(result);
    assert_eq!(
        board.current.get(&Point(2, 1)),
        Some(&Piece {
            color: Color::White,
            kind: Kind::Rook,
            has_moved: true,
        })
    );

    assert!(
        board.graveyard.get(Color::Black).contains(
            &Piece {
                color: Color::Black,
                kind: Kind::Pawn,
                has_moved: true,
            }
        )
    );
}

#[test]
fn test_move_out_of_bounds() {
    let mut board = Board {
        current: vec![(Point(1, 1), Piece::new(Color::White, Kind::Rook))]
            .into_iter()
            .collect(),
    };

    assert!(!board.move_piece(Point(1, 1), Point(0, 1)));
    assert!(!board.move_piece(Point(1, 1), Point(1, 0)));
    assert!(!board.move_piece(Point(1, 1), Point(9, 1)));
    assert!(!board.move_piece(Point(1, 1), Point(1, 9)));
}
