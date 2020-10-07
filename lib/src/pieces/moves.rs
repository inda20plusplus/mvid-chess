use crate::Point;

#[derive(Clone, PartialEq, Debug)]
pub struct DirectionalMove(pub Point, pub bool);

pub const ALL: [DirectionalMove; 16] = [
    DirectionalMove(Point(-1, 1), true),
    DirectionalMove(Point(0, 1), true),
    DirectionalMove(Point(1, 1), true),
    DirectionalMove(Point(-1, 0), true),
    DirectionalMove(Point(1, 0), true),
    DirectionalMove(Point(-1, -1), true),
    DirectionalMove(Point(0, -1), true),
    DirectionalMove(Point(1, -1), true),
    DirectionalMove(Point(-2, 1), false),
    DirectionalMove(Point(-1, 2), false),
    DirectionalMove(Point(1, 2), false),
    DirectionalMove(Point(2, 1), false),
    DirectionalMove(Point(-2, -1), false),
    DirectionalMove(Point(-1, -2), false),
    DirectionalMove(Point(1, -2), false),
    DirectionalMove(Point(2, -1), false),
];

pub const KING: [DirectionalMove; 8] = [
    DirectionalMove(Point(-1, 1), false),
    DirectionalMove(Point(0, 1), false),
    DirectionalMove(Point(1, 1), false),
    DirectionalMove(Point(-1, 0), false),
    DirectionalMove(Point(1, 0), false),
    DirectionalMove(Point(-1, -1), false),
    DirectionalMove(Point(0, -1), false),
    DirectionalMove(Point(1, -1), false),
];

pub const QUEEN: [DirectionalMove; 8] = [
    DirectionalMove(Point(-1, 1), true),
    DirectionalMove(Point(0, 1), true),
    DirectionalMove(Point(1, 1), true),
    DirectionalMove(Point(-1, 0), true),
    DirectionalMove(Point(1, 0), true),
    DirectionalMove(Point(-1, -1), true),
    DirectionalMove(Point(0, -1), true),
    DirectionalMove(Point(1, -1), true),
];

pub const BISHOP: [DirectionalMove; 4] = [
    DirectionalMove(Point(-1, 1), true),
    DirectionalMove(Point(1, 1), true),
    DirectionalMove(Point(-1, -1), true),
    DirectionalMove(Point(1, -1), true),
];

pub const ROOK: [DirectionalMove; 4] = [
    DirectionalMove(Point(0, 1), true),
    DirectionalMove(Point(-1, 0), true),
    DirectionalMove(Point(1, 0), true),
    DirectionalMove(Point(0, -1), true),
];

pub const KNIGHT: [DirectionalMove; 8] = [
    DirectionalMove(Point(-2, 1), false),
    DirectionalMove(Point(-1, 2), false),
    DirectionalMove(Point(1, 2), false),
    DirectionalMove(Point(2, 1), false),
    DirectionalMove(Point(-2, -1), false),
    DirectionalMove(Point(-1, -2), false),
    DirectionalMove(Point(1, -2), false),
    DirectionalMove(Point(2, -1), false),
];

pub const PAWN: [DirectionalMove; 0] = [];
