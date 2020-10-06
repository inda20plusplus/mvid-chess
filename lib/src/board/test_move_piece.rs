use super::tests::create_test_board;
use super::{Color, Kind, Piece, Point};
use std::collections::HashMap;

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

    assert!(board
        .graveyard
        .get(&Color::Black)
        .unwrap()
        .contains(&Piece {
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

    board.graveyard = HashMap::new();

    board.move_piece(Point(1, 1), Point(2, 1));

    assert!(board.graveyard.contains_key(&Color::Black));
    assert!(!board.graveyard.contains_key(&Color::White));
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

#[test]
fn test_enpassant_gets_set() {
    let mut board = create_test_board(vec![(Point(1, 1), Piece::new(Color::White, Kind::Pawn))]);

    board.move_piece(Point(1, 1), Point(1, 3));
    assert_eq!(board.enpassant, Some([Point(1, 2), Point(1, 3)]));
}

#[test]
fn test_enpassant_gets_unset() {
    let mut board = create_test_board(vec![(Point(1, 1), Piece::new(Color::White, Kind::Rook))]);

    board.enpassant = Some([Point(8, 6), Point(8, 5)]);
    board.move_piece(Point(1, 1), Point(1, 3));
    assert_eq!(board.enpassant, None);
}

#[test]
fn test_enpassant_removes_target_pawn() {
    let mut board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(1, 8), Piece::new(Color::Black, Kind::King)),
        (Point(1, 4), Piece {color: Color::White, kind: Kind::Pawn, has_moved: true}),
        (Point(2, 4), Piece {color: Color::Black, kind: Kind::Pawn, has_moved: true}),
        (Point(7, 5), Piece {color: Color::White, kind: Kind::Pawn, has_moved: true}),
        (Point(8, 5), Piece {color: Color::Black, kind: Kind::Pawn, has_moved: true}),
    ]);

    board.enpassant = Some([Point(1, 3), Point(1, 4)]);
    board.move_piece(Point(2, 4), Point(1, 3));
    assert_eq!(board.at_point(&Point(1, 4)), None);

    board.enpassant = Some([Point(8, 6), Point(8, 5)]);
    board.move_piece(Point(7, 5), Point(8, 4));
    assert_eq!(board.at_point(&Point(8, 5)), None);
}

#[test]
fn test_castling_king_side() {
    let mut board = create_test_board(vec![
        (Point(5, 1), Piece::new(Color::White, Kind::King)),
        (Point(8, 1), Piece::new(Color::White, Kind::Rook)),
        (Point(5, 8), Piece::new(Color::Black, Kind::King)),
        (Point(8, 8), Piece::new(Color::Black, Kind::Rook)),
    ]);

    board.move_piece(Point(5, 1), Point(7, 1));

    let rook = Piece {color: Color::White, kind: Kind::Rook, has_moved: true};
    let king = Piece {color: Color::White, kind: Kind::King, has_moved: true};
    assert_eq!(board.at_point(&Point(6, 1)), Some(rook));
    assert_eq!(board.at_point(&Point(7, 1)), Some(king));

    board.move_piece(Point(5, 8), Point(7, 8));

    let rook = Piece {color: Color::Black, kind: Kind::Rook, has_moved: true};
    let king = Piece {color: Color::Black, kind: Kind::King, has_moved: true};
    assert_eq!(board.at_point(&Point(6, 8)), Some(rook));
    assert_eq!(board.at_point(&Point(7, 8)), Some(king));
}

#[test]
fn test_castling_queen_side() {
    let mut board = create_test_board(vec![
        (Point(5, 1), Piece::new(Color::White, Kind::King)),
        (Point(1, 1), Piece::new(Color::White, Kind::Rook)),
        (Point(5, 8), Piece::new(Color::Black, Kind::King)),
        (Point(1, 8), Piece::new(Color::Black, Kind::Rook)),
    ]);

    board.move_piece(Point(5, 1), Point(3, 1));

    let rook = Piece {color: Color::White, kind: Kind::Rook, has_moved: true};
    let king = Piece {color: Color::White, kind: Kind::King, has_moved: true};
    assert_eq!(board.at_point(&Point(4, 1)), Some(rook));
    assert_eq!(board.at_point(&Point(3, 1)), Some(king));

    board.move_piece(Point(5, 8), Point(7, 8));

    let rook = Piece {color: Color::Black, kind: Kind::Rook, has_moved: true};
    let king = Piece {color: Color::Black, kind: Kind::King, has_moved: true};
    assert_eq!(board.at_point(&Point(4, 8)), Some(rook));
    assert_eq!(board.at_point(&Point(3, 8)), Some(king));
}

#[test]
fn test_castling_obstructed() {
    let mut board = create_test_board(vec![
        (Point(5, 1), Piece::new(Color::White, Kind::King)),
        (Point(1, 1), Piece::new(Color::White, Kind::Rook)),
        (Point(2, 1), Piece::new(Color::Black, Kind::Knight)),
        (Point(5, 8), Piece::new(Color::Black, Kind::King)),
        (Point(8, 8), Piece::new(Color::Black, Kind::Rook)),
        (Point(7, 8), Piece::new(Color::White, Kind::Knight)),
    ]);

    assert!(!board.move_piece(Point(5, 1), Point(3, 1)));
    assert!(!board.move_piece(Point(5, 8), Point(7, 8)));
}
