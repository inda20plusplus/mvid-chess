use crate::{Move, Point};

pub const KING: Vec<Move> = vec![
    Move(Point(-1,1), false),
    Move(Point(0,1), false),
    Move(Point(1,1), false),
    Move(Point(-1,0), false),
    Move(Point(1,0), false),
    Move(Point(-1,-1), false),
    Move(Point(0,-1), false),
    Move(Point(1,-1), false),
];

pub const QUEEN: Vec<Move> = vec![
    Move(Point(-1,1), true),
    Move(Point(0,1), true),
    Move(Point(1,1), true),
    Move(Point(-1,0), true),
    Move(Point(1,0), true),
    Move(Point(-1,-1), true),
    Move(Point(0,-1), true),
    Move(Point(1,-1), true),
];

pub const BISHOP: Vec<Move> = vec![
    Move(Point(-1,1), true),
    Move(Point(1,1), true),
    Move(Point(-1,-1), true),
    Move(Point(1,-1), true),
];

pub const ROOK: Vec<Move> = vec![
    Move(Point(0,1), true),
    Move(Point(-1,0), true),
    Move(Point(1,0), true),
    Move(Point(0,-1), true),
];

pub const KNIGHT: Vec<Move> = vec![
    Move(Point(-2,1), false),
    Move(Point(-1,2), false),
    Move(Point(1,2), false),
    Move(Point(2,1), false),
    Move(Point(-2,-1), false),
    Move(Point(-1,-2), false),
    Move(Point(1,-2), false),
    Move(Point(2,-1), false),
];

pub const PAWN: Vec<Move> = vec![];
