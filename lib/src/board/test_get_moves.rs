use super::tests::create_test_board;
use super::*;

#[test]
fn test_free_movement() {
    let mut board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(3, 2), Piece::new(Color::White, Kind::Rook)),
    ]);

    let possible_moves: Vec<Point> = board.get_allowed_moves(&Point(3, 2));

    let allowed_moves = vec![
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
    ];

    assert_eq!(possible_moves.as_sorted(), allowed_moves.as_sorted())
}

#[test]
fn test_movement_blocked_by_same_color() {
    let mut board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(3, 3), Piece::new(Color::White, Kind::Rook)),
        (Point(5, 3), Piece::new(Color::White, Kind::Pawn)),
        (Point(1, 3), Piece::new(Color::White, Kind::Pawn)),
        (Point(3, 5), Piece::new(Color::White, Kind::Pawn)),
        (Point(3, 1), Piece::new(Color::White, Kind::Pawn)),
    ]);

    let possible_moves: Vec<Point> = board.get_allowed_moves(&Point(3, 3));

    let allowed_moves = vec![Point(4, 3), Point(2, 3), Point(3, 4), Point(3, 2)];

    assert_eq!(possible_moves.as_sorted(), allowed_moves.as_sorted());
}

#[test]
fn test_completely_blocked() {
    let mut board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(3, 3), Piece::new(Color::White, Kind::Rook)),
        (Point(4, 3), Piece::new(Color::White, Kind::Pawn)),
        (Point(2, 3), Piece::new(Color::White, Kind::Pawn)),
        (Point(3, 4), Piece::new(Color::White, Kind::Pawn)),
        (Point(3, 2), Piece::new(Color::White, Kind::Pawn)),
    ]);

    let possible_moves: Vec<Point> = board.get_allowed_moves(&Point(3, 3));

    assert_eq!(possible_moves, vec![])
}

#[test]
fn test_blocked_by_opponent() {
    let mut board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(3, 3), Piece::new(Color::White, Kind::Rook)),
        (Point(5, 3), Piece::new(Color::Black, Kind::Pawn)),
        (Point(2, 3), Piece::new(Color::Black, Kind::Pawn)),
        (Point(3, 5), Piece::new(Color::Black, Kind::Pawn)),
        (Point(3, 2), Piece::new(Color::Black, Kind::Pawn)),
    ]);

    let possible_moves: Vec<Point> = board.get_allowed_moves(&Point(3, 3));

    let allowed_moves = vec![
        Point(4, 3),
        Point(5, 3),
        Point(2, 3),
        Point(3, 4),
        Point(3, 5),
        Point(3, 2),
    ];

    assert_eq!(possible_moves.as_sorted(), allowed_moves.as_sorted());
}

#[test]
fn test_pawn_normal_movement() {
    let mut board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(1, 8), Piece::new(Color::Black, Kind::King)),
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

    assert_eq!(board.get_allowed_moves(&Point(1, 3)), vec![Point(1, 4)]);
    assert_eq!(board.get_allowed_moves(&Point(8, 6)), vec![Point(8, 5)]);
}

#[test]
fn test_pawn_capture() {
    let mut board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(1, 8), Piece::new(Color::Black, Kind::King)),
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
        board.get_allowed_moves(&Point(3, 3)).as_sorted(),
        vec![Point(4, 4), Point(2, 4), Point(3, 4)].as_sorted()
    );
    assert_eq!(
        board.get_allowed_moves(&Point(4, 4)).as_sorted(),
        vec![Point(4, 3), Point(3, 3)].as_sorted()
    );
    assert_eq!(
        board.get_allowed_moves(&Point(2, 4)).as_sorted(),
        vec![Point(2, 3), Point(3, 3)].as_sorted()
    )
}

