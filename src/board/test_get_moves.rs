use super::test_setup::create_test_board;
use super::{Color, Kind, Piece, Point};

#[test]
fn test_free_movement() {
    let mut board = create_test_board(vec![(Point(3, 2), Piece::new(Color::White, Kind::Rook))]);

    let possible_moves: Vec<Point> = board.get_moves(Point(3, 2));

    assert_eq!(
        possible_moves.sort(),
        vec![
            Point(2, 2),
            Point(1, 2),
            Point(4, 2),
            Point(5, 2),
            Point(6, 2),
            Point(7, 2),
            Point(8, 2),
            Point(3, 1),
            Point(3, 3),
            Point(3, 4),
            Point(3, 5),
            Point(3, 6),
            Point(3, 7),
            Point(3, 8),
        ]
        .sort()
    )
}

#[test]
fn test_movement_blocked_by_same_color() {
    let mut board = create_test_board(vec![
        (Point(3, 3), Piece::new(Color::White, Kind::Rook)),
        (Point(5, 3), Piece::new(Color::White, Kind::Pawn)),
        (Point(1, 3), Piece::new(Color::White, Kind::Pawn)),
        (Point(3, 5), Piece::new(Color::White, Kind::Pawn)),
        (Point(3, 1), Piece::new(Color::White, Kind::Pawn)),
    ]);

    let possible_moves: Vec<Point> = board.get_moves(Point(3, 2));

    assert_eq!(
        possible_moves.sort(),
        vec![Point(4, 3), Point(2, 3), Point(3, 4), Point(3, 2)].sort()
    )
}

#[test]
fn test_completely_blocked() {
    let mut board = create_test_board(vec![
        (Point(3, 3), Piece::new(Color::White, Kind::Rook)),
        (Point(4, 3), Piece::new(Color::White, Kind::Pawn)),
        (Point(2, 3), Piece::new(Color::White, Kind::Pawn)),
        (Point(3, 4), Piece::new(Color::White, Kind::Pawn)),
        (Point(3, 2), Piece::new(Color::White, Kind::Pawn)),
    ]);

    let possible_moves: Vec<Point> = board.get_moves(Point(3, 2));

    assert_eq!(possible_moves, vec![])
}

#[test]
fn test_blocked_by_opponent() {
    let mut board = create_test_board(vec![
        (Point(3, 3), Piece::new(Color::White, Kind::Rook)),
        (Point(5, 3), Piece::new(Color::Black, Kind::Pawn)),
        (Point(2, 3), Piece::new(Color::Black, Kind::Pawn)),
        (Point(3, 5), Piece::new(Color::Black, Kind::Pawn)),
        (Point(3, 2), Piece::new(Color::Black, Kind::Pawn)),
    ]);

    let possible_moves: Vec<Point> = board.get_moves(Point(3, 2));

    assert_eq!(
        possible_moves.sort(),
        vec![
            Point(4, 3),
            Point(5, 3),
            Point(2, 3),
            Point(3, 4),
            Point(3, 5),
            Point(3, 2),
        ]
        .sort()
    )
}

#[test]
fn test_pawn_normal_movement() {
    let mut board = create_test_board(vec![
        (
            Point(1, 3),
            Piece {
                color: Color::White,
                kind: Kind::Pawn,
                has_moved: true,
            },
        ),
        (
            Point(8, 6),
            Piece {
                color: Color::Black,
                kind: Kind::Pawn,
                has_moved: true,
            },
        ),
    ]);

    assert_eq!(board.get_moves(Point(1, 3)), vec![Point(1, 4)]);
    assert_eq!(board.get_moves(Point(8, 6)), vec![Point(7, 6)]);
}

#[test]
fn test_pawn_capture() {
    let mut board = create_test_board(vec![
        (
            Point(3, 3),
            Piece {
                color: Color::White,
                kind: Kind::Pawn,
                has_moved: true,
            },
        ),
        (
            Point(4, 4),
            Piece {
                color: Color::Black,
                kind: Kind::Pawn,
                has_moved: true,
            },
        ),
        (
            Point(2, 4),
            Piece {
                color: Color::Black,
                kind: Kind::Pawn,
                has_moved: true,
            },
        ),
    ]);

    assert_eq!(
        board.get_moves(Point(3, 3)).sort(),
        vec![Point(4, 4), Point(2, 4), Point(3, 4)].sort()
    );
    assert_eq!(
        board.get_moves(Point(4, 4)).sort(),
        vec![Point(4, 3), Point(3, 3)].sort()
    );
    assert_eq!(
        board.get_moves(Point(2, 4)).sort(),
        vec![Point(2, 3), Point(3, 3)].sort()
    )
}

#[test]
fn test_pawn_has_not_moved() {
    let mut board = create_test_board(vec![
        (Point(1, 2), Piece::new(Color::White, Kind::Pawn)),
        (Point(1, 7), Piece::new(Color::Black, Kind::Pawn)),
    ]);

    assert_eq!(
        board.get_moves(Point(1, 2)).sort(),
        vec![Point(1, 3), Point(1, 4)].sort()
    );

    assert_eq!(
        board.get_moves(Point(1, 7)).sort(),
        vec![Point(1, 6), Point(1, 5)].sort()
    );
}
