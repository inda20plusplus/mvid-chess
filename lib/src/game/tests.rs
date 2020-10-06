use super::*;
use crate::pieces::*;

pub fn create_test_board(positions: Vec<(Point, Piece)>) -> Board {
    let mut current: [Option<Piece>; 64] = [None; 64];
    for (point, piece) in positions {
        current[point.index()] = Some(piece);
    }

    Board {
        current,
        graveyard: vec![(Color::White, vec![]), (Color::Black, vec![])]
            .into_iter()
            .collect(),
        height: (1..=8),
        width: (1..=8),
        enpassant: None,
    }
}

#[test]
fn test_turn_valid_move() {
    let board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(1, 8), Piece::new(Color::Black, Kind::King)),
        (Point(4, 4), Piece::new(Color::White, Kind::Rook)),
    ]);

    let mut game = Game {
        board: board,
        color: Color::White,
        promotion: None,
    };

    assert_eq!(game.turn(Point(4, 4), Point(5, 4)), TurnResult::Moved);

    assert_eq!(game.color, Color::Black);
}

#[test]
fn test_failure_on_invalid_move() {
    let board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(1, 8), Piece::new(Color::Black, Kind::King)),
        (Point(4, 4), Piece::new(Color::White, Kind::Rook)),
    ]);

    let mut game = Game {
        board: board,
        color: Color::White,
        promotion: None,
    };

    assert_eq!(game.turn(Point(4, 4), Point(5, 5)), TurnResult::Failed,);

    assert_eq!(game.color, Color::White)
}

#[test]
fn test_failure_on_moving_wrong_color() {
    let board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(1, 8), Piece::new(Color::Black, Kind::King)),
        (Point(4, 4), Piece::new(Color::White, Kind::Rook)),
    ]);

    let mut game = Game {
        board: board,
        color: Color::Black,
        promotion: None,
    };

    assert_eq!(game.turn(Point(4, 4), Point(4, 5)), TurnResult::Failed);

    assert_eq!(game.color, Color::Black)
}

#[test]
fn test_checks_opponent() {
    let board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(1, 5), Piece::new(Color::Black, Kind::King)),
        (Point(4, 4), Piece::new(Color::White, Kind::Rook)),
    ]);

    let mut game = Game {
        board: board,
        color: Color::White,
        promotion: None,
    };

    assert_eq!(game.turn(Point(4, 4), Point(4, 5)), TurnResult::Checked);

    assert_eq!(game.color, Color::Black);
}

#[test]
fn test_check_mate() {
    let board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(1, 5), Piece::new(Color::Black, Kind::King)),
        (Point(8, 8), Piece::new(Color::Black, Kind::Rook)),
        (Point(4, 4), Piece::new(Color::White, Kind::Rook)),
        (Point(5, 4), Piece::new(Color::White, Kind::Rook)),
        (Point(4, 6), Piece::new(Color::White, Kind::Rook)),
    ]);

    let mut game = Game {
        board: board,
        color: Color::White,
        promotion: None,
    };

    assert_eq!(
        game.turn(Point(4, 4), Point(4, 5)),
        TurnResult::GameEnd(EndResult::Win(Color::White)),
    );
}

#[test]
fn test_tie() {
    let board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(1, 5), Piece::new(Color::Black, Kind::King)),
        (Point(4, 6), Piece::new(Color::White, Kind::Rook)),
        (Point(4, 4), Piece::new(Color::White, Kind::Rook)),
        (Point(3, 3), Piece::new(Color::White, Kind::Rook)),
    ]);

    let mut game = Game {
        board: board,
        color: Color::White,
        promotion: None,
    };

    assert_eq!(
        game.turn(Point(3, 3), Point(2, 3)),
        TurnResult::GameEnd(EndResult::Tie)
    )
}

#[test]
fn test_promotion() {
    let board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(1, 8), Piece::new(Color::Black, Kind::King)),
        (Point(8, 7), Piece::new(Color::White, Kind::Pawn)),
        (Point(8, 2), Piece::new(Color::Black, Kind::Pawn)),
    ]);

    let mut game = Game {
        board: board,
        color: Color::White,
        promotion: None,
    };

    assert_eq!(game.turn(Point(8, 7), Point(8, 8)), TurnResult::Promotion);
    assert_eq!(game.promotion, Some((Point(8, 7), Point(8, 8))));
    assert_eq!(game.color, Color::White);
    assert_eq!(game.promote(Kind::Queen), TurnResult::Checked);
    assert_eq!(game.color, Color::Black);

    game.board.move_piece(Point(1, 8), Point(1, 7));

    assert_eq!(game.turn(Point(8, 2), Point(8, 1)), TurnResult::Promotion);
    assert_eq!(game.promotion, Some((Point(8, 2), Point(8, 1))));
    assert_eq!(game.color, Color::Black);
    assert_eq!(game.promote(Kind::Queen), TurnResult::Checked);
    assert_eq!(game.color, Color::White);
}
