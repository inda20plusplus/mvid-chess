use crate::{Move, Point};

pub const KING: [Move; 8] = [
    Move(Point(-1, 1), false),
    Move(Point(0, 1), false),
    Move(Point(1, 1), false),
    Move(Point(-1, 0), false),
    Move(Point(1, 0), false),
    Move(Point(-1, -1), false),
    Move(Point(0, -1), false),
    Move(Point(1, -1), false),
];

pub const QUEEN: [Move; 8] = [
    Move(Point(-1, 1), true),
    Move(Point(0, 1), true),
    Move(Point(1, 1), true),
    Move(Point(-1, 0), true),
    Move(Point(1, 0), true),
    Move(Point(-1, -1), true),
    Move(Point(0, -1), true),
    Move(Point(1, -1), true),
];

pub const BISHOP: [Move; 4] = [
    Move(Point(-1, 1), true),
    Move(Point(1, 1), true),
    Move(Point(-1, -1), true),
    Move(Point(1, -1), true),
];

pub const ROOK: [Move; 4] = [
    Move(Point(0, 1), true),
    Move(Point(-1, 0), true),
    Move(Point(1, 0), true),
    Move(Point(0, -1), true),
];

pub const KNIGHT: [Move; 8] = [
    Move(Point(-2, 1), false),
    Move(Point(-1, 2), false),
    Move(Point(1, 2), false),
    Move(Point(2, 1), false),
    Move(Point(-2, -1), false),
    Move(Point(-1, -2), false),
    Move(Point(1, -2), false),
    Move(Point(2, -1), false),
];

pub const PAWN: [Move; 0] = [];