#[test]
fn test_pawn_has_not_moved() {
    let mut board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(1, 8), Piece::new(Color::Black, Kind::King)),
        (Point(1, 2), Piece::new(Color::White, Kind::Pawn)),
        (Point(1, 7), Piece::new(Color::Black, Kind::Pawn)),
        (Point(2, 7), Piece::new(Color::Black, Kind::Pawn)),
        (Point(2, 6), Piece::new(Color::Black, Kind::Rook)),
    ]);

    assert_eq!(
        board.get_allowed_moves(&Point(1, 2)).as_sorted(),
        vec![Point(1, 3), Point(1, 4)].as_sorted()
    );

    assert_eq!(
        board.get_allowed_moves(&Point(1, 7)).as_sorted(),
        vec![Point(1, 6), Point(1, 5)].as_sorted()
    );

    assert_eq!(board.get_allowed_moves(&Point(2, 7)), vec![]);
}

#[test]
fn test_cannot_unblock_king() {
    let mut board = create_test_board(vec![
        (Point(5, 1), Piece::new(Color::White, Kind::King)),
        (Point(4, 2), Piece::new(Color::White, Kind::Pawn)),
        (Point(3, 3), Piece::new(Color::White, Kind::Knight)),
        (Point(2, 4), Piece::new(Color::Black, Kind::Bishop)),
        (Point(5, 3), Piece::new(Color::White, Kind::Rook)),
        (Point(5, 5), Piece::new(Color::Black, Kind::Rook)),
        (Point(6, 2), Piece::new(Color::White, Kind::Knight)),
        (Point(7, 3), Piece::new(Color::Black, Kind::Bishop)),
    ]);

    assert_eq!(
        board.get_allowed_moves(&Point(5, 3)).as_sorted(),
        vec![Point(5, 2), Point(5, 4), Point(5, 5)].as_sorted()
    );

    assert_eq!(
        board.get_allowed_moves(&Point(3, 3)).as_sorted(),
        vec![
            Point(4, 5),
            Point(5, 4),
            Point(5, 2),
            Point(4, 1),
            Point(2, 1),
            Point(1, 2),
            Point(1, 4),
            Point(2, 5)
        ]
        .as_sorted()
    );

    assert_eq!(
        board.get_allowed_moves(&Point(4, 2)).as_sorted(),
        vec![Point(4, 3), Point(4, 4)].as_sorted()
    );

    assert_eq!(board.get_allowed_moves(&Point(6, 2)), vec![]);
}

#[test]
fn test_king_cannot_move_to_danger() {
    let mut board = create_test_board(vec![
        (Point(4, 4), Piece::new(Color::White, Kind::King)),
        (Point(1, 1), Piece::new(Color::Black, Kind::King)),
        (Point(5, 8), Piece::new(Color::Black, Kind::Rook)),
        (Point(8, 4), Piece::new(Color::Black, Kind::Rook)),
        (Point(1, 4), Piece::new(Color::Black, Kind::Knight)),
        (Point(2, 7), Piece::new(Color::Black, Kind::Bishop)),
    ]);

    assert_eq!(
        board.get_allowed_moves(&Point(4, 4)).as_sorted(),
        vec![Point(4, 3)].as_sorted()
    );
}

#[test]
fn test_piece_must_protect_king_when_checked() {
    let mut board = create_test_board(vec![
        (Point(3, 4), Piece::new(Color::White, Kind::King)),
        (Point(1, 1), Piece::new(Color::Black, Kind::King)),
        (Point(8, 4), Piece::new(Color::Black, Kind::Rook)),
        (Point(5, 6), Piece::new(Color::White, Kind::Rook)),
        (Point(6, 6), Piece::new(Color::White, Kind::Bishop)),
        (Point(7, 2), Piece::new(Color::White, Kind::Knight)),
    ]);

    assert_eq!(board.get_allowed_moves(&Point(5, 6)), vec![Point(5, 4)]);

    assert_eq!(
        board.get_allowed_moves(&Point(6, 6)).as_sorted(),
        vec![Point(4, 4), Point(8, 4)].as_sorted()
    );

    assert_eq!(
        board.get_allowed_moves(&Point(7, 2)).as_sorted(),
        vec![Point(8, 4), Point(6, 4)].as_sorted()
    );
}
