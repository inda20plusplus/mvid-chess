mod board;
mod pieces;

struct Point(i8, i8);

pub enum Color {
    White,
    Black,
}

pub struct Move(Point, bool);
