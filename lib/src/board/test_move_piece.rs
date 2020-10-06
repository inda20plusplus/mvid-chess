use super::tests::create_test_board;
use super::{Color, Kind, Piece, Point};

#[test]
fn test_move_to_empty_and_sets_has_moved() {
    let mut board = create_test_board(vec![(Point(1, 1), Piece::new(Color::White, Kind::Rook))]);

    let result: bool = board.move_piece(Point(1, 1), Point(2, 1));

    assert!(result);
    assert_eq!(board.at_point(&Point(1, 1)), None);
    assert_eq!(
        board.at_point(&Point(2, 1)),
        Some(Piece {
            color: Color::White,
            kind: Kind::Rook,
            has_moved: true,
        })
    );
}

#[test]
fn test_move_to_occupied_by_same_color() {
    let mut board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::Rook)),
        (Point(2, 1), Piece::new(Color::White, Kind::Pawn)),
    ]);

    let result: bool = board.move_piece(Point(1, 1), Point(2, 1));

    assert!(!result);
    assert_eq!(
        board.at_point(&Point(1, 1)),
        Some(Piece {
            color: Color::White,
            kind: Kind::Rook,
            has_moved: false,
        })
    );
    assert_eq!(
        board.at_point(&Point(2, 1)),
        Some(Piece {
            color: Color::White,
            kind: Kind::Pawn,
            has_moved: false,
        })
    );
}

#[test]
fn test_move_to_occupied_by_other_color() {
    let mut board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::Rook)),
        (Point(2, 1), Piece::new(Color::Black, Kind::Pawn)),
    ]);

    let result: bool = board.move_piece(Point(1, 1), Point(2, 1));

    assert!(result);
    assert_eq!(
        board.at_point(&Point(2, 1)),
        Some(Piece {
            color: Color::White,
            kind: Kind::Rook,
            has_moved: true,
        })
    );

    assert!(board.graveyard[1].contains(&Piece {
        color: Color::Black,
        kind: Kind::Pawn,
        has_moved: false,
    }));
}

#[test]
fn test_move_to_occupied_by_other_color_empty_graveyard() {
    let mut board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::Rook)),
        (Point(2, 1), Piece::new(Color::Black, Kind::Pawn)),
    ]);

    board.graveyard = [vec![], vec![]];

    board.move_piece(Point(1, 1), Point(2, 1));

    assert_eq!(board.graveyard[0].len(), 0);
    assert_eq!(board.graveyard[1].len(), 1);
}

#[test]
fn test_move_out_of_bounds() {
    let mut board = create_test_board(vec![(Point(1, 1), Piece::new(Color::White, Kind::Rook))]);

    assert!(!board.move_piece(Point(1, 1), Point(0, 1)));
    assert!(!board.move_piece(Point(1, 1), Point(1, 0)));
    assert!(!board.move_piece(Point(1, 1), Point(9, 1)));
    assert!(!board.move_piece(Point(1, 1), Point(1, 9)));
}

#[test]
fn test_move_nonexistent_piece() {
    let mut board = create_test_board(vec![]);

    assert!(!board.move_piece(Point(1, 1), Point(2, 1)));
}

#[test]
fn test_move_to_source() {
    let mut board = create_test_board(vec![(Point(1, 1), Piece::new(Color::White, Kind::Rook))]);

    assert!(!board.move_piece(Point(1, 1), Point(1, 1)));
}
