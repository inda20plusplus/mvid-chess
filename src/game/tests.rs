use super::*;
use crate::pieces::*;

pub fn create_test_board(positions: Vec<(Point, Piece)>) -> Board {
    Board {
        current: positions.into_iter().collect(),
        graveyard: vec![(Color::White, vec![]), (Color::Black, vec![])]
            .into_iter()
            .collect(),
        height: (1..=8),
        width: (1..=8),
    }
}

#[test]
fn test_turn_valid_move() {
    let mut board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(1, 8), Piece::new(Color::Black, Kind::King)),
        (Point(4, 4), Piece::new(Color::White, Kind::Rook)),
    ]);
    
    let mut game = Game {
        board: board,
        color: Color::White,
    };

    assert_eq!(
        game.turn(Point(4, 4), Point(5, 4)),
        TurnResult::Moved
    );

    assert_eq!(game.color, Color::Black);
}

#[test]
fn test_failure_on_invalid_move() {
    let mut board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(1, 8), Piece::new(Color::Black, Kind::King)),
        (Point(4, 4), Piece::new(Color::White, Kind::Rook)),
    ]);

    let mut game = Game {
        board: board,
        color: Color::Black,
    };

    assert_eq!(
        game.turn(Point(4, 4), Point(5, 5)),
        TurnResult::Failed,
    );

    assert_eq!(
        game.color,
        Color::Black
    )
}

#[test]
fn test_checks_opponent() {
    let mut board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(1, 5), Piece::new(Color::Black, Kind::King)),
        (Point(4, 4), Piece::new(Color::White, Kind::Rook)),
    ]);

    let mut game = Game {
        board: board,
        color: Color::Black,
    };

    assert_eq!(
        game.turn(Point(4, 4), Point(5, 4)),
        TurnResult::Checked,
    );

    assert_eq!(
        game.color,
        Color::White,
    );
}

#[test]
fn test_check_mate() {
    let mut board = create_test_board(vec![
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
    };


    assert_eq!(
        game.turn(Point(4, 4), Point(5, 4)),
        TurnResult::GameEnd(EndResult::Win(Color::White)),
    );
}

#[test]
fn test_tie() {
    let mut board = create_test_board(vec![
        (Point(1, 1), Piece::new(Color::White, Kind::King)),
        (Point(1, 5), Piece::new(Color::Black, Kind::King)),
        (Point(4, 4), Piece::new(Color::White, Kind::Rook)),
        (Point(3, 3), Piece::new(Color::White, Kind::Rook)),
    ]);
    
    let mut game = Game {
        board: board,
        color: Color::Black,
    };

    assert_eq!(
        game.turn(Point(3, 3), Point(2, 3)),
        TurnResult::GameEnd(EndResult::Tie)
    )
}

